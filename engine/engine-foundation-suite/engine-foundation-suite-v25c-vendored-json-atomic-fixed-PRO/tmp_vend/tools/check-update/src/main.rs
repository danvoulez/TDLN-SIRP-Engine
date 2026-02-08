
use anyhow::*;
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
struct CargoToml { package: Package, dependencies: Option<toml::value::Table> }
#[derive(Deserialize)]
struct Package { name: String, version: String }

fn main()->Result<()>{
    let path = std::env::args().nth(1).unwrap_or_else(|| "Cargo.toml".into());
    let desired = std::env::args().nth(2).unwrap_or_else(|| "engine-core".into());
    let data = fs::read_to_string(&path)?;
    let cargo: CargoToml = toml::from_str(&data)?;
    let deps = cargo.dependencies.ok_or_else(|| anyhow!("no [dependencies]"))?;
    if let Some(v) = deps.get(&desired) {
        let want = v.as_str().unwrap_or("*");
        println!("Wrapper '{}' depends on {} = {}", cargo.package.name, desired, want);
    } else {
        bail!("dependency '{}' not found", desired);
    }
    Ok(())
}
