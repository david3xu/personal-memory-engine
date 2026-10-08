// Generated from memory-engine; do not edit.
export type Alternative = { option: string, reason?: string, };
export type Evidence = { 
/**
 * Supplied source text or description; never generated missing evidence.
 */
content: string, reference?: string, };
export type CaptureDecision = { 
/**
 * Reuse this exact key on retries; never reuse it for a different submission.
 */
request_id: string, 
/**
 * Must be true only after an explicit user choice, never an AI suggestion.
 */
user_confirmed: boolean, chosen_option: string, rationale?: string, alternatives: Array<Alternative>, evidence: Array<Evidence>, 
/**
 * Worker name is attribution metadata, not independent proof of user intent.
 */
worker: string, user_statement?: string, 
/**
 * Both revision references are absent for a new decision, or both supplied.
 */
decision_id?: string, supersedes_version_id?: string, };
export type DecisionVersion = { schema_version: 1, decision_id: string, version_id: string, 
/**
 * Server recording time, not an invented time of the user's actual decision.
 */
recorded_at: string, submission: CaptureDecision, };
