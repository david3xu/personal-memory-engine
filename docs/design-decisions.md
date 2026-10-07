# Personal Memory Engine — Design decisions

Recorded: 2026-10-07 (Australia/Perth)

Stage: product scope and conceptual design. Implementation is paused.

This is the initial decision ledger for the project. It records user choices from the design conversation, keeps assistant proposals separate, and leaves missing rationale or evidence explicitly absent. The user authorized this first Markdown record after reviewing the proposed snapshot and recording approach.

## Recording conventions

- A decision ID identifies a continuing design question; a version ID identifies one recorded choice on that question. These identifiers organize this Markdown ledger; they do not approve an application schema.
- Confirmed entries below reflect user instructions. A positive acknowledgment alone does not approve every detail of an assistant proposal.
- Rationale and rejection reasons are recorded only when the user stated them. “Not stated” means the conversation supplied no explicit reason.
- Source excerpts are user statements available in this conversation. No independently retrievable conversation URL or message IDs were supplied.
- The date above is the compilation date. Exact decision times were not separately captured.
- Subsequent explicit user choices will be appended. Revisions will link to earlier versions; historical entries will remain intact. An optional current-decision index can change without rewriting history.
- A question, concern, or argument will be recorded as related discussion unless the user explicitly changes the choice. Assistant proposals remain proposals until selected or approved.
- An implementation workflow may be discussed without authorizing implementation.

## Confirmed decisions

### D001 / D001-v1 — Prototype and open-source intent

- **Chosen direction:** Develop a small working prototype from a fresh codebase, intended for publication as open source on GitHub.
- **Stated rationale:** Not stated.
- **Rejected alternatives and reasons:** None stated.
- **Source:** “help me build a small working prototype of a user-owned decision memory app from a fresh code base”; “I will push this code base to the GitHub and make it open source.”
- **Previous version:** None.
- **Status:** Confirmed direction; implementation remains paused under D008.

### D002 / D002-v1 — Worker-mediated recording through MCP

- **Chosen direction:** An existing AI worker submits structured decisions through MCP as a byproduct of the user's conversation. The app does not scrape or monitor chats.
- **Stated rationale:** Not stated separately.
- **Explicitly excluded approaches:** Scraping and monitoring chats.
- **Reasons for exclusion:** Not stated.
- **Source:** “the AI worker records the structured decisions through one MCP as a byproduct of a conversation”; “our app does not scrape or monitor chats.”
- **Previous version:** None.
- **Status:** Confirmed.

### D003 / D003-v1 — User-owned memory

- **Chosen direction:** The memory belongs to the user; the project does not operate a service holding users' memory.
- **Stated rationale:** Not stated separately.
- **Explicitly excluded approach:** A project-operated service holding users' memory.
- **Reason for exclusion:** Not stated.
- **Source:** “user-owned decision memory app”; “we do not operate a service holding users' memory.”
- **Previous version:** None.
- **Status:** Confirmed boundary. Local versus user-hosted storage is undecided.

### D004 / D004-v1 — Explicit user decisions only

- **Chosen direction:** Record explicit user choices as decisions. AI suggestions are not user decisions. Related material is limited to the scope in D007.
- **Stated rationale:** Not stated separately.
- **Explicitly excluded approach:** Treating an AI suggestion as a user decision.
- **Reason for exclusion:** Not stated.
- **Source:** “one explicit user decision becomes one structured memory card”; “Do not treat AI suggestions as the user decision.”
- **Previous version:** None.
- **Status:** Confirmed. The exact capture threshold and attribution checks are undecided.

### D005 / D005-v1 — Minimum decision-card content

- **Chosen direction:** Capture the chosen option, stated rationale, rejected alternatives and their stated reasons, and available source evidence. Do not invent missing reasons or evidence.
- **Stated rationale:** The user introduced this as a minimum scope to make the first stage implementable.
- **Rejected alternatives and reasons:** None stated.
- **Source:** “to make sure this decision or this first stage is implementable, so I just define a minimum scope”; “the decision chosen option, stated rationale, and rejected alternatives, and the stated reasons”; “do not invent missing reasons or evidence.”
- **Previous version:** None.
- **Status:** Confirmed content requirements. Exact field names and schema are undecided.

