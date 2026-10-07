# Install and connect Personal Memory Engine

This preview supports macOS Apple Silicon. Download the app, install it, and use its setup buttons. Rust, Node.js, a source checkout and terminal commands are not required.

**Preview limitation:** the current package is unsigned and not notarized. macOS may block downloaded builds; normal public installation still needs Developer ID signing, notarization and a fresh-machine test. Real ChatGPT Work capture is also awaiting a user walkthrough. This is a testing preview, not the complete Stage 01 release.

## Install

1. Download the macOS Apple Silicon DMG from the project's preview release.
2. Open it and drag **Personal Memory Engine** to **Applications**.
3. Open **Personal Memory Engine**. Keep it in Applications; the recording helper and connector are inside the app.

If macOS blocks the preview, follow Apple's [instructions for opening a trusted app](https://support.apple.com/en-us/102445) only if you trust this project and understand it is an unsigned preview. Do not disable your Mac's security protections globally.

## Connect ChatGPT

1. Open **Connect ChatGPT** in the app.
2. Choose **Connect ChatGPT**. It prepares the bundled connector and opens its local plugin page in ChatGPT desktop.
3. Review **Personal Memory Engine** and choose **Install** in ChatGPT. Your account/workspace must support local desktop plugins in Work.
4. Return to Personal Memory Engine and choose **Open test chat**.
5. Review the synthetic blue-notebook choice in ChatGPT's message box and press **Send**. The app does not send it for you.
6. Return to Personal Memory Engine. **Recording verified** appears only after that fresh test choice is successfully saved in this local store. Its card appears under **Decisions**.

The app prepares its own local plugin catalog; it does not modify unrelated marketplaces or install a development runtime. Opening the plugin page does not prove installation. The supported links are documented in OpenAI's [desktop commands](https://learn.chatgpt.com/docs/reference/commands).

## Use it in a normal conversation

Enable or mention **Personal Memory Engine** in a desktop Work chat and ask it to record your explicit decisions. For example: “I choose the monthly plan because I want flexibility. Record this decision.” The worker should supply only your stated reasons, rejected alternatives and evidence. Suggestions you have not selected are not decisions.

When changing a choice, ask the worker to revise the saved decision. It reads the latest record and links the revision to that version. Open a card's history to inspect both choices. Missing reasons stay marked **Not stated**.

The setup receipt confirms a completed record, not a live connection or authenticated worker identity. Worker names are supplied by the worker. Closing the owner app does not itself revoke an installed plugin's access to its bundled helper.

## Pause and reconnect

Choose **Pause worker access** to block all worker reads and recordings, including running helpers. Saved cards remain available in the owner app. Choose **Reconnect ChatGPT** to resume and run a fresh test. To disable or uninstall the plugin in ChatGPT, open its plugin page there.

## If setup does not complete

- **ChatGPT did not open:** install/update ChatGPT desktop with Work and local-plugin support, then try Connect again. A plain cloud chat cannot start this local stdio helper.
- **Plugin page unavailable:** check host/workspace policy. The app remains usable as a local card viewer; this host route cannot be declared verified until a real test succeeds.
- **Tools missing after installation:** restart ChatGPT, then open a new test chat with the plugin enabled.
- **Test chat did not open:** expand **Test message / chat did not open**, copy the message, and send it in a new desktop Work chat with the plugin enabled.
- **Still waiting:** check that the worker called `record_decision` successfully. A failed save or a tool listing cannot pass. Starting another test replaces the pending verification request; send the newest message.
- **Moved or replaced the app:** choose Connect again. This updates the app-owned launcher used by installed copies.
- **Connector missing:** reinstall the complete app package.

## Other workers and local data

Expand **Advanced: other local workers and storage** for a compatible local MCP configuration and the private database location. Default macOS storage is `~/Library/Application Support/org.personalmemory.engine/memory.sqlite3`. Private memory is never included in the connector catalog or installer. Uninstalling the app does not automatically delete that separate storage directory.

Do not move only the SQLite file while a helper is using it; WAL files can contain committed decisions. Backup/restore and owner-selected public snapshots are later Stage 01 checkpoints. Public sharing is not in this preview.
