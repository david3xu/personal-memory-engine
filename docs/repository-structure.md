# Personal Memory Engine — Directory and contribution boundaries

Recorded: 2026-10-07 (Australia/Perth)

Status: Stage 01 foundation implemented in `crates/engine`, `crates/local-runtime`, `contracts/generated`, `web/owner`, `tests/contracts`, and `.github/workflows`. The engine dependency boundary is checked automatically; CODEOWNERS routes review to the maintainer. Broader extension directories and additional review requirements remain proposals. CODEOWNERS does not by itself require an approving review.

## Purpose

Help a contributor find a useful task and understand which changes affect decision integrity, ownership, or public access. Keep the reusable engine small while giving interfaces and integrations clear extension boundaries.

This document owns the target directory layout, dependency boundaries, contribution-area map, and proposed review tiers. The [foundation proposal](repository-foundation.md) owns long-term growth rationale; the [build brief](prototype-brief.md) owns first-stage scope; the [implementation plan](implementation-plan.md) owns execution milestones; the [branching document](branching-and-releases.md) owns branch lifecycle and current remote settings.

A folder marks responsibility, not a security sandbox or permission to bypass the engine. Changes outside the core can still change core behavior and therefore need elevated review.

## Proposed code directory

Keep one repository and one application instance initially. Create modules only when approved implementation needs them; do not scaffold empty directories for future capabilities.

```text
Cargo.toml                # Existing workspace; deeper module split below is a growth proposal
crates/
  engine/
    src/
      core/
        contracts/        # Authoritative versioned record schema and portable types
        decisions/        # Choice, revision, and missing-information rules
      application/
        operations/       # Capture, retrieve, revise, and selected publication operations
        ports/            # Small storage and publication interfaces
    tests/                # Engine behavior and port conformance
  local-runtime/
    src/
      adapters/
        mcp/              # Worker transport, input validation, and attribution metadata
        storage/          # Embedded SQLite adapter and its migrations
        publication/      # Hosted snapshot destination and approved read-only projection
      runtime/            # Local lifecycle, API transport, and instance configuration
    tests/                # Runtime and adapter integration with synthetic data
contracts/                # Existing Rust-derived schemas/types; future public schemas added as needed
web/
  owner/                  # TypeScript owner interface
    src/
      features/           # First run, cards, history, sharing preview, and recovery UI
    src-tauri/            # Tauri host: lifecycle and narrow validated engine operations
  public-view/            # TypeScript read-only published cards/history
  shared/                 # Browser-safe visual components only
tests/
  contracts/              # Cross-language compatibility and input validation
  integration/            # Connected worker/store/viewer and publication behavior
  web/
  fixtures/               # Synthetic decisions and publication examples
scripts/
  development/            # Reproducible checks and local development tasks
  packaging/              # Selected platform package creation and verification
  release/                # Maintainer-controlled release operations
.github/
  CONTRIBUTING.md
  SECURITY.md
  CODE_OF_CONDUCT.md
  pull_request_template.md
  ISSUE_TEMPLATE/
  CODEOWNERS              # Actual maintainer routing; mandatory approval remains unconfigured
  workflows/              # Actual checks added with implementation
```

Rust is selected for the engine and TypeScript for the interface. The engine crate now holds the core/application layers, while a local-runtime crate composes adapters and process lifecycle. Future directory entries remain proposed. Keep these responsibilities in modules rather than creating a crate for each feature. Create TypeScript project manifests with implementation, not as empty placeholders.

Tauri owner delivery, embedded SQLite, and selected public snapshots are now approved. The Tauri host is implemented under `web/owner/src-tauri` and reads reusable engine operations through the local SQLite runtime. Keep Tauri and SQLite dependencies out of the pure engine. The initial engineering implementation uses plain TypeScript/Vite, stdio MCP, rusqlite, and a macOS Apple Silicon package. These are implementation choices, not invented user decisions selecting a permanent platform boundary. The development browser view displays setup/layout only; authoritative records are accessed through Tauri. GitHub Pages is selected. Implemented sharing modules are `crates/public-snapshot` (pure projection/HTML), `crates/github-publication` (provider operations), `crates/local-runtime/src/sharing.rs` (private publication lifecycle), and the owner dialog/commands under `web/owner`. The public page is self-contained static HTML; no unused public frontend project is scaffolded.

The worker plugin lives separately in `plugins/personal-memory-engine`, with a marketplace catalog in `.agents/plugins/marketplace.json`. It provides worker instructions and a launcher for the installed helper; it does not add decision rules to the engine or load arbitrary plugin code into the owner app.

## Allowed dependency direction

