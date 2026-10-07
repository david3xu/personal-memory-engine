---
name: record-decisions
description: Record explicit user choices through the local Personal Memory Engine MCP when the user asks to keep decisions or invokes this workflow. Use for a new confirmed choice or a user-confirmed revision, not for ordinary discussion, AI suggestions, or automatic chat logging.
---

# Preserve an explicit decision

Use only the local Personal Memory Engine tools. This workflow has no access to arbitrary chat history and must not scrape, watch, or upload conversations.

1. Confirm there is an explicit user choice within the requested decision-recording scope. Questions, brainstorms, proposals, and unaccepted AI suggestions do not qualify. If intent is ambiguous, ask a focused question before recording it.
2. Submit one choice in `record_decision`. Set `user_confirmed` true only for that explicit user choice. Supply an accurate worker name. Use a new unique `request_id`; reuse it with identical arguments after an uncertain retry.
3. Include only rationale, rejected alternatives/reasons, user statements, and evidence actually stated or available. Omit missing information. Do not fill in plausible reasons, invented quotes, relationships, timestamps, or source links. Do not treat your suggested alternatives as rejected by the user.
4. For a changed choice, first use `list_decisions` and `decision_history` to find the correct decision and its current version. Supply both `decision_id` and `supersedes_version_id`. On a stale revision error, reread history; do not overwrite or blindly revise another version.
5. Acknowledge saving only after a successful tool result. If the helper is absent or storage fails, explain that no successful save is confirmed and help the user open/install the app or reconnect.

`list_decisions` returns the most recent 200 current versions. `decision_history` returns all preserved versions of one decision, newest first. Worker attribution remains an assertion; the server cannot independently verify a conversation.

These tools save/read local memory only. Publication is a separate owner action in the app. Never claim these tools publish a URL, synchronize a cloud account, or grant access to someone else.
