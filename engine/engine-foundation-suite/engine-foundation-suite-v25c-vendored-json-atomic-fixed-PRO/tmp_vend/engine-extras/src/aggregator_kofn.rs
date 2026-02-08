
use engine_core::providers::AggregatorStrategy;
use engine_core::model::{Decision, PolicyDecision, Wiring};

#[derive(Clone, Copy)]
pub struct KOfN { pub k: usize }
impl AggregatorStrategy for KOfN {
    fn aggregate(&self, wiring:&Wiring, decisions:&[PolicyDecision])->Decision {
        let ids = wiring.ids();
        let mut allows = 0usize;
        let mut has_doubt = false;
        for d in decisions.iter().filter(|d| !d.skipped && ids.contains(&d.policy_id)) {
            match d.decision {
                Decision::Allow => allows += 1,
                Decision::Doubt => has_doubt = true,
                Decision::Deny => {}
            }
        }
        if allows >= self.k { Decision::Allow }
        else if has_doubt { Decision::Doubt }
        else { Decision::Deny }
    }
}
