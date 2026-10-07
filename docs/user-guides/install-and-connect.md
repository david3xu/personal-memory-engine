# Install and connect Personal Memory Engine

This preview supports macOS Apple Silicon. Download the app, install it, and use its setup buttons. Rust, Node.js, a source checkout and terminal commands are not required.

**Preview limitation:** the current package is unsigned and not notarized. macOS may block downloaded builds; normal public installation still needs Developer ID signing, notarization and a fresh-machine test. Real ChatGPT Work capture is also awaiting a user walkthrough. This is a testing preview, not the complete Stage 01 release.

## Install

1. Obtain the macOS Apple Silicon preview DMG. No public release is published yet.
2. Open it and drag **Personal Memory Engine** to **Applications**.
3. Open **Personal Memory Engine**. Keep it in Applications; the recording helper and connector are inside the app.

If macOS blocks the preview, follow Apple's [instructions for opening a trusted app](https://support.apple.com/en-us/102445) only if you trust this project and understand it is an unsigned preview. Do not disable your Mac's security protections globally.

## Connect once

1. Open **Settings** in Personal Memory Engine and choose **Connect once**. This opts in to recording explicit decisions in supported chats on this desktop host.
2. Restart ChatGPT once to load the newly registered MCP tools. No plugin selection is required in each chat.
3. Return to Settings, expand **Check recording once**, and choose **Open test chat**.
4. Review the synthetic blue-notebook choice and press **Send**. The app does not send it for you.
5. Return to **Decisions** to see the saved card. Settings reports **Recording verified** only after that fresh test choice is saved locally.

Connect uses ChatGPT desktop's bundled connection manager to register this app's MCP; no separate CLI or developer runtime is needed. It preserves unrelated servers and refuses to replace a different server using the same name. You can inspect **personal-memory-engine** in ChatGPT's MCP settings. A registered configuration is not proof of successful recording.

The test link opens a **local Codex chat**. It does not select ChatGPT Work or verify cloud-mode access. Local host configuration is shared by supported desktop/Codex clients; availability in Work, web, or another mode must be tested separately. See OpenAI's [MCP guidance](https://learn.chatgpt.com/docs/extend/mcp?surface=desktop) and [desktop commands](https://learn.chatgpt.com/docs/reference/commands).

## Use a normal conversation

For example: “I choose the monthly plan because I want flexibility.” With recording enabled and the tools available, the AI is instructed to save that explicit choice without an `@` mention or “record this” command. Questions, brainstorming, and unaccepted AI suggestions are excluded. Only stated reasons, rejected alternatives and available evidence belong in the record. You can ask it not to save a particular choice.

For a changed choice, the AI reads the earlier record and appends a linked revision. Open a card to inspect its versions. The host and AI determine tool availability and invocation; MCP instructions alone cannot guarantee that every qualifying choice will be recorded. A successful tool result confirms the save.

The receipt confirms a completed record, not a live connection or authenticated worker identity. Closing the owner app does not revoke the helper's access.

## Pause and resume

In **Settings**, choose **Pause recording** to block all worker reads and writes, including running helpers. Cards remain available. Choose **Resume recording** to resume. To remove the host connection, remove **personal-memory-engine** in ChatGPT's MCP settings. An independently installed plugin is optional and can be managed separately; the primary flow does not need it.

## If setup does not complete

- **Host unavailable:** install/update ChatGPT desktop. Its bundled connection manager must be available.
- **Connection conflict:** another server uses the same name. Rename that server in the host settings before reconnecting; the app has not replaced it.
- **Tools disabled:** enable this server and its recording tools in ChatGPT's MCP settings.
- **Tools unavailable:** restart ChatGPT and use a new supported local chat. A cloud chat cannot automatically launch the local helper.
- **Test chat did not open:** expand **Chat did not open?**, copy the choice, and send it in a new supported desktop chat.
- **Still waiting:** a successful `record_decision` call is required. Tool discovery, a failed save, or a receipt from an earlier test cannot pass. Send the newest test if you started another.
- **Moved the app:** choose Connect again to repair the app-owned helper launcher.

## Other workers and local data

Expand **Connection help and other workers** for a compatible local MCP configuration and the private database location. Default macOS storage is `~/Library/Application Support/org.personalmemory.engine/memory.sqlite3`. Private memory is never included in the connector catalog or installer. Uninstalling the app does not automatically delete that separate storage directory.

Do not move only the SQLite file while a helper is using it; WAL files can contain committed decisions. Backup/restore and owner-selected public snapshots are later Stage 01 checkpoints. Public sharing is not in this preview.
