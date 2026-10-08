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
  if (!state) return desktopAvailable ? 'Checking connection' : 'Desktop needed';
  if (!state.enabled) return 'Paused';
  if (state.host_state !== 'registered') return 'Connection needed';
  if (state.receipt) return 'Recording verified';
  return state.test_request_id ? 'Waiting for test' : 'Connected · not tested';
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
  const connected = state?.host_state === 'registered';
  const connect = node('div', '', 'setup-card');
  connect.append(
    node('h2', workerLabel(state)),
    node(
      'p',
      'Your decisions stay on this Mac. Once connected, make explicit choices in a normal chat; the AI records them through the local engine. No plugin mention is needed.',
    ),
  );
  if (!connected || !state?.enabled) {
    connect.append(
      node(
        'p',
        'Connect enables recording for supported chats on this desktop host. It does not monitor conversations. You can pause access here.',
      ),
    );
    if (state?.host_state === 'unavailable')
      connect.append(node('p', 'Install or update ChatGPT desktop before connecting.', 'warning'));
    if (state?.host_state === 'conflict')
      connect.append(
        node(
          'p',
          'A different server uses this connection name. Resolve it in ChatGPT’s MCP settings; it has not been replaced.',
          'warning',
        ),
      );
    if (state?.host_state === 'disabled')
      connect.append(
        node(
          'p',
          'Enable Personal Memory Engine and its tools in ChatGPT’s MCP settings.',
          'warning',
        ),
      );
    addButton(
      connect,
      state?.enabled === false ? 'Resume recording' : 'Connect once',
      () =>
        actions.run(connectChatGPT, 'Connection saved. Restart ChatGPT once, then use a new chat.'),
      !state?.package_available || state.host_state === 'unavailable',
      true,
    );
  } else {
    connect.append(
      node(
        'p',
        state.receipt
          ? `A test choice was saved ${new Date(state.receipt.recorded_at).toLocaleString()}. This confirms that recording completed; it is not a live connection check.`
          : 'Connection saved. Restart ChatGPT once to load the tools, then send the test choice in a new chat.',
        'muted',
      ),
    );
    addButton(connect, 'Pause recording', () =>
      actions.run(pauseRecording, 'Worker access is paused. Saved decisions remain available.'),
    );
  }
  target.append(connect);
  if (connected && state?.enabled) {
    const test = document.createElement('details');
    test.className = 'setup-card';
    test.open = !state.receipt;
    test.append(node('summary', state.receipt ? 'Test recording again' : 'Check recording once'));
    test.append(
      node(
        'p',
        'Open a local desktop test chat, review the sample choice and press Send. The saved card will appear here. This test does not establish support in cloud ChatGPT or Work.',
      ),
    );
    addButton(
      test,
      state.test_request_id ? 'Open a new test chat' : 'Open test chat',
      () =>
        actions.run(
          startConnectionTest,
          'Send the sample choice in the new local chat, then return to Decisions.',
        ),
      false,
      true,
    );
    if (state.test_request_id && !state.receipt)
      test.append(node('p', 'Waiting for the sample choice to be saved.', 'warning'));
    if (state.test_prompt) {
      const fallback = document.createElement('details');
      fallback.append(node('summary', 'Chat did not open?'), node('blockquote', state.test_prompt));
      addButton(fallback, 'Copy sample choice', () => actions.copy(state.test_prompt ?? ''));
      test.append(fallback);
    }
    target.append(test);
  }
  const advanced = document.createElement('details');
  advanced.className = 'setup-card';
  advanced.append(node('summary', 'Connection help and other workers'));
  advanced.append(
    node(
      'p',
      'Local MCP availability depends on the chat mode and host. ChatGPT Work needs a separate successful test. If the AI reports tools unavailable, open ChatGPT’s MCP settings and check Personal Memory Engine, then restart.',
    ),
  );
  addButton(advanced, 'Connection guide', () =>
    actions.run(openConnectionDocs, 'Opened the connection guide.'),
  );
  if (connected)
    addButton(advanced, 'Repair connection', () =>
      actions.run(
        connectChatGPT,
        'Connection repaired. Restart ChatGPT if its tools were unavailable.',
      ),
    );
  if (local) {
    const config = JSON.stringify(
      {
        mcpServers: {
          'personal-memory': { command: local.helper_path, args: ['--data-dir', local.data_dir] },
        },
      },
      null,
      2,
    );
    advanced.append(node('p', 'For another compatible local MCP worker:'), node('pre', config));
    addButton(advanced, 'Copy worker configuration', () => actions.copy(config));
    advanced.append(node('p', 'Private database'), node('code', local.database_path, 'path'));
  }
  target.append(advanced);
}
