# App-led onboarding delivery

Approved by D031. The user's journey is download, move the app to Applications, open, connect ChatGPT, install the bundled plugin, and send a prefilled synthetic test. No terminal, source checkout or development runtime belongs in that journey.

## Boundaries

Rust's decision engine and contracts remain unchanged. The local-runtime adapter owns durable worker access controls and test receipts. The desktop host prepares an app-owned local plugin catalog from bundled resources and opens only fixed, constructed host links. TypeScript presents progress and errors. Private memory is never included in the catalog or distribution.

A successful tool discovery, plugin-page opening, earlier tool use or failed record cannot establish readiness. A fresh request ID and the explicit synthetic choice must reach the same local store successfully before the UI reports recording verified. The worker identity is supplied by the worker; the app cannot independently authenticate that attribution. A verification receipt is historical evidence, not a live connection heartbeat.

Pause blocks all worker tool reads and writes, including already running helper processes. It preserves saved decisions and desktop access. Owner controls and worker calls use a cross-process lock; mutable setup metadata is separate from append-only decision versions. Exact retries keep their existing core guarantees.

## Delivery order

1. Add and test durable access controls, fresh-test verification, restart and pause behavior.
2. Bundle the existing plugin, construct a catalog with installed helper paths, and test from a packaged app without the repository.
3. Replace primary terminal setup with graphical actions, visible waiting/success/error states and an advanced configuration disclosure.
4. Build and install the preview, update the user guide, and create a reusable drag-to-Applications package. Keep focused commits and push each verified checkpoint.
5. Complete real ChatGPT Work and fresh-machine checks, then signing/notarization before normal public release.

## Current release constraints

This Mac has no valid code-signing identity. Any current package must be labeled an unsigned preview. ChatGPT app controls are unavailable to this agent, so the user must send the test chat and inspect the host integration. Public snapshot publishing and backup/restore remain separate Stage 01 work.

## References

- [OpenAI desktop command links](https://learn.chatgpt.com/docs/reference/commands): local plugin pages and a new chat with an unsent prefilled prompt.
- [Local plugin catalogs](https://developers.openai.com/plugins/build/plugins): repository-root-relative plugin paths and cached installation.
- [Tauri bundled resources](https://v2.tauri.app/develop/resources/): preserve the source directory as a resource.
