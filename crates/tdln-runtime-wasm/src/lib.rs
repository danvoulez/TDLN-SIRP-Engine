use anyhow::Result;
use blake3::Hasher;
use serde_json::json;
use tdln_certified_runtime::{
    Card, CertifiedRuntime, ChainStep, Links, ReceiptProof, ReceiptSeal, RuntimeConfig,
};

pub struct WasmCertifiedRuntime {
    pub version: &'static str,
}

fn cid_bytes(bytes: &[u8]) -> String {
    let mut h = Hasher::new();
    h.update(bytes);
    format!("cid:b3:{}", h.finalize().to_hex())
}

fn cid_json(v: &serde_json::Value) -> String {
    let enc = serde_json::to_vec(&v).unwrap();
    cid_bytes(&enc)
}

impl CertifiedRuntime for WasmCertifiedRuntime {
    fn execute(
        &self,
        unit_bytes: &[u8],
        input_json: &serde_json::Value,
        cfg: &RuntimeConfig,
    ) -> Result<Card> {
        let memory_limit_bytes = (cfg.memory_max_mb as usize).saturating_mul(1024 * 1024);
        let exec_cfg = engine_exec_wasm::ExecConfig {
            fuel_limit: cfg.fuel,
            memory_limit_bytes,
            allow_imports: !cfg.deterministic,
        };
        let exec = engine_exec_wasm::WasmExecutor::new(exec_cfg)?;
        let output_bytes = exec.exec(unit_bytes, input_json)?;
        let output_json: serde_json::Value = serde_json::from_slice(&output_bytes)?;

        // Build proof/hash chain
        let input_cid = cid_json(input_json);
        let unit_cid = cid_bytes(unit_bytes);
        let output_cid = cid_json(&output_json);
        let run_manifest = json!({
            "unit_cid": unit_cid, "input_cid": input_cid, "cfg": { "deterministic": true, "fuel": cfg.fuel, "memory_max_mb": cfg.memory_max_mb }
        });
        let run_cid = cid_json(&run_manifest);

        let seal = ReceiptSeal {
            alg: "ed25519-blake3".into(),
            kid: "demo".into(),
            sig: base64::encode("DEMO"),
        };
        let proof = ReceiptProof {
            seal,
            hash_chain: vec![
                ChainStep {
                    kind: "input".into(),
                    cid: input_cid.clone(),
                },
                ChainStep {
                    kind: "output".into(),
                    cid: output_cid.clone(),
                },
            ],
            eer: Some(
                json!({ "runtime":{"name":"tdln-runtime-wasm","version": self.version, "hash":"b3:demo"}, "config": {"deterministic": cfg.deterministic, "fuel": cfg.fuel, "memory_max_mb": cfg.memory_max_mb}, "digests":{"unit_cid": unit_cid, "policy_cid": "cid:b3:policydemo"}, "wasmtime":{"version": "19.x"} }),
            ),
        };

        let card = Card {
            kind: "receipt.card.v1".into(),
            realm: "trust".into(),
            decision: "ACK".into(),
            output_cid,
            proof,
            refs: vec![
                json!({ "kind":"unit.wasm", "cid": unit_cid, "media_type":"application/wasm", "hrefs": ["tdln://objects/<cid>"] }),
            ],
            links: Links {
                url: String::new(),
                card_url: format!(
                    "https://cert.tdln.foundry/r/{}",
                    run_cid.replace("cid:", "")
                ),
            },
        };

        // Sign the card (DEMO key – replace in production)
        let mut card = card;
        let sk_b64 = std::env::var("TDLN_DEMO_SK_B64").unwrap_or_default();
        if !sk_b64.is_empty() {
            let bytes = serde_json::to_vec(&card).unwrap();
            // Reuse tdln-verify approach would be ideal; keeping simple here for demo
            card.proof.seal.kid = "demo".into();
            card.proof.seal.sig = base64::encode(blake3::hash(&bytes).as_bytes());
        }
        Ok(card)
    }
}

fn poi(missing: &[&str]) -> serde_json::Value {
    serde_json::json!({
        "present": true,
        "missing": missing,
    })
}
