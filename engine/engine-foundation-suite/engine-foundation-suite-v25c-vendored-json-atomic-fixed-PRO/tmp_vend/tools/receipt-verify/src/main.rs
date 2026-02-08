
use std::{fs::File, io::Read, path::PathBuf};
use clap::Parser;
use anyhow::{Result, anyhow};
use zip::ZipArchive;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(name="receipt-verify", about="Offline verifier for LogLine bundle.zip")]
struct Args {
    /// Path to bundle.zip
    #[arg(value_name="BUNDLE")]
    bundle: PathBuf,
    /// Path to ed25519 public key (base64)
    #[arg(long)]
    pubkey_b64: Option<PathBuf>,
    /// Strict mode: verify bundle_hash == local recompute; emit JSON report
    #[arg(long)]
    strict: bool,
    /// Output JSON report path (when --strict)
    #[arg(long)]
    out_report: Option<PathBuf>,
    /// Produce a signed proof JSON (ed25519 over blake3(cert))
    #[arg(long)]
    prove: bool,
    /// Signing key (ed25519 private key base64), required when --prove
    #[arg(long)]
    signing_key_b64: Option<PathBuf>,
}

#[derive(Deserialize)]
struct Receipt {
    kind: String,
    signatures: Option<serde_json::Value>,
    certification: Option<serde_json::Value>,
}

fn read_zip_file(mut za: &mut ZipArchive<File>, name: &str) -> Result<Vec<u8>> {
    let mut f = za.by_name(name)?;
    let mut buf = Vec::new();
    std::io::copy(&mut f, &mut buf)?;
    Ok(buf)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let f = File::open(&args.bundle)?;
    let mut za = ZipArchive::new(f)?;

    // Required artifacts inside bundle.zip
    let policy = read_zip_file(&mut za, "policy.canonical.json")?;
    let wasm   = read_zip_file(&mut za, "policy.wasm").ok();
    let cert   = read_zip_file(&mut za, "certification.json")?;

    // Compute digests
    let policy_digest = blake3::hash(&policy);
    let wasm_digest   = wasm.as_ref().map(|b| blake3::hash(b));

    // Parse receipt
    let receipt: Receipt = serde_json::from_slice(&cert)?;
    if receipt.kind != "verification.bundle.v1" && receipt.kind != "certification" {
        eprintln!("WARN: unexpected receipt kind: {}", receipt.kind);
    }

    // Recompute demo bundle hash
    let mut demo_hash = blake3::Hasher::new();
    demo_hash.update(&policy);
    if let Some(w) = wasm.as_ref() { demo_hash.update(w); }
    let demo_b3 = format!("b3:{}", demo_hash.finalize().to_hex());

    // Optional: verify issuer signature if pubkey provided
    if let Some(pk_path) = args.pubkey_b64 {
        let pk_b64 = std::fs::read_to_string(pk_path)?;
        // Expect signatures.bundle_hash + issuer_signature (ed25519-blake3) in certification.json
        let v = serde_json::from_slice::<serde_json::Value>(&cert)?;
        let sig = v.pointer("/signatures/issuer_signature")
            .and_then(|x| x.as_str()).ok_or_else(|| anyhow!("issuer_signature missing"))?;
        let bundle_hash = v.pointer("/signatures/bundle_hash")
            .and_then(|x| x.as_str()).ok_or_else(|| anyhow!("bundle_hash missing"))?;
        // For demo we rehash policy+wasm to compare (not strict)
        if demo_b3 != bundle_hash {
            eprintln!("WARN: local bundle hash {} != receipt {}", demo, bundle_hash);
        }
        // Verify signature
        let pk = base64::prelude::BASE64_STANDARD.decode(pk_b64.trim()).map_err(|_| anyhow!("invalid pubkey"))?;
        let sigb = base64::prelude::BASE64_STANDARD.decode(sig).map_err(|_| anyhow!("invalid signature b64"))?;
        let vk = ed25519_dalek::VerifyingKey::from_bytes(&pk.try_into().map_err(|_| anyhow!("pubkey size"))?)?;
        let digest = blake3::hash(&cert);
        let sig = ed25519_dalek::Signature::from_bytes(&sigb.try_into().map_err(|_| anyhow!("sig size"))?);
        vk.verify_strict(digest.as_bytes(), &sig).map_err(|e| anyhow!(e))?;
        println!("✔ issuer signature: OK");
    }

    println!("✔ policy.canonical.json b3:{}", policy_digest.to_hex());
    if let Some(w) = wasm_digest { println!("✔ policy.wasm b3:{}", w.to_hex()); }


if args.strict {
    // Expect fields in certification.json
    let v = serde_json::from_slice::<serde_json::Value>(&cert)?;
    let bundle_hash = v.pointer("/signatures/bundle_hash").and_then(|x| x.as_str()).unwrap_or("");
    let issuer = v.pointer("/certification/issuer/name").and_then(|x| x.as_str()).unwrap_or("unknown");
    let did = v.pointer("/certification/did").and_then(|x| x.as_str()).unwrap_or("");

    let mut ok = true;
    let mut issues: Vec<String> = Vec::new();
    if demo_b3 != bundle_hash {
        ok = false;
        issues.push(format!("bundle_hash mismatch: local {} vs receipt {}", demo_b3, bundle_hash));
    }

    let report = serde_json::json!({
        "ok": ok,
        "issuer": issuer,
        "did": did,
        "policy_b3": format!("b3:{}", blake3::hash(&policy).to_hex()),
        "wasm_b3": wasm.as_ref().map(|w| format!("b3:{}", blake3::hash(w).to_hex())),
        "bundle_hash": bundle_hash,
        "local_bundle_hash": demo_b3,
        "issues": issues
    });

    if let Some(out) = args.out_report {
        std::fs::write(out, serde_json::to_vec_pretty(&report)?)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&report)?);
    }

    if !ok { std::process::exit(2); }
}
    println!("✔ bundle parsed successfully");

