# Personal Memory Engine — First-stage build brief

Recorded: 2026-10-07 (Australia/Perth)

Status: active Stage 01 implementation authorized in D029. Core/storage and real-process MCP checks pass; the first macOS Apple Silicon desktop bundle builds. This engineering target is not a user decision excluding future platforms. Actual Work recording, selected public hosting, recovery, and a complete integrated release still require evidence.

## Target

Deliver a minimum local-first personal decision memory engine: a real AI worker records an explicit user choice through MCP, the record persists on the user's machine, a visual interface shows the card and preserved revisions, and an explicitly selected demonstration can be visited through a public URL.

A successful walkthrough should show one chosen option, its stated rationale and rejected alternatives when provided, a later changed choice linked to the original, both versions after restarting the local service, and an intentionally published view of synthetic example decisions. The synthetic public demonstration is a proposal, not approval to publish personal memory.

The confirmed direction is D021; local authoritative SQLite storage is D003-v3. See the [decision ledger](design-decisions.md) for requirements, rationale, available evidence, and their history. This brief defines the candidate delivery boundary; the [repository foundation](repository-foundation.md) describes later growth rationale, and the [directory proposal](repository-structure.md) defines module and contribution boundaries.

## Confirmed requirements

- Capture explicit user decisions through an existing AI worker using MCP. Do not scrape or monitor chats or treat the worker's suggestions as user decisions.
- Preserve chosen options, stated rationale, rejected alternatives with their stated reasons, and available source evidence. Do not invent missing content.
- Preserve earlier versions and link a changed choice to its earlier record.
- Store authoritative user memory on the user's local machine; the project does not operate a service holding users' memory.
- Provide a visual decision and history interface, plus a public URL other people can visit. Tauri owner delivery and selected read-only public snapshots are approved; supported platform, distribution details, exact selection controls, and hosting remain open.
- Start small while allowing later mature stages and reuse in a broader project. Split Stage 01 into reviewable steps that together deliver the full basic working app before adding broader features (D027); exact step grouping remains proposed.
- Use Rust for the memory engine (D025) and TypeScript for the interface (D026); Tauri for owner delivery, SQLite for local persistence, and public snapshots for selected sharing are approved. UI framework, runtime adapters, and detailed contracts remain to be selected.
- Make installation easy for users and make public-link sharing easy for readers (D022). Prioritize ease of use and work in ChatGPT Work (D028); actual folder access and MCP recording must be verified. First-platform distribution and snapshot hosting/lifecycle details still need design at their relevant steps.
- The supplied external brief requires one working promised capability, a reachable public URL, and a verified software change after the first version, with its build workflow in one ChatGPT Work conversation. These are separate from the user's application decisions.

## Proposed first implementation boundary

| Area | Proposed first-stage inclusion |
| --- | --- |
| Owner and instance | One owner, one machine, one authoritative local memory store |
| Installation | One selected primary platform with a packaged, straightforward launch path; measure setup and connection steps |
| Worker | One configured real worker, with explicit decision-recording instructions; select and verify its MCP connection before claiming support |
| Capture | Record an explicit choice and retrieve enough existing history to identify the record when a choice changes |
| Revision | Create a linked new version; validate references, avoid duplicate writes on retries, and preserve existing history |
| Storage | Embedded SQLite with durable local records and a versioned record format; no dependency on a project-operated memory database |
| Viewer | Decision list, card details, and a version timeline; useful empty and error states |
| Public demonstration | Owner deliberately publishes selected synthetic decisions/history as read-only hosted snapshots; the URL works with the local app stopped, subject to hosting availability |
| Architecture | Decision core, application operations, local storage, MCP adapter, viewer, and publication boundary separated within one repository |
| Delivery evidence | Real worker walkthrough, meaningful behavior checks, restart check, external URL verification, and an actual post-first-version software improvement |

Tauri is selected for the delivered owner application. A local browser viewer remains an optional development path for 01.1; final installation work completes by 01.4. The reusable Rust engine and SQLite adapter stay independent of the owner UI transport.

Related stated reasoning within a card remains in scope. Rich standalone argument records, causal relationship maps, and full workflow diagrams are proposed for later work; this staging choice does not withdraw the broader intent in D007.

## Proposed exclusions from the first implementation

- Multiple-device synchronization and conflict-resolution infrastructure.
- Team accounts, shared writable collections, and multi-owner permissions.
- Multiple production worker integrations or a public plugin-directory launch.
- Broad cross-platform installer support, automatic updating, or mobile apps unless separately selected. A usable installation and launch path for the selected primary platform is part of the proposed first stage.
- Rich relationship maps, workflow execution, embeddings, and AI-generated rationale.
- Passive conversation capture, an autonomous decision-maker, or a project-operated hosted memory service.
- Production reliability, security-audit, or scale guarantees beyond the verified prototype.

Export, backup, recovery, and deliberate deletion should have an explicit initial policy. A browser prototype must not treat disposable browser cache as the authoritative memory store. Any deliberately published copy exists outside the local-only private store; a live viewer served from the user's machine depends on that machine and its connection remaining available.

## Installation and sharing experience

Easy installation and public sharing are confirmed goals under D022. Tauri, SQLite, and selected public snapshots are approved. Detailed setup, lifecycle, and acceptance checks below remain design/verification work.

### Installation and first use

For the selected primary operating system, aim for download, install or unpack, and open. Bundle the necessary runtime where feasible; users should not need to clone the repository, compile code, install developer tools, or keep a terminal open. The Tauri app should manage the local engine lifecycle and present the TypeScript interface. Select the first supported platform, actual distribution format, and signing requirements before promising this experience.

