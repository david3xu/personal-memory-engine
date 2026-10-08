# Personal Memory Engine

A user-owned app for remembering your decisions: what you chose, why, and how your thinking changed. Your memory stays on your device.

[Explore the prototype](https://david3xu.github.io/personal-memory-engine/showcase/) · [Get started](docs/user-guides/install-and-connect.md) · [Documentation](docs/README.md)

## How it works

1. **Connect once.** Connect the local engine to a supported AI worker through MCP.
2. **Chat normally.** The AI records your explicit choices, using only reasons and evidence you actually provide.
3. **Review your decisions.** See cards and preserved versions when a choice changes.
4. **Share deliberately.** Preview selected decisions and publish a read-only link. Your private memory stays local.

Only explicit user decisions are recorded. It does not scrape chats or save AI suggestions as your choices. Missing reasons remain missing, and revisions preserve earlier records.

## Current prototype

Local recording, decision history and selected GitHub Pages sharing are working. The showcase includes actual app views and a synthetic decision history; readers can explore it without installing the app.

The desktop preview targets macOS Apple Silicon. It remains unsigned; signing, fresh-user installation testing and backup/restore are still in progress. Connection support depends on the AI host. See [verified behavior and remaining work](docs/implementation-status.md).

## Build and contribute

The foundation uses **Rust** for the engine and **TypeScript** for the interface, with Tauri and local SQLite storage. Decision rules stay separate from worker connections, presentation and publishing.

Start with the [development guide](CLAUDE.md) and [contribution guide](.github/CONTRIBUTING.md). Use synthetic examples and discuss changes to decision integrity, storage or access boundaries before implementation. Report vulnerabilities through the [security policy](.github/SECURITY.md).

Licensed under [Apache 2.0](LICENSE). See [NOTICE](NOTICE) for attribution.
