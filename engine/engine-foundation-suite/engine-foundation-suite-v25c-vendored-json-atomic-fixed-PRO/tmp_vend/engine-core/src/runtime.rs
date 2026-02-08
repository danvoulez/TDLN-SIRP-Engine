
use serde_json::json;
use serde_json::Value as Json;
use anyhow::{Result, anyhow};
use crate::model::*;
use crate::providers::*;

pub struct Engine<G,E,A,CX,CD,S,T>
where
  G: IdGen, E: ExprEval, A: AggregatorStrategy,
  CX: CanonProvider, CD: CidProvider, S: Signer, T: ReceiptSink
{
  chips: std::collections::HashMap<String, SemanticChip>,
  default_mode: EngineMode,
  id: G, expr: E, agg: A, canon: CX, cid: CD, signer: S, sink: T, clock: Box<dyn Clock>,
}

pub struct EngineBuilder<G,E,A,CX,CD,S,T>
where
  G: IdGen, E: ExprEval, A: AggregatorStrategy,
  CX: CanonProvider, CD: CidProvider, S: Signer, T: ReceiptSink
{
  chips: std::collections::HashMap<String, SemanticChip>,
  default_mode: Option<EngineMode>,
  id: Option<G>, expr: Option<E>, agg: Option<A>, canon: Option<CX>, cid: Option<CD>, signer: Option<S>, sink: Option<T>, clock: Option<Box<dyn Clock>>,
}
impl<G,E,A,CX,CD,S,T> Default for EngineBuilder<G,E,A,CX,CD,S,T>
where G:IdGen+Default, E:ExprEval+Default, A:AggregatorStrategy+Default, CX:CanonProvider+Default, CD:CidProvider+Default, S:Signer+Default, T:ReceiptSink+Default
{
  fn default()->Self {
    Self{ chips:std::collections::HashMap::new(), default_mode:None, id:Some(G::default()), expr:Some(E::default()), agg:Some(A::default()), canon:Some(CX::default()), cid:Some(CD::default()), signer:Some(S::default()), sink:Some(T::default()), clock:None }
  }
}
impl Default for crate::providers::DefaultExpr { fn default()->Self{ Self } }
impl Default for crate::providers::DefaultAggregator { fn default()->Self{ Self } }
impl Default for crate::providers::DefaultCanon { fn default()->Self{ Self } }
impl Default for crate::providers::DefaultCid { fn default()->Self{ Self } }
impl Default for crate::providers::NoopSigner { fn default()->Self{ Self } }
impl Default for crate::providers::NoopSink { fn default()->Self{ Self } }
impl Default for crate::providers::UlidGen { fn default()->Self{ Self } }

impl<G,E,A,CX,CD,S,T> EngineBuilder<G,E,A,CX,CD,S,T>
where G:IdGen, E:ExprEval, A:AggregatorStrategy, CX:CanonProvider, CD:CidProvider, S:Signer, T:ReceiptSink
{
  pub fn chip(mut self, c:SemanticChip)->Self{ self.chips.insert(c.id.clone(), c); self }
  pub fn chips(mut self, v:Vec<SemanticChip>)->Self{ for c in v { self.chips.insert(c.id.clone(), c); } self }
  pub fn default_mode(mut self, m:EngineMode)->Self{ self.default_mode = Some(m); self }
  pub fn id(mut self, v:G)->Self{ self.id=Some(v); self }
  pub fn expr(mut self, v:E)->Self{ self.expr=Some(v); self }
  pub fn agg(mut self, v:A)->Self{ self.agg=Some(v); self }
  pub fn canon(mut self, v:CX)->Self{ self.canon=Some(v); self }
  pub fn cid(mut self, v:CD)->Self{ self.cid=Some(v); self }
  pub fn signer(mut self, v:S)->Self{ self.signer=Some(v); self }
  pub fn sink(mut self, v:T)->Self{ self.sink=Some(v); self }
  pub fn clock(mut self, v:Box<dyn Clock>)->Self{ self.clock=Some(v); self }

  pub fn defaults() -> EngineBuilder<crate::providers::UlidGen, crate::providers::DefaultExpr, crate::providers::DefaultAggregator, crate::providers::DefaultCanon, crate::providers::DefaultCid, crate::providers::NoopSigner, crate::providers::NoopSink> {
      EngineBuilder::default()
  }

  pub fn build(self)->Engine<G,E,A,CX,CD,S,T> {
    Engine{
      chips: self.chips,
      default_mode: self.default_mode.unwrap_or_else(EngineMode::conservative),
      id: self.id.expect("id provider"),
      expr: self.expr.expect("expr provider"),
      agg: self.agg.expect("agg provider"),
      canon: self.canon.expect("canon provider"),
      cid: self.cid.expect("cid provider"),
      signer: self.signer.expect("signer"),
      sink: self.sink.expect("sink"),
      clock: self.clock.unwrap_or_else(|| Box::new(crate::providers::SysClock)),
    }
  }
}

