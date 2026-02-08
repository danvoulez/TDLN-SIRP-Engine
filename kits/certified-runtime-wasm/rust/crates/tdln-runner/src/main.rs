
use clap::{Parser, Subcommand};
use serde_json::json;
use std::fs;
use tdln_certified_runtime::{CertifiedRuntime, RuntimeConfig};
use tdln_runtime_wasm::WasmCertifiedRuntime;

#[derive(Parser)]
#[command(name="tdln-runner", version, about="Certified Runtime — WASM v1") ]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd
}

#[derive(Subcommand)]
enum Cmd {
    Run {
        #[arg(long)]
        wasm: String,
        #[arg(long)]
        input: String,
        #[arg(long, default_value_t=10000000)]
        fuel: u64,
        #[arg(long, default_value_t=256)]
        memory_max_mb: u64,
        #[arg(long)]
        out: Option<String>,
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Run { wasm, input, fuel, memory_max_mb, out } => {
            let unit_bytes = fs::read(&wasm)?;
            let input_json: serde_json::Value = serde_json::from_str(&fs::read_to_string(&input)?)?;
            let rt = WasmCertifiedRuntime { version: env!("CARGO_PKG_VERSION") };
            let cfg = RuntimeConfig { deterministic: true, fuel, memory_max_mb };
            let card = rt.execute(&unit_bytes, &input_json, &cfg)?;
            let s = serde_json::to_string_pretty(&card)?;
            if let Some(p) = out { fs::write(p, s)?; } else { println!("{}", s); }
        }
    }
    Ok(())
}
