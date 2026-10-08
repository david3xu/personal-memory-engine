// Prove the showcase publisher preserves unrelated public files and rejects unsafe deployment boundaries.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, mkdir, writeFile, symlink, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import {
  assertPreservedTree,
  readShowcaseAssets,
} from '../../scripts/distribution/showcase-assets.mjs';
import { publishShowcase } from '../../scripts/distribution/publish-showcase.mjs';
test('proposed publication cannot remove or modify existing snapshots or root', () => {
  const before = {
    truncated: false,
    tree: [
      { path: 'index.html', mode: '100644', type: 'blob', sha: 'root' },
      {
        path: 'shares/selected/index.html',
        mode: '100644',
        type: 'blob',
        sha: 'private-selection',
      },
      { path: 'showcase/index.html', mode: '100644', type: 'blob', sha: 'old-demo' },
    ],
  };
  const after = structuredClone(before);
  after.tree[2].sha = 'new-demo';
  assert.doesNotThrow(() => assertPreservedTree(before, after, new Set(['showcase/index.html'])));
  after.tree[1].sha = 'changed-snapshot';
  assert.throws(() => assertPreservedTree(before, after, new Set(['showcase/index.html'])));
  after.tree = after.tree.slice(1);
  assert.throws(() => assertPreservedTree(before, after, new Set(['showcase/index.html'])));
  assert.throws(() => assertPreservedTree({ ...before, truncated: true }, before, new Set()));
});
test('asset reading refuses symlink traversal and unexpected private files', async () => {
  const folder = await mkdtemp(join(tmpdir(), 'showcase-boundary-'));
  try {
    await mkdir(join(folder, 'safe'));
    await writeFile(join(folder, 'safe', 'private-memory.json'), 'private');
    await assert.rejects(readShowcaseAssets(folder), /allowlist/);
    await symlink(join(folder, 'safe'), join(folder, 'outside-link'));
    await assert.rejects(readShowcaseAssets(folder), /symlink/);
  } finally {
    await rm(folder, { recursive: true, force: true });
  }
});
function fakeApi({ differentSite = false, damagedTree = false } = {}) {
  const writes = [];
  const before = {
    truncated: false,
    tree: [
      { path: 'index.html', mode: '100644', type: 'blob', sha: 'root' },
      { path: 'shares/original/index.html', mode: '100644', type: 'blob', sha: 'selected' },
    ],
  };
  const api = async (endpoint, method = 'GET', body) => {
    if (method !== 'GET') {
      writes.push({ endpoint, method, body });
      if (endpoint.endsWith('/blobs')) return { sha: 'asset' };
      if (endpoint.endsWith('/trees')) return { sha: 'new-tree' };
      if (endpoint.endsWith('/commits')) return { sha: 'new-commit' };
      return {};
    }
    if (endpoint === 'user') return { login: 'owner' };
    if (endpoint === 'repos/owner/repo')
      return { private: false, archived: false, permissions: { admin: true }, name: 'repo' };
    if (endpoint.endsWith('/pages'))
      return {
        html_url: 'https://owner.github.io/repo/',
        source: { branch: differentSite ? 'main' : 'pme-public', path: '/' },
        build_type: 'legacy',
      };
    if (endpoint.includes('/git/ref/')) return { object: { sha: 'head' } };
    if (endpoint.includes('/git/commits/'))
      return {
        message: 'Personal Memory Engine: publish selected snapshot',
        tree: { sha: 'base-tree' },
      };
    if (endpoint.includes('/git/trees/base-tree')) return before;
    if (endpoint.includes('/git/trees/new-tree'))
      return {
        truncated: false,
        tree: [
          ...before.tree.map((entry) =>
            damagedTree && entry.path.startsWith('shares/')
              ? { ...entry, sha: 'corrupted' }
              : entry,
          ),
          { path: 'showcase/index.html', mode: '100644', type: 'blob', sha: 'asset' },
        ],
      };
    if (endpoint.includes('/contents/index.html'))
      return {
        encoding: 'base64',
        content: Buffer.from(
          "<meta name='pme-snapshot' content='personal-memory-engine-public-root'>",
        ).toString('base64'),
      };
    throw new Error('Unexpected endpoint');
  };
  return { api, writes };
}
test('publisher uses the existing base tree, one parent and a non-forced update', async () => {
  const { api, writes } = fakeApi();
  const result = await publishShowcase(
    api,
    'owner/repo',
    new Map([['showcase/index.html', Buffer.from('synthetic')]]),
  );
  assert.equal(result.url, 'https://owner.github.io/repo/showcase/');
  const tree = writes.find((write) => write.endpoint.endsWith('/trees')).body;
  assert.equal(tree.base_tree, 'base-tree');
  assert.deepEqual(
    tree.tree.map((entry) => entry.path),
    ['showcase/index.html'],
  );
  assert.deepEqual(writes.find((write) => write.endpoint.endsWith('/commits')).body.parents, [
    'head',
  ]);
  assert.deepEqual(writes.at(-1).body, { sha: 'new-commit', force: false });
});
test('different Pages ownership fails before writing and damaged trees never advance the branch', async () => {
  const wrong = fakeApi({ differentSite: true });
  await assert.rejects(publishShowcase(wrong.api, 'owner/repo', new Map()));
  assert.equal(wrong.writes.length, 0);
  const damaged = fakeApi({ damagedTree: true });
  await assert.rejects(
    publishShowcase(
      damaged.api,
      'owner/repo',
      new Map([['showcase/index.html', Buffer.from('synthetic')]]),
    ),
  );
  assert.ok(
    !damaged.writes.some(
      (write) => write.method === 'PATCH' || write.endpoint.endsWith('/commits'),
    ),
  );
});

test('publisher rejects path traversal before contacting GitHub', async () => {
  let contacted = false;
  await assert.rejects(
    publishShowcase(
      async () => {
        contacted = true;
      },
      'owner/repo',
      new Map([['showcase/../shares/original/index.html', Buffer.from('bad')]]),
    ),
  );
  assert.equal(contacted, false);
});