if args.prove {
    let sk_path = args.signing_key_b64.as_ref().ok_or_else(|| anyhow!("--signing_key_b64 required with --prove"))?;
    let sk_b64 = std::fs::read_to_string(sk_path)?;
    let sk_bytes = base64::prelude::BASE64_STANDARD.decode(sk_b64.trim()).map_err(|_| anyhow!("invalid signing key b64"))?;
    let sk = ed25519_dalek::SigningKey::from_bytes(&sk_bytes.try_into().map_err(|_| anyhow!("signing key size"))?);
    let pk = sk.verifying_key();
    let digest = blake3::hash(&cert);
    let sig = sk.sign(digest.as_bytes());
    let proof = serde_json::json!({
        "kind":"verify.proof.v1",
        "ts": format!("{}", time::OffsetDateTime::now_utc()),
        "bundle": args.bundle.file_name().unwrap().to_string_lossy(),
        "cert_b3": format!("b3:{}", blake3::hash(&cert).to_hex() ),
        "policy_b3": format!("b3:{}", blake3::hash(&policy).to_hex() ),
        "wasm_b3": wasm.as_ref().map(|w| format!("b3:{}", blake3::hash(w).to_hex())),
        "signature": base64::prelude::BASE64_STANDARD.encode(sig.to_bytes()),
        "public_key_b64": base64::prelude::BASE64_STANDARD.encode(pk.to_bytes())
    });
    if let Some(out) = args.out_report.as_ref() {
        let p = out.with_extension("proof.json");
        std::fs::write(&p, serde_json::to_vec_pretty(&proof)?)?;
        println!("✔ proof saved: {}", p.to_string_lossy());
    } else {
        println!("{}", serde_json::to_string_pretty(&proof)?);
    }
}

    Ok(())
}
