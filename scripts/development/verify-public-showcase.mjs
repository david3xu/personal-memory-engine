// Verify every approved showcase asset anonymously from an independent reader without using owner services.
import assert from 'node:assert/strict';
import { mkdtemp, mkdir, writeFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, dirname } from 'node:path';
import { showcaseFiles, readShowcaseAssets } from '../distribution/showcase-assets.mjs';
const url = new URL(process.env.PUBLIC_SHOWCASE_URL ?? '');
assert.equal(url.protocol, 'https:');
assert.ok(url.hostname.endsWith('.github.io'));
assert.equal(url.username, '');
assert.equal(url.password, '');
assert.equal(url.port, '');
assert.equal(url.search, '');
assert.equal(url.hash, '');
assert.ok(url.pathname.endsWith('/showcase/'));
const folder = await mkdtemp(join(tmpdir(), 'pme-public-showcase-'));
try {
  for (const file of showcaseFiles) {
    const response = await fetch(new URL(file, url), {
      redirect: 'error',
      signal: AbortSignal.timeout(20000),
    });
    assert.equal(response.status, 200, 'A public showcase asset is unavailable');
    const bytes = Buffer.from(await response.arrayBuffer());
    assert.ok(bytes.length < 5_000_000, 'Public asset too large');
    const path = join(folder, file);
    await mkdir(dirname(path), { recursive: true });
    await writeFile(path, bytes);
  }
  await readShowcaseAssets(folder);
  console.log(
    'Anonymous reader verified the complete static showcase and synthetic history boundary.',
  );
} finally {
  await rm(folder, { recursive: true, force: true });
}
