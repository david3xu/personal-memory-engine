---
name: record-decisions
description: Record explicit user choices through the local Personal Memory Engine MCP when the owner has enabled decision recording and makes an explicit choice or confirms a revision in normal conversation. No plugin mention or separate save request is needed. Exclude questions, brainstorming, AI suggestions, and transcript logging.
---

# Preserve an explicit decision

Use only the local Personal Memory Engine tools. This workflow has no access to arbitrary chat history and must not scrape, watch, or upload conversations.

1. While recording is enabled, recognize an explicit user choice in the normal conversation and record it without requiring an `@` mention or another save request. Respect a request not to save. Questions, brainstorms, proposals, and unaccepted AI suggestions do not qualify. If intent is ambiguous, ask a focused question before recording it.
2. Submit one choice in `record_decision`. Set `user_confirmed` true only for that explicit user choice. Supply an accurate worker name. Use a new unique `request_id`; reuse it with identical arguments after an uncertain retry. For the clearly labeled synthetic setup choice only, use its Setup reference as `request_id`.
3. Include only rationale, rejected alternatives/reasons, user statements, and evidence actually stated or available. Omit missing information. Do not fill in plausible reasons, invented quotes, relationships, timestamps, or source links. Do not treat your suggested alternatives as rejected by the user.
4. For a changed choice, first use `list_decisions` and `decision_history` to find the correct decision and its current version. Supply both `decision_id` and `supersedes_version_id`. On a stale revision error, reread history; do not overwrite or blindly revise another version.
5. Acknowledge saving only after a successful tool result. If the helper is absent or storage fails, explain that no successful save is confirmed and help the user open/install the app or reconnect.

`list_decisions` returns the most recent 200 current versions. `decision_history` returns all preserved versions of one decision, newest first. Worker attribution remains an assertion; the server cannot independently verify a conversation.

These tools save/read local memory only. Publication is a separate owner action in the app. Never claim these tools publish a URL, synchronize a cloud account, or grant access to someone else.
