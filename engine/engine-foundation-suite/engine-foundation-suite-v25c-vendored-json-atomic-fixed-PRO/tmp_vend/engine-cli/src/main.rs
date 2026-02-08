
use clap::{Parser, Subcommand};
use anyhow::Result;
use serde_json::json;
use engine_core::model::*;
use engine_core::runtime::Engine;
use engine_core::providers::*;
use engine_extras::aggregator_kofn::KOfN;
use engine_extras::expr_registry::{ExtensibleExpr, BasicRegistry};
use engine_extras::sink_filesystem::FsSink;
use engine_extras::signer_ed25519::Ed25519Signer;
use engine_registry::file_registry::FileRegistry;
use engine_registry::schema::EngineRegistryEntry;
use engine_audit::report::AuditReportV1;
use engine_audit::sink_fs::FsAudit;

#[derive(Parser, Debug)]
#[command(name="engine")]
#[command(about="Generic JSON✯Atomic engine with registry/auth/audit primitives")]
struct Cli { #[command(subcommand)] cmd: Cmd }

#[derive(Subcommand, Debug)]
enum Cmd {
  /// Execute example unit (JSON✯Atomic) and write a generic receipt + audit
  Run {
    #[arg(short, long)] input: String,
    #[arg(short, long, default_value = "./out")] outdir: String,
    #[arg(long, default_value_t = 2)] k: usize,
    #[arg(long)] sign_pem: Option<String>,
  },
  /// Put an artifact into the file-based registry (generic JSON record)
  RegistryPut {
    #[arg(long)] name: String,
    #[arg(long)] version: String,
    #[arg(long)] cid: String,
    #[arg(long, default_value = "./registry")] regdir: String,
  }
}

fn signer_from(pem_path: &Option<String>) -> Box<dyn Signer> {
    if let Some(p) = pem_path {
        if let Ok(pem) = std::fs::read_to_string(p) {
            if let Ok(s) = Ed25519Signer::from_pkcs8_pem("engine-key", &pem) {
                return Box::new(s);
            }
        }
    }
    Box::new(NoopSigner)
}

fn run_example(input_path:&str, outdir:&str, k:usize, sign_pem:&Option<String>) -> Result<()> {
    std::fs::create_dir_all(&outdir)?;
    // Example policies (generic)
    let pa = PolicyBit::new("has_role","actor has role")
      .requires(&["actor","role"])
      .condition(Expression::eq(Expression::context(&["actor","role"]), Expression::literal("admin"))).build();
    let pb = PolicyBit::new("has_quota","quota > 0")
      .requires(&["actor","quota"])
      .condition(Expression::gt(Expression::context(&["actor","quota"]), Expression::literal(0))).build();
    let pc = PolicyBit::new("resource_ok","resource not restricted")
      .requires(&["resource","restricted"])
      .condition(Expression::not(Expression::context(&["resource","restricted"]))).build();

    let unit = engine_core::AtomicUnit::builder("unit_required")
      .policy(pa).policy(pb).policy(pc)
      .wiring(Wiring::All{ policies: vec!["has_role".into(), "has_quota".into(), "resource_ok".into()] })
      .build();

    let rt = Engine::default()
      .unit(unit)
      .agg(KOfN{ k })
      .expr(ExtensibleExpr{ reg: BasicRegistry::new() })
      .sink(FsSink::new(&outdir))
      .build();

    let input_json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&input_path)?)?;
    let mut receipt = rt.execute("unit_required", input_json, None)?;

    // Optional sign
    let signer = signer_from(sign_pem);
    if let Some(sig) = signer.sign(serde_json::to_string(&receipt.proof.hash_chain)?.as_bytes()) {
        receipt.proof.signature = Some(base64::encode(sig));
    }

    // Write ReceiptCard v1 (brand-agnostic)
    let card = json!({
      "kind":"receipt.card.v1",
      "unit_id": receipt.unit_id,
      "mode_public_safe": receipt.mode.is_public_safe(),
      "input": { "cid": receipt.input.cid },
      "output": { "cid": receipt.output.cid },
      "decision": receipt.decision,
      "policy_decisions": receipt.policy_decisions,
      "missing": receipt.missing,
      "proof": receipt.proof,
      "ts": receipt.timestamp
    });
    let out_card = format!("{}/receipt_card.json", outdir);
    std::fs::write(&out_card, serde_json::to_string_pretty(&card)?)?;

    // Emit a tiny audit report to FS (generic)
    let audit = AuditReportV1{
      kind: "audit.report.v1".into(),
      audit_id: ulid::Ulid::new().to_string(),
      ts: receipt.timestamp.clone(),
      actor: "example-cli".into(),
      plan: json!({"engine":"example","k":k}),
      limits: json!({"row_cap": 0, "time_cap_ms": 0}),
      proofs: json!({"result_digest": receipt.output.cid, "inputs_root": receipt.input.cid}),
      receipt
    };
    let auditor = FsAudit::new(&format!("{}/audit", outdir));
    auditor.emit(&audit)?;

    println!("✅ decision card: {}", out_card);
    println!("🧾 audit report: {}/audit/...", outdir);
    Ok(())
}

fn reg_put(name:&str, version:&str, cid:&str, regdir:&str) -> Result<()> {
  let reg = FileRegistry::new(regdir);
  let e = EngineRegistryEntry{ kind:"engine.registry.entry.v1".into(), id: ulid::Ulid::new().to_string(), name:name.into(), version:version.into(), cid:cid.into(), meta: serde_json::json!({}) };
  let p = reg.put(&e)?;
  println!("📚 registry entry -> {}", p.display());
  Ok(())
}

fn main() -> Result<()> {
  let args = Cli::parse();
  match args.cmd {
    Cmd::Run { input, outdir, k, sign_pem } => run_example(&input, &outdir, k, &sign_pem),
    Cmd::RegistryPut { name, version, cid, regdir } => reg_put(&name, &version, &cid, &regdir),
  }
}
