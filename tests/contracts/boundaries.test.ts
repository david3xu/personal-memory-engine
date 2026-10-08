// Verify that portable decision rules cannot acquire host or storage dependencies.
import test from 'node:test';
import { execFileSync } from 'node:child_process';
test('reusable engine stays independent of adapters', () => {
  execFileSync(process.execPath, ['scripts/development/check-boundaries.mjs']);
});