impl<G,E,A,CX,CD,S,T> Engine<G,E,A,CX,CD,S,T>
where
  G: IdGen, E: ExprEval, A: AggregatorStrategy,
  CX: CanonProvider, CD: CidProvider, S: Signer, T: ReceiptSink
{
  pub fn execute(&self, chip_id:&str, input: Json, mode: Option<EngineMode>) -> Result<ExecutionReceipt> {
    let start = std::time::Instant::now();
    let chip = self.chips.get(chip_id).ok_or_else(|| anyhow!("Chip not found: {chip_id}"))?;
    let mode = mode.unwrap_or_else(|| self.default_mode.clone());

    if !mode.allows_all(&chip.required_effects) {
      return Ok(denied_receipt(chip, mode, input, "Effects not allowed".into()));
    }

    let input_canon_bytes = self.canon.canon(&input);
    let input_canon: Json = serde_json::from_slice(&input_canon_bytes)?;
    let input_cid = self.cid.cid(&input_canon_bytes);

    let mut decisions = vec![];
    let mut hash_chain = vec![input_cid.clone()];

    for p in &chip.policies {
      let d = eval_policy_with(&self.expr, p, &input_canon, &mode);
      let h = self.cid.cid(&self.canon.canon(&json!({
          "policy": p.id,
          "policy_hash": p.hash,
          "decision": d.decision,
          "skipped": d.skipped,
          "input_cid": input_cid
      })));

      hash_chain.push(h);
      decisions.push(d);
    }

    let final_decision = self.agg.aggregate(&chip.wiring, &decisions);

    let output = json!({
      "chip_id": chip.id,
      "decision": final_decision,
      "policy_count": decisions.len(),
    });
    let output_canon_bytes = self.canon.canon(&output);
    let output_canon: Json = serde_json::from_slice(&output_canon_bytes)?;
    let output_cid = self.cid.cid(&output_canon_bytes);
    hash_chain.push(output_cid.clone());

    let missing = build_missing(&decisions);

    let to_sign = self.canon.canon(&json!({ "input": input_cid, "output": output_cid, "hash_chain": hash_chain }));
    let signature = self.signer.sign(&to_sign).map(|b| base64::encode(b));

    let receipt = ExecutionReceipt {
      chip_id: chip.id.clone(),
      chip_hash: chip.hash.clone().unwrap_or_default(),
      mode,
      input: CanonSlot{ raw: input, canon: input_canon, cid: input_cid },
      policy_decisions: decisions,
      output: CanonSlot{ raw: output.clone(), canon: output_canon, cid: output_cid },
      decision: final_decision,
      missing,
      proof: Proof { hash_chain, signature },
      timestamp: self.clock.now_rfc3339(),
      duration_ns: start.elapsed().as_nanos() as u64,
    };

    let _ = self.sink.emit(&receipt);
    Ok(receipt)
  }
}

