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

const connectionSchema = z
  .object({
    schema_version: z.literal(1),
    enabled: z.boolean(),
    test_request_id: z.string().nullable(),
    receipt: z
      .object({ version_id: z.string(), recorded_at: z.string(), worker: z.string() })
      .strict()
      .nullable(),
    package_available: z.boolean(),
    host_state: z.enum(['unavailable', 'missing', 'registered', 'disabled', 'conflict']),
    test_prompt: z.string().nullable(),
  })
  .strict();
export type WorkerStatus = z.infer<typeof connectionSchema>;
export async function getWorkerStatus(): Promise<WorkerStatus> {
  return connectionSchema.parse(await invoke<unknown>('connection_status'));
}
export async function connectChatGPT(): Promise<void> {
  await invoke('connect_chatgpt');
}
export async function startConnectionTest(): Promise<void> {
  await invoke('start_connection_test');
}
export async function pauseRecording(): Promise<void> {
  await invoke('pause_recording');
}
