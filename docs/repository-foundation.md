# Personal Memory Engine — Repository foundation proposal

Recorded: 2026-10-07 (Australia/Perth)

Status: reusable foundation design with Stage 01 implementation started under D029. The Rust core, local SQLite adapter, contracts, and checks are implemented; future extensions remain proposals. The [public repository](https://github.com/david3xu/personal-memory-engine) has been created under `david3xu`. Apache License 2.0 is selected, with attribution to the verified owner identifier. Selected languages and remaining implementation choices are recorded in the [build brief](prototype-brief.md); hosting remains open; confidential conduct-reporting contact details are not yet supplied.

## Purpose and design boundary

The user wants a small first-stage implementation that can later become a reusable part of a broader project and welcome outside contributions. The confirmed product requirements and their rationale are recorded in [the decision ledger](design-decisions.md). The supplied project brief and its acceptance criteria are recorded there as well.

This document describes how to organize the repository around those requirements. It does not expand the approved product scope or approve a particular stack.

## Reusable architecture

Keep one repository and one application instance initially. The [directory and contribution-boundary proposal](repository-structure.md) defines the canonical module layout, dependency direction, extension areas, and stronger foundation review. Decision rules stay independent of worker transport, storage drivers, interface, and hosting. Extract a library only when another project needs the implemented engine.

## Long-term product and growth proposal

This is a future design proposal, not approved implementation scope. The user has asked that architecture consider a useful mature product as well as the first prototype. Local authoritative SQLite storage remains governed by D003-v3, and Tauri owner delivery and selected hosted snapshots are now approved; the current choices and their history are in the build brief/ledger.

The proposed product is a personal decision memory engine usable across AI workers: capture explicit choices, retrieve their stated rationale, examine changes, and deliberately share selected records. Future retrieval could help a worker consult earlier choices with the owner's permission. No passive chat monitoring or automatic conversion of AI advice into user decisions is introduced.

### Interface and deployment choices

A local engine should own the decision rules and persistence. Its operations should be callable without requiring a network connection between internal modules. A local browser viewer could call a loopback API; a desktop package could embed the same engine and viewer. A future mobile client would require its own storage and synchronization design. A browser prototype therefore need not determine the mature application's packaging, but reusing a UI or core across platforms requires deliberate boundaries and verification.

The approved first-stage Tauri application should manage its local engine and guide worker connection. Packaging details and actual connection access remain to be verified; the first capture checkpoint may use a development browser viewer. Public viewers should consume an explicitly selected published representation, not obtain unrestricted access to the local memory store or recording tools.

### Different meanings of scale

| Growth dimension | Proposed response | Unresolved design work |
| --- | --- | --- |
| More users | Independent local instances keep each person's authoritative memory on their own machine | Distribution, updates, support, and any optional user-controlled shared infrastructure |
| More decisions per user | Indexed queries, pagination, and derived views over preserved versions | Measure actual volume, query cost, and storage limits before changing databases |
| More AI workers | Reuse the decision contract through MCP and scoped adapters | Worker-specific connection support, attribution checks, and permissions |
| Multiple devices | Optional synchronization or replication of version records | Device identities, encryption and key recovery, backup, deletion, and concurrent revision handling |
| Shared projects | Explicitly owned shared collections distinct from personal memory | Access rules, authorship, revocation limits, and deciding who can resolve competing choices |
| More public readers | Host deliberately published snapshots independently of the local engine, or use an owner-controlled live gateway | Published data exists outside the machine for snapshots; live access depends on the owner's machine and connectivity |

The number of users alone does not require a project-operated central memory database. Any future hosted sync service, even one carrying encrypted records, would need a separate ownership decision; it is not approved under the current no-operated-memory-service boundary.

### Foundations to consider before coding

Use stable decision and version identifiers, explicit predecessor references, a versioned portable record format, source attribution, and clear recording-versus-decision timestamps. Keep missing reasons absent. Retries should not duplicate a decision. Preserve competing revisions rather than deciding semantic precedence from timestamps alone; an append-only history does not by itself solve synchronization conflicts.

Keep the core independent of storage drivers, worker transports, user interfaces, and publication. Plan schema migration, export/import, backup/restore, and deliberate deletion so users can retain and move their records as the app evolves. These details need selection in the build brief or later capability briefs; synchronization and team collaboration are not first-stage requirements.

The [build brief](prototype-brief.md) defines candidate scope, while the [implementation plan](implementation-plan.md) organizes its user journeys and verification. The build brief records the approved Tauri, SQLite, and public snapshot choices. Exact operation/record contracts, first-platform distribution, recovery, MCP access, and publication setup/lifecycle still need design at their relevant steps.

Reference: [Ink & Switch's local-first software research](https://www.inkandswitch.com/essay/local-first/) discusses ownership, local storage, multiple devices, and collaboration. It informs this proposal; no specific synchronization algorithm or library is selected.

## Directory organization

See the [canonical directory proposal](repository-structure.md) for code, test, script, and documentation responsibilities, and the [documentation index](README.md) for current files. Create source, reference, and workflow files when authorized implementation actually needs them. Community policies remain in `.github`; public project introduction and actual developer commands remain in the root README and development guide.

## Public-facing documents

| Document | Content to include | Accuracy requirement |
| --- | --- | --- |
| README | Project purpose, user-ownership boundary, current status, first-stage scope, architecture overview, working setup, demonstration, contribution and security links, and selected license | State that the project is in design until code exists; add commands and a demo link only after they work; distinguish planned integrations from tested ones |
| LICENSE | Exact standard text for the selected license and required attribution | License and copyright attribution require selection before publication; do not fabricate a copyright holder or modify standard legal terms |
| CONTRIBUTING | Local setup, actual checks, useful contribution areas, issue-first guidance for major changes, review process, synthetic test data, and compatibility expectations | Discuss schema, storage, and security changes before a PR; provide actual commands after choosing the stack |
| SECURITY | Supported release status, scope of security reports, private reporting route, and disclosure process | No stable release or audit exists yet; verify the private reporting route before inviting reports; do not invent a support commitment |
| CODE_OF_CONDUCT | Expected respectful behavior, scope, enforcement process, and reporting contact | Select and identify a real confidential reporting channel; do not invent an email address |
| Issue forms | Reproduction steps, version/environment, expected and observed behavior, or proposed use case | Ask for redacted logs and synthetic examples; direct vulnerabilities to the verified private route |
| PR template | Problem, resulting behavior, verification, relevant decision/issue, and data/schema changes | Require evidence appropriate to the change; identify breaking changes and data migrations |
| CHANGELOG | Unreleased changes and later release notes | Record actual changes; distinguish a software improvement from a user changing a stored decision |

The existing design ledger is the historical source of design choices. The build brief defines the selected first-stage scope and acceptance checks. The architecture document describes the implemented system when it exists. Avoid copying the same authoritative information across all three.

## Selected license

The user selected Apache License 2.0. The unmodified standard text is included in [LICENSE](../LICENSE), with project attribution in [NOTICE](../NOTICE). The user did not state a license-selection rationale. No broader-project license was supplied, so future integration must consider that project's actual licensing requirements.

Track third-party dependency and asset licenses. The source-code license does not authorize publication of personal memory records or decide their ownership.

Reference: [official Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).

## Privacy and security boundaries

The project stores structured memory, so its repository and tests must use synthetic or explicitly approved public examples. Personal memory databases, exports, source chat excerpts, authentication tokens, local configuration, backups, and runtime logs should be excluded from version control by default.

The first threat model should identify who controls the instance and storage, who may record and read decisions, which data is deliberately published, and how untrusted worker input reaches the viewer. It should cover authentication and authorization, safe rendering of submitted text and links, request limits, and retry behavior. These are implementation requirements to define and test, not a claim that security is already provided.

The public demo and private memory boundary must be explicit. Schema validation checks structure; it cannot independently establish that a worker correctly attributed a statement to the user. Documentation should explain that limitation.

GitHub private vulnerability reporting is enabled and verified for the public repository. A security-policy file alone does not enable that repository feature. Keep conduct-reporting contacts separate from vulnerability disclosure channels unless the chosen policy explicitly covers both.

References: [GitHub community profiles](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/about-community-profiles-for-public-repositories) and [GitHub private vulnerability reporting](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/configure-vulnerability-reporting/configure-for-a-repository).

## Development and release foundation

When implementation starts, use a single documented check command covering the selected stack's formatter, linter, strict type checking, tests, and build. Run those checks in CI and through the local pre-commit mechanism required by the development contract.

Tests should verify meaningful behavior: preserved versions, missing reasons and evidence, retries, invalid or stale revision references, durable persistence, and intended access boundaries. The first real worker integration also needs conversation-based capture checks because unit tests cannot prove worker attribution.

Use one reproducible dependency installation method and its lockfile. Add dependency-update automation when dependencies exist. CI should use minimum necessary permissions and should not expose publishing credentials to untrusted pull requests.

For public releases, maintain a changelog and versioned data contract. Changes that break existing records require an explicit migration or compatibility strategy. A release must include accurate setup instructions and the verified demo URL where applicable.

## Contribution and maintenance model

Propose a maintainer-led project initially. Contributors can improve documentation, synthetic examples, tests, adapters, and usability. Major changes to decision semantics, ownership, schema compatibility, or deployment access should first be discussed in an issue. The verified maintainer `@david3xu` holds merge and release authority. The [contribution-boundary proposal](repository-structure.md#proposed-contribution-tiers) details stronger review for foundational behavior and project-control changes.

Start with ordinary pull requests and a clear review process. Any additional contributor agreement or sign-off policy remains a separate choice. Do not promise response times, long-term release support, or a governance structure that has not been agreed.

## Before creating or publishing the GitHub repository

1. Confirm GitHub owner/account or organization, repository name, and initial visibility.
2. Select the source license and required attribution; identify the broader project's license if relevant.
3. Draft accurate README and community documents, with real reporting contacts.
4. Review the existing ledger and other intended public documents for private excerpts, local details, and unpublished third-party material. Do not publish the supplied project brief wholesale by default.
5. Select the stage-one scope and record the unresolved implementation choices in the build brief.
6. When code exists, verify setup and all required checks before claiming a working release.
7. Configure repository security reporting and contribution/review settings when the remote repository exists.
8. Complete the required ChatGPT Work build, use, improvement, and publishing workflow; retain actual evidence of the post-first-version improvement.

## Current state

The project contains design documentation and local repository configuration. Local Git is initialized with `main` as its initial branch; the documentation baseline is published and `stage/00-repository-foundation` is the active preparation branch. See [branching and remote preparation](branching-and-releases.md) for the proposed stage workflow. Apache License 2.0 and the owner `david3xu` are selected; remaining application choices are listed in the [build brief](prototype-brief.md). No application code or deployment has been created.
