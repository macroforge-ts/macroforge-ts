/**
 * Smoke test: boot a real vtsls server with the ts-derive plugin and
 * rapidly open two macro-using files to mimic editor file switches.
 * Fails if the server crashes or returns an error response.
 */

const test = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawn } = require("node:child_process");

const VTSLS_BIN = path.resolve(
  __dirname,
  "../../../node_modules/@vtsls/language-server/bin/vtsls.js",
);
const PLUGIN_PATH = path.resolve(__dirname, ".."); // packages/ts-derive-plugin

function lspMessage(id, method, params) {
  return { jsonrpc: "2.0", id, method, params };
}

function lspNotification(method, params) {
  return { jsonrpc: "2.0", method, params };
}

function encodeMessage(msg) {
  const body = Buffer.from(JSON.stringify(msg), "utf8");
  return Buffer.concat([
    Buffer.from(`Content-Length: ${body.length}\r\n\r\n`, "utf8"),
    body,
  ]);
}

function makeFixtureDir() {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "tsmacros-vtsls-"));
  const macros = `
export declare const Derive: (...args: any[]) => ClassDecorator;
export declare const Debug: any;
`;
  const one = `import { Derive, Debug } from "./macros";
/** @derive(Debug) */
class One { id: string; }
const o = new One();
o.id;
`;
  const two = `import { Derive, Debug } from "./macros";
/** @derive(Debug) */
class Two { name: string; }
const t = new Two();
t.name;
`;
  fs.writeFileSync(path.join(root, "macros.ts"), macros);
  fs.writeFileSync(path.join(root, "one.ts"), one);
  fs.writeFileSync(path.join(root, "two.ts"), two);
  return {
    root,
    files: {
      one: {
        path: path.join(root, "one.ts"),
        uri: "file://" + path.join(root, "one.ts"),
        text: one,
      },
      two: {
        path: path.join(root, "two.ts"),
        uri: "file://" + path.join(root, "two.ts"),
        text: two,
      },
    },
  };
}

function startVtsls(cwd) {
  const child = spawn("node", [VTSLS_BIN, "--stdio"], {
    cwd,
    stdio: ["pipe", "pipe", "pipe"],
  });

  let buffer = Buffer.alloc(0);
  const messages = [];
  let closed = false;
  let exitCode = null;
  let stderr = "";

  function feed(chunk) {
    buffer = Buffer.concat([buffer, chunk]);
    while (true) {
      const sep = buffer.indexOf("\r\n\r\n");
      if (sep === -1) break;
      const header = buffer.slice(0, sep).toString("utf8");
      const match = header.match(/Content-Length: (\d+)/i);
      if (!match) {
        buffer = buffer.slice(sep + 4);
        continue;
      }
      const length = Number(match[1]);
      const total = sep + 4 + length;
      if (buffer.length < total) break;
      const body = buffer.slice(sep + 4, total).toString("utf8");
      buffer = buffer.slice(total);
      try {
        messages.push(JSON.parse(body));
      } catch (err) {
        messages.push({ error: `Failed to parse: ${err}`, raw: body });
      }
    }
  }

  child.stdout.on("data", feed);
  child.stderr.on("data", (d) => {
    stderr += d.toString();
  });
  child.on("close", (code) => {
    closed = true;
    exitCode = code;
  });

  return {
    child,
    messages,
    isClosed: () => closed,
    exitCode: () => exitCode,
    stderr: () => stderr,
    send: (msg) => child.stdin.write(encodeMessage(msg)),
  };
}

test("vtsls + ts-derive-plugin survives file switching", async (t) => {
  // Skip if vtsls is not installed
  if (!fs.existsSync(VTSLS_BIN)) {
    t.skip("vtsls binary missing - run npm install");
    return;
  }

  const { root, files } = makeFixtureDir();
  const server = startVtsls(root);

  // Initialize
  server.send(
    lspMessage(1, "initialize", {
      processId: process.pid,
      rootUri: "file://" + root,
      capabilities: {
        textDocument: { hover: {}, diagnostic: {} },
        workspace: { workspaceFolders: true },
      },
      initializationOptions: {
        typescript: {
          tsserver: {
            pluginPaths: [PLUGIN_PATH],
            globalPlugins: [
              {
                name: "@macroforge/macroforge-derive-plugin",
                location: PLUGIN_PATH,
                languages: ["typescript", "typescriptreact"],
                enableForWorkspaceTypeScriptVersions: true,
              },
            ],
          },
        },
      },
    }),
  );
  server.send(lspNotification("initialized", {}));

  const open = (file) =>
    server.send(
      lspNotification("textDocument/didOpen", {
        textDocument: {
          uri: file.uri,
          languageId: "typescript",
          version: 1,
          text: file.text,
        },
      }),
    );

  // Open two files in quick succession
  open(files.one);
  open(files.two);

  // Also poke hover requests to exercise mapping
  server.send(
    lspMessage(2, "textDocument/hover", {
      textDocument: { uri: files.one.uri },
      position: { line: 2, character: 10 },
    }),
  );
  server.send(
    lspMessage(3, "textDocument/hover", {
      textDocument: { uri: files.two.uri },
      position: { line: 2, character: 10 },
    }),
  );

  // Wait for a short window to see if the server crashes
  await new Promise((resolve) => setTimeout(resolve, 500));

  // Tear down
  server.child.kill();

  assert.strictEqual(
    server.exitCode(),
    null,
    `vtsls exited early. stderr: ${server.stderr()}`,
  );

  const errorResponses = server.messages.filter(
    (m) => m && m.error && m.error !== null,
  );
  assert.strictEqual(
    errorResponses.length,
    0,
    `Received error responses: ${JSON.stringify(errorResponses, null, 2)}`,
  );
});
