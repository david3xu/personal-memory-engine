// Guide explicit selection, frozen public preview and verified link sharing in one owner dialog.
import type { DecisionVersion } from '../../../contracts/generated/records';
import {
  sharingStatus,
  githubAccount,
  connectGitHub,
  openGitHubSignIn,
  prepareSnapshot,
  publishSnapshot,
  verifySnapshot,
  withdrawSnapshot,
  openSnapshot,
  type Publication,
  type ShareStatus,
} from './sharing-bridge';
function node<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  text = '',
  className = '',
): HTMLElementTagNameMap[K] {
  const item = document.createElement(tag);
  item.textContent = text;
  item.className = className;
  return item;
}
function button(
  text: string,
  action: () => void,
  className = 'button secondary',
): HTMLButtonElement {
  const item = node('button', text, className);
  item.type = 'button';
  item.onclick = action;
  return item;
}
function field(label: string, input: HTMLInputElement): HTMLLabelElement {
  const item = node('label', '', 'share-field');
  item.append(node('span', label), input);
  return item;
}
function toggle(text: string): { label: HTMLLabelElement; input: HTMLInputElement } {
  const input = node('input');
  input.type = 'checkbox';
  return { label: field(text, input), input };
}
function renderPreview(entry: Publication, target: HTMLElement): void {
  target.replaceChildren(node('h3', entry.snapshot.title));
  for (const card of entry.snapshot.cards) {
    const article = node('article', '', 'share-preview-card');
    card.versions.forEach((version, index) => {
      const section = node('section');
      section.append(
        node(
          'strong',
          `${index === card.versions.length - 1 ? 'Current choice' : 'Earlier version'}: ${version.chosen_option}`,
        ),
        node('p', `Recorded ${version.recorded_at}`, 'muted'),
        node('h4', 'Stated rationale'),
        node('p', version.rationale ?? 'Not stated'),
      );
      if (version.alternatives.length) {
        section.append(node('h4', 'Rejected alternatives'));
        for (const alt of version.alternatives)
          section.append(node('p', `${alt.option}: ${alt.reason ?? 'Reason not stated'}`));
      }
      if (version.evidence.length) {
        section.append(node('h4', 'Included source evidence'));
        for (const item of version.evidence) {
          section.append(node('p', item.content));
          if (item.reference) section.append(node('p', item.reference, 'reference'));
        }
      }
      article.append(section);
    });
    target.append(article);
  }
}
export function openSharing(records: DecisionVersion[]): void {
  const existing = document.querySelector<HTMLDialogElement>('#sharing-dialog');
  if (existing?.open) return;
  existing?.remove();
  const dialog = node('dialog', '', 'sharing-dialog');
  dialog.id = 'sharing-dialog';
  const heading = node('div', '', 'share-heading');
  heading.append(
    node('h2', 'Share selected decisions'),
    button('Close', () => dialog.close()),
  );
  const intro = node(
    'p',
    'Choose what to share, review the public copy, then publish a read-only link. Your private database stays on this Mac.',
  );
  const error = node('p', '', 'error hidden');
  error.setAttribute('role', 'alert');
  const account = node('div', '', 'share-account');
  const selection = node('div', '', 'share-selection');
  const selected = new Set<string>();
  const title = node('input');
  title.value = 'Shared decisions';
  title.maxLength = 160;
  const history = toggle('Include earlier versions');
  const evidence = toggle('Include source evidence');
  const preview = node('div');
  const publicationActions = node('div', '', 'share-actions');
  const links = node('div');
  const repository = node('input');
  repository.placeholder = 'https://github.com/your-account/sharing-repository';
  let draft: Publication | undefined;
  let busy = false;
  let generation = 0;
  let lastStatus: ShareStatus | undefined;
  let loginTimer: number | undefined;
  let verificationTimer: number | undefined;
  let startedAt = 0;
  function invalidate(): void {
    generation++;
    draft = undefined;
    preview.replaceChildren();
    publicationActions.replaceChildren();
  }
  function showError(value: unknown): void {
    error.textContent = value instanceof Error ? value.message : String(value);
    error.classList.remove('hidden');
  }
  async function run(action: () => Promise<void>): Promise<void> {
    if (busy) return;
    busy = true;
    error.classList.add('hidden');
    dialog.querySelectorAll<HTMLButtonElement>('button').forEach((item) => (item.disabled = true));
    try {
      await action();
    } catch (value) {
      // A failed request can still have reached GitHub; recover its persisted pending state.
      try {
        await loadStatus();
      } catch {
        // Preserve the original actionable error when local state is unavailable too.
      }
      showError(value);
    } finally {
      busy = false;
      dialog
        .querySelectorAll<HTMLButtonElement>('button')
        .forEach((item) => (item.disabled = false));
    }
  }
  for (const record of records) {
    const choice = toggle(record.submission.chosen_option);
    choice.input.onchange = () => {
      if (choice.input.checked) selected.add(record.decision_id);
      else selected.delete(record.decision_id);
      invalidate();
    };
    selection.append(choice.label);
  }
  title.oninput = invalidate;
  history.input.onchange = invalidate;
  evidence.input.onchange = invalidate;
  const prepare = button(
    'Preview public copy',
    () => {
      void run(async () => {
        if (!selected.size) throw new Error('Choose at least one decision.');
        const requested = generation;
        const next = await prepareSnapshot(
          [...selected],
          title.value,
          history.input.checked,
          evidence.input.checked,
        );
        if (requested !== generation) return;
        draft = next;
        renderPreview(next, preview);
        publicationActions.replaceChildren(
          node(
            'p',
            'Everything shown above will be public. Published content may remain in GitHub history or readers’ copies after withdrawal.',
            'muted',
          ),
          button(
            'Publish this preview',
            () => {
              void run(async () => {
                if (!draft) throw new Error('Prepare the preview again.');
                const entry = await publishSnapshot(draft.id, repository.value.trim());
                draft = undefined;
                publicationActions.replaceChildren(
                  node(
                    'p',
                    'Submitted. The public page is being verified; deployment can take a few minutes.',
                  ),
                );
                await loadStatus();
                startVerification(entry.id);
              });
            },
            'button primary',
          ),
        );
      });
    },
    'button primary',
  );
  function showLinks(state: ShareStatus): void {
    links.replaceChildren();
    if (!state.entries.length) return;
    links.append(node('h3', 'Your published copies'));
    for (const entry of [...state.entries].reverse()) {
      const item = node('article', '', 'share-link');
      item.append(
        node('strong', entry.snapshot.title),
        node(
          'p',
          entry.status === 'live'
            ? 'Public link verified'
            : entry.status === 'withdrawn'
              ? 'Live copy withdrawn'
              : entry.status === 'withdrawal_pending'
                ? 'Withdrawal is deploying'
                : 'Publication is awaiting verification',
          'muted',
        ),
      );
      if (entry.status === 'live' && entry.url) {
        item.append(
          node('p', entry.url, 'reference'),
          button('Copy URL', () => {
            void run(async () => {
              await navigator.clipboard.writeText(entry.url!);
              item.append(node('span', ' URL copied.'));
            });
          }),
          button('Open public page', () => {
            void run(() => openSnapshot(entry.id));
          }),
        );
      }
      if (entry.status === 'pending' || entry.status === 'withdrawal_pending')
        item.append(
          button('Check public page', () => {
            void run(async () => {
              await verifySnapshot(entry.id);
              await loadStatus();
            });
          }),
        );
      if (entry.status === 'pending')
        item.append(
          button('Retry this publication', () => {
            void run(async () => {
              await publishSnapshot(entry.id, entry.repository!);
              await loadStatus();
              startVerification(entry.id);
            });
          }),
        );
      if (entry.status === 'withdrawal_pending')
        item.append(
          button('Retry withdrawal', () => {
            void run(async () => {
              await withdrawSnapshot(entry.id);
              await loadStatus();
              startVerification(entry.id);
            });
          }),
        );
      if (entry.status === 'live' || entry.status === 'pending')
        item.append(
          button('Withdraw live copy', () => {
            const confirm = node('div');
            confirm.append(
              node(
                'p',
                'Withdraw this live page? Local decisions stay intact. GitHub history and downloaded copies may remain.',
              ),
              button('Confirm withdrawal', () => {
                void run(async () => {
                  await withdrawSnapshot(entry.id);
                  await loadStatus();
                  startVerification(entry.id);
                });
              }),
              button('Keep published', () => confirm.remove()),
            );
            item.append(confirm);
          }),
        );
      links.append(item);
    }
  }
  async function loadStatus(): Promise<void> {
    const next = await sharingStatus();
    lastStatus = next;
    if (!repository.value) repository.value = next.repository;
    showLinks(next);
    if (next.login.error) showError(next.login.error);
  }
  function startVerification(id: string): void {
    window.clearInterval(verificationTimer);
    startedAt = Date.now();
    verificationTimer = window.setInterval(() => {
      if (!dialog.open || Date.now() - startedAt > 300000) {
        window.clearInterval(verificationTimer);
        return;
      }
      void run(async () => {
        const next = await verifySnapshot(id);
        await loadStatus();
        if (next.status === 'live' || next.status === 'withdrawn')
          window.clearInterval(verificationTimer);
      });
    }, 10000);
  }
  async function loadAccount(): Promise<void> {
    try {
      const name = await githubAccount();
      account.replaceChildren(node('p', `Publishing with GitHub account ${name}.`));
    } catch {
      account.replaceChildren(
        node('p', 'Connect your GitHub account once to publish public copies.'),
        button('Connect GitHub', () => {
          void run(async () => {
            await connectGitHub();
            account.replaceChildren(node('p', 'Complete GitHub’s sign-in in your browser.'));
            loginTimer = window.setInterval(() => {
              void run(async () => {
                await loadStatus();
                if (lastStatus?.login.code) {
                  account.replaceChildren(
                    node('p', `GitHub sign-in code: ${lastStatus.login.code}`),
                    button('Open GitHub sign-in', () => {
                      void run(openGitHubSignIn);
                    }),
                  );
                }
                if (!lastStatus?.login.running) {
                  window.clearInterval(loginTimer);
                  await loadAccount();
                }
              });
            }, 3000);
          });
        }),
      );
    }
  }
  dialog.append(
    heading,
    intro,
    error,
    account,
    field('Public GitHub repository', repository),
    field('Snapshot title', title),
    node('h3', 'Select decisions'),
    selection,
    history.label,
    evidence.label,
    prepare,
    preview,
    publicationActions,
    links,
  );
  document.body.append(dialog);
  dialog.showModal();
  dialog.addEventListener('close', () => {
    window.clearInterval(loginTimer);
    window.clearInterval(verificationTimer);
  });
  void run(async () => {
    await loadStatus();
    await loadAccount();
  });
}
