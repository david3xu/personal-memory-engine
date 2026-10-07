# Personal Memory Engine

A local-first, user-owned personal decision memory engine that records explicit choices, stated reasoning, and evolving history through MCP.

[Public GitHub repository](https://github.com/david3xu/personal-memory-engine)

## Project status

This project is in design and repository preparation. There is no runnable application, verified MCP integration, stable release, or published demo URL yet. Application implementation is paused until the first-stage build brief is approved. The project uses Apache License 2.0, and its selected GitHub owner is `david3xu`.

## What it aims to do

An existing AI worker submits a structured record when a user explicitly makes a decision. The record preserves the chosen option, stated rationale, rejected alternatives and their stated reasons, and available source evidence. Missing reasons and evidence remain absent.

When the user changes a choice, a new version links to the earlier record instead of silently overwriting it. A visual interface shows decision cards and preserved history. Related user-stated reasoning stays distinguishable from an actual changed choice.

The app does not scrape or monitor conversations and does not treat AI suggestions as user decisions. The project does not operate a central service holding users' personal memory. The user's authoritative memory is to be stored on their local machine; the storage format and connection arrangement remain undecided. See D003-v2 in the [decision ledger](docs/design-decisions.md).

## First prototype direction

Rust is selected for the memory engine (D025), and TypeScript for the interface (D026). Tauri is a candidate for desktop packaging; it has not been selected.

The confirmed direction is a minimum local-first personal decision memory engine that can expand into more capable and mature stages later. See D021 in the [decision ledger](docs/design-decisions.md). Easy installation and public-link sharing are also confirmed usability goals under D022; the installation and publishing mechanisms remain open.

The earlier first-stage direction was a browser-based web app with visual decision cards and history, plus public links for other people to visit. The interface format is now under review following the explicit local storage requirement. See D020 and the subsequent format discussion in the [decision ledger](docs/design-decisions.md).

The proposed minimum implementation is one user-controlled instance, one real worker connection, durable records, and a version timeline. A sample-data demonstration and read-only sharing of owner-selected records are proposals. The exact sharing scope, remaining framework/runtime choices, storage, access rules, and hosting are not yet finalized.

The supplied project brief requires a working public URL, the promised core behavior, and at least one verified improvement after the first version. Its build workflow specifies one ChatGPT Work conversation.

## Reuse

The proposed architecture separates decision rules from MCP, persistence, the viewer, and deployment. This lets the small prototype later serve a broader project without tying decision semantics to one host or database. A separate published library is not being created at this stage.

## Design records

Start with the [documentation index](docs/README.md).

- [Decision ledger](docs/design-decisions.md): user decisions, available supporting statements, proposals, and open questions.
- [First-stage build brief](docs/prototype-brief.md): draft target, implementation boundary, acceptance checks, roadmap, and choices still open.
- [User-first implementation plan](docs/implementation-plan.md): installation-to-sharing milestones, evidence, and proposed GitHub Actions/stage integration.
- [Directory and contribution boundaries](docs/repository-structure.md): canonical target layout, extension areas, and stronger foundation review proposals.
- [Repository foundation](docs/repository-foundation.md): reusable architecture rationale and open-source foundation.
- [Branches and remote preparation](docs/branching-and-releases.md): proposed implementation-stage branches and publication workflow.

## Development and contributions

The project currently contains documentation. Setup and check commands will be documented when the implementation stack exists and those commands have been verified. Future contributions may include tests, synthetic examples, adapters, documentation, and interface improvements.

Changes to decision semantics, record compatibility, data ownership, or security boundaries should be discussed before implementation. Read the [contribution guide](.github/CONTRIBUTING.md) and [code of conduct](.github/CODE_OF_CONDUCT.md). Use issues and pull requests for focused proposals and improvements; vulnerabilities follow the [security policy](.github/SECURITY.md).

## Personal data and security

Use synthetic data in public examples and tests. Do not commit personal memory databases, private source chat excerpts, exports, credentials, or runtime logs. The [security policy](.github/SECURITY.md) describes private vulnerability reporting and the current pre-implementation status. A dedicated confidential conduct-reporting contact has not yet been established.

The app can validate structure and preserve versions, but cannot independently prove that a worker accurately attributed a statement in an external chat. The prototype must test attribution behavior with real conversation examples.

## License

Licensed under [Apache License 2.0](LICENSE). See [NOTICE](NOTICE) for project attribution. The source-code license does not authorize publication of private user memory.
