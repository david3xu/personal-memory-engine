# Personal Memory Engine — User-first implementation plan

Recorded: 2026-10-07 (Australia/Perth)

Status: active Stage 01 execution following D029. Branch `stage/01-working-prototype` and implementation CI are created. Core/SQLite and real-process MCP checks pass. The packaged desktop is installed and opens on the maintainer’s Mac; its local plugin is installed/enabled and starts the helper. The owner’s ChatGPT Work recording test passed, with its persisted local receipt checked. Full recovery/distribution remains unverified. Selected GitHub Pages sharing is implemented; live publication evidence is tracked in the checkpoint record. See [checkpoint evidence](implementation-status.md).

## Purpose and sources of truth

Organize implementation around what users must accomplish: install, connect a worker, record an explicit choice, inspect preserved history, share selected content, and retain their memory across restarts and recovery.

The [decision ledger](design-decisions.md) owns confirmed choices and their history. The [build brief](prototype-brief.md) owns candidate first-stage scope and acceptance checks. This plan owns work order, user-experience milestones, and proposed automation. The [branching document](branching-and-releases.md) owns branch lifecycle and existing protection settings. The [directory proposal](repository-structure.md) owns module/dependency and contribution boundaries; the [foundation proposal](repository-foundation.md) owns later growth rationale.

When implementation changes scope, record the explicit user choice and update the brief before adding the feature. Do not treat a proposed milestone as approval of its framework, storage, or hosting.

## Users and expected experience

| User | Desired outcome | Experience to design |
| --- | --- | --- |
| New memory owner | Start recording without learning the development stack | Download and open the supported package, see where data lives, follow connection guidance, and verify one sample decision |
| Returning memory owner | Understand and retain earlier choices | Reopen the app, find cards, inspect versions and evidence, change a choice through the worker, and recover from a backup |
| Public reader | Understand deliberately shared decisions | Open a URL without installing the app, see its publication/version scope and selected history, and read comfortably on a phone |

The proposed everyday owner flow is: open the app, connect if needed, continue chatting normally with the configured worker, inspect a recorded choice, and deliberately publish a selected view when desired. This does not introduce chat monitoring or guarantee that every worker will call a tool automatically. Worker instructions and actual tests must establish the supported recording behavior.

The public reader uses a separate read-only experience. Installing the app, authenticating a worker, and publishing a view are owner tasks, not requirements for the reader.

## Close choices at the step that needs them

Ease of use comes first, and ChatGPT Work is the selected working context under D028. Keep the prescribed build/use/improve/publish evidence in the same Work conversation. Building through Work and using an MCP recording tool there are separate paths that both need verification.

Before implementing the first capture slice, establish repository access in that Work conversation, the available recording route (transport, permissions, authentication), and the initial versioned record/local persistence contract. Rust and TypeScript are selected, with Tauri for delivered owner packaging, embedded SQLite for persistence, and selected read-only public snapshots approved. A local browser viewer remains an optional development path, with a clear launch action, connection status, empty state, and card view. Do not require the user to approve desktop packaging and public hosting together before starting engine/interface work.

The product sequence is: implement the Rust/SQLite/MCP service and TypeScript viewer, install or launch the app, connect its MCP server to ChatGPT Work, then verify a real explicit decision appears as a durable card. Verify route/account prerequisites early; the actual tool connection and end-to-end test occur after the server exists. An early Tauri build may support that test; final distribution/installation usability completes by 01.4. This sequence is separate from attaching the source repository to the Work conversation used for building.

Select remaining choices when they become necessary:

1. **01.1:** Verify the actual Work/worker MCP route; define the embedded SQLite driver/schema and portable record/operation contracts in durable per-user local storage. Keep data separate from app binaries, the repository, and browser cache. Establish a recoverable development-data policy; complete end-user recovery controls in 01.4.
2. **Before package work / 01.4:** Select the first supported operating system, distribution format, and signing plan for the approved Tauri desktop shell. Investigate packaging friction early where useful, while keeping it separate from capture-checkpoint completion. Complete clean installation, restart/reinstall retention, backup/restore, and deletion behavior before the minimum release.
3. **Before 01.3:** Configure the approved hosted snapshot route: select the destination, selection controls, and publication/update/withdrawal semantics. Only the selected public copy is hosted; verify it remains readable with the local app stopped.
4. **Before 01.5:** Reconcile the completed steps with the agreed full minimum build brief, user walkthrough, and actual checks. Do not claim the first complete release until those checks pass.

