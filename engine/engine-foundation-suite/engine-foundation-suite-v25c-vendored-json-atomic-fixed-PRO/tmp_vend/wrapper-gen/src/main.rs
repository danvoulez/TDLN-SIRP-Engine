
use clap::{Parser, Subcommand};
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(name="wrapper-gen")]
#[command(about="Generate a ready-to-run wrapper from a neutral template")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd
}
#[derive(Subcommand, Debug)]
enum Cmd {
    /// Create a new wrapper in target directory
    New {
        /// Do not include docs spine
        #[arg(long, default_value_t = false)]
        no_docs: bool,
        /// Optional manifest file (idea.manifest.v1)
        #[arg(long)]
        manifest: Option<String>,
        /// Wrapper preset: premium|minimal
        #[arg(long, default_value = "premium")]
        preset: String,
        /// Hex color (e.g., #10B981)
        #[arg(long, default_value = "#0EA5E9")]
        color: String,
        /// Flavors: comma-separated from {code,data}
        #[arg(long, default_value = "code,data")]
        flavors: String,
        /// Folder to create
        #[arg(long)]
        dir: String,
        /// Wrapper name (PascalCase becomes kebab-case for crates)
        #[arg(long)]
        name: String,
        /// Include HTTP service (axum-based)
        #[arg(long, default_value_t = true)]
        http: bool,
    }
}

fn copy_template(to:&Path, name:&str, http:bool, color:&str, flavors:&str, preset:&str) -> Result<()> {
    let tpl_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/basic");
    fs::create_dir_all(to)?;

    for entry in WalkDir::new(&tpl_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() { continue; }
        let rel = entry.path().strip_prefix(&tpl_root).unwrap();
        let target = to.join(rel);
        fs::create_dir_all(target.parent().unwrap())?;
        let mut content = fs::read_to_string(entry.path()).context("reading template file")?;

        // Replace placeholders
        let kebab = name_to_kebab(name);
        content = content.replace("{{WRAPPER_NAME}}", name);
        content = content.replace("{{WRAPPER_NAME_KEBAB}}", &kebab);
        content = content.replace("{{THEME_COLOR}}", color);
        content = content.replace("{{FLAVORS}}", flavors);
        content = content.replace("{{PRESET}}", preset);

        // Conditionally remove http files
        if !http && rel.to_string_lossy().contains("service") {
            continue;
        }
        fs::write(&target, content)?;
    }
    Ok(())
}

fn name_to_kebab(s:&str) -> String {
    let re = Regex::new(r"([a-z0-9])([A-Z])").unwrap();
    let s = re.replace_all(s, "$1-$2").to_string();
    s.to_lowercase().replace(' ', "-")
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.cmd {
        Cmd::New { dir, name, http, color, flavors, preset, manifest, no_docs } => {
            let to = PathBuf::from(&dir);
            copy_template(&to, &name, http, &color, &flavors, &preset)?;
        if !no_docs { copy_docs(&to, &name, &color, &preset, &flavors)?; }
            println!("✨ Wrapper created at {}", to.display());
        }
    }
    Ok(())
}


use std::fs;
use std::path::Path;
fn copy_docs(to:&Path, name:&str, color:&str, preset:&str, flavors:&str) -> anyhow::Result<()> {
    let kit = Path::new("wrapper-gen/templates/docs-kit");
    if !kit.exists() { return Ok(()); }
    let dst = to.join("docs");
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(kit)? {
        let p = entry?.path();
        let rel = p.file_name().unwrap().to_string_lossy().to_string();
        let mut content = std::fs::read_to_string(&p)?;
        content = content.replace("{{WRAPPER_NAME}}", name);
        content = content.replace("{{THEME_COLOR}}", color);
        content = content.replace("{{PRESET}}", preset);
        content = content.replace("{{FLAVORS}}", flavors);
        std::fs::write(dst.join(rel), content)?;
    }
    Ok(())
}
