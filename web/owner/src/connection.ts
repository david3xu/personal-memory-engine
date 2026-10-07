// Present the owner's graphical connection journey and an optional advanced configuration.
import {
  desktopAvailable,
  connectChatGPT,
  startConnectionTest,
  pauseRecording,
  openConnectionDocs,
  type LocalStatus,
  type WorkerStatus,
} from './bridge';
type Actions = {
  run: (action: () => Promise<void>, message: string) => void;
  copy: (text: string) => void;
  busy: boolean;
};
function node(tag: string, text = '', className = ''): HTMLElement {
  const result = document.createElement(tag);
  result.textContent = text;
  result.className = className;
  return result;
}
export function workerLabel(state?: WorkerStatus): string {
  if (!state) return desktopAvailable ? 'Checking setup' : 'Desktop needed';
  if (!state.enabled) return 'Paused';
  if (state.receipt) return 'Recording verified';
  return state.test_request_id ? 'Waiting for test' : 'Not verified';
}
export function renderSetup(
  target: HTMLElement,
  local: LocalStatus | undefined,
  state: WorkerStatus | undefined,
  actions: Actions,
): void {
  target.replaceChildren();
  const addButton = (
    container: HTMLElement,
    label: string,
    action: () => void,
    disabled = false,
    primary = false,
  ): void => {
    const button = document.createElement('button');
    button.textContent = label;
    button.className = `button ${primary ? 'primary' : 'secondary'}`;
    button.disabled = actions.busy || disabled || !desktopAvailable;
    button.addEventListener('click', action);
    container.append(button);
  };
  const progress = node('div', '', 'setup-progress');
  progress.setAttribute('role', 'status');
  progress.append(
    node('strong', workerLabel(state)),
    node(
      'p',
      state?.receipt && state.enabled
        ? 'Your test choice was saved on this device. This confirms a completed recording, not a live connection.'
        : state?.enabled === false
          ? 'Workers cannot read or record decisions. Your saved cards are still available here.'
          : 'No conversation is monitored. Only choices you ask your worker to record are saved.',
    ),
  );
  target.append(progress);
  const localCard = node('div', '', 'setup-card');
  localCard.append(
    node('h2', '1. Your memory stays here'),
    node(
      'p',
      desktopAvailable
        ? 'The app keeps your decisions on this Mac. There is no memory account or database to set up.'
        : 'Open the installed desktop app to keep decisions on your device. This browser view is for development.',
    ),
  );
  if (state?.package_available)
    localCard.append(node('p', 'The recording helper and connector are included.', 'success'));
  else if (desktopAvailable && state)
    localCard.append(
      node('p', 'This package is incomplete. Reinstall the app before connecting.', 'warning'),
    );
  target.append(localCard);
  const connect = node('div', '', 'setup-card');
  connect.append(
    node('h2', '2. Connect ChatGPT'),
    node(
      'p',
      'Open the bundled plugin page in ChatGPT desktop and choose Install. Then return here. Your account or workspace must allow local plugins in Work.',
    ),
  );
  addButton(
    connect,
    state?.enabled === false
      ? 'Reconnect ChatGPT'
      : state?.catalog_prepared
        ? 'Open plugin page'
        : 'Connect ChatGPT',
    () =>
      actions.run(connectChatGPT, 'In ChatGPT, install Personal Memory Engine, then return here.'),
    !state?.package_available,
    true,
  );
  if (state?.catalog_prepared)
    connect.append(
      node(
        'p',
        'The connector is prepared. Opening its page does not confirm installation. If tools are missing after installing, restart ChatGPT and try the test.',
        'muted',
      ),
    );
  addButton(connect, 'Setup help', () =>
    actions.run(openConnectionDocs, 'Opened the desktop plugin guide.'),
  );
  target.append(connect);
  const test = node('div', '', 'setup-card');
  test.append(
    node('h2', '3. Send a test choice'),
    node(
      'p',
      'Open a test chat with a clearly labeled sample choice already in the message box. Review it and press Send in ChatGPT. Return here to see the saved card.',
    ),
  );
  addButton(
    test,
    state?.receipt
      ? 'Run another test'
      : state?.test_request_id
        ? 'Start a new test chat'
        : 'Open test chat',
    () =>
      actions.run(
        startConnectionTest,
        'Review the sample choice and press Send in ChatGPT. This app is waiting for its saved card.',
      ),
    !state?.catalog_prepared || !state.enabled,
    true,
  );
  if (state?.receipt && state.enabled) {
    test.append(
      node(
        'p',
        `Test saved ${new Date(state.receipt.recorded_at).toLocaleString()}. Reported worker: ${state.receipt.worker}.`,
        'success',
      ),
    );
    test.append(
      node(
        'p',
        'Worker names are supplied by the worker. Next, ask it to record your explicit choices in a normal conversation. Changes preserve earlier versions.',
        'muted',
      ),
    );
  } else if (state?.test_request_id && state.enabled) {
    test.append(
      node(
        'p',
        'Waiting for this test choice. If the chat did not open, copy the message below into a new Work chat with the plugin enabled.',
        'warning',
      ),
    );
  }
  if (state?.test_prompt && state.enabled) {
    const fallback = document.createElement('details');
    fallback.append(
      node('summary', 'Test message / chat did not open'),
      node('blockquote', state.test_prompt),
    );
    addButton(fallback, 'Copy test message', () => actions.copy(state.test_prompt ?? ''));
    fallback.append(
      node(
        'p',
        'Check that the plugin is enabled and the worker reports a successful record_decision call. A tool error or tool listing will not pass this test.',
        'muted',
      ),
    );
    test.append(fallback);
  }
  target.append(test);
  if (state?.enabled && state.catalog_prepared) {
    const pause = node('div', '', 'setup-card');
    pause.append(
      node('h2', 'You control worker access'),
      node(
        'p',
        'Pause stops worker reads and recordings, including running helpers. Your existing cards stay intact. Use the plugin page in ChatGPT to disable or uninstall the connector there.',
      ),
    );
    addButton(pause, 'Pause worker access', () =>
      actions.run(pauseRecording, 'Worker access is paused. Saved decisions remain available.'),
    );
    target.append(pause);
  }
  const advanced = document.createElement('details');
  advanced.className = 'setup-card';
  advanced.append(node('summary', 'Advanced: other local workers and storage'));
  if (local) {
    advanced.append(node('p', 'Private database'), node('code', local.database_path, 'path'));
    const config = JSON.stringify(
      {
        mcpServers: {
          'personal-memory': { command: local.helper_path, args: ['--data-dir', local.data_dir] },
        },
      },
      null,
      2,
    );
    advanced.append(
      node(
        'p',
        'A compatible local MCP worker can use this configuration. No local address is exposed to cloud ChatGPT by this setup.',
      ),
      node('pre', config),
    );
    addButton(advanced, 'Copy worker configuration', () => actions.copy(config));
  }
  target.append(advanced);
}
