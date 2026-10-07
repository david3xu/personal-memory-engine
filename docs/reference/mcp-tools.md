# Local MCP tools

The helper is `memory-mcp --data-dir <application-data-directory>`, communicating over stdio. Omitting `--data-dir` uses the same OS per-user application directory as the desktop. It does not accept a public URL or start an HTTP listener. A local worker launches it directly. ChatGPT requires a compatible connector or private MCP tunnel route; placing a localhost address in ChatGPT is not sufficient.

## record_decision

Required fields are `request_id`, `user_confirmed`, `chosen_option`, and `worker`. `user_confirmed` must be true only for an explicit user choice. It is a worker assertion, not independently verified user consent. Optional fields are `rationale`, `alternatives` (option/optional reason), `evidence` (content/optional reference), and `user_statement`. Do not invent absent information or references. Empty supplied strings are rejected; omit missing fields. Arrays default to empty; unknown fields are rejected.

For a new decision omit both `decision_id` and `supersedes_version_id`. To revise, first retrieve current history and supply both references. A stale version or a reference to another decision fails. Reuse an identical `request_id` and payload on retries; a different submission needs a new key.

The result is the durable `DecisionVersion`, including engine schema version, decision/version IDs, local recording time, and the submitted fields. Recording time does not establish when the user made the choice. Generated contracts are in `contracts/generated`; runtime bounds are enforced by the engine.

## list_decisions

No arguments. Returns `{records: [...]}` containing latest versions of the most recent 200 decisions, newest first. This limit is deliberate and does not erase older records. Full pagination is later work.

## decision_history

Takes `{decision_id: "..."}`. Returns every preserved version of that decision, newest first. Unknown IDs return an error.

## Boundary

No tool publishes records, deletes prior versions, reads external chats, or calls a model. The tools cannot independently verify the worker's attribution or invent source evidence. Use synthetic real-conversation checks before claiming reliable worker behavior.
