// Enforce the approved pure dependencies of the reusable engine.
import { readFileSync } from 'node:fs';
const manifest = readFileSync(new URL('../../crates/engine/Cargo.toml', import.meta.url), 'utf8');
const allowed = new Set(['serde', 'serde_json', 'schemars', 'thiserror', 'ts-rs']);
let dependencySection = false;
for (const line of manifest.split('\n')) {
  const header = line.match(/^\[([^\]]+)\]/);
  if (header) {
    dependencySection = /(^|\.)(dependencies|dev-dependencies|build-dependencies)$/.test(header[1]);
    if (/dependencies\./.test(header[1]))
      throw new Error('Engine dependency subtables need foundation review.');
    continue;
  }
  if (!dependencySection) continue;
  const dependency = line.match(/^([a-zA-Z0-9_-]+)(?:\.workspace)?\s*=/)?.[1];
  if (dependency && !allowed.has(dependency)) {
    throw new Error(
      `Engine dependency ${dependency} needs foundation review; adapters belong in the local runtime.`,
    );
  }
}
