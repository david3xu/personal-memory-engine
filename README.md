# Personal Memory Engine

A local-first, user-owned personal decision memory engine that records explicit choices, stated reasoning, and evolving history through MCP.

[Public GitHub repository](https://github.com/david3xu/personal-memory-engine)

## Project status

Stage 01 implementation is active on `stage/01-working-prototype`, authorized in D029. The first working checkpoint records decisions through MCP, stores linked versions in local SQLite, and displays cards/history in a packaged Tauri app. The macOS Apple Silicon app and local worker plugin have been installed and checked on the maintainer’s Mac. Actual ChatGPT Work capture remains a user walkthrough; public snapshot sharing and recovery are later checkpoints. There is no stable release or published demo URL yet. See the [implementation evidence](docs/implementation-status.md) and [install/connect guide](docs/user-guides/install-and-connect.md). The project uses Apache License 2.0, and its selected GitHub owner is `david3xu`.

## What it aims to do

An existing AI worker submits a structured record when a user explicitly makes a decision. The record preserves the chosen option, stated rationale, rejected alternatives and their stated reasons, and available source evidence. Missing reasons and evidence remain absent.

When the user changes a choice, a new version links to the earlier record instead of silently overwriting it. A visual interface shows decision cards and preserved history. Related user-stated reasoning stays distinguishable from an actual changed choice.

The app does not scrape or monitor conversations and does not treat AI suggestions as user decisions. The project does not operate a central service holding users' personal memory. The user's authoritative memory is to be stored on their local machine; embedded SQLite is selected for persistence, while the connection arrangement still needs verification. See D003-v3 in the [decision ledger](docs/design-decisions.md).

## First prototype direction

Rust is selected for the memory engine (D025), and TypeScript for the interface (D026). Ease of use comes first, with ChatGPT Work as the selected working context (D028); repository access and direct MCP recording are verified; a real Work conversation remains to be tested. Tauri desktop delivery, embedded SQLite storage, and owner-selected read-only public snapshots are approved (D020-v2, D003-v3, D012-v2).

The confirmed direction is a minimum local-first personal decision memory engine that can expand into more capable and mature stages later. See D021 in the [decision ledger](docs/design-decisions.md). Easy installation and public-link sharing are also confirmed usability goals under D022; the current development package targets macOS Apple Silicon; clean-machine installation, signing, hosting, and publication update/withdrawal details remain open.

The owner app will use Tauri; public readers use a browser URL showing deliberately published snapshots. A local browser viewer can support early development. D020-v2 replaces the earlier browser-only owner delivery direction while preserving that history in the [decision ledger](docs/design-decisions.md).

The proposed minimum implementation is one user-controlled instance, one real worker connection, durable records, and a version timeline. Synthetic examples are proposed for the public demonstration. Snapshot sharing is selected; the initial interface uses plain TypeScript/Vite and generated Rust record contracts. Selection controls, public snapshot contracts, access rules, and hosting remain to be implemented.

The supplied project brief requires a working public URL, the promised core behavior, and at least one verified improvement after the first version. Its build workflow specifies one ChatGPT Work conversation.

## Reuse

The implemented recording architecture separates decision rules from MCP, persistence, the viewer, and deployment. This lets the small prototype later serve a broader project without tying decision semantics to one host or database. A separate published library is not being created at this stage.

## Design records

Start with the [documentation index](docs/README.md).

- [Decision ledger](docs/design-decisions.md): user decisions, available supporting statements, proposals, and open questions.
- [First-stage build brief](docs/prototype-brief.md): draft target, implementation boundary, acceptance checks, roadmap, and choices still open.
- [User-first implementation plan](docs/implementation-plan.md): installation-to-sharing milestones, evidence, and proposed GitHub Actions/stage integration.
- [Directory and contribution boundaries](docs/repository-structure.md): canonical target layout, extension areas, and stronger foundation review proposals.
- [Repository foundation](docs/repository-foundation.md): reusable architecture rationale and open-source foundation.
- [Branches and remote preparation](docs/branching-and-releases.md): proposed implementation-stage branches and publication workflow.

## Development and contributions

Install Rust stable, Node.js 24 or later, and pnpm 10.32.1, then run `pnpm install --frozen-lockfile` and `pnpm check`. The verified checks cover formatting, lint, TypeScript, dependency boundaries, frontend compilation, Rust warnings, persistence/integrity tests, and generated-contract drift. Run `pnpm desktop:dev` for the desktop or `pnpm desktop:build` for the app bundle (macOS needs Xcode command-line tools). The bundle includes its MCP helper. Initial builds are unsigned development packages, not a notarized public release. Contributions may include tests, synthetic examples, adapters, documentation, and interface improvements.

Changes to decision semantics, record compatibility, data ownership, or security boundaries should be discussed before implementation. Read the [contribution guide](.github/CONTRIBUTING.md) and [code of conduct](.github/CODE_OF_CONDUCT.md). Use issues and pull requests for focused proposals and improvements; vulnerabilities follow the [security policy](.github/SECURITY.md).

## Personal data and security

Use synthetic data in public examples and tests. Do not commit personal memory databases, private source chat excerpts, exports, credentials, or runtime logs. The [security policy](.github/SECURITY.md) describes private vulnerability reporting and the current development-prototype status. A dedicated confidential conduct-reporting contact has not yet been established.

The app can validate structure and preserve versions, but cannot independently prove that a worker accurately attributed a statement in an external chat. The prototype must test attribution behavior with real conversation examples.

## License

Licensed under [Apache License 2.0](LICENSE). See [NOTICE](NOTICE) for project attribution. The source-code license does not authorize publication of private user memory.
