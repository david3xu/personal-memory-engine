// Catch drift between portable/desktop manifests and accidental remote recording configuration.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
const root = new URL('../../plugins/personal-memory-engine/', import.meta.url);
const read = (path: string) => JSON.parse(readFileSync(new URL(path, root), 'utf8'));
test('worker plugin retains one identity and local stdio in both host formats', () => {
  const portable = read('plugin.json');
  const compatibility = read('.codex-plugin/plugin.json');
  assert.equal(portable.name, compatibility.name);
  assert.equal(portable.version, compatibility.version);
  assert.deepEqual(portable.extensions['com.openai'].interface, compatibility.interface);
  assert.ok(portable.extensions['com.openai'].interface.shortDescription.length <= 30);
  for (const file of ['mcp.json', '.mcp.json']) {
    const servers = Object.values(read(file).mcpServers) as Record<string, unknown>[];
    assert.equal(servers.length, 1);
    for (const server of servers) {
      assert.equal(server.type, 'stdio');
      assert.equal(server.url, undefined);
    }
  }
  assert.equal(portable.hooks, undefined);
  assert.equal(compatibility.hooks, undefined);
});
