// Verify the deliberately public synthetic demo from an independent CI reader without credentials.
import assert from 'node:assert/strict';
const url = new URL(process.env.PUBLIC_DEMO_URL ?? '');
assert.equal(url.protocol, 'https:');
assert.ok(url.hostname.endsWith('.github.io'));
assert.equal(url.username, '');
assert.equal(url.password, '');
assert.equal(url.port, '');
assert.equal(url.search, '');
assert.equal(url.hash, '');
const id = url.pathname.match(/\/shares\/([a-f0-9-]{36})\/$/)?.[1];
assert.ok(id, 'Use a published snapshot link');
const response = await fetch(url, { redirect: 'error', signal: AbortSignal.timeout(20000) });
assert.equal(response.status, 200);
const html = await response.text();
assert.ok(html.includes(`<meta name='pme-snapshot' content='${id}'>`));
assert.ok(html.includes('A blue cover for my demo notebook'));
assert.ok(html.includes('I prefer blue'));
for (const prohibited of [
  '<script',
  '<iframe',
  'request_id',
  'decision_id',
  'version_id',
  'user_statement',
  'worker',
  'record_decision',
])
  assert.ok(!html.includes(prohibited), `Unexpected public content: ${prohibited}`);
console.log(
  'Anonymous independent reader verified the selected synthetic demo and static-only boundary.',
);