| Layer | May depend on | Must avoid |
| --- | --- | --- |
| Core | Its canonical contracts and approved pure schema utilities | Worker transports, databases, UI, filesystem paths, hosting providers, and application operations |
| Application | Core and its own ports | Concrete database drivers, worker-specific APIs, UI components, and publishing credentials |
| Adapters | Application contracts/ports and the core contracts needed to validate input/output | Creating independent decision semantics or bypassing validated operations |
| Runtime | The selected application operations and concrete adapters | Decision rules hidden in route handlers or launch scripts |
| Owner UI | Browser-safe read contracts and a narrow owner API | Direct database access, embedded credentials, or duplicated revision logic |
| Public viewer | Published read-only contracts and browser-safe UI | Local private-store access, recording operations, MCP credentials, and private runtime dependencies |

Maintain one authoritative record schema in Rust. Derive browser-safe read and published schemas/TypeScript types from it; avoid hand-maintaining conflicting copies. Select the generation mechanism before creating generated files. Runtime validation still checks external input, including MCP submissions and UI requests; static TypeScript types do not validate received data. Keep transport commands small, versioned, and covered by cross-language contract checks. The public publication representation should contain only explicitly selected fields and versions; it is not the unrestricted internal store contract.

Storage adapters implement atomicity and version preservation through a defined port and shared conformance checks. Publication adapters take the approved public representation rather than unrestricted database access. New adapter dependencies or network access must be reviewed for their actual permissions and data flow.

A shared visual-components folder must not become a mixed collection of storage, transport, or authorization helpers. Name folders for their actual responsibility; avoid generic `utils` or `misc` directories. Tests mirror the responsibility they verify. Cross-layer behavior belongs in application/contract/integration-style tests rather than being hidden in UI-only tests.

## Documentation directory

Keep current canonical design files at their published paths. Introduce audience-specific subdirectories only when working behavior and documentation exist; no current document relocation is required by this proposal.

```text
docs/
  README.md                      # Documentation navigation and audience map
  design-decisions.md            # Append-only choices, revisions, and proposals
  prototype-brief.md             # Candidate first-stage scope and acceptance checks
  implementation-plan.md         # User journeys, milestone work, and evidence
  repository-structure.md        # This layout and contribution-boundary proposal
  repository-foundation.md       # Long-term architecture/growth and community foundation
  branching-and-releases.md      # Branch lifecycle and remote protection state
  CHANGELOG.md                   # Actual changes and later release notes
  guides/                        # Future: how users install, connect, share, and recover
  architecture/                  # Future: how the implemented layers and data flows work
  reference/                     # Future: exact versioned record/tool/extension contracts
```

The root README introduces the product and points into this index. The root development guide holds actual developer commands and gotchas. `.github` contains contributor and community policies. A user installation guide must not substitute source-building instructions for the selected installable package.

Keep each topic in one canonical document: decisions explain what was selected and why; the brief defines scope; the plan defines work/evidence; architecture explains implemented behavior; reference describes exact contracts; guides explain user tasks. Link rather than copy. Create reference and guide files only when the corresponding contract or behavior exists, with accurate version/support status.

Changes to historical decision meaning require an appended correction or revision with evidence. Contributors may identify an error or propose a choice, but must not manufacture user approval, missing rationale, or a completed milestone. Broader product scope changes are recorded after the product owner makes an explicit choice.

## Proposed contribution tiers

All contributions use the existing PR workflow and scope rules. The tiers below describe stronger proposed requirements for implementation; they are not a claim that automatic approval enforcement exists today.

| Tier | Examples and paths | Proposed acceptance requirements |
| --- | --- | --- |
| Routine contribution | User-guide clarity, synthetic fixtures, accessibility, visual polish, and focused tests for existing behavior | Focused PR, relevant user outcome, actual checks, and maintainer review; no hidden scope or semantic changes |
| Contract-bound extension | `web/owner`, `web/public-view`, MCP or publishing adapters, installation improvements | Agreed feature scope, relevant adapter/contract tests, user-experience evidence, and review of new dependencies/permissions; elevated tier if it changes privacy or access |
| Foundation change | `crates/engine`, storage/migrations, authentication/authorization, publication selection, record compatibility, and invariant tests | Maintainer-agreed design before semantic changes, preservation tests, compatibility/migration/recovery analysis, updated contract documentation, and explicit foundation review before merge |
| Project control change | CI, packaging/signing/release scripts, ownership rules, security policy, supported platforms, or the historical design record | Maintainer agreement appropriate to the change; protect release credentials and data boundaries; record product choices explicitly; do not weaken checks silently |

