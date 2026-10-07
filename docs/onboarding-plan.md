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

No Developer ID Application identity is available on this Mac. An Apple Development identity is present, but it does not complete normal outside-App-Store distribution. Any current package must be labeled an unsigned preview. ChatGPT app controls are unavailable to this agent, so the user must send the test chat and inspect the host integration. Public snapshot publishing and backup/restore remain separate Stage 01 work.

## References

- [OpenAI desktop command links](https://learn.chatgpt.com/docs/reference/commands): local plugin pages and a new chat with an unsent prefilled prompt.
- [Local plugin catalogs](https://developers.openai.com/plugins/build/plugins): repository-root-relative plugin paths and cached installation.
- [Tauri bundled resources](https://v2.tauri.app/develop/resources/): preserve the source directory as a resource.


## Connection correction — D032/D033

Preparing a plugin catalog was insufficient: the user's synthetic chat reported `record_decision` unavailable. The main setup will register a dedicated global MCP through the desktop application's bundled, supported CLI. This affects the same local Codex host, not all cloud ChatGPT modes. Registration uses only this app's fixed server name and launcher, rejects same-name conflicts, preserves unrelated host configuration, and is distinct from proof of recording. The owner opts in by choosing Connect. The MCP's initialization instructions guide automatic explicit-choice capture while enabled; no transcript watcher or hook is added.

The plugin remains an optional distribution of instructions and MCP metadata. Fix its portable schema and update its version, but remove plugin installation and `@` selection from the primary user journey. The connection test uses an unsent, ordinary synthetic choice plus a nonce, with no tool-schema fields in the user-facing prompt. The supported deep link opens a local Codex chat; label it accurately. A separate Work test is required before claiming Work support.

Verify registration against a temporary isolated host configuration using the real bundled CLI, and request MCP status/tool discovery through its app-server without creating a thread or calling a model. Never use a direct diagnostic write to satisfy the real owner's test receipt. On the decision page remove repeated status counters and empty optional sections; keep connection controls and diagnostic information in settings.
