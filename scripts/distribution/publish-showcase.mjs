// Add a checked static showcase to an existing owner Pages branch without changing any snapshot or root file.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { resolve, dirname } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import {
  readShowcaseAssets,
  assertPreservedTree,
  showcaseMarker,
  showcaseFiles,
} from './showcase-assets.mjs';
export async function publishShowcase(api, repository, files) {
  assert.match(repository, /^[A-Za-z0-9_-]+\/[A-Za-z0-9_.-]+$/);
  const allowed = new Set(showcaseFiles.map((file) => 'showcase/' + file));
  for (const [path, bytes] of files) {
    assert.ok(
      allowed.has(path) && Buffer.isBuffer(bytes),
      'Only allowlisted showcase assets may change',
    );
  }
  const [owner, repo] = repository.split('/');
  const base = 'repos/' + repository;
  const account = await api('user');
  assert.equal(account.login.toLowerCase(), owner.toLowerCase(), 'Use your own public repository');
  const details = await api(base);
  assert.equal(details.private, false);
  assert.equal(details.archived, false);
  assert.equal(details.permissions.admin, true);
  assert.equal(details.name, repo);
  const pages = await api(base + '/pages');
  const baseURL =
    'https://' +
    owner.toLowerCase() +
    '.github.io/' +
    (repo.toLowerCase() === owner.toLowerCase() + '.github.io' ? '' : repo + '/');
  assert.equal(pages.html_url, baseURL);
  assert.equal(pages.source.branch, 'pme-public');
  assert.equal(pages.source.path, '/');
  assert.equal(pages.build_type, 'legacy');
  const head = await api(base + '/git/ref/heads/pme-public');
  const commit = await api(base + '/git/commits/' + head.object.sha);
  assert.ok(
    commit.message.startsWith('Personal Memory Engine:'),
    'Unrecognized publication workflow',
  );
  const before = await api(base + '/git/trees/' + commit.tree.sha + '?recursive=1');
  assert.equal(before.truncated, false);
  const root = await api(base + '/contents/index.html?ref=pme-public');
  const rootHTML = Buffer.from(root.content, 'base64').toString();
  assert.equal(root.encoding, 'base64');
  assert.ok(
    rootHTML.includes("<meta name='pme-snapshot' content='personal-memory-engine-public-root'>"),
    'Unrecognized public root',
  );
  for (const path of files.keys())
    assert.ok(path.startsWith('showcase/'), 'Only showcase paths may change');
  const existing = before.tree.find((entry) => entry.path === 'showcase/index.html');
  if (existing) {
    const previous = await api(base + '/git/blobs/' + existing.sha);
    assert.ok(
      Buffer.from(previous.content, 'base64').toString().includes(showcaseMarker),
      'Existing showcase belongs to another workflow',
    );
  }
  const entries = [];
  for (const [path, bytes] of files) {
    const blob = await api(base + '/git/blobs', 'POST', {
      content: bytes.toString('base64'),
      encoding: 'base64',
    });
    entries.push({ path, mode: '100644', type: 'blob', sha: blob.sha });
  }
  const tree = await api(base + '/git/trees', 'POST', {
    base_tree: commit.tree.sha,
    tree: entries,
  });
  const after = await api(base + '/git/trees/' + tree.sha + '?recursive=1');
  assertPreservedTree(before, after, new Set(files.keys()));
  for (const entry of entries) {
    const actual = after.tree.find((item) => item.path === entry.path);
    assert.ok(actual, 'Proposed tree must contain the requested static asset');
    for (const key of ['path', 'mode', 'type', 'sha']) assert.equal(actual[key], entry[key]);
  }
  const next = await api(base + '/git/commits', 'POST', {
    message: 'Personal Memory Engine: publish synthetic project showcase',
    tree: tree.sha,
    parents: [head.object.sha],
  });
  // A concurrent app publication must fail this non-forced update, never be overwritten.
  await api(base + '/git/refs/heads/pme-public', 'PATCH', { sha: next.sha, force: false });
  return { url: baseURL + 'showcase/', commit: next.sha };
}
function cliApi(endpoint, method = 'GET', body) {
  const args = ['api', '--hostname', 'github.com', '--method', method, endpoint];
  if (body) args.push('--input', '-');
  const result = spawnSync(process.env.GH_BIN || 'gh', args, {
    input: body ? JSON.stringify(body) : undefined,
    encoding: 'utf8',
    timeout: 30000,
    env: { ...process.env, GH_PROMPT_DISABLED: '1', GH_DEBUG: '' },
  });
  assert.equal(result.status, 0, 'GitHub request failed; no raw connector output is exposed');
  return result.stdout ? JSON.parse(result.stdout) : null;
}
if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  const repository = process.argv[2];
  assert.ok(repository, 'Usage: publish-showcase.mjs owner/repository');
  const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
  const files = await readShowcaseAssets(resolve(root, 'web/showcase/dist'));
  console.log(JSON.stringify(await publishShowcase(cliApi, repository, files)));
}