### D006 / D006-v1 — Preserve versions and link changes

- **Chosen direction:** Preserve earlier decision versions. A changed choice creates a linked new version rather than silently overwriting history; attach source evidence when available.
- **Stated rationale:** Preserve history, as explicitly stated by the user. No additional tradeoff rationale was given.
- **Explicitly excluded approach:** Silently overwriting decision history.
- **Reason for exclusion:** The user requested preservation of versions and history.
- **Source:** “Available source evidence linked to the earlier record if the decision changes”; “preserve versions rather than silently overwriting history.”
- **Previous version:** None.
- **Status:** Confirmed. Storage-level enforcement and deletion behavior are undecided.

### D007 / D007-v1 — Related reasoning and relationship views

- **Chosen direction:** Preserve user-stated arguments, options, and decisions directly related to a decision. Enable examination of rationale, relationships, and changes over time through maps, timelines, or workflows.
- **Stated rationale:** The user wants to see relationships and why options were selected or not selected.
- **Rejected alternatives and reasons:** None stated.
- **Source:** “When the users change the decision or have an argument of this decision, have the options, we record all related decisions”; “have a map, or have a workflow, have a timing series, what's the relationship”; “why the users select this decision or not, the rationale.”
- **Previous version:** None.
- **Status:** Confirmed product intent. Exact record types, relationship vocabulary, and visualization behavior are undecided.

### D008 / D008-v1 — Discuss first and maintain a Markdown ledger

- **Chosen direction:** Pause implementation and discuss the project first. Create this first Markdown record after approval, then maintain it as the conversation continues.
- **Stated rationale:** The user wants to discuss the project before implementing it. No further reason was stated.
- **Explicitly deferred approach:** Implementing the prototype now.
- **Reason for deferral:** Discussion comes first, as requested by the user.
- **Source:** “don't implement it now. We just discuss this project first”; “after I approve, you will create a Markdown file to record this first, and then we will update it regularly”; “record the Markdown file we discussed before.”
- **Previous version:** None.
- **Status:** Confirmed. This authorizes recording, not application implementation.

### D009 / D009-v1 — Project name

- **Chosen direction:** Personal Memory Engine, with `personal-memory-engine` as the intended local folder and GitHub repository name.
- **Stated rationale:** Not stated. The assistant's naming rationale is not attributed to the user.
- **Rejected alternatives and reasons:** None explicitly stated. `decision-memory` was the earlier working folder name, followed by the assistant's `decision-trail`, `decision-ledger`, and `choice-map` suggestions.
- **Source:** “How about the personal memory engine? Something like this one as the GitHub repo name”; subsequent instruction: “Okay, now create a sample photo and then record the Markdown file we discussed before.”
- **Source interpretation:** “Photo” is interpreted as “folder,” following the user's explicit clarification that the earlier discussion concerned a local folder. The naming exchange is treated as adoption of Personal Memory Engine.
- **Previous version:** None; the earlier working name had no recorded decision version.
- **Status:** Adopted project name. No GitHub repository has been created, and name availability has not been checked.

### D010 / D010-v1 — Local project location

- **Chosen direction:** Keep the project folder under `~/Developer/`.
- **Stated rationale:** Not stated.
- **Rejected alternatives and reasons:** None explicitly stated. A preliminary empty folder also exists in the Codex workspace from the initial setup.
- **Source:** “I'd like to create another folder at the developer directory”; “this developer directory is correct”; “It's about local folder.”
- **Previous version:** None.
- **Status:** Confirmed. This ledger lives in the project's `docs/` directory.

## Assistant proposals — not approved application design

These were discussed as possible ways to realize the confirmed scope. They are not recorded as user decisions.

- **P001:** Separate application record types for decision versions, related arguments, source evidence, and relationships.
- **P002:** Application decision IDs, version IDs, previous-version links, and separately stored recording and decision times.
- **P003:** A small relationship vocabulary: supersedes, supports, challenges, depends on, and related to.
- **P004:** Distinguish questions or concerns from an actual changed choice; do not infer causal links from timestamps.
- **P005:** Use attribution and supporting excerpts to make worker submissions reviewable. The app cannot independently prove what happened in an external chat.
- **P006:** Append-only normal recording can coexist with deliberate user-controlled deletion. No deletion policy has been selected.
- **P007:** A local MCP server, local storage, and a small reviewing interface were initially proposed by the assistant. No stack, transport, or storage choice was approved.

