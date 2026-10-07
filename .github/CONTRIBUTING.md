# Contributing to Personal Memory Engine

Thank you for helping develop a user-owned decision memory engine.

## Current stage

The repository currently contains design documentation. Application code, a verified MCP integration, and a deployed prototype do not exist yet. Useful contributions now include reviewing the data model, finding ambiguity in recording rules, improving documentation, and proposing synthetic scenarios for testing.

## Discuss substantial changes first

Open an issue before changing decision semantics, record compatibility, storage ownership, authentication, or the first-stage scope. Explain the concrete user need and how the proposal preserves explicit user attribution, missing-information handling, and earlier decision versions.

For small documentation corrections, a focused pull request is welcome.

## Contribution areas and foundation review

The [directory and contribution-boundary proposal](../docs/repository-structure.md) identifies interface, installation, worker integration, publication, tests, and documentation entry points. Code paths do not yet exist and the proposed review tiers are under design review; they are not a claim of automated enforcement. Start with a scoped issue for a new feature or integration.

Changes to core decision rules, durable schemas, storage/migrations, access controls, publication selection, or checks that enforce those rules need the substantial-change discussion above. A change in an adapter or UI can still affect the foundation. Explain semantic and compatibility effects rather than assuming a folder makes a change low risk. Bug fixes and refactors preserving approved semantics may be proposed with relevant evidence.

Do not silently change historical decisions, invent user approval or missing rationale, bypass revision rules, or publish personal memory by default. Treat changes to security policy, CI, ownership, and release configuration as project-control changes requiring maintainer agreement. Use private vulnerability reporting for sensitive security discussions.

## Contribution workflow

1. Read the [README](../README.md), [decision ledger](../docs/design-decisions.md), and [branch plan](../docs/branching-and-releases.md).
2. Fork the repository and create a descriptive branch from `main`. If you are contributing to an active stage, agree on that stage as the base first.
3. Keep the change focused and include only material relevant to the project.
4. Check changed documentation with `git diff --check` and verify its links.
5. Open a pull request explaining the problem, resulting behavior, and verification performed. Identify any record-schema or compatibility implications.

Runtime setup and automated application checks will be documented when the implementation stack has been selected and those commands actually work. Do not claim that unimplemented or unrun checks passed.

## Data and attribution rules

- Use synthetic examples. Do not submit private memory, raw conversations, credentials, exports, backups, or runtime logs containing personal information.
- Only explicit user choices are decisions. Keep AI suggestions distinguishable from user selections.
- Do not invent rationale, rejection reasons, source evidence, or relationships.
- Preserve earlier versions and identify revisions and corrections explicitly.
- Include suitable behavioral verification for application changes once code exists.

## Review and scope

The repository maintainer, [@david3xu](https://github.com/david3xu), decides whether to merge changes and publish releases. Major changes should have an agreed issue and compatibility plan. No response-time or long-term release-support commitment is currently offered.

Follow the [code of conduct](CODE_OF_CONDUCT.md). Send vulnerability reports through the [security policy](SECURITY.md), rather than public issues.

## License

The project uses [Apache License 2.0](../LICENSE). Its contribution provisions apply to contributions submitted for inclusion. No additional contributor agreement or sign-off policy is imposed at this stage.
