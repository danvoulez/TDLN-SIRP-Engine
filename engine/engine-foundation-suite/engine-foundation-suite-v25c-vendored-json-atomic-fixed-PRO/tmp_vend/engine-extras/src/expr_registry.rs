
use engine_core::providers::ExprEval;
use engine_core::model::Expression;
use anyhow::{Result, anyhow};
use serde_json::Value as Json;
use std::collections::HashMap;

pub trait FnRegistry: Send + Sync { fn call(&self, name:&str, args:&[Json]) -> Result<Json>; }

pub struct ExtensibleExpr<R: FnRegistry> { pub reg: R }

impl<R:FnRegistry> ExprEval for ExtensibleExpr<R> {
    fn eval(&self, expr:&Expression, ctx:&Json) -> Result<Json> {
        use engine_core::providers::DefaultExpr;
        match expr {
            Expression::FunctionCall{function, arguments} => {
                let args: Vec<_> = arguments.iter().map(|a| ExtensibleExpr{ reg: self.reg }.eval(a, ctx)).collect::<Result<_>>()?;
                self.reg.call(function, &args)
            },
            _ => DefaultExpr{}.eval(expr, ctx)
        }
    }
}

pub struct BasicRegistry { fns: HashMap<String, fn(&[Json])->Result<Json>> }
impl BasicRegistry {
    pub fn new() -> Self {
        let mut fns: HashMap<String, fn(&[Json])->Result<Json>> = HashMap::new();
        fns.insert("starts_with".into(), |a| {
            let (s,p) = (a.get(0).and_then(|v| v.as_str()).unwrap_or(""), a.get(1).and_then(|v| v.as_str()).unwrap_or(""));
            Ok(Json::Bool(s.starts_with(p)))
        });
        fns.insert("ends_with".into(), |a| {
            let (s,p) = (a.get(0).and_then(|v| v.as_str()).unwrap_or(""), a.get(1).and_then(|v| v.as_str()).unwrap_or(""));
            Ok(Json::Bool(s.ends_with(p)))
        });
        fns.insert("in_set".into(), |a| {
            let s = a.get(0).cloned().unwrap_or(Json::Null);
            let set = a.get(1).and_then(|v| v.as_array()).cloned().unwrap_or_default();
            Ok(Json::Bool(set.contains(&s)))
        });
        Self{ fns }
    }
}
impl Default for BasicRegistry { fn default()->Self{ Self::new() } }
impl FnRegistry for BasicRegistry {
    fn call(&self, name:&str, args:&[Json]) -> Result<Json> {
        if let Some(f)=self.fns.get(name) { Ok(f(args)?) } else { Err(anyhow!("Unknown function: {name}")) }
    }
}
