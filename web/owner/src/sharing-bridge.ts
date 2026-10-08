// Validate the owner-only sharing transport without importing private capture contracts into public pages.
import { invoke } from '@tauri-apps/api/core';
import { z } from 'zod';
const version = z
  .object({
    chosen_option: z.string(),
    rationale: z.string().optional(),
    recorded_at: z.string(),
    alternatives: z.array(z.object({ option: z.string(), reason: z.string().optional() }).strict()),
    evidence: z.array(z.object({ content: z.string(), reference: z.string().optional() }).strict()),
  })
  .strict();
const snapshot = z
  .object({
    schema_version: z.literal(1),
    title: z.string(),
    cards: z.array(z.object({ versions: z.array(version) }).strict()),
  })
  .strict();
const publication = z
  .object({
    id: z.string(),
    snapshot,
    repository: z.string().nullable(),
    url: z.string().nullable(),
    commit: z.string().nullable(),
    status: z.enum(['draft', 'pending', 'live', 'withdrawal_pending', 'withdrawn']),
  })
  .strict();
const status = z
  .object({
    repository: z.string(),
    entries: z.array(publication),
    login: z
      .object({ code: z.string().nullable(), running: z.boolean(), error: z.string().nullable() })
      .strict(),
  })
  .strict();
export type Publication = z.infer<typeof publication>;
export type ShareStatus = z.infer<typeof status>;
export async function sharingStatus(): Promise<ShareStatus> {
  return status.parse(await invoke('sharing_status'));
}
export async function githubAccount(): Promise<string> {
  return z.string().parse(await invoke('github_account'));
}
export async function connectGitHub(): Promise<void> {
  await invoke('connect_github');
}
export async function openGitHubSignIn(): Promise<void> {
  await invoke('open_github_signin');
}
export async function prepareSnapshot(
  ids: string[],
  title: string,
  history: boolean,
  evidence: boolean,
): Promise<Publication> {
  return publication.parse(await invoke('prepare_snapshot', { ids, title, history, evidence }));
}
export async function publishSnapshot(id: string, repository: string): Promise<Publication> {
  return publication.parse(await invoke('publish_snapshot', { id, repository }));
}
export async function verifySnapshot(id: string): Promise<Publication> {
  return publication.parse(await invoke('verify_snapshot', { id }));
}
export async function withdrawSnapshot(id: string): Promise<Publication> {
  return publication.parse(await invoke('withdraw_snapshot', { id }));
}
export async function openSnapshot(id: string): Promise<void> {
  await invoke('open_snapshot', { id });
}
