
use serde::{Serialize, Deserialize};
use serde_json::Value as Json;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Decision { Allow, Deny, Doubt }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub policy_id: String,
    pub policy_hash: String,
    pub decision: Decision,
    pub evaluation_ns: u64,
    pub error: Option<String>,
    pub skipped: bool,
    pub missing_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator { And, Or, Eq, Neq, Gt, Lt, Gte, Lte, In, Not, Exists }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="type")]
pub enum Expression {
    Literal{ value: Json },
    ContextRef{ path: Vec<String>, fallback: Option<Json> },
    Binary{ operator: Operator, left: Box<Expression>, right: Box<Expression> },
    Unary{ operator: Operator, argument: Box<Expression> },
    FunctionCall{ function: String, arguments: Vec<Expression> },
    Conditional{ test: Box<Expression>, consequent: Box<Expression>, alternate: Box<Expression> },
}
impl Expression {
    pub fn literal<T: Into<Json>>(v:T)->Self { Self::Literal{ value: v.into() } }
    pub fn context(path:&[&str])->Self { Self::ContextRef{ path: path.iter().map(|s| s.to_string()).collect(), fallback: None } }
    pub fn eq(l:Expression, r:Expression)->Self { Self::Binary{ operator:Operator::Eq, left:Box::new(l), right:Box::new(r) } }
    pub fn gt(l:Expression, r:Expression)->Self { Self::Binary{ operator:Operator::Gt, left:Box::new(l), right:Box::new(r) } }
    pub fn not(a:Expression)->Self { Self::Unary{ operator:Operator::Not, argument:Box::new(a) } }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Effect { Read, Write, Network, Filesystem, Process, Wasm }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope { pub allow: Vec<String>, pub deny: Vec<String> }
impl Scope {
    pub fn permits(&self, path:&str)->bool {
        fn m(p:&str, g:&str)->bool {
            glob::Pattern::new(g).map(|pat| pat.matches(p)).unwrap_or(false)
        }
        if self.deny.iter().any(|g| m(path,g)) { return false; }
        self.allow.iter().any(|g| m(path,g))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineMode {
    pub enabled_effects: std::collections::HashSet<Effect>,
    pub scopes: HashMap<Effect, Scope>,
    pub active_policies: std::collections::HashSet<String>,
}
impl EngineMode {
    pub fn conservative()->Self {
        use Effect::*;
        let mut e = HashSet::new(); e.insert(Read);
        Self{ enabled_effects:e, scopes:HashMap::new(), active_policies:HashSet::new() }
    }
    pub fn allows(&self, eff:Effect)->bool { self.enabled_effects.contains(&eff) }
    pub fn allows_all(&self, req:&[Effect])->bool { req.iter().all(|e| self.enabled_effects.contains(e)) }
    pub fn is_policy_active(&self, id:&str)->bool { self.active_policies.is_empty() || self.active_policies.contains(id) }
    pub fn permits(&self, eff:Effect, path:Option<&str>)->bool {
        if !self.allows(eff.clone()) { return false; }
        if let Some(p)=path { if let Some(sc)=self.scopes.get(&eff){ return sc.permits(p); } }
        true
    }
    pub fn is_public_safe(&self)->bool {
        !self.enabled_effects.contains(&Effect::Process) && !self.enabled_effects.contains(&Effect::Wasm)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyBit {
    pub id: String,
    pub title: Option<String>,
    pub hash: Option<String>,
    pub condition: Expression,
    #[serde(default)]
    pub required_fields: Vec<Vec<String>>,
}
impl PolicyBit {
    pub fn new(id:&str, title:&str)->Self {
        Self{ id:id.into(), title:Some(title.into()), hash:None, condition:Expression::literal(true), required_fields:vec![] }
    }
    pub fn condition(mut self, e:Expression)->Self { self.condition=e; self }
    pub fn requires(mut self, path:&[&str])->Self { self.required_fields.push(path.iter().map(|s| s.to_string()).collect()); self }
    pub fn build(self)->Self { self }
    pub fn check_required_fields(&self, ctx:&Json)->Vec<Vec<String>> {
        let mut miss = vec![];
        for p in &self.required_fields {
            let mut cur = ctx;
            let mut ok = true;
            for k in p {
                if let Some(v)=cur.get(k){ cur = v; } else { ok=false; break; }
            }
            if !ok { miss.push(p.clone()); }
        }
        miss
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Wiring {
    All{ policies: Vec<String> },
    Any{ policies: Vec<String> },
    Sequential{ policies: Vec<String> },
    Majority{ policies: Vec<String> },
    Weighted{ policies: Vec<String>, weights: Vec<f64>, threshold: f64 },
}
impl Wiring { pub fn ids(&self)->Vec<String> {
    match self {
        Wiring::All{policies}|Wiring::Any{policies}|Wiring::Sequential{policies}|Wiring::Majority{policies}|Wiring::Weighted{policies,..} => policies.clone(),
    }
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticChip {
    pub id: String,
    pub name: Option<String>,
    pub policies: Vec<PolicyBit>,
    pub wiring: Wiring,
    pub required_effects: Vec<Effect>,
    pub hash: Option<String>,
}
impl SemanticChip {
    pub fn builder(id:&str)->Builder { Builder{ chip: Self{ id:id.into(), name:None, policies:vec![], wiring: Wiring::All{policies:vec![]}, required_effects:vec![], hash:None } } }
    pub fn simple(id:&str, pols:Vec<PolicyBit>, wiring:Wiring)->Self { Self{ id:id.into(), name:None, policies:pols, wiring, required_effects:vec![], hash:None } }
    pub fn with_required_effects(mut self, e:Vec<Effect>)->Self { self.required_effects = e; self }
}
pub struct Builder{ chip: SemanticChip }
impl Builder {
    pub fn name(mut self, n:&str)->Self { self.chip.name=Some(n.into()); self }
    pub fn policy(mut self, p:PolicyBit)->Self { self.chip.policies.push(p); self }
    pub fn wiring(mut self, w:Wiring)->Self { self.chip.wiring=w; self }
    pub fn build(self)->SemanticChip { self.chip }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonSlot { pub raw: Json, pub canon: Json, pub cid: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingInfo {
    pub id: String,
    pub reason: String,
    pub missing_fields: Vec<String>,
    pub missing_evidence: Vec<String>,
    pub resolution_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub hash_chain: Vec<String>,
    pub signature: Option<String>, // base64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub chip_id: String,
    pub chip_hash: String,
    pub mode: EngineMode,
    pub input: CanonSlot,
    pub policy_decisions: Vec<PolicyDecision>,
    pub output: CanonSlot,
    pub decision: Decision,
    pub missing: Option<MissingInfo>,
    pub proof: Proof,
    pub timestamp: String,
    pub duration_ns: u64,
}
