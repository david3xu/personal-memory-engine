// Reject host/storage/transport dependencies in the reusable engine.
import { readFileSync } from 'node:fs';
const manifest = readFileSync(new URL('../../crates/engine/Cargo.toml', import.meta.url), 'utf8');
if (/tauri|rusqlite|rmcp|tokio|reqwest|dirs\s*=/.test(manifest))
  throw new Error('The engine must not depend on concrete adapters or hosts.');
