# Development guide

## Project state

Stage 01 implementation is active. Rust owns decision contracts and operations; SQLite, MCP, and Tauri are separate adapters. Plain TypeScript/Vite is the initial interface implementation. Real worker connectivity, complete installation, recovery, and snapshot hosting still need end-to-end evidence.

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

For documentation changes, run `git diff --check` and verify local Markdown links. Run `pnpm install --frozen-lockfile`, then `pnpm check`. This runs formatting, lint, TypeScript, architecture boundaries, web compilation, Clippy, integrity/persistence tests, and generated-contract drift. Generate contracts with `cargo run -p memory-engine --example export_contracts`; do not edit generated files. Local Rust discovery is handled in `scripts/development/rust-env.sh`. CI repeats checks and separately builds the desktop on macOS.

## Data rules

Use synthetic public examples. Never commit private memory, raw source conversations, credentials, runtime exports, backups, or sensitive logs. Respect the ignore rules and inspect every staged file before publication.

The core decision rules should stay independent of MCP, storage drivers, UI, and hosting. Implement SQLite through the storage adapter, Tauri through the owner host, and snapshot publication through an approved projection; do not put these dependencies or decision rules in the wrong layer. Version handling and missing-information behavior need meaningful tests when implementation begins.

## License and release

The repository uses Apache License 2.0. Follow the stage workflow and maintain the changelog. A change to a stored user decision is different from the prototype improvement required by the supplied project brief.
