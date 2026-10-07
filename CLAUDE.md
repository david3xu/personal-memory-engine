# Development guide

## Project state

Stage 01 implementation is active. Rust owns decision contracts and operations; SQLite, MCP, and Tauri are separate adapters. Plain TypeScript/Vite is the initial interface implementation. The desktop installs/opens on the maintainer’s Mac and displays a real decision captured through a direct stdio MCP client. The local worker plugin installs/enables and discovers tools. Real ChatGPT Work capture, clean-machine installation, recovery, and snapshot hosting still need end-to-end evidence.

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

- `pnpm desktop:build` builds the app and bundles its independent MCP helper.
- `MCP_HELPER="<app>/Contents/MacOS/memory-mcp" pnpm test:package` checks the packaged protocol flow in isolated synthetic storage.
- Local plugin manifests/launcher are under `plugins/personal-memory-engine`; host installation commands are in the [user guide](docs/user-guides/install-and-connect.md).
- End-to-end evidence and remaining release checks are recorded in [implementation status](docs/implementation-status.md). Computer controls cannot operate the ChatGPT app in this session; the real Work conversation test needs the user.

The early app has no publication control or backup/recovery UI yet. Those are separate Stage 01 checkpoints; do not label this first recording checkpoint a complete Stage 01 release.
