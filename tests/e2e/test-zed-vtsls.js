/**
 * Test vtsls plugin loading using the same vtsls binary that Zed uses
 */
import { spawn } from 'node:child_process';
import path from 'node:path';
import fs from 'node:fs';

const ZED_VTSLS = '/Users/jakoblochinski/Library/Application Support/Zed/languages/vtsls/node_modules/@vtsls/language-server/bin/vtsls.js';
const REPO_ROOT = path.resolve(import.meta.dirname, '../..');
const PLUGIN_PATH = path.join(REPO_ROOT, 'packages', 'typescript-plugin', 'dist', 'index.js');
const LOG_DIR = path.join(REPO_ROOT, 'tmp-zed-vtsls-logs');

// Create log directory
fs.mkdirSync(LOG_DIR, { recursive: true });

console.log('Using vtsls:', ZED_VTSLS);
console.log('Plugin path:', PLUGIN_PATH);
console.log('Log directory:', LOG_DIR);

// Check if files exist
if (!fs.existsSync(ZED_VTSLS)) {
  console.error('Zed vtsls not found!');
  process.exit(1);
}
if (!fs.existsSync(PLUGIN_PATH)) {
  console.error('Plugin not found!');
  process.exit(1);
}

const server = spawn('node', [ZED_VTSLS, '--stdio'], {
  cwd: REPO_ROOT,
  env: { ...process.env },
});

let msgId = 0;
const lspMessage = (id, method, params) => {
  const msg = JSON.stringify({ jsonrpc: '2.0', id, method, params });
  return `Content-Length: ${Buffer.byteLength(msg)}\r\n\r\n${msg}`;
};

// Send initialize with our plugin config - using vtsls.tsserver namespace
const initParams = {
  processId: process.pid,
  rootUri: `file://${path.join(REPO_ROOT, 'playground', 'svelte')}`,
  capabilities: {},
  initializationOptions: {
    vtsls: {
      tsserver: {
        logDirectory: LOG_DIR,
        logVerbosity: 'verbose',
        globalPlugins: [{
          name: '@macroforge/typescript-plugin',
          location: path.join(REPO_ROOT, 'packages', 'typescript-plugin'),
          languages: ['typescript', 'typescriptreact'],
          enableForWorkspaceTypeScriptVersions: true,
        }],
      },
    },
  },
};

console.log('\nSending initialize with options:', JSON.stringify(initParams.initializationOptions, null, 2));

server.stdin.write(lspMessage(++msgId, 'initialize', initParams));

let initialized = false;
server.stdout.on('data', (data) => {
  const text = data.toString();
  console.log('Response:', text.slice(0, 500));

  // After initialize response, send initialized notification and workspace config
  if (!initialized && text.includes('"id":1')) {
    initialized = true;
    console.log('\n--- Sending initialized notification ---');
    const initializedMsg = JSON.stringify({ jsonrpc: '2.0', method: 'initialized', params: {} });
    server.stdin.write(`Content-Length: ${Buffer.byteLength(initializedMsg)}\r\n\r\n${initializedMsg}`);

    console.log('--- Sending workspace/didChangeConfiguration ---');
    const configMsg = JSON.stringify({
      jsonrpc: '2.0',
      method: 'workspace/didChangeConfiguration',
      params: {
        settings: {
          vtsls: {
            tsserver: {
              logDirectory: LOG_DIR,
              logVerbosity: 'verbose',
              globalPlugins: [{
                name: '@macroforge/typescript-plugin',
                location: path.join(REPO_ROOT, 'packages', 'typescript-plugin'),
                languages: ['typescript', 'typescriptreact'],
                enableForWorkspaceTypeScriptVersions: true,
              }],
            },
          },
        },
      },
    });
    server.stdin.write(`Content-Length: ${Buffer.byteLength(configMsg)}\r\n\r\n${configMsg}`);
  }
});

server.stderr.on('data', (data) => {
  console.error('stderr:', data.toString());
});

// Wait a bit then check for logs
setTimeout(() => {
  console.log('\n--- Checking for tsserver logs ---');
  const logs = fs.readdirSync(LOG_DIR);
  if (logs.length > 0) {
    console.log('Log files created:', logs);
    const logFile = logs.find(f => f.includes('tsserver'));
    if (logFile) {
      const content = fs.readFileSync(path.join(LOG_DIR, logFile), 'utf8');
      console.log('\n--- First 100 lines of log ---');
      console.log(content.split('\n').slice(0, 100).join('\n'));
    }
  } else {
    console.log('No log files created - initialization options not applied!');
  }
  server.kill();
  process.exit(0);
}, 5000);
