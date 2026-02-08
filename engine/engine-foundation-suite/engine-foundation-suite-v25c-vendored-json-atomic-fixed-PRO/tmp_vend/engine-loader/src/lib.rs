
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use parking_lot::RwLock;
use std::sync::Arc;
use engine_core::{AtomicUnit};
use engine_core::model::{PolicyBit, Wiring, Expression, Aggregator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitSpec {
    pub id: String,
    pub description: Option<String>,
    pub policies: Vec<PolicySpec>,
    pub wiring: WiringSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicySpec {
    pub id: String,
    pub description: Option<String>,
    pub requires: Option<Vec<Vec<String>>>, // list of JSON pointer paths split
    pub condition: ExprSpec,
    pub fallback: Option<String>, // "Allow"|"Deny"|"Doubt"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="kind", rename_all="snake_case")]
pub enum ExprSpec {
    Literal { value: serde_json::Value },
    ContextRef { path: Vec<String> },
    Binary { operator: String, left: Box<ExprSpec>, right: Box<ExprSpec> },
    Unary { operator: String, argument: Box<ExprSpec> },
    FunctionCall { function: String, arguments: Vec<ExprSpec> },
    Conditional { test: Box<ExprSpec>, consequent: Box<ExprSpec>, alternate: Box<ExprSpec> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="type", rename_all="snake_case")]
pub enum WiringSpec {
    All { policies: Vec<String> },
    Any { policies: Vec<String> },
    Majority { policies: Vec<String> },
    Sequential { policies: Vec<String> },
    Weighted { policies: Vec<String>, weights: Vec<f64>, threshold: f64 },
    Graph { nodes: Vec<String>, aggregator: String }
}

fn expr_from_spec(s:&ExprSpec) -> Result<Expression> {
    use Expression as E;
    Ok(match s {
        ExprSpec::Literal{ value } => E::Literal{ value: value.clone() },
        ExprSpec::ContextRef{ path } => E::ContextRef{ path: path.clone(), fallback: None },
        ExprSpec::Binary{ operator, left, right } => {
            let op = match operator.as_str() {
                "and"=>"And","or"=>"Or","eq"=>"Eq","neq"=>"Neq","gt"=>"Gt","lt"=>"Lt","gte"=>"Gte","lte"=>"Lte","in"=>"In",
                other => return Err(anyhow!("unknown binary operator {other}"))
            };
            E::Binary{ operator: op.parse().unwrap_or_default(), left: Box::new(expr_from_spec(left)?), right: Box::new(expr_from_spec(right)?)} }
        ExprSpec::Unary{ operator, argument } => {
            let op = match operator.as_str() { "not"=>"Not","exists"=>"Exists", other=> return Err(anyhow!("unknown unary operator {other}")) };
            E::Unary{ operator: op.parse().unwrap_or_default(), argument: Box::new(expr_from_spec(argument)?)} }
        ExprSpec::FunctionCall{ function, arguments } => {
            E::FunctionCall{ function: function.clone(), arguments: arguments.iter().map(|a| expr_from_spec(a)).collect::<Result<Vec<_>>>()? }
        }
        ExprSpec::Conditional{ test, consequent, alternate } => {
            E::Conditional{ test: Box::new(expr_from_spec(test)?), consequent: Box::new(expr_from_spec(consequent)?), alternate: Box::new(expr_from_spec(alternate)?)}}
    })
}

fn wiring_from_spec(w:&WiringSpec)->Result<Wiring>{
    Ok(match w {
        WiringSpec::All{ policies } => Wiring::All{ policies: policies.clone() },
        WiringSpec::Any{ policies } => Wiring::Any{ policies: policies.clone() },
        WiringSpec::Majority{ policies } => Wiring::Majority{ policies: policies.clone() },
        WiringSpec::Sequential{ policies } => Wiring::Sequential{ policies: policies.clone() },
        WiringSpec::Weighted{ policies, weights, threshold } => Wiring::Weighted{ policies: policies.clone(), weights: weights.clone(), threshold: *threshold },
        WiringSpec::Graph{ nodes, aggregator } => {
            let agg = match aggregator.as_str(){ "all"=>Aggregator::All, "any"=>Aggregator::Any, "majority"=>Aggregator::Majority, "first"=>Aggregator::First, "last"=>Aggregator::Last, _=>Aggregator::All };
            Wiring::Graph{ nodes: nodes.clone(), aggregator: agg, edges: vec![] }
        }
    })
}

pub fn unit_from_spec(spec:&UnitSpec)->Result<AtomicUnit>{
    let mut b = engine_core::model::SemanticChip::builder(&spec.id);
    for p in &spec.policies {
        let mut pb = PolicyBit::new(&p.id, p.description.clone().unwrap_or_default());
        pb = pb.condition(expr_from_spec(&p.condition)?);
        if let Some(reqs) = &p.requires {
            for r in reqs {
                pb = pb.requires(&r.iter().map(|s| s.as_str()).collect::<Vec<_>>());
            }
        }
        if let Some(fb) = &p.fallback {
            let f = match fb.as_str(){ "Allow"=>engine_core::model::Decision::Allow, "Deny"=>engine_core::model::Decision::Deny, _=>engine_core::model::Decision::Doubt };
            pb = pb.fallback(f);
        }
        b = b.policy(pb.build());
    }
    b = b.wiring(wiring_from_spec(&spec.wiring)?);
    Ok(b.build())
}

#[derive(Clone)]
pub struct UnitStore {
    inner: Arc<RwLock<Vec<AtomicUnit>>>,
    pub dir: PathBuf,
}

impl UnitStore {
    pub fn new<P: AsRef<Path>>(dir:P)->Self {
        Self{ inner: Arc::new(RwLock::new(Vec::new())), dir: dir.as_ref().into() }
    }
    pub fn list(&self)->Vec<AtomicUnit>{ self.inner.read().clone() }
    pub fn get(&self, id:&str)->Option<AtomicUnit>{ self.inner.read().iter().find(|u| u.id==id).cloned() }
    pub fn replace_all(&self, units:Vec<AtomicUnit>) { *self.inner.write() = units; }
}

pub async fn load_units_from_dir<P: AsRef<Path>>(dir:P)->Result<Vec<AtomicUnit>>{
    let mut units = Vec::new();
    if !dir.as_ref().exists(){ return Ok(units) }
    for entry in std::fs::read_dir(dir.as_ref())? {
        let p = entry?.path();
        if p.extension().and_then(|e| e.to_str()) == Some("json") {
            let s = std::fs::read_to_string(&p)?;
            let spec:UnitSpec = serde_json::from_str(&s)?;
            units.push(unit_from_spec(&spec)?);
        } else if p.extension().and_then(|e| e.to_str()) == Some("yaml") || p.extension().and_then(|e| e.to_str()) == Some("yml") {
            let s = std::fs::read_to_string(&p)?;
            let spec:UnitSpec = serde_yaml::from_str(&s)?;
            units.push(unit_from_spec(&spec)?);
        }
    }
    Ok(units)
}

pub async fn watch_units(store:UnitStore)->Result<()>{
    use notify::{recommended_watcher, Event, RecursiveMode, Watcher};
    let dir = store.dir.clone();
    if !dir.exists(){ std::fs::create_dir_all(&dir)?; }
    // initial load
    if let Ok(u) = load_units_from_dir(&dir).await { store.replace_all(u) }

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Event>(32);
    tokio::task::spawn_blocking(move || {
        let mut w = recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(ev) = res { let _ = tx.blocking_send(ev); }
        }).expect("watcher");
        w.watch(&dir, RecursiveMode::NonRecursive).expect("watch");
        std::thread::park(); // stay alive
    });

    while let Some(_ev) = rx.recv().await {
        if let Ok(u) = load_units_from_dir(&dir).await { store.replace_all(u) }
    }
    Ok(())
}
