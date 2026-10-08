// Reproduce a synthetic MCP revision in fresh isolated storage and build only public presentation assets.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { readFile, writeFile, mkdir, mkdtemp, rm, cp } from 'node:fs/promises';
import { join, resolve, dirname, delimiter } from 'node:path';
import { fileURLToPath } from 'node:url';
import { tmpdir, homedir } from 'node:os';
import { connectMcp } from './mcp-client.mjs';
import { readShowcaseAssets } from '../distribution/showcase-assets.mjs';
const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const source = join(root, 'web/showcase');
const output = join(source, 'dist');
const env = {
  ...process.env,
  PATH: join(homedir(), '.cargo/bin') + delimiter + (process.env.PATH ?? ''),
};
function cargo(args, capture = false) {
  const result = spawnSync('cargo', args, {
    cwd: root,
    env,
    encoding: 'utf8',
    stdio: capture ? 'pipe' : 'inherit',
  });
  assert.equal(result.status, 0, 'Cargo operation failed');
  return result.stdout;
}
await rm(output, { recursive: true, force: true });
await mkdir(output, { recursive: true });
const keep = process.argv.includes('--keep-demo');
if (keep) await mkdir(join(root, 'work'), { recursive: true });
const folder = await mkdtemp(join(keep ? join(root, 'work') : tmpdir(), 'pme-showcase-'));
let client;
try {
  cargo(['build', '-p', 'memory-local-runtime', '--bin', 'memory-mcp', '--locked']);
  const metadata = JSON.parse(
    cargo(['metadata', '--format-version', '1', '--no-deps', '--locked'], true),
  );
  const helper = join(
    metadata.target_directory,
    'debug',
    process.platform === 'win32' ? 'memory-mcp.exe' : 'memory-mcp',
  );
  const scenario = JSON.parse(await readFile(join(source, 'scenario.json'), 'utf8'));
  client = await connectMcp(helper, ['--data-dir', folder]);
  const tools = await client.call('tools/list');
  assert.deepEqual(tools.tools.map((tool) => tool.name).sort(), [
    'decision_history',
    'list_decisions',
    'record_decision',
  ]);
  const initial = {
    ...scenario.initial,
    request_id: 'synthetic-showcase-initial',
    user_confirmed: true,
    worker: 'Synthetic showcase protocol client',
  };
  const callTool = async (name, args) => {
    const result = await client.call('tools/call', { name, arguments: args });
    assert.equal(result.isError, false, 'The synthetic MCP call failed');
    return result.structuredContent;
  };
  const first = await callTool('record_decision', initial);
  assert.deepEqual(
    await callTool('record_decision', initial),
    first,
    'Retry must not create another version',
  );
  assert.equal(first.submission.evidence.length, 0, 'Unstated evidence must remain absent');
  const revision = {
    ...scenario.revision,
    request_id: 'synthetic-showcase-revision',
    user_confirmed: true,
    worker: initial.worker,
    decision_id: first.decision_id,
    supersedes_version_id: first.version_id,
  };
  const second = await callTool('record_decision', revision);
  await client.close();
  client = await connectMcp(helper, ['--data-dir', folder]);
  const { records } = await callTool('decision_history', { decision_id: first.decision_id });
  assert.equal(records.length, 2);
  assert.deepEqual(records, [second, first], 'Both unchanged versions must survive helper restart');
  assert.equal((await callTool('list_decisions', {})).records.length, 1);
  const input = join(folder, 'synthetic-records.json');
  await writeFile(input, JSON.stringify(records), { mode: 0o600 });
  cargo([
    'run',
    '-p',
    'memory-public-snapshot',
    '--example',
    'render_demo',
    '--locked',
    '--',
    input,
    join(output, 'example'),
  ]);
  const pagePath = join(output, 'example/index.html');
  let page = await readFile(pagePath, 'utf8');
  for (const record of records) {
    const readable =
      new Date(record.recorded_at).toLocaleString('en-GB', {
        dateStyle: 'medium',
        timeStyle: 'short',
        timeZone: 'UTC',
      }) + ' UTC';
    page = page.replaceAll(record.recorded_at, readable);
  }
  await writeFile(pagePath, page);
  await cp(join(source, 'index.html'), join(output, 'index.html'));
  await cp(join(source, 'style.css'), join(output, 'style.css'));
  await cp(join(source, 'assets'), join(output, 'assets'), { recursive: true });
  await writeFile(
    join(output, 'verification.json'),
    JSON.stringify(
      {
        schema_version: 1,
        mode: 'scripted-synthetic-mcp-fixture',
        verified_at: second.recorded_at,
        checks: [
          'real-stdio-tool-discovery',
          'record-and-idempotent-retry',
          'linked-revision',
          'two-versions-after-helper-restart',
          'public-positive-projection',
        ],
        ai_conversation_claim: false,
      },
      null,
      2,
    ) + '\n',
  );
  for (const value of [
    first.decision_id,
    first.version_id,
    second.version_id,
    initial.request_id,
    revision.request_id,
    initial.user_statement,
    revision.user_statement,
    folder,
  ]) {
    assert.ok(
      !page.includes(value),
      'The public example must omit transport and source-statement fields',
    );
  }
  await readShowcaseAssets(output);
  console.log(
    'Built the public showcase from isolated synthetic MCP records. No owner database was read.',
  );
  if (keep) console.log('Isolated screenshot database: ' + folder);
} finally {
  if (client) await client.close();
  if (!keep) await rm(folder, { recursive: true, force: true });
}
