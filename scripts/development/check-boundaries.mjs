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

// The public projection can depend on portable decision types, never transport or storage.
const projection = readFileSync(
  new URL('../../crates/public-snapshot/Cargo.toml', import.meta.url),
  'utf8',
);
const projectionAllowed = new Set(['memory-engine', 'serde', 'serde_json']);
let projectionSection = false;
for (const line of projection.split('\n')) {
  const header = line.match(/^\[([^\]]+)\]/);
  if (header) {
    projectionSection = /(^|\.)(dependencies|dev-dependencies|build-dependencies)$/.test(header[1]);
    if (/dependencies\./.test(header[1]))
      throw new Error('Projection dependency subtables need foundation review.');
    continue;
  }
  const dependency = line.match(/^([a-zA-Z0-9_-]+)(?:\.workspace)?\s*=/)?.[1];
  if (projectionSection && dependency && !projectionAllowed.has(dependency))
    throw new Error(`Public projection dependency ${dependency} needs foundation review.`);
}
