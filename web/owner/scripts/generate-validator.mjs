// Build a static schema validator: the desktop CSP deliberately disallows runtime eval.
import Ajv2020 from 'ajv/dist/2020.js';
import standaloneCode from 'ajv/dist/standalone/index.js';
import { readFile, writeFile } from 'node:fs/promises';
const schema = JSON.parse(
  await readFile(
    new URL('../../../contracts/generated/decision-version.schema.json', import.meta.url),
    'utf8',
  ),
);
const ajv = new Ajv2020({
  strict: false,
  formats: { uint32: true },
  code: { source: true, esm: true },
});
const code =
  '// Generated from the Rust-owned record schema; do not edit.\n' +
  standaloneCode(ajv, ajv.compile(schema)) +
  '\n';
const destination = new URL('../src/generated/validate-record.mjs', import.meta.url);
if (process.argv.includes('--check')) {
  if ((await readFile(destination, 'utf8')) !== code)
    throw new Error('Static record validator is stale. Run pnpm contracts:generate.');
} else {
  await writeFile(destination, code);
}
