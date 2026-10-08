// Minimal development client for inspecting the packaged stdio helper without a model.
import { spawn } from 'node:child_process';
import { createInterface } from 'node:readline';
import { once } from 'node:events';
export async function connectMcp(command, args = [], env = process.env) {
  const child = spawn(command, args, { env, stdio: ['pipe', 'pipe', 'inherit'] });
  const pending = new Map();
  let nextId = 0;
  const exit = once(child, 'exit');
  const lines = createInterface({ input: child.stdout });
  const fail = (error) => {
    for (const request of pending.values()) request.reject(error);
    pending.clear();
  };
  child.on('error', fail);
  lines.on('line', (line) => {
    let message;
    try {
      message = JSON.parse(line);
    } catch {
      fail(new Error('The helper wrote non-protocol output to stdout.'));
      return;
    }
    const request = pending.get(message.id);
    if (!request) return;
    pending.delete(message.id);
    if (message.error) request.reject(new Error(message.error.message));
    else request.resolve(message.result);
  });
  const notify = (method, params) =>
    child.stdin.write(
      JSON.stringify({ jsonrpc: '2.0', method, ...(params ? { params } : {}) }) + '\n',
    );
  const call = (method, params = {}) =>
    new Promise((resolve, reject) => {
      const id = ++nextId;
      const timer = setTimeout(() => {
        pending.delete(id);
        reject(new Error(`MCP request timed out: ${method}`));
      }, 10000);
      pending.set(id, {
        resolve: (value) => {
          clearTimeout(timer);
          resolve(value);
        },
        reject: (error) => {
          clearTimeout(timer);
          reject(error);
        },
      });
      child.stdin.write(JSON.stringify({ jsonrpc: '2.0', id, method, params }) + '\n');
    });
  await call('initialize', {
    protocolVersion: '2025-11-25',
    capabilities: {},
    clientInfo: { name: 'personal-memory-development-check', version: '0.1.0' },
  });
  notify('notifications/initialized');
  return {
    call,
    async close() {
      child.stdin.end();
      const timer = setTimeout(() => child.kill(), 5000);
      try {
        await exit;
      } finally {
        clearTimeout(timer);
        lines.close();
      }
    },
  };
}
