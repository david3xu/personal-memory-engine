# Personal Memory Engine — Repository foundation proposal

Recorded: 2026-10-07 (Australia/Perth)

Status: proposed repository design. Application implementation remains paused. The [public repository](https://github.com/david3xu/personal-memory-engine) has been created under `david3xu`. Apache License 2.0 is selected, with attribution to the verified owner identifier. The implementation stack and hosting remain open; confidential conduct-reporting contact details are not yet supplied.

## Purpose and design boundary

The user wants a small first-stage implementation that can later become a reusable part of a broader project and welcome outside contributions. The confirmed product requirements and their rationale are recorded in [the decision ledger](design-decisions.md). The supplied project brief and its acceptance criteria are recorded there as well.

This document describes how to organize the repository around those requirements. It does not expand the approved product scope or approve a particular stack.

## Reusable architecture

Keep one repository and one application deployment for the first prototype. Organize the code by responsibility so the decision engine can be extracted as a library later if another project actually needs it.

| Boundary | Responsibility | Dependency rule |
| --- | --- | --- |
| Decision core | Decision records, version relationships, missing-information semantics, and revision rules | Must not depend on MCP, a web framework, database drivers, host accounts, or deployment configuration |
| Application operations | Capture a decision, read cards and history, and coordinate persistence and retry handling | Uses the core and a small storage interface |
| Storage adapter | Durable records and atomic writes using the selected store | Implements the storage interface; preserves the core's versioning rules |
| MCP adapter | Validate tool calls, scope access, and translate inputs and results | Calls application operations; must not duplicate decision rules |
| Viewer | Display cards, source attribution, and preserved history | Consumes the same versioned record contract |
| Deployment | Runtime configuration, persistent data location, and publication | Configures the application without changing decision semantics |

Prefer explicit dependency boundaries over separate services or separately published packages at this stage. Use a versioned record contract so future viewers and adapters can recognize compatible data. Choose one authoritative schema representation after selecting the implementation stack; avoid hand-maintaining contradictory schemas across layers.

## Proposed layout

This is a target layout, not a list of existing files. Create source and workflow files when their implementation is authorized; do not add empty folders just to match this diagram.

```text
personal-memory-engine/
  README.md
  LICENSE
  CLAUDE.md
  .gitignore
  .editorconfig
  .env.example
  .github/
    CONTRIBUTING.md
    SECURITY.md
    CODE_OF_CONDUCT.md
    pull_request_template.md
    ISSUE_TEMPLATE/
      bug_report.yml
      feature_request.yml
      config.yml
    workflows/
      ci.yml
    dependabot.yml
  docs/
    design-decisions.md
    repository-foundation.md
    branching-and-releases.md
    prototype-brief.md
    architecture.md
    CHANGELOG.md
  src/
    core/
    application/
    adapters/
      mcp/
      storage/
    server/
  web/
  tests/
    core/
    integration/
    fixtures/
  scripts/
```

The root README introduces the project. Community policy files live in `.github/`, a supported discovery location, keeping the root small. `CLAUDE.md` will provide project commands and development guidance once the stack exists; it must not contain private personal context or machine-specific secrets. CI configurations and dependency automation must match actual project tooling rather than claim checks that do not run.

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

Propose a maintainer-led project initially. Contributors can improve documentation, synthetic examples, tests, adapters, and usability. Major changes to decision semantics, ownership, schema compatibility, or deployment access should first be discussed in an issue. Merge and release authority should be clearly identified once the GitHub owner is selected.

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

The project contains design documentation and local repository configuration. Local Git is initialized with `main` as its initial branch; its verified public remote is being connected for the first documentation baseline. See [branching and remote preparation](branching-and-releases.md) for the proposed stage workflow. Apache License 2.0 and the owner `david3xu` are selected; the application stack remains open. No application code or deployment has been created.
