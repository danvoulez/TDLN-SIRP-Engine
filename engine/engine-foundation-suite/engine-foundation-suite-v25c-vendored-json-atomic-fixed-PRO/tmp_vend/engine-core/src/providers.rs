
use serde_json::Value as Json;
use anyhow::{Result, anyhow};
use crate::model::*;

pub trait CanonProvider: Send + Sync { fn canon(&self, v:&Json) -> Vec<u8>; }
pub trait CidProvider: Send + Sync { fn cid(&self, bytes:&[u8]) -> String; }

pub trait Clock: Send + Sync { fn now_rfc3339(&self) -> String; }
pub trait IdGen: Send + Sync { fn new_ulid(&self) -> String; }

pub trait Signer: Send + Sync {
  fn sign(&self, msg:&[u8]) -> Option<Vec<u8>>;
  fn kid(&self) -> Option<String> { None }
}

pub trait ExprEval: Send + Sync {
  fn eval(&self, expr:&Expression, ctx:&Json) -> Result<Json>;
}

pub trait AggregatorStrategy: Send + Sync {
  fn aggregate(&self, wiring:&Wiring, decisions:&[PolicyDecision]) -> Decision;
}

pub trait ReceiptSink: Send + Sync {
  fn emit(&self, receipt:&crate::model::ExecutionReceipt) -> Result<()>;
}

// Defaults
pub struct DefaultCanon;
impl CanonProvider for DefaultCanon {
    fn canon(&self, v:&Json)->Vec<u8> {
        fn sort(v:&Json)->Json {
            match v {
                Json::Array(a) => Json::Array(a.iter().map(sort).collect()),
                Json::Object(m) => {
                    let mut ks: Vec<_> = m.keys().cloned().collect();
                    ks.sort();
                    let mut out = serde_json::Map::new();
                    for k in ks { out.insert(k.clone(), sort(&m[&k])); }
                    Json::Object(out)
                },
                _ => v.clone()
            }
        }
        serde_json::to_string(&sort(v)).unwrap().into_bytes()
    }
}
pub struct DefaultCid;
impl CidProvider for DefaultCid {
    fn cid(&self, b:&[u8])->String { format!("b3:{}", blake3::hash(b).to_hex()) }
}
pub struct SysClock;
impl Clock for SysClock { fn now_rfc3339(&self)->String { chrono::Utc::now().to_rfc3339() } }
pub struct UlidGen;
impl IdGen for UlidGen { fn new_ulid(&self)->String { ulid::Ulid::new().to_string() } }
pub struct NoopSigner;
impl Signer for NoopSigner { fn sign(&self,_:&[u8])->Option<Vec<u8>>{ None } }
pub struct DefaultExpr;
impl DefaultExpr {
    fn as_bool(v:&Json)->Result<bool>{
        Ok(match v {
            Json::Bool(b)=>*b,
            Json::Null=>false,
            Json::Number(n)=>n.as_f64().map(|f| f!=0.0 && f.is_finite()).unwrap_or(false),
            Json::String(s)=>!s.is_empty(),
            Json::Array(a)=>!a.is_empty(),
            Json::Object(o)=>!o.is_empty(),
        })
    }
    fn as_number(v:&Json)->Result<f64>{
        match v {
            Json::Number(n) if n.is_f64()=>Ok(n.as_f64().unwrap()),
            Json::Number(n)=> Ok(n.as_i64().map(|i| i as f64).or_else(|| n.as_u64().map(|u| u as f64)).ok_or_else(|| anyhow!("Invalid number"))?),
            Json::String(s)=> s.parse::<f64>().map_err(|_| anyhow!("Cannot parse as number: {s}")),
            Json::Bool(b)=> Ok(if *b {1.0} else {0.0}),
            _ => Err(anyhow!("Cannot convert to number: {v:?}")),
        }
    }
}
impl ExprEval for DefaultExpr {
    fn eval(&self, expr:&Expression, ctx:&Json)->Result<Json>{
        use Expression::*; use Operator::*;
        match expr {
            Literal{value} => Ok(value.clone()),
            ContextRef{path, fallback} => {
                let mut cur = ctx;
                for k in path { match cur.get(k){ Some(v)=>cur=v, None=> return Ok(fallback.clone().unwrap_or(Json::Null)) } }
                Ok(cur.clone())
            },
            Binary{operator, left, right} => {
                let l = self.eval(left, ctx)?; let r = self.eval(right, ctx)?;
                Ok(match operator {
                    And => Json::Bool(Self::as_bool(&l)? && Self::as_bool(&r)?),
                    Or  => Json::Bool(Self::as_bool(&l)? || Self::as_bool(&r)?),
                    Eq  => Json::Bool(l==r),
                    Neq => Json::Bool(l!=r),
                    Gt  => Json::Bool(Self::as_number(&l)? >  Self::as_number(&r)?),
                    Lt  => Json::Bool(Self::as_number(&l)? <  Self::as_number(&r)?),
                    Gte => Json::Bool(Self::as_number(&l)? >= Self::as_number(&r)?),
                    Lte => Json::Bool(Self::as_number(&l)? <= Self::as_number(&r)?),
                    In  => {
                        match &r {
                            Json::Array(a)=> Json::Bool(a.contains(&l)),
                            Json::String(s)=> Json::Bool(l.as_str().map(|t| s.contains(t)).unwrap_or(false)),
                            _=> Json::Bool(false)
                        }
                    },
                    _ => return Err(anyhow!("Invalid binary operator")),
                })
            },
            Unary{operator, argument} => {
                let a = self.eval(argument, ctx)?;
                Ok(match operator {
                    Not => Json::Bool(!Self::as_bool(&a)?),
                    Exists => Json::Bool(!a.is_null()),
                    _ => return Err(anyhow!("Invalid unary operator")),
                })
            },
            FunctionCall{function, arguments} => {
                let args: Vec<_> = arguments.iter().map(|a| self.eval(a, ctx)).collect::<Result<_>>()?;
                match function.as_str() {
                    "length" => {
                        if let Some(s)=args.first().and_then(|v| v.as_str()){ Json::Number((s.len() as u64).into()) }
                        else if let Some(a)=args.first().and_then(|v| v.as_array()){ Json::Number((a.len() as u64).into()) }
                        else { Json::Number(0u64.into()) }
                    },
                    "is_string" => Json::Bool(args.first().map(|v| v.is_string()).unwrap_or(false)),
                    "is_number" => Json::Bool(args.first().map(|v| v.is_number()).unwrap_or(false)),
                    _ => return Err(anyhow!("Unknown function: {function}")),
                }.pipe(Ok)
            },
            Conditional{test, consequent, alternate} => {
                if Self::as_bool(&self.eval(test, ctx)?)? { self.eval(consequent, ctx) } else { self.eval(alternate, ctx) }
            }
        }
    }
}
trait Pipe<T>{ fn pipe<F:FnOnce(T)->R, R>(self, f:F)->R; }
impl<T> Pipe<T> for T { fn pipe<F:FnOnce(T)->R, R>(self, f:F)->R { f(self) } }

