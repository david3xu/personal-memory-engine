// Owner interface: display local cards, preserved history, and honest worker setup states.
import './style.css';
import { desktopAvailable, getDecisions, getHistory, getStatus, type LocalStatus } from './bridge';
import type { DecisionVersion } from '../../../contracts/generated/records';
const root = document.querySelector<HTMLDivElement>('#app');
if (!root) throw new Error('Application root is missing');
root.innerHTML = `
<aside class="sidebar">
  <div class="brand"><span class="brand-mark">m</span><div>Personal Memory<span>YOUR DECISIONS, YOURS TO KEEP</span></div></div>
  <div class="nav-label">WORKSPACE</div>
  <button id="nav-decisions" class="nav-button active"><span>▦</span> Decisions <span id="nav-count" class="count">0</span></button>
  <button id="nav-connection" class="nav-button"><span>↔</span> Worker connection</button>
  <div class="sidebar-note"><span class="local-dot"></span> Private by default<p>Your memory lives on this device.<br>You choose what to share.</p></div>
  <div class="sidebar-footer">Personal Memory Engine <span>Early prototype · 0.1.0</span></div>
</aside>
<main>
  <header><div class="breadcrumb">Your workspace <span>/</span> <strong id="breadcrumb-page">Decisions</strong></div><span class="privacy-chip"><span class="local-dot"></span> Local memory</span></header>
  <div id="error" class="error hidden" role="alert"></div>
  <section id="decisions-page" class="page">
    <div class="page-heading"><div><div class="eyebrow">A LITTLE CONTEXT GOES A LONG WAY</div><h1>Your decisions</h1><p>The choices you made, the reasons you gave, and how they changed.</p></div><button id="refresh" class="button secondary">↻ Refresh</button></div>
    <div class="summary-strip"><div><strong id="decision-total">0</strong><span>decisions kept</span></div><div><strong>Preserved</strong><span>earlier versions</span></div><div><strong id="worker-state">Not verified</strong><span id="worker-caption">worker activity</span></div></div>
    <div class="content-layout"><section id="cards" aria-label="Decision cards"></section><section id="detail" class="detail-panel" aria-label="Selected decision"></section></div>
  </section>
  <section id="connection-page" class="page hidden">
    <div class="page-heading"><div><div class="eyebrow">KEEP YOUR NORMAL CONVERSATION</div><h1>Connect your AI worker</h1><p>Your worker records a choice. This app keeps it on your device.</p></div></div>
    <div id="connection-content"></div>
  </section>
  <div id="notice" role="status" aria-live="polite"></div>
</main>`;
function el<T extends HTMLElement = HTMLElement>(selector: string): T {
  const element = document.querySelector<T>(selector);
  if (!element) throw new Error(`Missing interface element: ${selector}`);
  return element;
}
function node(tag: string, text = '', className = ''): HTMLElement {
  const result = document.createElement(tag);
  result.textContent = text;
  result.className = className;
  return result;
}
function button(
  label: string,
  action: () => void,
  className = 'button secondary',
): HTMLButtonElement {
  const result = document.createElement('button');
  result.textContent = label;
  result.className = className;
  result.addEventListener('click', action);
  return result;
}
function date(value: string): string {
  const d = new Date(value);
  return Number.isNaN(d.getTime())
    ? value
    : d.toLocaleString(undefined, { dateStyle: 'medium', timeStyle: 'short' });
}
function showError(error: unknown): void {
  const target = el('#error');
  target.textContent = error instanceof Error ? error.message : String(error);
  target.classList.remove('hidden');
}
function notify(message: string): void {
  el('#notice').textContent = message;
}
async function copy(value: string): Promise<void> {
  try {
    await navigator.clipboard.writeText(value);
    notify('Copied to clipboard.');
  } catch {
    notify('Copy is unavailable here. Select and copy the displayed text.');
  }
}
function page(connection: boolean): void {
  el('#decisions-page').classList.toggle('hidden', connection);
  el('#connection-page').classList.toggle('hidden', !connection);
  el('#nav-decisions').classList.toggle('active', !connection);
  el('#nav-connection').classList.toggle('active', connection);
  el('#breadcrumb-page').textContent = connection ? 'Worker connection' : 'Decisions';
}
let decisions: DecisionVersion[] = [];
let status: LocalStatus | undefined;
let selectedId: string | undefined;
let historyRequest = 0;
let refreshBusy = false;
let previousFingerprint = '';
function block(title: string, value: string | undefined, container: HTMLElement): void {
  const section = node('section', '', 'record-section');
  section.append(node('h3', title), node('p', value ?? 'Not stated', value ? '' : 'unstated'));
  container.append(section);
}
async function select(id: string): Promise<void> {
  selectedId = id;
  renderCards();
  const request = ++historyRequest;
  try {
    const history = await getHistory(id);
    if (request !== historyRequest) return;
    renderDetail(history);
  } catch (error) {
    showError(error);
  }
}
function renderCards(): void {
  const target = el('#cards');
  target.replaceChildren();
  if (!decisions.length) {
    const empty = node('div', '', 'empty-state');
    empty.append(
      node('div', '↳', 'empty-icon'),
      node('h2', 'Your choices deserve context.'),
      node(
        'p',
        'Connect an AI worker, make an explicit choice in your conversation, and keep the decision here.',
      ),
    );
    empty.append(button('Set up a worker →', () => page(true), 'button primary'));
    const steps = node('div', '', 'empty-steps');
    for (const [i, text] of [
      'Connect your worker',
      'Make a choice in chat',
      'See the saved decision',
    ].entries()) {
      const step = node('div');
      step.append(node('span', String(i + 1)), node('p', text));
      steps.append(step);
    }
    empty.append(steps);
    target.append(empty);
    el('#detail').classList.add('hidden');
    return;
  }
  el('#detail').classList.remove('hidden');
  const heading = node('div', '', 'list-heading');
  heading.append(node('span', 'LATEST CHOICES'), node('span', `${decisions.length} shown`));
  target.append(heading);
  for (const record of decisions) {
    const card = button(
      '',
      () => {
        void select(record.decision_id);
      },
      `decision-card${record.decision_id === selectedId ? ' selected' : ''}`,
    );
    const meta = node('div', '', 'card-meta');
    meta.append(
      node('span', record.submission.supersedes_version_id ? 'REVISED CHOICE' : 'DECISION'),
      node('span', new Date(record.recorded_at).toLocaleDateString()),
    );
    card.append(
      meta,
      node('h2', record.submission.chosen_option),
      node(
        'p',
        record.submission.rationale ?? 'Rationale was not stated.',
        record.submission.rationale ? '' : 'unstated',
      ),
    );
    const footer = node('div', '', 'card-footer');
    footer.append(node('span', record.submission.worker), node('span', 'View context →'));
    card.append(footer);
    target.append(card);
  }
  target.append(
    node(
      'p',
      'Showing the most recent 200 decisions. Earlier records remain in local storage.',
      'list-footnote',
    ),
  );
}
function renderDetail(history: DecisionVersion[], versionIndex = 0): void {
  const record = history[versionIndex];
  if (!record) return;
  const target = el('#detail');
  target.replaceChildren();
  const label = node('div', '', 'detail-heading');
  label.append(
    node('span', versionIndex === 0 ? 'CURRENT VERSION' : 'EARLIER VERSION', 'eyebrow'),
    node('span', 'Private', 'small-chip'),
  );
  target.append(label);
  target.append(
    node('h2', record.submission.chosen_option),
    node('p', `Recorded ${date(record.recorded_at)}`, 'muted'),
  );
  block('Stated rationale', record.submission.rationale, target);
  block('User statement', record.submission.user_statement, target);
  const alternatives = node('section', '', 'record-section');
  alternatives.append(node('h3', 'Rejected alternatives'));
  if (!record.submission.alternatives.length)
    alternatives.append(node('p', 'Not stated', 'unstated'));
  for (const alternative of record.submission.alternatives) {
    const item = node('div', '', 'alternative');
    item.append(
      node('strong', alternative.option),
      node('p', alternative.reason ?? 'Reason not stated', alternative.reason ? '' : 'unstated'),
    );
    alternatives.append(item);
  }
  target.append(alternatives);
  const evidence = node('section', '', 'record-section');
  evidence.append(node('h3', 'Available evidence'));
  if (!record.submission.evidence.length) evidence.append(node('p', 'None supplied', 'unstated'));
  for (const item of record.submission.evidence) {
    evidence.append(node('p', item.content));
    if (item.reference) evidence.append(node('p', item.reference, 'reference'));
  }
  target.append(evidence);
  const timeline = node('section', '', 'timeline record-section');
  timeline.append(
    node('h3', `Version history · ${history.length}`),
    node(
      'p',
      'Earlier choices stay intact. Select a version to read its original context.',
      'muted',
    ),
  );
  history.forEach((version, index) => {
    const item = button(
      '',
      () => renderDetail(history, index),
      `timeline-item${index === versionIndex ? ' active' : ''}`,
    );
    item.append(
      node('span', `Version ${history.length - index}${index === 0 ? ' · Current' : ''}`),
      node('strong', version.submission.chosen_option),
      node('small', date(version.recorded_at)),
    );
    timeline.append(item);
  });
  target.append(timeline);
  const attribution = node('div', '', 'attribution');
  attribution.append(
    node('span', `Submitted by ${record.submission.worker}`),
    node('small', 'Worker attribution is not independent verification of the user’s statement.'),
  );
  target.append(attribution);
}
const testPrompt =
  'This is a synthetic test decision: I choose a blue cover for my demo notebook because I prefer blue. Please record this explicit choice in Personal Memory Engine. No rejected alternatives or source links were stated.';
