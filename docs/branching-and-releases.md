# Personal Memory Engine — Branching and remote preparation

Recorded: 2026-10-07 (Australia/Perth)

Status: PR #8 merged the working prototype into `main`. The original stage branches remain published; `stage/01-submission-showcase` carries the separate showcase checkpoint. Later implementation stages remain proposals. The [GitHub repository](https://github.com/david3xu/personal-memory-engine) is public under `david3xu`, with Apache License 2.0.

## Branch responsibilities

| Branch | Purpose | Completion condition |
| --- | --- | --- |
| `main` | Reviewed project baseline; initially documentation, later the latest accepted working prototype | Accurately represents the completed work; implementation changes pass actual checks before merging |
| `stage/00-repository-foundation` | Prepare license, contributor policies, build brief, and repository automation | Required owner, attribution, license, and reporting choices are settled; documentation reflects actual project state |
| `stage/01-working-prototype` | Build the smallest approved installable recording, history, and sharing path | Selected package installs and launches; real worker capture and linked versions persist locally; readers can open the verified demonstration URL |
| `stage/02-tested-improvement` | Improve the first prototype after actual use | The user observes a concrete problem, requests a change, and verifies it; updated published prototype works |
| `stage/03-next-capability` | Placeholder naming pattern for a later approved enhancement | Specific scope and checks are defined when this stage starts; maps, additional adapters, or extraction into a library are possibilities, not approved features |

`main` contains the accepted prototype. The foundation and working-prototype branches are retained. The showcase task branches from this baseline; Stage 02 and later capability branches remain proposals. The first prototype's final feature list remains governed by the approved build brief.

The [user-first implementation plan](implementation-plan.md) maps installation, recording, history, sharing, recovery, and tested improvement to these stages and defines the implemented recording checks and later sharing/recovery acceptance checks. The [Stage 01 step plan](implementation-plan.md#stage-01-deliver-a-complete-minimum-in-small-steps) proposes checkpoints inside one active stage branch, with the complete minimum accepted before Stage 02. No application workflow or future stage branch is created by the plan.

## Branch lifecycle

1. Start a stage from the latest accepted `main` after its scope is selected.
2. Keep one active stage branch for this small project. Create later branches when work starts so they inherit prior accepted stages.
3. Use a draft pull request to show work in progress when a GitHub remote exists.
4. Run the stage's documented checks and review the resulting behavior.
5. Merge the completed stage through a pull request into `main`.
6. Add an annotated checkpoint tag to the accepted commit, then retire the completed working branch if no longer needed.

For an external contribution, branch from the current contribution target and state the intended PR base. Normally that target is `main`; changes to an active stage should explicitly target that stage. Short task branches may use `feat/<topic>`, `fix/<topic>`, or `docs/<topic>` when separate contributors or independent changes make them useful. Every final stage merge into `main` still needs its acceptance checks.

Branch names organize ongoing work. Tags identify completed snapshots. Avoid using permanently diverging stage branches as the record of completed stages.

## Suggested checkpoint names

- `foundation-v1`: accepted repository foundation.
- `prototype-v1`: first complete tested and published prototype.
- `prototype-v2`: verified prototype after the required change.

Tags will only be created when their checkpoints actually exist. Formal semantic-version release naming and data-schema compatibility are separate choices. A Git tag documents the code checkpoint; it does not replace the decision ledger or the required same-conversation build evidence.

## Review and protection settings

`main` is protected on GitHub: pull requests are required, review conversations must be resolved, force pushes and deletion are disabled, and the rules also apply to administrators. Required approving reviews are set to zero while there is one maintainer. Required code-owner review is disabled; the CODEOWNERS file routes reviews to the sole maintainer. Linux `checks` and macOS `desktop` jobs run for pushes and pull requests and have passed for the first recording checkpoint. After explicit user approval in D030, both `checks` and `desktop` are required on `main`, with up-to-date branches and administrator enforcement. GitHub binds these checks to GitHub Actions. The applied settings were verified through the API. The [contribution-boundary proposal](repository-structure.md#proposed-enforcement-and-its-limits) describes planned foundation review and its limits.

Do not require a second person's approval while the project has only one maintainer; enable an appropriate review requirement when another reviewer is available. Do not configure nonexistent CI check names or claim that local documentation files provide remote branch protection. The active stage can receive similar protections as the contributor group grows.

## Remote preparation

- **Repository name:** `personal-memory-engine`, adopted in D009.
- **Selected GitHub account:** `david3xu`, confirmed by the user and verified through the connector and command-line authentication.
- **Remote owner:** `david3xu`, selected by the user.
- **Remote name:** `origin` for the project's own repository.
- **Visibility:** public, selected by the user and verified on GitHub.
- **License:** Apache License 2.0, selected by the user; unmodified standard text is in [LICENSE](../LICENSE).
- **Attribution:** the verified owner identifier `david3xu`, recorded in [NOTICE](../NOTICE). Private vulnerability reporting is enabled. A dedicated confidential conduct-reporting contact remains open.
- **Verified remote repository:** [https://github.com/david3xu/personal-memory-engine](https://github.com/david3xu/personal-memory-engine).

Local Git was initialized with `main` as its initial branch. The documentation foundation has been reviewed for credentials, private absolute paths, valid YAML, and local links. The initial documentation baseline is published on `main`, and the foundation stage branch is published. The working recording, history and sharing source is available on main; a complete signed Stage 01 release is not yet claimed.

The owner, public visibility, and license are selected. The initial documentation commit is published, `origin` points to the verified GitHub repository, private vulnerability reporting is enabled, and `main` protection is configured. Future updates follow the branch-and-pull-request workflow. Avoid independently initializing a remote README or license if the local repository already supplies those files, so the histories start consistently.

## Publication checks

- Inspect every intended tracked file before the first public push, including design notes and source excerpts.
- Keep memory databases, private exports, credentials, runtime logs, and local settings outside Git. A `.gitignore` prevents accidental addition of new matching files; it does not remove secrets or data already committed.
- Verify the selected license and attribution, README status, and actual reporting routes.
- Inspect the local branch and remote URL before pushing. Do not force-push to resolve an unexpected remote history.
- Only claim branch protection, CI, security reporting, or deployment success after verifying those features on the remote.

## Relationship to the project brief

The brief requires one working prototype at a shareable URL and a real change after its first version. Stage 01 and Stage 02 represent those separate software checkpoints. A user changing a decision stored in memory tests version handling; it does not satisfy the required software improvement by itself.

The owner confirmed that the full build/use/improve workflow happened in one ChatGPT Work conversation. This is an owner report; Git history and local receipts do not independently authenticate the client mode.

## References

- [GitHub flow](https://docs.github.com/en/get-started/using-github/github-flow): descriptive branches, pull requests, checks, and retiring completed branches.
- [Protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches): remote settings for review, checks, force pushes, and deletion.
- [Adding locally hosted code to GitHub](https://docs.github.com/en/migrations/importing-source-code/using-the-command-line-to-import-source-code/adding-locally-hosted-code-to-github): local repository preparation, remote creation, and linking local history to a remote.


## Independent public-demo check

The optional `public-demo` job checks the deliberately published synthetic notebook snapshot from a GitHub-hosted runner without credentials. Set the repository's nonsecret `PUBLIC_DEMO_URL` variable to that selected snapshot URL to enable it. Forks can leave it unset. This diagnostic is separate from required `checks` and `desktop`; a hosting outage does not change the approved main protection settings. If the demo is intentionally withdrawn or replaced, update/remove the variable and documentation link together. It never uploads private memory.
