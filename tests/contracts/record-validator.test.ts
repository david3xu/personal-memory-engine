// Verify unsupported records never enter the view, while absent reasons remain valid.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import validate from '../../web/owner/src/generated/validate-record.mjs';
const record = {
  schema_version: 1,
  decision_id: 'synthetic',
  version_id: 'version',
  recorded_at: '2026-10-07T00:00:00Z',
  submission: {
    request_id: 'test',
    user_confirmed: true,
    chosen_option: 'Synthetic choice',
    alternatives: [],
    evidence: [],
    worker: 'test',
  },
};
test('generated interface validator rejects unsupported and unexpected fields', () => {
  assert.equal(validate(record), true);
  assert.equal(validate({ ...record, schema_version: 2 }), false);
  assert.equal(validate({ ...record, unexpected: true }), false);
  assert.equal(
    validate({ ...record, submission: { ...record.submission, chosen_option: 1 } }),
    false,
  );
});