## Open questions

- What qualifies as sufficiently explicit user selection, rejection, or evaluation?
- Which related arguments should be captured, and how should their speaker and source be represented?
- What evidence is optional, and what attribution must every submission contain?
- How should the worker find and identify the earlier decision when a user changes it?
- Where does each user keep their memory: locally, in user-hosted storage, or another user-controlled arrangement?
- Which worker integrations and MCP transport should the first prototype support?
- How should retries, competing updates, mistaken attribution, and corrections be handled?
- Which map or timeline view belongs in the first prototype? What explicit information would a workflow view require?
- What export, backup, and deletion controls should the user have?
- Which implementation stack and open-source license should be selected?

## Append-only update procedure

1. Identify an explicit user decision or related user-stated discussion.
2. Keep assistant suggestions and unresolved questions separate.
3. Append a new decision entry, or a new version with the same decision ID when an existing choice changes.
4. Include only stated rationale, rejection reasons, and available supporting evidence.
5. Link a changed choice to its earlier version and record a change reason only if supplied.
6. Preserve historical entries. If a record is mistaken, append a linked correction rather than silently editing its meaning.
7. Update a current-decision index if useful; the appended entries remain the historical source of truth.

## Initial recording event

- **Date:** 2026-10-07 (Australia/Perth); exact decision times not captured.
- **Action:** Compiled D001–D010 from the conversation and separated assistant proposals and open questions.
- **Authorization:** The user requested creation of the project folder and the previously discussed Markdown record.
- **Implementation state:** No application code, working MCP server, storage engine, or GitHub repository has been created.


## Stage-one requirements update — 2026-10-07

The following entries are appended to the initial ledger. Earlier entries remain unchanged; implementation is still paused.

### D011 / D011-v1 — Prove the smallest working release first

- **Chosen direction:** Before generating application code, define the minimum scope. The first stage should deliver the smallest implementation that can be tested and published, establish that it works, and then inform further development.
- **Stated rationale:** The user wants to make sure it works first and establish what can actually be built.
- **Rejected alternatives and reasons:** None explicitly stated. The user has not selected a final stage-one feature list or stack.
- **Source:** “before we generate the code, I want to think about the minimum scope”; “at the first stage just deliver the smallest implementation and check what we can actually build around test and publish”; “I just make sure it works first”.
- **Related decisions:** D001 (prototype), D008 (discussion before implementation).
- **Previous version:** None.
- **Status:** Confirmed delivery approach. Acceptance criteria below remain proposed.

### D012 / D012-v1 — A visible, shareable URL

- **Chosen direction:** The first-stage delivery must have a URL that other people can open and view.
- **Stated rationale:** The user wants the URL to be visible and shareable with other people.
- **Rejected alternatives and reasons:** None stated.
- **Source:** “I also need URL can be visible, can be shared with other people.”
- **Related decisions:** D003 (user-owned memory), D007 (decision views), D011 (tested and published minimum release).
- **Previous version:** None.
- **Status:** Confirmed requirement. Whether the URL is a sample-data demo, a live owner-controlled app, or an explicitly shared view is undecided. This does not authorize publishing private memory.

### Related context — Handshake and MCP assumptions

- The user said this work is based on a Handshake project. The local Handshake workspace is readable, but the particular brief governing this app has not been identified. No submission requirements are inferred from unrelated Handshake projects.
- The user said, “this one is based on the handshake project” and “We can assume it's up” when discussing MCP. MCP availability is therefore a planning assumption, not a verified working connection.
- An assumed available MCP connection does not establish that a real worker correctly selects and submits a user decision. That behavior still needs end-to-end verification.

### Additional assistant proposals — pending user selection

