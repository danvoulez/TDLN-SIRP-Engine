
use clap::{Parser, Subcommand};
use serde_json::{json, Value};
use tdln_canon::json_atomic_stringify;
use tdln_cid::cid_from_json;
use tdln_receipt::{Card, Proof, ChainStep, Seal, Links, RefItem};
use tdln_verify::{verify_rref_11, Verdict};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name="tdln", version, about="TDLN Rust CLI — give -> URL -> receipt") ]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd
}

#[derive(Subcommand)]
enum Cmd {
    /// Sign a card.json with an ed25519 secret key (base64)
    Sign { #[arg(long)] path: PathBuf, #[arg(long)] sk_b64: String, #[arg(long, default_value="engine-prod-2026-02")] kid: String },
    /// Verify a card.json against a verifying key (base64)
    VerifySeal { #[arg(long)] path: PathBuf, #[arg(long)] vk_b64: String },
    /// Produce a demo card (receipt.card.v1) and print CARD_URL
    DemoCard {
        #[arg(long, default_value="trust")]
        realm: String,
        #[arg(long, default_value="certify")]
        intent: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Verify a card.json using RREF v1.1 structural rules
    VerifyCard {
        #[arg(long)]
        path: PathBuf
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::DemoCard { realm, intent, out } => {
            let run_manifest = json!({ "realm": realm, "intent": intent, "inputs": {"policy":{"kind":"tdln","cid":"cid:b3:abc"}}, "ts":"2026-02-07T00:00:00Z" });
            let run_cid = cid_from_json(&run_manifest); // b3:<...>
            let card_url = format!("https://cert.tdln.foundry/r/{}", run_cid);
            let card = Card {
                kind: "receipt.card.v1".into(),
                realm: "trust".into(),
                decision: "ACK".into(),
                unit_id: Some("cid:b3:unitdemo".into()),
                policy_id: Some("cid:b3:policydemo".into()),
                output_cid: "cid:b3:outdemo".into(),
                proof: Proof { 
                    seal: Seal { alg: "ed25519-blake3".into(), kid: "demo".into(), sig: base64::encode("DEMO") },
                    hash_chain: vec![ChainStep{kind:"input".into(), cid:"cid:b3:indemo".into()}, ChainStep{kind:"output".into(), cid:"cid:b3:outdemo".into()}]
                },
                poi: None,
                refs: vec![RefItem{
                    kind:"unit.manifest".into(), cid:"cid:b3:unitdemo".into(), media_type:"application/json".into(),
                    size: None, hrefs: vec!["https://registry.tdln.foundry/v1/objects/cid:b3:unitdemo".into(), "tdln://objects/cid:b3:unitdemo".into()], private: Some(false)
                }],
                links: Links { card_url }
            };
            println!("CARD_URL: {}", card.links.card_url);
            if let Some(dir) = out {
                fs::create_dir_all(&dir)?;
                let p = dir.join("card.json");
                fs::write(&p, serde_json::to_string_pretty(&card)?)?;
                let m = dir.join("run.manifest.json");
                fs::write(&m, serde_json::to_string_pretty(&run_manifest)?)?;
                println!("Wrote {}", p.display());
            }
        }
        Cmd::VerifyCard { path } => {
            let data = fs::read_to_string(path)?;
            let card: Card = serde_json::from_str(&data)?;
            let verdict = verify_rref_11(&card);
            match verdict {
                Verdict::Pass => println!(r#"{{\"result\":\"PASS\"}}"#),
                Verdict::Warn(code) => println!(r#"{{\"result\":\"WARN\",\"code\":\"{}\"}}"#, code),
                Verdict::Fail(code) => { println!(r#"{{\"result\":\"FAIL\",\"code\":\"{}\"}}"#, code); std::process::exit(2); }
            }
        }
    }
    Ok(())
}


        Cmd::Sign { path, sk_b64, kid } => {
            let data = fs::read_to_string(&path)?;
            let mut card: Card = serde_json::from_str(&data)?;
            tdln_verify::sign_card(&mut card, &sk_b64, &kid);
            fs::write(&path, serde_json::to_string_pretty(&card)?)?;
            println!("SIGNED {}", path.display());
        }
        Cmd::VerifySeal { path, vk_b64 } => {
            let data = fs::read_to_string(&path)?;
            let card: Card = serde_json::from_str(&data)?;
            let ok = tdln_verify::verify_seal(&card, &vk_b64);
            println!(r#"{{"seal":"{}"}}"#, if ok {"PASS"} else {"FAIL"});
        }
