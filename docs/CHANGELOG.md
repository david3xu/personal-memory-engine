# Changelog

## Unreleased

- Replaced per-chat plugin setup with direct, one-time desktop MCP registration using the host's bundled connection manager. Added normal-chat explicit-choice guidance and a plain synthetic test prompt.
- Verified real desktop-host tool discovery in isolated configuration, preserving unrelated settings and refusing name conflicts. Work capture remains a separate acceptance check.
- Moved connection controls into Settings and removed repeated status summaries and empty optional context from decision cards.
- Corrected optional plugin MCP schema metadata and bumped its package to 0.1.2.

- Applied the user-approved required Linux/macOS CI checks on main and verified administrator enforcement.

- Added visible desktop plugin installation guidance and a fixed destination for browser help.

- Added a separate desktop worker plugin, portable/compatibility manifests, local launcher, and explicit-choice recording skill.
- Verified local plugin installation/enablement and packaged helper discovery; documented actual Work conversation checks as pending.
- Installed and opened the owner app on the maintainer’s Mac; displayed the user’s explicit implementation choice saved through stdio MCP.

- Added the Tauri owner app with private cards, stated context, preserved version timeline, worker setup, and local activity/status.
- Built a macOS Apple Silicon app bundle containing its independent MCP helper.
- Generated a static browser validator to preserve the desktop content security policy without runtime evaluation.

- Added a standalone stdio MCP helper and real-process protocol checks for recording, retry, linked revision, reading, and restart.
- Restricted newly created local storage directories and SQLite files/sidecars to the current user on Unix.

- Started Stage 01 following explicit implementation authorization (D029).
- Added a dependency-isolated Rust engine, generated portable contracts, and append-only SQLite adapter.
- Verified explicit-choice validation, absent reasons, restart persistence, linked revisions, retry/concurrency handling, and database overwrite prevention.
- Added workspace formatting/lint/type/build checks, contract drift verification, CODEOWNERS, and implementation CI.

- Added the design decision ledger and repository foundation proposals.
- Prepared a documentation-only repository under Apache License 2.0.
- Added contribution and security guidelines, community expectations, and issue and pull-request templates.
- Planned separate implementation-stage branches and checkpoint tags.
- Recorded local-first storage, interface review, and installation/sharing usability requirements.
- Added a draft first-stage build brief and future growth proposal.
- Added a user-first implementation plan with installation-to-sharing milestones and proposed Actions/branch integration.
- Added a canonical directory/contribution-boundary proposal and documentation index, with clearer foundation-review guidance.
- Recorded Rust for the engine and TypeScript for the interface; the frontend framework was left open at that design checkpoint.
- Refined the proposed Rust/TypeScript directory boundary, portable contracts, and implementation checks.
- Recorded minimum-first incremental delivery and split the proposed Stage 01 plan into five user-visible checkpoints with integrated release evidence.
- Recorded ease-of-use priority and ChatGPT Work context; separated first-capture prerequisites from later packaging and public-sharing choices.
- Recorded approved Tauri owner delivery, embedded SQLite storage, and selected read-only public snapshots as linked decision revisions; updated current scope, layout, and staged work.

The first local recording checkpoint is published on the active Stage 01 branch; no complete Stage 01 release or public snapshot demo is published yet.

### App-led onboarding

- Bundle the existing local worker plugin inside the desktop app and prepare an isolated app-owned catalog.
- Open the plugin page and a prefilled, unsent test chat through supported desktop links.
- Verify a fresh persisted setup choice; keep tool activity distinct from recording evidence.
- Add durable worker pause, preserving append-only decisions and owner access.
- Provide drag-to-Applications preview packaging and a graphical user guide. Signing and real Work/fresh-machine checks remain outstanding.