- **P008 — Smallest functional path:** One user-controlled memory store; capture a decision and a linked revision through an MCP tool; preserve available rationale, rejection reasons, and evidence; display cards and a version timeline. Start with one real worker integration. Preserve missing information explicitly. A concern alone does not change the chosen option.
- **P009 — Public demonstration:** Publish a read-only sample-data view or explicitly selected snapshot, while keeping real memory under the user's control. The public viewer is separate from the write-capable MCP endpoint. A public demo page is not itself a working MCP integration.
- **P010 — Alternative live delivery:** If the first URL must show real, live memory, use an owner-controlled deployment with authenticated access. The public URL requirement does not by itself require a centrally operated service storing other users' memory.
- **P011 — Proposed release checks:** One real worker records an explicit user choice; a revision preserves the first version and links to it; omitted rationale and evidence remain absent; a suggested option alone does not become a decision; retrying one submission does not duplicate it; records survive a restart; the published URL opens in another browser and exposes only the intended demo or shared data. Worker attribution behavior is tested with conversation examples, not claimed to be independently provable by schema validation.
- **P012 — Proposed scope boundary:** Use a simple version timeline for the first release; retain maps and full workflow visualization as later possibilities unless the Handshake brief requires them in the first stage. This is a proposed staging choice, not a withdrawal of D007.

### New open questions

- Which exact Handshake project brief or task supplies this app's delivery requirements?
- Does the first shareable URL need only to demonstrate the app using sample decisions, or must other people interact with a live recording workflow?
- Which worker should provide the first real end-to-end capture demonstration?
- Who controls the initial deployment and its persistent data?
- Is a timeline sufficient for stage one, or is a relationship map required immediately?

### Documentation checked for connection feasibility

