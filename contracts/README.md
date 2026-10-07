# Portable contracts

`crates/engine/src/contracts.rs` is authoritative. Run `cargo run -p memory-engine --example export_contracts` to regenerate `generated/` TypeScript types and JSON schemas. CI checks for drift. Incoming MCP submissions also pass core validation; frontend records pass the derived JSON schema validator. Missing optional reasons/evidence are never synthesized.
