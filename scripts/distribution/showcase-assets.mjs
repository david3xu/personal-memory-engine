// Validate the explicit public showcase asset boundary before any publication operation.
import assert from 'node:assert/strict';
import { readdir, readFile, lstat } from 'node:fs/promises';
import { join } from 'node:path';
export const showcaseFiles = Object.freeze([
  'index.html',
  'style.css',
  'example/index.html',
  'example/snapshot.json',
  'verification.json',
  'assets/decision-workspace.jpg',
  'assets/decision-history.jpg',
  'assets/sharing-preview.jpg',
]);
export const showcaseMarker = 'personal-memory-engine-showcase-v1';
export async function readShowcaseAssets(directory) {
  const found = [];
  async function walk(folder, prefix = '') {
    assert.ok((await lstat(folder)).isDirectory(), 'Asset directory must not be a symlink');
    for (const name of await readdir(folder)) {
      const path = join(folder, name);
      const item = await lstat(path);
      assert.ok(!item.isSymbolicLink(), 'Public assets must not be symlinks');
      const relative = prefix + name;
      if (item.isDirectory()) await walk(path, relative + '/');
      else {
        assert.ok(item.isFile(), 'Public assets must be regular files');
        found.push(relative);
      }
    }
  }
  await walk(directory);
  assert.deepEqual(
    found.sort(),
    [...showcaseFiles].sort(),
    'Publish only the explicit public asset allowlist',
  );
  const files = new Map();
  for (const file of showcaseFiles) {
    const bytes = await readFile(join(directory, file));
    assert.ok(bytes.length > 0 && bytes.length < 5_000_000, 'Public asset size is unsupported');
    if (file.endsWith('.jpg'))
      assert.ok(
        bytes.subarray(0, 3).equals(Buffer.from([255, 216, 255])),
        'Screenshots must be actual JPEG files',
      );
    else {
      const text = bytes.toString('utf8');
      for (const forbidden of [
        'file://',
        '/Users/',
        'memory.sqlite3',
        'onboarding-',
        '<script',
        '<iframe',
      ]) {
        assert.ok(
          !text.includes(forbidden),
          'Unexpected private path or active content in public assets',
        );
      }
      if (file.endsWith('.html')) {
        assert.ok(
          text.includes('Content-Security-Policy'),
          'Static pages must constrain active content',
        );
        assert.ok(
          !/<(?:form|input|button)\b/i.test(text),
          'The showcase must not expose owner commands or accept data',
        );
        for (const match of text.matchAll(/(src|href)=["']([^"']+)["']/g)) {
          const target = match[2];
          if (match[1] === 'src')
            assert.ok(!target.includes(':'), 'Images must be local static assets');
          assert.ok(!target.startsWith('//'), 'No protocol-relative external assets');
          if (target.startsWith('https:'))
            assert.ok(
              target.startsWith('https://github.com/'),
              'Only explicit GitHub documentation/source links are allowed',
            );
          else assert.ok(!/^[a-z][a-z0-9+.-]*:/i.test(target), 'Unsupported asset URL');
        }
      }
    }
    files.set('showcase/' + file, bytes);
  }
  assert.ok(
    files.get('showcase/index.html').toString().includes(showcaseMarker),
    'Missing showcase ownership marker',
  );
  const receipt = JSON.parse(files.get('showcase/verification.json').toString());
  assert.equal(receipt.mode, 'scripted-synthetic-mcp-fixture');
  assert.equal(receipt.ai_conversation_claim, false);
  const snapshot = JSON.parse(files.get('showcase/example/snapshot.json').toString());
  assert.deepEqual(Object.keys(snapshot).sort(), ['cards', 'schema_version', 'title']);
  assert.equal(snapshot.schema_version, 1);
  assert.equal(snapshot.cards.length, 1);
  assert.deepEqual(Object.keys(snapshot.cards[0]), ['versions']);
  const versions = snapshot.cards[0].versions;
  assert.equal(versions.length, 2);
  for (const version of versions) {
    assert.deepEqual(Object.keys(version).sort(), [
      'alternatives',
      'chosen_option',
      'evidence',
      'rationale',
      'recorded_at',
    ]);
    for (const alt of version.alternatives)
      assert.deepEqual(Object.keys(alt).sort(), ['option', 'reason']);
    for (const item of version.evidence)
      assert.deepEqual(Object.keys(item).sort(), ['content', 'reference']);
  }
  assert.equal(versions[0].chosen_option, 'Take the train for the weekend trip');
  assert.equal(versions[1].chosen_option, 'Take the coach for the weekend trip');
  const example = files.get('showcase/example/index.html').toString();
  assert.ok(
    example.includes('SYNTHETIC DEMONSTRATION') && example.includes('Earlier versions (1)'),
  );
  for (const forbidden of ['decision_id', 'version_id', 'request_id', 'user_statement', 'worker']) {
    assert.ok(
      !example.includes(forbidden) && !JSON.stringify(snapshot).includes(forbidden),
      'Private transport fields must not enter the public example',
    );
  }
  return files;
}
export function assertPreservedTree(before, after, allowedPaths) {
  assert.equal(before.truncated, false, 'Cannot verify a truncated existing tree');
  assert.equal(after.truncated, false, 'Cannot verify a truncated proposed tree');
  // Ancestor tree objects change when a child changes; compare all leaf entries outside the explicit allowlist.
  const leaves = (tree) =>
    tree.tree
      .filter((entry) => entry.type !== 'tree' && !allowedPaths.has(entry.path))
      .map(({ path, mode, type, sha }) => ({ path, mode, type, sha }))
      .sort((a, b) => a.path.localeCompare(b.path));
  assert.deepEqual(
    leaves(after),
    leaves(before),
    'Publication must preserve every non-showcase file, including all shared snapshots',
  );
}