Classification follows behavior, not only changed paths. For example, a UI change that publishes an extra private source excerpt is a foundation/access change. An adapter that overwrites old versions is a foundation failure even if it never edits `crates/engine`. New platform or worker support still needs a scoped milestone; a contribution-friendly area is not automatic approval to add an out-of-scope feature.

A bug fix or pure refactor within the agreed core semantics can be proposed as a focused PR with preservation evidence. It need not invent a new product choice. A proposal to change semantics needs an agreed design issue or private security discussion where appropriate, followed by an explicit decision if product requirements change.

### Foundation change checklist

- Identify the user problem, affected invariant, related design decision, and intended before/after behavior.
- Establish maintainer agreement on any semantics, compatibility, or scope change before implementation.
- Demonstrate preservation of earlier versions, missing-information semantics, explicit-user attribution boundaries, retries, and intended read/write access as relevant.
- If the durable record format or store changes, explain old-record compatibility, migration, backup, failure recovery, and selected deletion behavior.
- Keep contracts documented and the engine independent of UI, worker, storage, and host dependencies.
- Include meaningful tests and a changelog entry where behavior or compatibility changes; do not remove an invariant test merely to make a failing implementation pass.
- Use private vulnerability reporting rather than public sensitive reproduction data for security fixes.

### Changes the project should decline under its current scope

- Treating AI advice as an explicit user choice, inventing missing reasons or evidence, or scraping/monitoring conversations.
- Silently rewriting old choices or bypassing validated operations to write decision records directly.
- Publishing private memory, source excerpts, credentials, or unselected records by default.
- Adding a project-operated hosted memory service, new telemetry of personal memory, or mandatory external accounts without a new ownership/scope decision.
- Breaking existing records without an agreed compatibility and recovery plan.
- Weakening checks, ownership controls, or protected history as an incidental part of another feature.

These are merge/scope boundaries for this project's implementation. A changed product boundary must be discussed and explicitly recorded; it cannot be introduced as an incidental adapter or UI change. No blanket ban on outside core proposals or new licensing restriction is proposed.

## Contributor entry points

| User need | Where a contributor can help | Boundary to preserve |
| --- | --- | --- |
| Install without developer tools | Packaging, launcher, first-run guidance, clean-install tests | Durable memory location and selected platform support |
| Connect the worker reliably | MCP setup/help, connection diagnostics, scoped adapter tests | Actual worker capability and explicit decision attribution |
| Understand earlier choices | Card/history accessibility, synthetic scenarios, reference clarity | Version preservation and no invented reasons/links |
| Share deliberately | Preview usability, public-reader layout, approved publication adapters | Explicit selection and separation from recording/private storage |
| Keep memory over time | Backup/recovery guidance and preservation tests | Storage and migrations receive foundation review |
| Help future developers | Documentation index, architecture explanations, contract examples | Proposals and actual supported behavior remain distinguishable |

Initially the verified maintainer is `@david3xu`; no other code owner or team is invented. Identify additional trusted reviewers and their actual repository permissions when contributors join.

## Proposed enforcement and its limits

Current remote review settings are verified and recorded in the [branching document](branching-and-releases.md#review-and-protection-settings), the canonical protection-status source. The review-tier enforcement below is planned, not remotely configured. `CODEOWNERS` and implementation workflows now exist on the Stage 01 branch; mandatory code-owner approval and required status checks are not configured yet.

Propose review routing with `.github/CODEOWNERS` when the layout is implemented. Include core, application operations, storage/migrations, access/publication selection, relevant invariant tests, CI/release scripts, and CODEOWNERS itself. Code ownership routes review; mandatory approval requires the corresponding branch protection/ruleset setting. Protect the base branch used for contributor PRs, including an active stage where applicable, and ensure its ownership/check configuration is present there.

While there is one maintainer, do not pretend there is independent approval or configure an unavailable second reviewer. Plan mandatory foundation-owner approval after an eligible additional reviewer is identified, with a workable policy for maintainer-authored changes. Listing multiple owners does not require all their approvals; a second required reviewer is a separate rule. Avoid describing CODEOWNERS as a path-specific editing permission or protection against all semantic bypasses.

When code exists, propose CI checks for allowed imports, contract conformance, schema compatibility fixtures, revision/retry/persistence behavior, publication boundaries, and the actual formatter/linter/types/build checks. Protect relevant tests and check configuration with the same review standard as their code. Automation can find import or structural violations; maintainers still review semantic effects and actual user experience.

Official references: [GitHub code owners](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners) and [pull request reviews](https://docs.github.com/en/pull-requests/reference/pull-request-reviews). Required-review configuration is not changed by this design document.