pub struct DefaultAggregator;
impl AggregatorStrategy for DefaultAggregator {
  fn aggregate(&self, wiring:&Wiring, decisions:&[PolicyDecision])->Decision {
      use Decision::*;
      let map: std::collections::HashMap<_,_> = decisions.iter().filter(|d| !d.skipped).map(|d| (d.policy_id.clone(), d.decision.clone())).collect();
      match wiring {
          Wiring::All{policies} => {
              let rel: Vec<_> = policies.iter().filter_map(|id| map.get(id)).collect();
              if rel.is_empty() { return Deny; }
              if rel.iter().all(|d| **d==Allow) { Allow }
              else if rel.iter().any(|d| **d==Doubt) { Doubt }
              else { Deny }
          },
          Wiring::Any{policies} => {
              let rel: Vec<_> = policies.iter().filter_map(|id| map.get(id)).collect();
              if rel.is_empty() { return Deny; }
              if rel.iter().any(|d| **d==Allow) { Allow }
              else if rel.iter().any(|d| **d==Doubt) { Doubt }
              else { Deny }
          },
          Wiring::Sequential{policies} => {
              for id in policies {
                  if let Some(d)=map.get(id) {
                      match d { Allow=>{}, Doubt=> return Doubt, Deny=> return Deny }
                  }
              }
              Allow
          },
          Wiring::Majority{policies} => {
              let rel: Vec<_> = policies.iter().filter_map(|id| map.get(id)).collect();
              if rel.is_empty() { return Deny; }
              let allows = rel.iter().filter(|d| ***d==Allow).count();
              if allows > rel.len()/2 { Allow }
              else if rel.iter().any(|d| **d==Doubt) { Doubt }
              else { Deny }
          },
          Wiring::Weighted{policies, weights, threshold} => {
              if policies.len()!=weights.len(){ return Deny; }
              let mut sum=0.0;
              for (id,w) in policies.iter().zip(weights.iter()) {
                  match map.get(id) {
                      Some(Decision::Allow)=> sum += *w,
                      _ => {}
                  }
              }
              if sum >= *threshold { Allow } else { Deny }
          },
      }
  }
}

pub struct NoopSink;
impl ReceiptSink for NoopSink { fn emit(&self, _:&crate::model::ExecutionReceipt)->Result<()> { Ok(()) } }
