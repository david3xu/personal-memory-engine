# Install and connect the first desktop prototype

The current development package targets macOS Apple Silicon. It includes the app and its standalone MCP helper; end users do not need Rust, Node.js, or a database server to open the app. This package is unsigned and not notarized. Installation on a clean second machine and normal public distribution still require verification; do not describe it as a production release.

## Open the app

1. Extract the development ZIP, then move **Personal Memory Engine.app** into your user Applications folder or the system Applications folder.
2. Open it. Its **Worker connection** page shows where private memory is stored and whether the bundled helper exists.
3. Keep the app in Applications. The local worker plugin finds its helper there; uninstalling the app does not remove the separate per-user data directory.

On macOS the default database is in `~/Library/Application Support/org.personalmemory.engine/memory.sqlite3`. Do not move only the SQLite file while a process is using it; active WAL files can contain committed data. Use the backup workflow when available.

## Connect ChatGPT Work on desktop

This repository includes a desktop-only plugin with recording instructions and stdio MCP configuration. It uses the installed app's helper; it does not launch code from the repository or require a cloud memory service. The portable manifest and older Codex compatibility manifest are kept together under `plugins/personal-memory-engine`.

For this development checkpoint, register and install through the supported local host CLI:

```sh
codex plugin marketplace add david3xu/personal-memory-engine --ref stage/01-working-prototype
codex plugin add personal-memory-engine@personal-memory-engine
```

For a local checkout, replace the first command's repository argument with that checkout's absolute path. The current maintainer installation has been verified through the host CLI. Public directory distribution and a one-click installer are later checks.

Open the ChatGPT desktop app, review the installed **Personal Memory Engine** plugin, and start a new Work chat with it enabled. Use its `record-decisions` skill or ask it to preserve your explicit choices. Tool availability depends on host/workspace policy. Installation alone does not prove a successful conversation-to-card capture. See the official [plugin guidance](https://learn.chatgpt.com/docs/plugins).

## Verify the real conversation flow

1. Explicitly choose an option in the Work conversation and ask the worker to record it. A clearly labeled synthetic test is useful for initial setup.
2. Confirm that the worker calls `record_decision` successfully and that the card appears in this app.
3. Revise that same choice explicitly. Confirm that the worker reads the current record, submits its two revision references, and that both versions remain visible.
4. Reopen the app and confirm both versions persist. Test a suggestion without selecting it: it must not become a user decision.

The app's **Last MCP tool use** is activity history, not a live connection or an independent verification of the worker's attribution.

## Other workers and cloud ChatGPT

A local MCP worker can use the configuration shown in the app directly. Cloud-hosted ChatGPT cannot reach a stdio process through a localhost URL. A separate compatible remote route is needed; OpenAI's [Secure MCP Tunnel](https://developers.openai.com/api/docs/guides/secure-mcp-tunnels) supports forwarding to a private stdio server but requires tunnel/account permissions. It is not required for a supported local desktop plugin. Neither route is claimed as verified for a particular Work chat until the conversation test succeeds.
