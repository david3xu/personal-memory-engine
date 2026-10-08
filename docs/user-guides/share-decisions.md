# Share selected decisions

The owner app publishes a frozen, read-only copy to GitHub Pages under your account. Readers need only the link; your Mac and app can be closed. Your private SQLite memory and MCP recording tools stay local.

## First-time setup

1. Have a GitHub account and a public repository you own with administrator access. Create one in GitHub's browser interface if needed; a dedicated sharing repository is easiest. Repository creation is not automated in this preview.
2. Open **Decisions → Share decisions**. If needed, choose **Connect GitHub**, open the sign-in page and enter the displayed code. Complete GitHub's approval in your browser.
3. Paste that repository's URL. Use its exact displayed name. The app remembers the destination for later publications.

The official GitHub CLI is included in the app; no separate installation or terminal commands are required. It uses its own GitHub authentication and credentials, outside the memory database. Existing compatible authentication can be reused. Browser login uses the CLI's standard authorization scopes, which are broader than a single snapshot. The CLI normally uses the operating system credential store but can fall back to its configuration file if that store is unavailable. Review GitHub's authorization screen. Do not paste tokens into the app or a chat. Fresh-account browser sign-in remains an end-to-end verification requirement for this preview.

The app uses an isolated `pme-public` branch and configures GitHub Pages from its root. It refuses private repositories, repositories outside the signed-in owner's account, and an existing unrelated Pages site or branch. The repository's source branch is not modified. GitHub Pages availability and account limits still apply.

## Publish

1. Select the decisions you want to share. Nothing is selected initially.
2. Enter a snapshot title. **Include earlier versions** and **Include source evidence** are off initially; enable them only when you want those fields public.
3. Choose **Preview public copy**. Review every choice, rationale, alternative, stated reason, date and included evidence. Missing reasons stay missing. Request identifiers, worker identity and source user statements are excluded.
4. Choose **Publish this preview**. This is the deliberate publication action. Everything in the preview becomes public, including in the repository's Git history.
5. Wait for **Public link verified**, then use **Copy URL** or **Open public page**.

A submitted commit is not yet a live link. The app anonymously checks that the public page matches the frozen preview exactly. Deployment can take several minutes. If you close the dialog, reopen it and choose **Check public page**. Interrupted or failed attempts stay visible and can be checked or retried; an error after submission does not prove that no public content was uploaded.

## Changes and withdrawal

A later local revision does not update a public copy. Prepare and publish a new snapshot to get a new URL. Previously published copies stay available until explicitly withdrawn.

In **Your published copies**, choose **Withdraw live copy**, review the notice, and confirm. The app replaces the live page with a withdrawal notice and verifies its deployment. If an attempt is interrupted, check or retry withdrawal. Private decisions and version history remain intact.

Withdrawal removes the content from the current live page. It does not purge public repository history, caches or copies readers already downloaded. Treat publication as disclosure of the full preview. The public page has no scripts, analytics, recording endpoint or access to unselected memory.

## Preview limitations

The package remains unsigned and unnotarized. Backup/restore and a fresh-Mac installation are still pending. Check the [implementation evidence](../implementation-status.md) for what has actually been verified; this sharing feature alone does not complete Stage 01.