The first-run flow should show where memory is stored, guide connection to the selected worker, and verify the connection with a synthetic sample. Distinguish worker authentication and connection availability from local app health. Provide clear recovery for a disconnected worker or unavailable service. Some account permissions and publishing setup may remain unavoidable; disclose and minimize those steps rather than hiding them behind a one-click claim.

Keep the authoritative database in a durable per-user application-data location, outside the installation folder, repository, temporary files, and disposable browser cache. Launches and reinstallations must not silently reset it. Define backup/export and update behavior; future schema migrations must preserve history and provide recovery from failure.

### Sharing

The approved sharing direction uses a deliberate flow: select a card and versions, preview exactly what will become public, publish, then copy the verified URL. Rationale, rejected-option reasons, and source evidence may contain private information, so the preview must cover those fields rather than only the title. Personal memory stays private until explicitly selected for publication; first-stage verification uses synthetic records.

Hosted read-only snapshots are selected for URLs that work while the local app is stopped, subject to hosting availability. The owner selects/configures the publishing destination, which stores only the deliberately published copy. This is separate from the private SQLite store and write-capable MCP connection. GitHub Pages is selected in D034. New publications have new URLs; private revisions do not update them, and withdrawal replaces the live page with a notice. no project-operated private-memory service or live local-backed sharing is approved for this delivery.

Define whether later decision revisions change an existing link, whether older versions appear, and how publication is withdrawn. Label the public view's version and publication time so readers understand its scope. Removing a hosted publication prevents future access there but cannot erase copies already downloaded by readers.

Show accurate states for publishing, failure, and success; do not claim a link is published before the destination succeeds. Keep publishing credentials out of source control and out of public pages. Visitors should be able to open the published view without installing the app or connecting an AI worker. New-owner installation and public-reader access are separate user journeys.

### Usability verification

Test the selected distribution in a clean environment without the maintainer's existing development setup. Verify installation, launch, synthetic capture, restart persistence, and the selected backup/recovery procedure. Test the public link in a private browser session on another device or network. Repeat the public viewer check with the owner's local app/service stopped; a selected hosted snapshot must remain readable. A real new-user walkthrough is needed before claiming installation is easy for ordinary users.

## Proposed acceptance checks

1. In a real worker conversation, an explicit synthetic user choice is recorded through MCP and displayed as a local card with the supplied content.
2. A test conversation containing only an AI suggestion or an unresolved user question does not produce a user-decision card.
3. Missing rationale, rejection reasons, and evidence remain absent. Supplied source excerpts or links retain attribution; nonexistent chat URLs are not fabricated.
4. A changed choice creates a linked version, and both choices remain visible. A concern alone does not silently change the chosen option.
5. Retrying the same submission creates no duplicate version. Invalid revision references fail clearly without modifying prior records.
6. Records and history survive a local service restart. The owner can identify the data location and perform the selected backup/recovery procedure.
7. The viewer safely renders submitted content, presents understandable failure and empty states, and shows the history on a narrow screen.
8. The chosen connection enforces intended recording access; public visitors can see only deliberately published demonstration content and cannot write through the recording path.
9. A publication action yields a public URL whose contents match the selected synthetic records. Verify it in a private browser window and from another device or connection. Verify the hosted snapshot remains readable with the owner's local app/service stopped.
10. After the first usable version, the user actually uses it, identifies a specific improvement, requests that software change, and verifies the result. A stored decision revision is not itself a software improvement.
11. Setup instructions and the selected stack's formatting, lint, type, tests, and build checks work before claiming a working release.
12. The selected user distribution installs and launches in a clean environment without the maintainer's development setup; first-run guidance makes storage and worker connection status understandable. A new-user walkthrough validates the intended ease of use.
13. Publication preview matches the public page; publishing failures are visible; subsequent-version and withdrawal behavior follow the selected policy. Readers can open the public page without installing the app.

The attribution examples test worker behavior; schema validation cannot independently prove what was said in an external conversation. Append-only versions preserve history but do not by themselves solve future concurrent-update semantics.

## Execution plan

See the [user-first implementation plan](implementation-plan.md) for work order, milestone evidence, and proposed GitHub Actions checks. Its [Stage 01 delivery steps](implementation-plan.md#stage-01-deliver-a-complete-minimum-in-small-steps) connect a first real capture slice to complete installation, history, sharing, recovery, and integrated delivery, followed by actual-use improvement. The [branching document](branching-and-releases.md) owns branch lifecycle and current protection settings. Those plans do not select the remaining choices below or establish that the required ChatGPT Work build conversation occurred.

## Choices to close at the relevant delivery step

1. **Interface and installation:** Tauri owner delivery is selected. Select the first supported operating system, distribution format, and signing plan. Cards plus timeline are the minimum visual proposal.
2. **Storage and recovery:** Embedded SQLite is selected. Define the driver, record contract, migrations, and initial export/backup/restore/deletion policy.
3. **Worker and connection:** Which real worker is tested first, what transport/authentication connects it to the local service, and is that route available to the user?
4. **Public sharing:** GitHub Pages is selected; frozen previews and explicit withdrawal are implemented. Verify live publication and withdrawal against the installed preview and an independent anonymous reader.
5. **Build context:** Use the prescribed single ChatGPT Work conversation for the build/use/improve/publish workflow if completing the supplied external project brief.

For 01.1, establish actual Work repository/recording access and agree the capture/storage contract before implementation. Close installation and publishing choices before their corresponding steps, as described in the implementation plan. Unselected technology choices are not implied approvals; the complete minimum release still requires the full brief's applicable checks.
