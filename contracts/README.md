# Portable contracts

`crates/engine/src/contracts.rs` is authoritative. Run `pnpm contracts:generate` to regenerate the TypeScript/JSON schemas and standalone browser validator. CI checks for drift. Incoming MCP submissions also pass core validation; frontend records pass the derived JSON schema validator. Missing optional reasons/evidence are never synthesized.
