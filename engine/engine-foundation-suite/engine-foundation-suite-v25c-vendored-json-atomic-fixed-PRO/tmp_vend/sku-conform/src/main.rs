
use anyhow::{Result, anyhow};
use std::fs;
use jsonschema::{Draft, JSONSchema};
use serde_json::Value as Json;

fn main()->Result<()>{
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        eprintln!("usage: sku-conform --manifest product.json --schema schemas/idea.manifest.v1.schema.json [--check-wrapper PATH]");
        std::process::exit(2);
    }
    let mut manifest_path = None;
    let mut schema_path = None;
    let mut wrapper_path = None;
    let mut i=0;
    while i<args.len() {
        match args[i].as_str() {
            "--manifest" => { i+=1; manifest_path = Some(args[i].clone()); },
            "--schema" => { i+=1; schema_path = Some(args[i].clone()); },
            "--check-wrapper" => { i+=1; wrapper_path = Some(args[i].clone()); },
            other => return Err(anyhow!("unknown arg {other}"))
        }
        i+=1;
    }
    if let (Some(mp), Some(sp)) = (manifest_path, schema_path) {
        let m: Json = serde_json::from_str(&fs::read_to_string(mp)?)?;
        let s: Json = serde_json::from_str(&fs::read_to_string(sp)?)?;
        let compiled = JSONSchema::options().with_draft(Draft::Draft202012).compile(&s)?;
        if let Err(e) = compiled.validate(&m) {
            println!("❌ manifest invalid:");
            for err in e { println!("- {err}"); }
            std::process::exit(1);
        } else {
            println!("✅ manifest valid");
        }
    }
    if let Some(wp) = wrapper_path {
        let must = ["static/index.html","openapi.yaml",".env.example","Dockerfile","Makefile"];
        let mut ok = true;
        for f in must {
            let p = format!("{}/{}", wp, f);
            if !std::path::Path::new(&p).exists() {
                println!("❌ missing {}", f); ok=false;
            } else {
                println!("✅ {}", f);
            }
        }
        if ok { println!("✅ wrapper structure OK"); } else { std::process::exit(1); }
    }
    Ok(())
}
