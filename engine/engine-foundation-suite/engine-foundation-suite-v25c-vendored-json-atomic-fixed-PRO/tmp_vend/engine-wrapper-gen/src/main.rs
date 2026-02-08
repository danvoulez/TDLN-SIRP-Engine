
use clap::Parser;
use anyhow::{Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name="engine-wrapper-gen")]
#[command(about="Generate a ready-to-run wrapper from a neutral template")]
struct Cli {
  /// Wrapper name (e.g., acme-trust)
  #[arg(long)]
  name: String,
  /// Output directory
  #[arg(long, default_value = "./wrappers")]
  outdir: String,
  /// Include HTTP server (engine-http) glue
  #[arg(long, default_value_t = true)]
  http: bool,
  /// Enable S3-compatible sink in template Cargo (feature only; user wires deps)
  #[arg(long, default_value_t = true)]
  s3_feature: bool
}

fn replace_tokens(s:&str, name:&str) -> String {
  s.replace("__WRAPPER_NAME__", name)
}

fn copy_template(src:&Path, dst:&Path, name:&str) -> Result<()> {
  for entry in WalkDir::new(src) {
    let e = entry?;
    if e.file_type().is_dir() { continue; }
    let rel = e.path().strip_prefix(src).unwrap();
    let out_path = dst.join(rel);
    if let Some(pdir) = out_path.parent() { fs::create_dir_all(pdir)?; }
    let mut contents = fs::read_to_string(e.path())?;
    contents = replace_tokens(&contents, name);
    fs::write(&out_path, contents)?;
  }
  Ok(())
}

fn main() -> Result<()> {
  let args = Cli::parse();
  let template = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../wrapper-template");
  if !template.exists() { return Err(anyhow!("template not found at {:?}", template)); }

  let out_root = PathBuf::from(&args.outdir).join(&args.name);
  if out_root.exists() { return Err(anyhow!("output already exists: {:?}", out_root)); }
  fs::create_dir_all(&out_root)?;

  copy_template(&template, &out_root, &args.name)?;

  // Toggle features in template's Cargo.toml if requested
  let cpath = out_root.join("Cargo.toml");
  let mut cargo = fs::read_to_string(&cpath)?;
  if args.s3_feature {
    cargo.push_str("\n# enable s3-compatible feature at your discretion\n");
  }
  fs::write(&cpath, cargo)?;

  println!("✅ wrapper generated at {}", out_root.display());
  println!("→ cd {} && cargo build", out_root.display());
  Ok(())
}
