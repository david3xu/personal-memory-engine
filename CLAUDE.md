# Development guide

## Project state

Stage 01 implementation is active. Rust owns decision contracts and operations; SQLite, MCP, and Tauri are separate adapters. Plain TypeScript/Vite is the initial interface implementation. The desktop installs/opens on the maintainer’s Mac and displays a real decision captured through a direct stdio MCP client. The app bundles the helper, registers its MCP once using ChatGPT desktop’s bundled connection manager, and opens an unsent local test chat. It verifies a fresh persisted synthetic choice. Decision cards stay separate from connection settings. The real desktop app-server discovers all three tools in isolated configuration. The owner’s synthetic chat test passed, its persisted receipt was verified, and this existing chat used the actual MCP tools to save D032/D033 without a plugin mention. The owner confirmed the successful test was in ChatGPT Work. Durable pause blocks worker access without changing cards. Clean-machine installation, recovery, still need end-to-end evidence. The owner-only sharing dialog now freezes selected public projections, submits them to an isolated GitHub Pages branch, verifies exact anonymous public bytes before showing a live link, and tracks withdrawal separately. The official GitHub CLI is bundled; it owns authentication, separate from private memory. The installed app published and verified a synthetic demo, withdrew a second copy, and preserved all local cards. The demo remains readable with the owner app closed. Independent runner verification is pending.

## Portable installation contracts

The app version is authoritative in Cargo's workspace package. Tauri inherits it when its version is omitted; the interface displays the host's `app_version`. The private frontend package has no separate version. The portable plugin's `plugin.json` owns its independently versioned connector; the Rust adapter derives it from that manifest and compatibility metadata must agree.

Host discovery checks an explicit absolute `PERSONAL_MEMORY_HOST_CLI` override first, then standard system/per-user ChatGPT and Codex app locations, then a bounded macOS Spotlight query for the verified compatible host bundle ID. An invalid override fails closed. The bundled CLI's internal layout remains host-specific; a different layout needs an adapter update or explicit override, not a global filesystem scan.

Opening/refreshing the owner app repairs only an existing app-owned launcher from its current bundled helper. The registered launcher path and separate database stay stable; pause and receipts are preserved. Launchers that are symlinks/non-files fail closed. Other workers' copied direct-helper configurations must be recopied after an app move. Preview help links are pinned to an existing documentation commit, independent of branch lifetime; release preparation must refresh that pin when the guide changes.

## Canonical records

- Product choices and their history: [design decision ledger](docs/design-decisions.md).
- Candidate first-stage scope and checks: [draft build brief](docs/prototype-brief.md).
- Work order, user journeys, and proposed automation: [implementation plan](docs/implementation-plan.md).
- Target layout, dependency boundaries, and review tiers: [repository structure](docs/repository-structure.md).
- Reusable architecture rationale and community foundation: [repository foundation](docs/repository-foundation.md).
- Stage workflow and remote settings: [branching and releases](docs/branching-and-releases.md).
- Contributor process: [contribution guide](.github/CONTRIBUTING.md).

Keep historical decision entries intact. Append revisions or corrections and distinguish user choices from assistant proposals. Update current-state documents when the actual project state changes.

## Current checks

For documentation changes, run `git diff --check` and verify local Markdown links. Run `pnpm install --frozen-lockfile`, then `pnpm check`. This runs formatting, lint, TypeScript, architecture boundaries, web compilation, Clippy, integrity/persistence tests, and generated-contract drift. Generate contracts and the static web validator with `pnpm contracts:generate`; do not edit generated files. Local Rust discovery is handled in `scripts/development/rust-env.sh`. CI repeats checks and separately builds the desktop on macOS. The verified `checks` and `desktop` jobs are required on `main`, including administrator merges; branches must be up to date. See the branching guide for the complete protection settings.

## Data rules

Use synthetic public examples. Never commit private memory, raw source conversations, credentials, runtime exports, backups, or sensitive logs. Respect the ignore rules and inspect every staged file before publication.

The core decision rules should stay independent of MCP, storage drivers, UI, and hosting. Implement SQLite through the storage adapter, Tauri through the owner host, and snapshot publication through an approved projection; do not put these dependencies or decision rules in the wrong layer. Version handling and missing-information behavior are covered by integrity/persistence checks; changes must preserve those guarantees.

## License and release

The repository uses Apache License 2.0. Follow the stage workflow and maintain the changelog. A change to a stored user decision is different from the prototype improvement required by the supplied project brief.


## Desktop and worker verification

- `pnpm desktop:build` builds the app with its independent MCP helper and connector resources.
- `pnpm desktop:package` also produces a drag-to-Applications DMG.
- `MEMORY_PACKAGE_SOURCE="<app>/Contents/Resources/plugin-source" MEMORY_PACKAGE_HELPER="<app>/Contents/MacOS/memory-mcp" cargo test -p memory-local-runtime --test worker_package --locked` verifies installed connector preparation and recording without developer runtimes.
- `MCP_HELPER="<app>/Contents/MacOS/memory-mcp" pnpm test:package` checks the packaged protocol flow in isolated synthetic storage.
- Local plugin manifests/launcher are under `plugins/personal-memory-engine`; graphical owner setup is in the [user guide](docs/user-guides/install-and-connect.md).
- `MEMORY_HOST_CLI="<ChatGPT.app>/Contents/Resources/codex-cli/bin/codex" cargo test -p memory-local-runtime --test host_connection --locked -- --ignored --nocapture` verifies real host registration and discovery in isolated configuration without creating a chat or calling a model.
- End-to-end evidence and remaining release checks are recorded in [implementation status](docs/implementation-status.md). Computer controls cannot operate ChatGPT in this session; Work-mode evidence is the owner’s report combined with the checked persisted test receipt, not an independently authenticated client-mode assertion.

The early app has selected publication controls; backup/recovery UI is pending. Do not label the preview a complete Stage 01 release.

## Public sharing boundaries

`memory-public-snapshot` is a pure positive projection and HTML renderer; `memory-github-publication` handles owner-account GitHub operations. The local runtime stores frozen publication metadata separately from SQLite. The owner desktop exposes publication commands; MCP has no publication tool. Nothing is selected by default. Earlier versions and source evidence require separate opt-in, followed by a full preview. Request/version/decision identifiers, worker metadata and source user statements are never projected.

The publishing adapter refuses private repositories and unrelated existing Pages configurations. It writes only static approved pages to an orphan `pme-public` branch and uses non-forced updates. Its root must match the app-owned landing page before an existing branch is changed. New publications have new URLs; revisions of private decisions do not silently update published copies. Pending operations survive restart and can be checked/retried. A withdrawal replaces the live page with a no-content notice; GitHub history and downloaded copies can remain. See the [sharing guide](docs/user-guides/share-decisions.md) for account prerequisites and credential limitations.
