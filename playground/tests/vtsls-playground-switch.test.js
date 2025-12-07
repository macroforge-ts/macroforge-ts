/**
 * Boots vtsls with the ts-derive plugin and opens two real playground files,
 * waiting between them to mimic a user switching files in an editor.
 */

import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawn } from 'node:child_process';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const REPO_ROOT = path.resolve(__dirname, '..', '..');
const LOG_DIR = path.join(REPO_ROOT, 'tmp-vtsls-logs');
const VTSLS_BIN_CANDIDATES = [
  path.resolve(REPO_ROOT, 'node_modules/@vtsls/language-server/bin/vtsls.js'),
  path.resolve(__dirname, 'node_modules/@vtsls/language-server/bin/vtsls.js'),
];
const VTSLS_BIN = VTSLS_BIN_CANDIDATES.find((p) => fs.existsSync(p));
const PLUGIN_DIR = path.resolve(REPO_ROOT, 'packages/tsserver-plugin-macroforge');
const PLUGIN_DIST = path.resolve(PLUGIN_DIR, 'dist');
const PLUGIN_NAME = '@macroforge/tsserver-plugin-macroforge';
const PLUGIN_PATH = path.join(PLUGIN_DIST, 'index.js');

const FILES = [
  path.resolve(REPO_ROOT, 'playground/svelte/src/lib/demo/macro-user.ts'),
  path.resolve(
    REPO_ROOT,
    'playground/svelte/src/lib/demo/field-controller.ts',
  ),
];

function encode(msg) {
  const body = Buffer.from(JSON.stringify(msg), 'utf8');
  return Buffer.concat([
    Buffer.from(`Content-Length: ${body.length}\r\n\r\n`, 'utf8'),
    body,
  ]);
}

function lspMessage(id, method, params) {
  return { jsonrpc: '2.0', id, method, params };
}
function lspNotification(method, params) {
  return { jsonrpc: '2.0', method, params };
}

function waitForResponse(messages, id, timeout = 4000) {
  const start = Date.now();
  return new Promise((resolve, reject) => {
    const timer = setInterval(() => {
      const hit = messages.find((m) => m && m.id === id);
      if (hit) {
        clearInterval(timer);
        resolve(hit);
      } else if (Date.now() - start > timeout) {
        clearInterval(timer);
        reject(new Error(`timeout waiting for response ${id}`));
      }
    }, 50);
  });
}

function startServer() {
  fs.mkdirSync(LOG_DIR, { recursive: true });
  const tsLogFile = path.join(LOG_DIR, 'tsserver.log');
  const NODE_PATHS = [
    path.join(REPO_ROOT, 'node_modules'),
    path.join(__dirname, 'node_modules'),
    path.join(REPO_ROOT, 'playground', 'macro', 'node_modules'),
  ].filter(fs.existsSync);
  const NODE_PATH = NODE_PATHS.join(path.delimiter);

  const child = spawn('node', [VTSLS_BIN, '--stdio'], {
    cwd: REPO_ROOT,
    stdio: ['pipe', 'pipe', 'pipe'],
    env: {
      ...process.env,
      TSS_LOG: `-logToFile true -file ${tsLogFile} -level verbose`,
      NODE_PATH,
    },
  });

  let buf = Buffer.alloc(0);
  const messages = [];
  let stderr = '';
  let exitCode = null;
  let exitSignal = null;
  let closed = false;

  child.stdout.on('data', (chunk) => {
    buf = Buffer.concat([buf, chunk]);
    while (true) {
      const idx = buf.indexOf('\r\n\r\n');
      if (idx === -1) break;
      const header = buf.slice(0, idx).toString('utf8');
      const m = header.match(/Content-Length: (\d+)/i);
      if (!m) {
        buf = buf.slice(idx + 4);
        continue;
      }
      const len = Number(m[1]);
      const end = idx + 4 + len;
      if (buf.length < end) break;
      const body = buf.slice(idx + 4, end).toString('utf8');
      buf = buf.slice(end);
      try {
        messages.push(JSON.parse(body));
      } catch (err) {
        messages.push({ error: `parse error: ${err}`, raw: body });
      }
    }
  });
  child.stderr.on('data', (d) => {
    stderr += d.toString();
  });
  child.on('close', (code, signal) => {
    closed = true;
    exitCode = code;
    exitSignal = signal;
  });

  return {
    send: (msg) => child.stdin.write(encode(msg)),
    messages,
    stderr: () => stderr,
    isClosed: () => closed,
    exitCode: () => exitCode,
    exitSignal: () => exitSignal,
    stop: () => {
      if (!closed) child.kill();
    },
  };
}

function fileUri(p) {
  return 'file://' + p;
}

function toPosition(text, needle) {
  const idx = text.indexOf(needle);
  if (idx === -1) return { line: 0, character: 0 };
  const prefix = text.slice(0, idx);
  const lines = prefix.split('\n');
  return { line: lines.length - 1, character: lines[lines.length - 1].length };
}

