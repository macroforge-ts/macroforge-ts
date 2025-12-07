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
const PLUGIN_PATH = path.resolve(REPO_ROOT, 'packages/tsserver-plugin-macroforge');
const PLUGIN_NAME = '@macroforge/tsserver-plugin-macroforge';

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
              pluginPaths: [PLUGIN_PATH],
              plugins: [
                {
                  name: PLUGIN_NAME,
                  location: PLUGIN_PATH,
                },
              ],
              globalPlugins: [
                {
                  name: PLUGIN_NAME,
                  location: PLUGIN_PATH,
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

  // Explicit IDs we can await
  const defIds = {
    macroUserToJSON: Math.random(),
    formModelMakeProps: Math.random(),
  };

  // Open first file, hover/definition
  sendOpen(first, firstText, 1);
  await wait(200);
  sendHover(first, firstText, 'MacroUser');
  server.send(
    lspMessage(defIds.macroUserToJSON, 'textDocument/definition', {
      textDocument: { uri: fileUri(first) },
      position: toPosition(firstText, 'toJSON'),
    }),
  );

  // Open second file, hover/definition
  sendOpen(second, secondText, 1);
  await wait(200);
  sendHover(second, secondText, 'FormModel');
  server.send(
    lspMessage(defIds.formModelMakeProps, 'textDocument/definition', {
      textDocument: { uri: fileUri(second) },
      position: toPosition(secondText, 'makeFormModelBaseProps'),
    }),
  );

  // Apply a change to force re-parse on second
  sendChange(second, secondText + '\n// change', 2);
  await wait(200);
  sendHover(second, secondText, 'FormModel');

  // Switch focus back to first with version bump
  sendChange(first, firstText + '\n// change', 2);
  await wait(200);
  sendHover(first, firstText, 'MacroUser');
  sendDefinition(first, firstText, 'MacroUser');

  await wait(500);
  // Give the server time to process background work and crash if unstable
  await wait(2000);

  // Capture stderr early for debugging assertions
  const stderrOutput = server.stderr();

  // Validate definitions came back (proves plugin expanded macros)
  const defToJSON = await waitForResponse(server.messages, defIds.macroUserToJSON).catch(
    (err) => ({ error: err.message }),
  );
  const defMakeProps = await waitForResponse(server.messages, defIds.formModelMakeProps).catch(
    (err) => ({ error: err.message }),
  );

  assert.ok(
    Array.isArray(defToJSON.result) && defToJSON.result.length > 0,
    `definition for toJSON should resolve, got ${JSON.stringify(defToJSON)}`
      + `\nstderr:\n${stderrOutput}`,
  );
  assert.ok(
    Array.isArray(defMakeProps.result) && defMakeProps.result.length > 0,
    `definition for makeFormModelBaseProps should resolve, got ${JSON.stringify(defMakeProps)}`
      + `\nstderr:\n${stderrOutput}`,
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