Tauri, embedded SQLite, and owner-selected public snapshots are approved in D020-v2, D003-v3, and D012-v2. The recording checkpoint now uses plain TypeScript/Vite, rusqlite, generated Rust contracts, and a macOS Apple Silicon development build. Broader platform/account compatibility and public distribution remain open. GitHub Pages hosting and the frozen-copy/explicit-withdrawal lifecycle are implemented; the owner’s Work capture passed on the maintainer setup. A production hosted memory service, multi-device sync, team accounts, and multiple worker adapters remain outside the proposed first stage.

Official [Work setup](https://learn.chatgpt.com/docs/get-started-with-work) and [local projects](https://learn.chatgpt.com/docs/projects?surface=app) describe approved local folder access. A cloud conversation does not automatically inherit the local repository or running processes; verify actual access rather than assuming it from the product name.

## Stage 01: deliver a complete minimum in small steps

D027 confirms that the first stage should reach a complete basic delivery before broader features. The five steps below are a proposed grouping of M0–M5, not five independent products. Each step extends the same working app and ends with a demonstration, focused checks, and a reproducible checkpoint. The milestone details below retain the work and evidence for each step.

| Step | What the user can do at its end | Completion evidence | Milestone mapping |
| --- | --- | --- | --- |
| 01.1 — Record one real choice | Open the local viewer through a clear launch path, connect the selected worker, make an explicit synthetic choice, and see a locally saved card with supplied rationale/alternatives/evidence | Real MCP conversation to durable storage to UI; restart retains the card; missing fields remain absent; invalid/retried submissions behave correctly; connection and save failures are understandable | Close capture-related M0 choices; combine the M1 runtime/viewer foundation with M2; finish packaging in 01.4 |
| 01.2 — Change a choice without losing history | Record a later choice and inspect both linked versions in a simple history view | Actual worker revision, old-content preservation, retry/reference checks, and unresolved concerns do not change the choice | M3 |
| 01.3 — Share deliberately | Select synthetic content, preview it, publish, and give another person a read-only URL | Public page matches selection; an external/private-session reader can open it; recording and unselected memory remain inaccessible; selected update/withdrawal behavior works; hosted snapshots remain readable with the local app stopped | M4 |
| 01.4 — Install and retain memory | Follow a short installation/connection guide, reopen or reinstall, and back up/restore the history | Clean-environment package test, new-user walkthrough, persistence and synthetic restore checks, documented data location and deletion behavior | Complete M1 packaging and M5 recovery; retain the same engine and records |
| 01.5 — Receive the complete minimum | Use installation, recording, revision, recovery, and sharing together in the same accepted version | Full build-brief acceptance review, meaningful checks, released-package walkthrough, real worker verification, and final external URL check against the exact accepted commit | Combined M1–M5 acceptance and first-prototype checkpoint |

Start with a real vertical slice: Rust engine, local storage, MCP, and TypeScript card view connected in 01.1. An empty viewer or mocked worker can help development but does not complete that step. An early package may test launch/distribution feasibility, but is not required to complete 01.1. Step 01.4 finishes the supported installation and recovery experience. End users should not need developer tools or a persistent terminal for the completed minimum release.

Define stable record identifiers, the versioned record contract, revision references, and validated operation boundaries from 01.1. Implement the user-visible revision path in 01.2 without rewriting the first records. Durable writes, intended recording access, missing-information handling, and safe rendering apply as soon as their paths exist. They are not optional polish to postpone until the last step.

### Complete minimum delivery boundary

The first complete delivery consists of one owner/machine, one supported installation route, one real worker connection, explicit decision cards, preserved linked revisions, durable local storage and the selected recovery policy, plus deliberate read-only publication at a verified URL. Basic empty/error states and clear connection/save/publication status are part of making that journey work.

Keep the UI small: list, card detail/history, connection guidance, publication preview, and a recovery action. Exact screens and backup controls follow the selected policies. Relationship graphs, richer search, extra workers, additional operating systems, sync, team features, automatic updating, and advanced visual customization need later scope decisions. The boundary does not remove any confirmed data-integrity or ownership requirement.

If a step fails its completion evidence, fix that step before accumulating broader features. Earlier synthetic development checkpoints may be tried and reviewed, with their missing capabilities clearly stated; they are not the full minimum release or a claim of readiness for personal memory. Create `prototype-v1` only after 01.5 verifies the complete journey. This includes a downloadable supported package, working demonstration URL, concise quick start, recorded check results/limitations, and a known accepted code baseline.

### Return for improvements after the baseline works

Preserve the accepted code checkpoint and the user's records. In Stage 02, let the user actually use the first version, record one observed problem or need, make the requested software change, and verify both the improvement and the existing minimum journey. The issue can concern usability, connection reliability, sharing, or another observed difficulty; do not invent it in advance.

Additional capabilities follow a separately scoped later stage. Each feature starts from the accepted baseline, defines one user outcome and acceptance evidence, addresses record compatibility if needed, and retains relevant regression checks. A new feature must not require users to reset their memory. Do not expand Stage 01 indefinitely to include every promising idea.

### Step workflow inside one Stage 01 branch

Use one active `stage/01-working-prototype` branch after build authorization. Track 01.1–01.5 as scoped issues/checklists and focused commits or short task PRs into that stage. Use the existing draft stage-to-main PR to show integrated progress. Do not create five permanently diverging stage branches. Attach demonstration/check evidence to each completed step and rerun affected earlier checks when new work changes their behavior. The final stage-to-main merge follows the existing branch lifecycle after complete minimum acceptance; Stage 02 starts from that accepted baseline.

No step branch, issue, package, workflow, or tag is created by this design update. Actual commands and verified Actions jobs are introduced with implementation, using the automation plan below.

## Milestones driven by user outcomes

### M0 — Agree on the journey and remove the connection uncertainty

- **User outcome:** The owner knows what they will install, where their memory will live, which worker can record it, and what a reader will see.
- **Work:** Close the choices above; sketch first-run, empty, connected, disconnected, decision-detail, history, publication-preview, and failure states. Select a synthetic example and verify the candidate worker connection using a minimal feasibility setup when implementation is authorized.
- **Evidence:** Selected scope and a documented supported connection path. Record unsupported account access or an unavailable publishing path plainly; do not claim feasibility as a successful end-to-end app.
- **Target alignment:** D003-v2 (local storage), D021 (minimum local-first engine), D022 (installation and sharing).

### M1 — Launch foundation and a supported installable package

- **User outcome:** In 01.1, open a clear development launch path and viewer; by 01.4, download, install or unpack, and open without compiling source or leaving a terminal running.
- **Work:** Establish the Rust and TypeScript scaffolds and lockfiles, initial record schema, local launcher/lifecycle, durable data location, and smallest viewer shell. Add Rust formatting, Clippy, meaningful tests, and builds; add TypeScript formatting, lint, strict type checks, meaningful tests, and builds. Provide one documented local check entry point, pre-commit checks, and matching CI before domain implementation. Verify generated contracts remain consistent and validate external inputs at runtime; TypeScript types alone are insufficient. Explore an early Tauri package when useful to expose packaging difficulties; complete the supported Tauri package by 01.4. Final desktop distribution is not required for the 01.1 capture checkpoint.
- **Experience details:** Show local app health and first-run guidance. Explain any unavoidable platform or account steps; do not call setup one-click if it is not. Prevent duplicate launches from unexpectedly starting independent stores or leaving orphaned services.
- **Evidence:** For 01.1, an understandable launch/empty state and working local checks. For 01.4, a clean-environment package launch without the maintainer's developer tools. A development launch or early package is not the completed prototype.
- **Target alignment:** D022. Keep engine and viewer boundaries aligned with the foundation proposal.

### M2 — The owner connects a worker and records the first decision

- **User outcome:** Connection status is understandable; an explicit choice made in conversation appears as a local card.
- **Work:** Implement validated capture/retrieval operations and durable atomic SQLite writes; add the MCP adapter and connection guidance; connect one actual worker. Provide scoped recording access and a minimal sample test.
- **Experience details:** The card shows the chosen option and only supplied rationale, alternatives, reasons, and evidence. Missing information is visibly absent. Give understandable errors and recovery steps when the worker is disconnected or storage fails. A failed write must not be presented as saved.
- **Evidence:** Real synthetic conversation capture, persistence after restart, and behavior checks for retries and rejected invalid input. Test an AI suggestion and an unresolved question without turning them into user decisions.
- **Target alignment:** D002, D004, D005, D003-v2. Unit tests supplement the actual conversation walkthrough; they do not establish worker attribution by themselves.

### M3 — The owner understands changed decisions

- **User outcome:** A later choice appears as a linked new version; the earlier choice and its reasons remain readable.
- **Work:** Implement revision references, current/history queries, and the card detail/timeline view. Define the selected policy for stale or competing revisions; do not infer semantic precedence from a timestamp.
- **Experience details:** Distinguish the current choice from older versions. Keep recording time separate from decision time when the latter is supplied. A concern alone must not silently change a choice. A retried submission should not create a duplicate card or version.
- **Evidence:** Real worker revision walkthrough plus checks for preservation, retries, invalid references, and the selected stale-update policy.
- **Target alignment:** D006 and the proposed minimum history view under D007. Full relationship maps remain later work.

### M4 — The owner shares a view and a reader opens it

- **User outcome:** Select content, preview it, publish it, and copy a working URL; the recipient reads it without installation.
- **Work:** Implement the hosted snapshot publishing adapter and public read-only viewer. Public pages consume the approved published representation, separate from the recording endpoint and unrestricted local store.
- **Experience details:** Preview versions, rationale, alternatives, and source evidence, not just card titles. Show publish progress, accurate success or failure, publication/version scope, and the selected update/withdrawal behavior. A failed publish must not claim a new public link exists.
- **Evidence:** Synthetic publication whose page matches the preview; public-reader verification in a private session and another device or network; anonymous visitors cannot record or access unselected records. Verify hosted snapshots remain readable with the local app/service stopped. Use narrow-screen and empty/error checks.
- **Target alignment:** D012 and D022. First-stage public tests use synthetic records; no personal memory is published by default.

### M5 — The owner keeps their memory and receives the first usable version

- **User outcome:** Reopening, reinstalling, and following recovery guidance retain the decision history.
- **Work:** Finish the selected export/backup/restore/deletion policy, packaging, and quick-start documentation. Test the released package rather than only a development server. Keep backups and actual memory out of source control, build logs, and CI artifacts.
- **Experience details:** Display the durable data location and a straightforward backup/recovery action. Uninstall and deletion behavior must be explicit. Any update or migration path introduced at this stage must preserve records and offer recovery on failure.
- **Evidence:** Clean install and restart walkthrough, reinstall preservation, and restoration of a synthetic backup. Verify that restored records and version links match the original. Complete the build brief's relevant checks and publish an accurately labeled first prototype package/checkpoint.
- **Target alignment:** D003-v2, D006, D011, D022. Only call the package easy to install after an actual new-user walkthrough; clean-runner tests alone do not establish usability.

### M6 — Actual use produces a verified improvement

- **User outcome:** A concrete difficulty observed while using the first version is improved and the user can verify the result.
- **Work:** Let the user install/use/try the first version, record the actual observation and requested change, implement the change, and rerun affected checks. Reverify the final package and public URL.
- **Evidence:** First-version checkpoint, observed issue or need, requested software change, and verified before/after result from the prescribed build conversation. Do not invent a defect in advance or count a stored decision revision as the software improvement.
- **Target alignment:** D011 and the supplied external brief's iteration requirement. Later capabilities get separate scope decisions.

## Keep implementation matched to the target

For every milestone PR, include the user action, expected visible result, relevant decision/brief requirement, and verification evidence. Classify contribution risk by actual behavior using the [review-tier proposal](repository-structure.md#proposed-contribution-tiers); a UI or adapter change affecting data or access receives foundation-level review. Mark automated checks separately from a real worker conversation, clean installation, new-user test, or external-reader walkthrough.

Use synthetic examples throughout automated tests and public demos. When a milestone exposes a gap in the selected scope, update the design record and brief before broadening implementation. Prefer a working slice of the whole journey over many isolated modules that have not been connected.

Reusable modules remain inside one repository. No microservices or separately published library are needed for this proposed first stage. Sync and team features require later authority and conflict decisions even though identifiers, portable records, and version relationships should accommodate future evolution.

## Proposed GitHub Actions plan

No Actions workflows currently exist for this application. Add workflows only when their scripts and runtime exist, with real commands and verified run results. Candidate file/job names below are proposals, not existing required checks.

| Automation | Proposed trigger and scope | What it verifies | What it cannot establish |
| --- | --- | --- | --- |
| Documentation check | PRs targeting `main` or the active stage; changes to project documents | Markdown consistency, local links, whitespace, and documented status | Approval of product choices or actual app usability |
| Application CI | PRs targeting `main` or `stage/**`, plus accepted stage/main pushes | Reproducible Rust/TypeScript setup, formatting, Clippy/lint, strict TypeScript checks, contract consistency, meaningful tests, and builds; one local check command mirrors CI | Correct capture in an actual external worker or account access |
| Package smoke test | Candidate implementation PRs and stage checkpoints, on the selected supported OS | Build/install/start the user package, isolated synthetic storage, restart persistence, and package contents | Desktop trust prompts and all human first-run friction |
| UI and publication-contract checks | Implementation PRs with synthetic fixtures and a local test publishing destination | Cards/history rendering, preview contents, failure handling, and read-only published representation | Real provider credentials, internet availability, or a working public URL |
| Demo publication | Explicit maintainer dispatch for an accepted commit and selected hosting route | Publish synthetic demonstration assets and verify the resulting URL | Authorization to publish a user's personal memory |
| Release package | Accepted tagged checkpoint or maintainer dispatch with recorded checks | Package reproducibly, attach installable assets and checksums, and generate accurate release notes | Successful installation on a real new user's machine without a walkthrough |

Implementation details to follow when creating workflows:

- Use ordinary `pull_request` checks for contributed code. Give validation jobs read-only repository permissions and no signing or publication credentials. Do not execute untrusted PR code in a privileged publishing path.
- Separate package testing from signed release/publication jobs. Use the chosen platform's actual packaging and trust requirements; signatures cannot be replaced by merely passing a build test.
- Run required checks for both stage-targeted and main-targeted PRs. Do not accidentally skip a required check with broad path filters; use a stable final required job that reports the applicable result.
- Add only actual verified job/context names to branch protection after the workflow runs successfully. Keep checks simple while there is one selected supported platform.
- CI artifacts hold packages, synthetic reports, and sanitized evidence, never the owner's memory database, private source excerpts, tokens, backups, or personal runtime logs. Set appropriate retention and avoid excessive repeated runs.
- Keep a manual or explicitly selected release/publication trigger initially. This protects the user-owned publication boundary; ordinary commits do not automatically publish local memory.
- GitHub Actions executes the build/check/publish jobs. It is not the runtime database or a permanent connection to the owner's local app. A downloadable Actions artifact and an end-user GitHub Release asset are different delivery surfaces.

Official references: [workflow triggers, branches, and permissions](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax), [workflow artifacts](https://docs.github.com/en/actions/tutorials/store-and-share-data), and [GitHub Releases](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases). These inform the automation proposal; workflows and required application checks remain unimplemented.

## Branch milestones and integration

The existing [branch lifecycle](branching-and-releases.md) remains the source of truth. Map the user journeys to it as follows:

| Branch | Planned contents | Exit evidence |
| --- | --- | --- |
| `stage/00-repository-foundation` | Current design notes, build brief, this plan, and pre-build choices | Accurate documentation and selected minimum scope; current policy/contact questions remain visible |
| `stage/01-working-prototype` | Steps 01.1–01.5: M0 feasibility after authorization, connected M1/M2 slice, then M3–M5 and integrated acceptance | One usable installable package, actual worker capture/revision, durable local memory, reader-visible URL, and applicable acceptance evidence |
| `stage/02-tested-improvement` | M6 based on the accepted first version | Observed user difficulty and verified software improvement, with updated installable package and public view |
| `stage/03-next-capability` | A later selected capability | New scoped brief and user evidence; no feature is preapproved |

Create each implementation-stage branch from the latest accepted `main` when its work starts. Future branch names are planned here; do not create them all now from the documentation baseline. Maintain one active stage and a draft stage-to-main PR showing progress. Merge only when that stage's exit evidence exists. Preserve accepted milestones with tags and release notes rather than permanently diverging branches.

Within an active stage, a contributor may use a short feature branch targeting that stage, such as `feat/first-run`, `feat/decision-capture`, `feat/decision-history`, `feat/public-sharing`, or `feat/backup-recovery`. These are optional proposed names, not current branches. A solo maintainer can instead use focused commits on the active stage. Stage-to-main integration still follows the protected-main PR process.

Do not create a `prototype-v1` checkpoint until M1–M5 work together and the first prototype is actually usable. Create `stage/02-tested-improvement` after the accepted first-version baseline, then retain the observed change at the later checkpoint. Git branches/tags supplement rather than replace the external brief's single-conversation evidence.

## Immediate next step

Rust for the engine, TypeScript for the interface, ease of use, and ChatGPT Work as the working context are confirmed. Establish repository access and an actual recording route in the intended Work conversation, then define the 01.1 SQLite/record contract. Use the approved Tauri delivery and public snapshot direction while closing their platform, hosting, and lifecycle details at the corresponding steps. This plan does not establish an active Work build, working MCP connection, or approval of unselected technologies. Application building starts within the agreed scope and authorization.
