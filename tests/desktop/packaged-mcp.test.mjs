// Verify the installed/bundled executable in an isolated synthetic store.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { connectMcp } from '../../scripts/development/mcp-client.mjs';
const helper = process.env.MCP_HELPER;
test(
  'packaged helper discovers tools and preserves revised records across restart',
  { skip: !helper },
  async () => {
    const folder = await mkdtemp(join(tmpdir(), 'personal-memory-package-'));
    let client;
    try {
      client = await connectMcp(helper, ['--data-dir', folder]);
      const tools = await client.call('tools/list');
      assert.equal(tools.tools.length, 3);
      const record = {
        request_id: 'package-check-1',
        user_confirmed: true,
        chosen_option: 'Synthetic demo: use a blue notebook',
        rationale: 'The synthetic participant prefers blue.',
        worker: 'Packaged protocol test',
      };
      const first = await client.call('tools/call', { name: 'record_decision', arguments: record });
      assert.equal(first.isError, false);
      const value = first.structuredContent;
      assert.equal(value.submission.evidence.length, 0);
      const second = await client.call('tools/call', {
        name: 'record_decision',
        arguments: {
          ...record,
          request_id: 'package-check-2',
          chosen_option: 'Synthetic demo: use a green notebook',
          decision_id: value.decision_id,
          supersedes_version_id: value.version_id,
        },
      });
      assert.equal(second.isError, false);
      await client.close();
      client = await connectMcp(helper, ['--data-dir', folder]);
      const versionsResponse = await client.call('tools/call', {
        name: 'decision_history',
        arguments: { decision_id: value.decision_id },
      });
      assert.equal(versionsResponse.structuredContent.records.length, 2);
      assert.deepEqual(versionsResponse.structuredContent.records[1], value);
    } finally {
      if (client) await client.close();
      await rm(folder, { recursive: true, force: true });
    }
  },
);