- [OpenAI: Add custom MCP server](https://developers.openai.com/api/docs/guides/custom-mcp-server): ChatGPT custom MCP connections support read and write tools, server URL or tunnel connections, and streaming HTTP or SSE. Workspace access restrictions still apply.
- [OpenAI: Authentication](https://developers.openai.com/plugins/build/auth): User-specific data and write actions should authenticate users; authenticated MCP integrations use the documented OAuth authorization flow.
- These are feasibility references, not evidence of this project's own successful connection or an approved hosting decision.

### Recording event

- **Date:** 2026-10-07 (Australia/Perth); exact decision times not captured.
- **Action:** Appended D011–D012, related user context, proposed stage-one design and checks, and unresolved deployment questions.
- **Implementation state:** No application code generated and no deployment performed.


## Supplied project brief — 2026-10-07

**Source:** User-supplied attachment, `Pasted text.txt`, titled “Turn an Idea into a Working Prototype.” No public source URL was supplied. These are external project requirements, not newly invented user decisions. They refine the delivery context for D001, D011, and D012; historical entries remain unchanged.

### Explicit acceptance criteria

| ID | Requirement stated in the brief | Implication for this project |
| --- | --- | --- |
| R001 | “It’s live at a URL someone else can open” | Verify the published URL from a private browser window; the public deliverable must be reachable by another person. |
| R002 | “It does the one thing your idea promised” | The recording and version-preservation promise must function. A sample-only interface cannot establish that MCP capture works. |
| R003 | “You changed at least one thing after the first version” | Build a first version, use it, identify a concrete problem, change the prototype, and retest. A revised decision stored as data is not itself a change to the prototype. |

### Required build context and described workflow

- The brief states that the project happens in one ChatGPT conversation and requires ChatGPT Work.
- This design discussion and Markdown ledger do not establish that the prescribed ChatGPT Work build workflow has been completed.
- Both independent building and the guided route are allowed. The user has not selected a route.
- The described workflow is to build, use the result, try to break it, make changes, verify those changes, publish, and open the live URL in a private window before submitting it.
- Sites, Netlify, and Vercel are suggested publishing options in the supplied brief, not mandatory choices. No hosting choice, access, current pricing, or deployment suitability has been verified for this project.
- The brief's lists of possible features and visual changes are examples, not requirements to implement all of them.
- A GitHub repository is part of the user's open-source intent (D001), but is not one of the brief's three stated acceptance criteria.

### Proposed stage-one design refinement — not yet selected

- **P013 — Refinement of P008–P010:** Use one owner-controlled demo instance with sample decisions, one tested AI-worker/MCP connection, persistent storage, and a public read-only viewer. The authenticated worker records a choice and later records its linked revision; visitors can inspect cards and the preserved history. This is a functional demo of the recording workflow rather than a prepopulated page alone. No central service holding other users' personal memory is required.
- **P014 — Smallest visual scope:** A decision list, a detail view showing recorded rationale and rejected options, and a version timeline. Full relationship maps remain a later staging proposal; D007's broader product direction is unchanged.
- **P015 — Prototype iteration evidence:** Retain a short record of the first version, what the user actually observed during use, the change requested in the same build conversation, and the verified result. Do not invent a defect or preclaim a successful iteration.
- **P016 — Worker correctness limits:** Test explicit choices, suggestions, missing reasons, and changed choices through conversation examples. Schema validation can enforce input structure and storage rules; it cannot prove that an external worker's attribution is true or prevent every semantic hallucination.
- **P017 — Build context:** Carry the approved design into one ChatGPT Work conversation for the required build, use, revision, and publication workflow. This remains a plan until that workflow is actually performed.

### Proposed completion evidence

1. A real configured worker submits a sample user decision through MCP; the viewer displays the stored card.
2. A subsequent changed choice creates a linked version, preserving the original.
3. Missing rationale and evidence stay explicitly absent; a suggested option alone is not recorded as a user decision in the tested example.
4. A retry does not duplicate one submission, and records survive restart.
5. The user uses the first prototype, requests at least one concrete application change, and verifies the changed version.
6. The published URL opens in a private window and exposes only the intended sample records; anonymous visitors cannot write through the recording endpoint.
7. The required build and revision workflow is carried out in one ChatGPT Work conversation.

### Remaining choices

- Is an owner-operated, sample-data instance with a publicly viewable working history the desired first release, or must visitors be able to record their own decisions?
- Which worker is the first tested integration, and is its connection actually available?
- Where will the owner-controlled instance run and keep persistent data?
- Does the user select the proposed timeline-only visual scope for stage one?
- Which build route and publishing option should be used in the prescribed ChatGPT Work conversation?

### Recording event

- **Date:** 2026-10-07 (Australia/Perth).
- **Action:** Appended the supplied brief's acceptance criteria and workflow requirements, plus clearly marked design refinements and open choices.
- **Implementation state:** No application code, deployment, or prototype iteration performed.


## Backend-oriented app direction — 2026-10-07

### D013 / D013-v1 — Memory backend with a visual interface

- **Chosen direction:** Build a small backend-oriented memory app. MCP submits the user's explicit decisions made during ChatGPT Work conversations; an interface displays decision cards and related history. Publishing provides an external URL for viewing the app.
- **Scope interpretation:** “Record this ChatGPT work” is interpreted within the established decision-only boundary in D002 and D004, not as permission to capture whole conversations or unrelated work.
- **Stated rationale:** Not stated separately. The user describes the intended first stage as a mini app.
- **Rejected alternatives and reasons:** None stated.
- **Source:** “we just create a mini app”; “use MCP to record this ChatGPT work”; “we can think about this is a backend app”; “This app, it has the interface, can visualize the decision card or other things”; “the external URL for publish”.
- **Related decisions:** D002 (MCP recording), D004 (explicit decisions only), D006 (version preservation), D007 (visual views), D011 (minimum tested release), D012 (shareable URL).
- **Previous version:** None.
- **Status:** Confirmed high-level app direction. No backend stack, storage system, authentication method, hosting provider, or complete first-stage visual feature list has been selected.

### Design clarification — proposal and implications

- The memory service stores and retrieves structured records; MCP is its worker-facing interface; the viewer is its human-facing interface.
- Publishing is a deployment and access requirement. It makes the interface reachable but still requires persistent storage and a decision about what public visitors may read or write.
- The external viewer URL does not automatically establish a working worker/MCP connection.
- The previously proposed owner-controlled instance and publicly readable sample-data viewer remain proposals; this high-level architecture statement does not select visitor accounts or public access to real memory.
- No implementation is authorized by this design discussion; D008 remains in effect.

### Recording event

- **Date:** 2026-10-07 (Australia/Perth).
- **Action:** Appended D013 and deployment implications while preserving prior entries.
- **Implementation state:** No application code generated and no deployment performed.


## Reuse and open-source foundation — 2026-10-07

### D014 / D014-v1 — Reusable part of a broader project

- **Chosen direction:** Design this small app so its code can be reused later as part of a broader project.
- **Stated rationale:** The user says this app is a very small piece of a broader project.
- **Rejected alternatives and reasons:** None stated.
- **Source:** “this is a very small piece of a broader project”; “make sure our codebase can be reusable later”.
- **Related decisions:** D001 (small open-source prototype), D011 (minimum release), D013 (backend and viewer).
- **Previous version:** None.
- **Status:** Confirmed design requirement. Specific module boundaries and packaging remain proposed.

### D015 / D015-v1 — Foundation for open-source contributors

- **Chosen direction:** Plan the repository for public open-source use and future outside contributions, including a README, license, contribution guidelines, security information, and other appropriate repository foundations.
- **Stated rationale:** The user considers the project important and anticipates that other people may contribute later.
- **Rejected alternatives and reasons:** None stated.
- **Source:** “I plan to open source this project”; “README Markdown file, license, and contribution, security”; “maybe later other people can contribute to this open source project”.
- **Related decisions:** D001 (open-source intent), D009 (project name), D014 (reuse).
- **Previous version:** None.
- **Status:** Confirmed preparation direction. A GitHub owner, license, reporting contacts, detailed policies, and remote creation have not been selected or performed.

### Additional assistant proposals

- **P018 — Reusable module boundary:** Keep decision rules independent of MCP, database drivers, UI, and hosting. Start with one repository and extract a library later when another project needs it.
- **P019 — Community foundation:** Prepare an accurate README, a selected standard license, contribution/security/conduct documents, issue and PR templates, a changelog, and working development checks when the stack is chosen.
- **P020 — Repository design proposal:** [Repository foundation](repository-foundation.md) contains the proposed layout, responsibilities, contribution process, privacy boundaries, and publication checklist. It remains a proposal, not an approved stack or complete set of policies.
- **P021 — License candidates:** Apache-2.0 and MIT are under discussion. No license has been selected; any broader-project license constraint should be identified first.

### Recording event

- **Date:** 2026-10-07 (Australia/Perth).
- **Action:** Appended reuse and contribution requirements and saved the clearly marked repository foundation proposal.
- **Implementation state:** Design documentation only; no application code, GitHub repository, or deployment created.


## Stage branches and GitHub preparation — 2026-10-07

### D016 / D016-v1 — Separate branches for implementation stages

- **Chosen direction:** Plan different branches for different implementation stages and prepare the local project for a GitHub remote.
- **Stated rationale:** The app is part of a bigger project and the user wants the branching structure designed in advance.
- **Rejected alternatives and reasons:** None stated.
- **Source:** “because we think about this is a bigger project”; “different branches for the different implementation stage”; “design this in advance as well, and then prepare for the GitHub remote grade”.
- **Source interpretation:** “Remote grade” is interpreted as preparation for the GitHub remote. No GitHub owner, visibility, or license is selected by this statement.
- **Related decisions:** D014 (reuse), D015 (open-source foundation), D011 (minimum release), D008 (implementation paused).
- **Previous version:** None.
- **Status:** Confirmed branching requirement and local-preparation direction. The exact branch names, merge/protection rules, stage boundaries, and tags remain proposals.

### Additional assistant proposals and observed context

- **P022 — Stage lifecycle:** Use `main` as the reviewed baseline and one active `stage/<number>-<purpose>` branch at a time. Create future branches from accepted `main` when their work starts, merge completed stages through PRs, and retain checkpoint tags.
- **P023 — Stage names:** Repository foundation, working prototype, tested improvement, and later approved capability. [Branching and remote preparation](branching-and-releases.md) is the detailed proposal.
- **Observed GitHub context:** The connected GitHub account is `david3xu`. This is a possible repository owner, not a user-selected owner. No remote repository URL or availability has been verified.
- **Local preparation:** A design-stage README, ignore rules, and editor settings are being added; local Git is initialized with `main` as the initial branch name. No commit, stage branch, remote, or tag has yet been created.
- **Open choices:** License, attribution, personal versus organization ownership, initial visibility, real reporting contacts, and approval of the detailed branch plan.

### Recording event

- **Date:** 2026-10-07 (Australia/Perth).
- **Action:** Appended D016, saved the branch proposal, and prepared the documentation-only local repository.
- **Implementation state:** No application code generated and no GitHub repository or deployment created.


## Public GitHub repository and license — 2026-10-07

### D017 / D017-v1 — Connected personal account as repository owner

- **Chosen direction:** Use the connected GitHub account, `david3xu`, for `personal-memory-engine`.
- **Stated rationale:** Not stated.
- **Rejected alternatives and reasons:** None stated.
- **Source:** “already connected the Davisway to GitHub”.
- **Source interpretation:** The spoken account reference is resolved to the connected account `david3xu`, verified through both the GitHub connector and command-line authentication. No different GitHub login was supplied.
- **Related decisions:** D009 (repository name), D015 (open-source foundation), D016 (remote preparation).
- **Previous version:** None.
- **Status:** Selected owner.

### D018 / D018-v1 — Make the repository public now

- **Chosen direction:** Create the GitHub repository with public visibility now.
- **Stated rationale:** Not stated.
- **Rejected alternatives and reasons:** None explicitly stated. Initial private visibility had been offered as an alternative; no rejection reason was given.
- **Source:** “make it visible now, public now”.
- **Related decisions:** D001 (open-source intent), D017 (owner).
- **Previous version:** None.
- **Status:** Confirmed publication instruction. This applies to the repository foundation, not to private user memory or an unbuilt application.

### D019 / D019-v1 — Apache License 2.0

- **Chosen direction:** License the project under Apache License 2.0.
- **Stated rationale:** Not stated. The assistant's earlier patent-grant rationale is not attributed to the user.
- **Rejected alternatives and reasons:** None explicitly stated. MIT was previously considered; no rejection reason was supplied.
- **Source:** “use the Apache 2.0 license”.
- **Related decisions:** D014 (reuse), D015 (contributors).
- **Previous version:** None.
- **Status:** Selected license. The standard Apache license text is included in `LICENSE`; project attribution uses the verified repository owner's GitHub identifier rather than an invented legal name.

### Recording event

- **Date:** 2026-10-07 (Australia/Perth).
- **Action:** Appended the owner, immediate public visibility, and license choices. Prepared public repository documentation and contributor policies.
- **Implementation state:** No application code generated. Application implementation remains paused under D008.


### Verified repository creation event — 2026-10-07

- **Repository:** [https://github.com/david3xu/personal-memory-engine](https://github.com/david3xu/personal-memory-engine).
- **Owner and visibility:** `david3xu`, public; verified on GitHub.
- **License:** Official Apache License 2.0 text added; project attribution uses `david3xu`.
- **Security reporting:** GitHub private vulnerability reporting enabled and verified.
- **Publication review:** Intended public files checked for credential patterns and private absolute paths; local Markdown links resolve and issue-template YAML parses.
- **Next repository operation:** Publish the initial documentation baseline and prepare `stage/00-repository-foundation`; later application branches remain future work.
- **Implementation state:** No application code or hosted prototype exists; implementation remains paused.


### Verified foundation publication event — 2026-10-07

- **Published baseline:** Initial documentation commit `4648b62c93e5e4da95b6be7a6c917e3ee199c528` verified on remote `main`.
- **Published branches:** `main` and `stage/00-repository-foundation`; future application-stage branches are not created yet.
- **License detection:** GitHub recognizes `Apache-2.0`.
- **Main protection:** Pull requests and resolved review conversations required; administrator enforcement enabled; force pushes and deletion disabled. No nonexistent CI checks or second-reviewer requirement configured.
- **Security reporting:** Private vulnerability reporting verified enabled. A dedicated confidential conduct-reporting contact remains open.
- **Working stage:** Repository foundation and the still-pending first-stage build brief; application implementation remains paused.