test('vtsls playground switch', async (t) => {
  if (!VTSLS_BIN || !fs.existsSync(VTSLS_BIN)) {
    t.skip('vtsls binary missing - run npm install (root or playground/tests)');
    return;
  }

  const server = startServer();

  // initialize
  server.send(
    lspMessage(1, 'initialize', {
      processId: process.pid,
      rootUri: fileUri(path.join(REPO_ROOT, 'playground', 'svelte')),
      workspaceFolders: [
        {
          uri: fileUri(path.join(REPO_ROOT, 'playground', 'svelte')),
          name: 'svelte',
        },
      ],
      capabilities: {
        textDocument: { hover: {}, diagnostic: {} },
        workspace: { workspaceFolders: true },
      },
      initializationOptions: {
        typescript: {
            tsserver: {
              logDirectory: LOG_DIR,
              logVerbosity: 'verbose',
              allowLocalPluginLoads: true,
              pluginProbeLocations: [PLUGIN_DIST, PLUGIN_DIR],
              plugins: [
                {
                  name: PLUGIN_PATH
                },
              ],
              globalPlugins: [
                {
                  name: PLUGIN_PATH,
                  languages: ['typescript', 'typescriptreact'],
                  enableForWorkspaceTypeScriptVersions: true,
                },
              ],
            },
        },
      },
    }),
  );
  server.send(lspNotification('initialized', {}));

  const [first, second] = FILES;
  const firstText = fs.readFileSync(first, 'utf8');
  const secondText = fs.readFileSync(second, 'utf8');

  const sendOpen = (uri, text, version) =>
    server.send(
      lspNotification('textDocument/didOpen', {
        textDocument: {
          uri: fileUri(uri),
          languageId: 'typescript',
          version,
          text,
        },
      }),
    );

  const sendClose = (uri) =>
    server.send(
      lspNotification('textDocument/didClose', {
        textDocument: { uri: fileUri(uri) },
      }),
    );

  const sendChange = (uri, text, version) =>
    server.send(
      lspNotification('textDocument/didChange', {
        textDocument: { uri: fileUri(uri), version },
        contentChanges: [{ text }],
      }),
    );

  const sendHover = (uri, text, needle) =>
    server.send(
      lspMessage(Math.random(), 'textDocument/hover', {
        textDocument: { uri: fileUri(uri) },
        position: toPosition(text, needle),
      }),
    );

  const sendDefinition = (uri, text, needle) =>
    server.send(
      lspMessage(Math.random(), 'textDocument/definition', {
        textDocument: { uri: fileUri(uri) },
        position: toPosition(text, needle),
      }),
    );

  const wait = (ms) => new Promise((r) => setTimeout(r, ms));

  // Open first file, hover/definition
  sendOpen(first, firstText, 1);
  // Wait for plugin to fully initialize and expand macros (~500ms+ needed)
  await wait(1500);
  sendHover(first, firstText, 'MacroUser');
  // Send definition request (result may be empty for generated methods, which is expected)
  sendDefinition(first, firstText, 'toJSON');

  // Open second file, hover/definition
  sendOpen(second, secondText, 1);
  await wait(1000);
  sendHover(second, secondText, 'FormModel');
  // Send definition request for generated method
  sendDefinition(second, secondText, 'makeFormModelBaseProps');

  // Give the server time to process background work and crash if unstable
  await wait(500);
  sendHover(first, firstText, 'MacroUser');
  sendDefinition(first, firstText, 'MacroUser');

  // Send hover request on toJSON() call - if macros work, this should resolve
  const hoverToJSONId = Math.random();
  server.send(
    lspMessage(hoverToJSONId, 'textDocument/hover', {
      textDocument: { uri: fileUri(first) },
      position: toPosition(firstText, 'toJSON'),
    }),
  );

  // Give the server time to process
  await wait(2000);

  // Capture stderr early for debugging assertions
  const stderrOutput = server.stderr();

  // Wait for hover response
  const hoverResponse = await waitForResponse(server.messages, hoverToJSONId, 5000).catch(
    (err) => ({ error: err.message }),
  );

  // The hover on toJSON should return info about the method
  // If macros didn't expand, toJSON wouldn't exist and hover would return null/empty
  assert.ok(
    hoverResponse.result !== null && hoverResponse.result !== undefined,
    `Hover on toJSON() should return method info (proves macros expanded). ` +
    `Got: ${JSON.stringify(hoverResponse, null, 2)}\nstderr:\n${stderrOutput}`,
  );

  // Additional check: hover result should contain content about the method
  const hoverContent = hoverResponse.result?.contents;
  assert.ok(
    hoverContent !== undefined,
    `Hover should have contents. Got: ${JSON.stringify(hoverResponse.result, null, 2)}`,
  );

  // Check if server died on its own before we stop it
  const exitedEarly = server.isClosed();
  const exitCode = server.exitCode();
  const exitSignal = server.exitSignal();

  const errorResponses = server.messages.filter(
    (m) => m && m.error && m.error !== null,
  );
  assert.strictEqual(
    errorResponses.length,
    0,
    `vtsls returned error responses: ${JSON.stringify(errorResponses, null, 2)}`,
  );

  server.stop();

  // Fail if server exited unexpectedly (exitCode is null if still running when stopped)
  assert.strictEqual(
    exitedEarly ? exitCode : null,
    null,
    `vtsls exited early. exitCode=${exitCode} signal=${exitSignal} stderr=${stderrOutput}`,
  );

  // Surface stderr from the server as a failure
  assert.strictEqual(
    stderrOutput.trim(),
    '',
    `vtsls wrote to stderr: ${stderrOutput}`,
  );

  // Emit log file hint if present
  if (fs.existsSync(LOG_DIR)) {
    const logs = fs
      .readdirSync(LOG_DIR)
      .filter((f) => f.startsWith('tsserver') && f.endsWith('.log'))
      .map((f) => path.join(LOG_DIR, f));
    if (logs.length > 0) {
      logs.sort((a, b) => fs.statSync(b).mtimeMs - fs.statSync(a).mtimeMs);
      console.log(`tsserver log: ${logs[0]}`);
    } else {
      console.log('tsserver log: none found (check TSS_LOG env override)');
    }
  }
});