pub fn eval_policy_with(E: &dyn ExprEval, policy:&PolicyBit, ctx:&Json, mode:&EngineMode) -> PolicyDecision {
    let start = std::time::Instant::now();

    if !mode.is_policy_active(&policy.id) {
        return PolicyDecision {
            policy_id: policy.id.clone(),
            policy_hash: policy.hash.clone().unwrap_or_default(),
            decision: Decision::Allow,
            evaluation_ns: start.elapsed().as_nanos() as u64,
            error: None,
            skipped: true,
            missing_fields: vec![],
        };
    }

    let missing = policy.check_required_fields(ctx);
    if !missing.is_empty() {
        return PolicyDecision {
            policy_id: policy.id.clone(),
            policy_hash: policy.hash.clone().unwrap_or_default(),
            decision: Decision::Doubt,
            evaluation_ns: start.elapsed().as_nanos() as u64,
            error: Some(format!("Missing required fields: {:?}", missing)),
            skipped: false,
            missing_fields: missing.into_iter().map(|p| p.join(".")).collect(),
        };
    }

    let (decision, error) = match E.eval(&policy.condition, ctx) {
        Ok(v) => {
            let b = match v {
                Json::Bool(b)=>b,
                Json::Null=>false,
                Json::Number(n)=> n.as_f64().map(|f| f!=0.0 && f.is_finite()).unwrap_or(false),
                Json::String(s)=> !s.is_empty(),
                Json::Array(a)=> !a.is_empty(),
                Json::Object(o)=> !o.is_empty(),
            };
            if b { (Decision::Allow, None) } else { (Decision::Deny, None) }
        },
        Err(e) => (Decision::Doubt, Some(e.to_string())),
    };

    PolicyDecision {
        policy_id: policy.id.clone(),
        policy_hash: policy.hash.clone().unwrap_or_default(),
        decision,
        evaluation_ns: start.elapsed().as_nanos() as u64,
        error,
        skipped: false,
        missing_fields: vec![],
    }
}

pub fn build_missing(decisions:&[PolicyDecision]) -> Option<MissingInfo> {
    let missing_fields: Vec<_> = decisions.iter()
        .filter(|d| d.decision == Decision::Doubt)
        .flat_map(|d| d.missing_fields.clone())
        .collect();
    let doubt_policies: Vec<_> = decisions.iter()
        .filter(|d| d.decision == Decision::Doubt || d.error.is_some())
        .map(|d| d.policy_id.clone()).collect();

    if missing_fields.is_empty() && doubt_policies.is_empty() { return None; }
    let reason = if !missing_fields.is_empty() { "missing_fields" } else { "policy_doubt" };
    let resolution_hint = if !missing_fields.is_empty() {
        Some(format!("Provide missing fields: {}", missing_fields.join(", ")))
    } else if !doubt_policies.is_empty() {
        Some(format!("Review policies: {}", doubt_policies.join(", ")))
    } else { None };

    Some(MissingInfo{
        id: ulid::Ulid::new().to_string(),
        reason: reason.into(),
        missing_fields,
        missing_evidence: doubt_policies,
        resolution_hint,
    })
}

pub fn denied_receipt(chip:&SemanticChip, mode:EngineMode, input:Json, reason:String)->ExecutionReceipt {
    let canon = crate::providers::DefaultCanon{}.canon(&input);
    let input_cid = crate::providers::DefaultCid{}.cid(&canon);
    let out = json!({"error": reason, "decision":"deny"});
    let out_canon = crate::providers::DefaultCanon{}.canon(&out);
    let out_cid = crate::providers::DefaultCid{}.cid(&out_canon);
    ExecutionReceipt{
        chip_id: chip.id.clone(), chip_hash: chip.hash.clone().unwrap_or_default(),
        mode, input: CanonSlot{ raw: input, canon: serde_json::from_slice(&canon).unwrap(), cid: input_cid.clone() },
        policy_decisions: vec![],
        output: CanonSlot{ raw: out, canon: serde_json::from_slice(&out_canon).unwrap(), cid: out_cid.clone() },
        decision: Decision::Deny, missing: None,
        proof: Proof{ hash_chain: vec![input_cid, out_cid], signature: None },
        timestamp: chrono::Utc::now().to_rfc3339(), duration_ns: 0
    }
}

// Convenience
impl Engine<crate::providers::UlidGen, crate::providers::DefaultExpr, crate::providers::DefaultAggregator, crate::providers::DefaultCanon, crate::providers::DefaultCid, crate::providers::NoopSigner, crate::providers::NoopSink> {
    pub fn default() -> EngineBuilder<crate::providers::UlidGen, crate::providers::DefaultExpr, crate::providers::DefaultAggregator, crate::providers::DefaultCanon, crate::providers::DefaultCid, crate::providers::NoopSigner, crate::providers::NoopSink> {
        EngineBuilder::default()
    }
}


// === Atomic (JSON✯) public builder alias ===================================
pub trait UnitBuilder {
    fn unit(self, unit: crate::model::SemanticChip) -> Self;
}
impl<C,E,A,X,S,SI,SK> UnitBuilder for EngineBuilder<C,E,A,X,S,SI,SK> 
where EngineBuilder<C,E,A,X,S,SI,SK>: Sized {
    fn unit(mut self, unit: crate::model::SemanticChip) -> Self {
        self = self.chip(unit);
        self
    }
}
