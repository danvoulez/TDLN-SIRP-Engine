
use anyhow::{Result, anyhow};
use serde_json::Value as Json;
use std::fs;
use ed25519_dalek::{Verifier, PublicKey};

fn main()->Result<()> {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() { eprintln!("usage: receipt-verify <receipt_card.json> [--pubkey pub.pem]"); std::process::exit(2); }
    let card_path = args.remove(0);
    let pubkey_pem = match args.as_slice() {
        ["--pubkey", p] => Some(p.to_string()),
        _ => None
    };
    let s = fs::read_to_string(&card_path)?;
    let card: Json = serde_json::from_str(&s)?;

    // Basic structure checks
    let proof = card.get("proof").ok_or_else(|| anyhow!("missing proof"))?;
    let chain = proof.get("hash_chain").ok_or_else(|| anyhow!("missing hash_chain"))?.as_array().ok_or_else(|| anyhow!("hash_chain not array"))?;

    if chain.len() < 2 {
        return Err(anyhow!("hash_chain too short"));
    }

    // Optional signature verification
    if let Some(sig_b64) = proof.get("signature").and_then(|v| v.as_str()) {
        if let Some(pem_path) = pubkey_pem {
            let pem = std::fs::read_to_string(pem_path)?;
            let pk = PublicKey::from_public_key_pem(&pem)?;
            let msg = serde_json::to_string(&chain)?;
            let sig = base64::decode(sig_b64)?;
            pk.verify(msg.as_bytes(), &ed25519_dalek::Signature::from_bytes(&sig)?)?;
            println!("🔏 signature: OK");
        } else {
            println!("ℹ️ signature present but no --pubkey provided; skipped");
        }
    } else {
        println!("ℹ️ no signature");
    }

    // Surface CIDs
    let input_cid = card.pointer("/input/cid").and_then(|v| v.as_str()).unwrap_or("");
    let output_cid = card.pointer("/output/cid").and_then(|v| v.as_str()).unwrap_or("");
    println!("✅ receipt OK  | input CID: {input_cid} | output CID: {output_cid} | chain_len: {}", chain.len());
    Ok(())
}
