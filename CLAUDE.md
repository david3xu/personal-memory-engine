# Development guide

## Project state

Personal Memory Engine is in design and repository preparation. Application implementation is paused until the first-stage build brief is approved. Rust is selected for the engine and TypeScript for the interface. Desktop packaging, UI framework, storage, and connection/publishing mechanisms remain open. No runtime implementation, verified MCP integration, application build, or hosted prototype exists yet.

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

For documentation changes, run `git diff --check` and verify local Markdown links. Rust and TypeScript formatting, lint, type, test, and build commands must be added when implementation establishes the toolchains; do not invent setup instructions or claim nonexistent CI.

## Data rules

Use synthetic public examples. Never commit private memory, raw source conversations, credentials, runtime exports, backups, or sensitive logs. Respect the ignore rules and inspect every staged file before publication.

The core decision rules should stay independent of MCP, storage drivers, UI, and hosting. Version handling and missing-information behavior need meaningful tests when implementation begins.

## License and release

The repository uses Apache License 2.0. Follow the stage workflow and maintain the changelog. A change to a stored user decision is different from the prototype improvement required by the supplied project brief.
