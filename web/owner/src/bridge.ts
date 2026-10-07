// Validate host responses using generated domain schemas and explicit transport envelopes.
import { invoke, isTauri } from '@tauri-apps/api/core';
import { z } from 'zod';
import type { DecisionVersion } from '../../../contracts/generated/records';
import validateRecord from './generated/validate-record.mjs';
const statusSchema = z
  .object({
    data_dir: z.string(),
    database_path: z.string(),
    helper_path: z.string(),
    helper_available: z.boolean(),
    last_tool_use: z.string().nullable(),
    app_version: z.string(),
  })
  .strict();
export type LocalStatus = z.infer<typeof statusSchema>;
export const desktopAvailable = isTauri();
function records(value: unknown): DecisionVersion[] {
  return z
    .array(z.unknown())
    .parse(value)
    .map((item) => {
      if (!validateRecord(item))
        throw new Error('A stored card has an unsupported format. Your data has not been changed.');
      return item;
    });
}
export async function getStatus(): Promise<LocalStatus> {
  return statusSchema.parse(await invoke<unknown>('local_status'));
}
export async function getDecisions(): Promise<DecisionVersion[]> {
  return records(await invoke<unknown>('list_decisions'));
}
export async function getHistory(id: string): Promise<DecisionVersion[]> {
  return records(await invoke<unknown>('decision_history', { decisionId: id }));
}
export async function openConnectionDocs(): Promise<void> {
  await invoke('open_connection_docs');
}
