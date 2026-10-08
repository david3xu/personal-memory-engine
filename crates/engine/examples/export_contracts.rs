// Generate portable schemas and TypeScript declarations from the Rust contracts.
use memory_engine::{Alternative, CaptureDecision, DecisionVersion, Evidence};
use std::{fs, path::Path};
use ts_rs::{Config, TS};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts/generated");
    let check = std::env::args().any(|arg| arg == "--check");
    if !check {
        fs::create_dir_all(&output)?;
    }
    let declarations = format!(
        "// Generated from memory-engine; do not edit.\n{}\n{}\n{}\n{}\n",
        Alternative::decl(&Config::default()),
        Evidence::decl(&Config::default()),
        CaptureDecision::decl(&Config::default()),
        DecisionVersion::decl(&Config::default())
    )
    .replace("type ", "export type ");
    emit(&output.join("records.ts"), declarations, check)?;
    emit(
        &output.join("decision-version.schema.json"),
        serde_json::to_string_pretty(&schemars::schema_for!(DecisionVersion))? + "\n",
        check,
    )?;
    emit(
        &output.join("capture-decision.schema.json"),
        serde_json::to_string_pretty(&schemars::schema_for!(CaptureDecision))? + "\n",
        check,
    )?;
    Ok(())
}

fn emit(path: &Path, value: String, check: bool) -> Result<(), Box<dyn std::error::Error>> {
    if check {
        if fs::read_to_string(path)? != value {
            return Err(format!("Generated contract is stale: {}", path.display()).into());
        }
    } else {
        fs::write(path, value)?;
    }
    Ok(())
}