function renderConnection(): void {
  const target = el('#connection-content');
  target.replaceChildren();
  const overview = node('div', '', 'setup-card');
  overview.append(node('h2', '1. Open your local memory'));
  overview.append(
    node(
      'p',
      desktopAvailable
        ? 'Local storage is ready. The MCP helper uses this same directory.'
        : 'This is the browser development view. Open the desktop app to access local memory.',
    ),
  );
  if (status) {
    overview.append(
      node('code', status.database_path, 'path'),
      node(
        'p',
        status.helper_available
          ? 'The packaged recording helper is ready.'
          : 'Recording helper is missing. Rebuild or reinstall this package.',
        status.helper_available ? 'success' : 'warning',
      ),
    );
  }
  target.append(overview);
  const connection = node('div', '', 'setup-card');
  connection.append(node('h2', '2. Connect a worker'));
  connection.append(
    node(
      'p',
      'A local MCP worker starts the helper below. ChatGPT Work on desktop can use the local plugin packaged with this project. Cloud-hosted ChatGPT needs a separate private tunnel route. Availability depends on your account and workspace.',
    ),
  );
  if (status) {
    const config = JSON.stringify(
      {
        mcpServers: {
          'personal-memory': { command: status.helper_path, args: ['--data-dir', status.data_dir] },
        },
      },
      null,
      2,
    );
    connection.append(
      node('h3', 'Local worker configuration'),
      node('pre', config),
      button('Copy configuration', () => {
        void copy(config);
      }),
    );
    const quote = (value: string): string => `'${value.replaceAll("'", "'\\''")}'`;
    const command = `${quote(status.helper_path)} --data-dir ${quote(status.data_dir)}`;
    connection.append(
      node('h3', 'Helper command for an optional cloud tunnel'),
      node('pre', command),
      button('Copy helper command', () => {
        void copy(command);
      }),
    );
  }
  const link = document.createElement('a');
  link.textContent = 'Read OpenAI’s desktop plugin setup ↗';
  link.href = 'https://learn.chatgpt.com/docs/plugins';
  link.target = '_blank';
  link.rel = 'noopener noreferrer';
  connection.append(link);
  connection.append(
    node(
      'p',
      'A local plugin runs on this device. A cloud chat cannot reach the helper through a localhost URL alone. Verify actual recording in your chosen Work chat.',
      'muted',
    ),
  );
  target.append(connection);
  const check = node('div', '', 'setup-card');
  check.append(
    node('h2', '3. Verify one choice'),
    node(
      'p',
      'Once the worker lists the memory tools, use a clearly labeled synthetic decision. Verify its card here, then revise it in chat and inspect both versions.',
    ),
    node('blockquote', testPrompt),
    button('Copy test message', () => {
      void copy(testPrompt);
    }),
  );
  check.append(
    node(
      'p',
      status?.last_tool_use
        ? `Last MCP tool use: ${date(status.last_tool_use)}. This is activity history, not a live connection check.`
        : 'No MCP tool use has been observed for this local store.',
      'muted',
    ),
  );
  target.append(check);
}
async function refresh(force = false): Promise<void> {
  if (!desktopAvailable || refreshBusy) return;
  refreshBusy = true;
  try {
    const [nextDecisions, nextStatus] = await Promise.all([getDecisions(), getStatus()]);
    status = nextStatus;
    decisions = nextDecisions;
    el('#error').classList.add('hidden');
    el('#decision-total').textContent = String(decisions.length);
    el('#nav-count').textContent = String(decisions.length);
    el('#worker-state').textContent = status.last_tool_use ? 'Tool used' : 'Not verified';
    el('#worker-caption').textContent = status.last_tool_use
      ? date(status.last_tool_use)
      : 'worker activity';
    const fingerprint = JSON.stringify([decisions, status]);
    if (force || fingerprint !== previousFingerprint) {
      previousFingerprint = fingerprint;
      renderConnection();
      if (!selectedId || !decisions.some((record) => record.decision_id === selectedId))
        selectedId = decisions[0]?.decision_id;
      renderCards();
      if (selectedId) await select(selectedId);
    }
  } catch (error) {
    showError(error);
  } finally {
    refreshBusy = false;
  }
}
el('#nav-decisions').addEventListener('click', () => page(false));
el('#nav-connection').addEventListener('click', () => page(true));
el('#refresh').addEventListener('click', () => {
  void refresh(true);
});
renderCards();
renderConnection();
if (desktopAvailable) {
  void refresh(true);
  window.setInterval(() => {
    void refresh();
  }, 3000);
} else {
  el('#worker-state').textContent = 'Desktop needed';
}
