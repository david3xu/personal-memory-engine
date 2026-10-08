# Submission and showcase implementation plan

Updated: 2026-10-08

Status: implemented and published from `stage/01-submission-showcase`, branched from the accepted main prototype. Merge verification is tracked in [PR #9](https://github.com/david3xu/personal-memory-engine/pull/9). This is presentation and demonstration work within Stage 01, not a claim that the broader desktop release is complete.

## Outcome

Give a visitor one clear entry point to understand the problem, inspect a meaningful preserved decision history, see the actual local app, and find the open-source code. Keep the owner workspace and individual shared snapshots focused on decisions.

## Boundaries and contribution areas

| Location | Responsibility | Contribution expectations |
| --- | --- | --- |
| `web/showcase/` | Static project story, accessible walkthrough, public screenshots and styles | Presentation contributions; no owner commands, private-store reads, accounts or analytics |
| `scripts/development/build-showcase.mjs` | Reproduce a clearly labeled synthetic choice/revision using the real stdio MCP helper in fresh isolated storage | Must never fall back to the owner's default data directory; validate linked history after restart |
| `crates/public-snapshot/examples/render_demo.rs` | Render the synthetic history using the existing approved public projection | Reuse projection rules; no new storage/transport dependencies or schema changes |
| `scripts/distribution/publish-showcase.mjs` | Publish only an explicit static showcase asset allowlist under `showcase/` on the existing Pages branch | Verify existing site ownership, preserve root and every shared snapshot, reject force updates |
| `tests/showcase/` | Verify asset boundaries and public-reader behavior | Test meaningful isolation and preservation guarantees |
| `docs/submission-improvements.md` | Scope, checkpoints and verification evidence | Keep claims aligned with actual behavior |

The decision engine, SQLite adapter, MCP tools and optional worker plugin keep their existing responsibilities. Presentation code must not import owner UI commands or connect public readers to MCP. No new feature flags or synthetic cards are inserted into a real owner's memory.

## Delivery steps

1. **Plan and branch.** Save this plan before implementation and push focused checkpoints through a PR.
2. **Meaningful synthetic example.** A fictional traveler first chooses a train, then changes to a coach after a timetable constraint changes. Use supplied scenario statements/reasons only. Verify retries, linked versions, and both records after a helper restart. Label this as scripted protocol evidence, not a live AI conversation.
3. **Project showcase.** Explain the product in plain language, show its local ownership model, offer a short guided walkthrough, and link to a rendered revision example, the existing owner-selected snapshot, source and installation guide.
4. **Actual app views.** Open the existing desktop build against the isolated synthetic database; capture cards/history and selection/preview only. No real owner cards, account names, filesystem paths or private record identifiers may enter public assets.
5. **Presentation polish.** Use readable dates, clear current/earlier context, responsive layout, keyboard navigation and restrained styling on the showcase. Avoid unrelated graphs, integrations and extra owner controls.
6. **Project accuracy.** Reconcile current documentation with PR #8 having merged. Keep unsigned distribution, fresh-machine installation and recovery limitations visible. Keep README short.
7. **Verify and publish.** Run required checks, review all staged files, inspect desktop/mobile presentation and keyboard flow, publish the additive showcase directory, and verify anonymous public assets plus unchanged existing snapshot bytes. Create a reviewable PR; respect the required main checks.

## Acceptance

- A visitor understands the app and reaches the demo/source within the first screen.
- The example has a stated rationale, rejected alternatives with stated reasons, and a linked changed choice; earlier context remains intact.
- Synthetic example content is explicitly labeled everywhere it could be mistaken for personal memory.
- Desktop images come from the actual existing app using isolated synthetic storage.
- The public site can be read without installing the app or opening the owner's local service.
- The walkthrough distinguishes scripted MCP verification, actual app screenshots and the separately owner-confirmed Work test.
- The publisher changes only its asset allowlist under `showcase/`; existing root and `shares/` content are preserved.
- Required Linux/macOS checks pass before any merge; the repository remains clean after focused commits.
- Documentation points to current evidence rather than claiming the preview is a signed general release.

## Follow-up product readiness

Signed/notarized distribution, fresh-user installation, backup/restore and broader AI-host compatibility remain governed by the existing implementation plan. They are not added to this presentation checkpoint.

## Evidence

- `pnpm check` passed: formatter/lint/contracts/TypeScript, owner build, Rust warnings/integrity/runtime checks, five showcase publication-boundary tests and the real-process demo build.
- The scripted fixture used a fresh isolated database: the initial retry returned the identical record, the revision linked to the latest version, and both unchanged records were read after restarting the helper.
- The installed desktop preview was opened with the existing development storage override pointing to that isolated store. Its actual current/earlier views and frozen history/evidence preview were inspected. Public images contain only the committed fictional scenario; no worker or GitHub connection was made from that instance.
- Browser checks at 1280px and 390px found no horizontal overflow. Native disclosure controls opened/collapsed by pointer and keyboard. The richer example displayed earlier reasons when expanded.
- All local Markdown links resolve; `git diff --check` is clean. Stale PR #8 and protection-status wording was reconciled in current documentation.
- The additive publisher preserved every preexisting non-showcase leaf entry and created publication commit `d26e9ae3dba417f81c9fc5a49c87fcacd1ce2f30`. Pages reported that commit built successfully.
- Anonymous HTTPS verified all allowed showcase assets and the existing notebook snapshot. The live [project showcase](https://david3xu.github.io/personal-memory-engine/showcase/) links the [synthetic revision example](https://david3xu.github.io/personal-memory-engine/showcase/example/) and the separate owner-published notebook snapshot.
- The nonsecret `PUBLIC_SHOWCASE_URL` variable enables an independent GitHub-hosted reader job. Required `checks` and `desktop` remain the main merge gate; current results are attached to PR #9.
- The short walkthrough is an expandable static guide with actual screenshots, not a screen-recorded AI conversation. The desktop binary and worker plugin are unchanged by this checkpoint.

## Publication references

The additive publisher uses GitHub's [base-tree API](https://docs.github.com/en/rest/git/trees) to retain existing content and [non-forced reference updates](https://docs.github.com/en/rest/git/refs) to refuse overwriting a concurrent publication. These are developer operations, separate from the owner's app sharing lifecycle.
