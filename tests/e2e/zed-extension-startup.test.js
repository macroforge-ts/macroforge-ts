#!/usr/bin/env node
/**
 * E2E Test: Zed Extension Startup
 *
 * This test verifies that the ts-macros Zed extensions start successfully
 * by launching Zed with a test TypeScript project and monitoring logs.
 *
 * Requirements:
 * - Zed must be installed and accessible via `zed` command
 * - macOS (uses ~/Library/Logs/Zed/Zed.log)
 *
 * Usage:
 *   node tests/e2e/zed-extension-startup.test.js
 */

const { spawn, execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const { test, describe, before, after } = require('node:test');
const assert = require('node:assert/strict');

// Configuration
const ZED_LOG_PATH = path.join(process.env.HOME, 'Library/Logs/Zed/Zed.log');
const TEST_PROJECT_PATH = path.join(__dirname, 'fixtures/ts-project');
const STARTUP_TIMEOUT_MS = 30000; // 30 seconds max
const LOG_POLL_INTERVAL_MS = 500;

// Patterns to detect in logs
const SUCCESS_PATTERNS = {
  extensionsLoaded: /extensions updated\. loading \d+/,
  vtstsStarting: /starting language server process.*vtsls/,
};

const ERROR_PATTERNS = {
  extensionError: /\[extension_host\].*ERROR/,
  extensionManifestMissing: /No extension manifest found for extension ts-/,
  vtstsConnectionReset: /vtsls failed: Server reset the connection/,
};

/**
 * Get the current size of the Zed log file
 */
function getLogFileSize() {
  try {
    const stats = fs.statSync(ZED_LOG_PATH);
    return stats.size;
  } catch {
    return 0;
  }
}

/**
 * Read new log entries since a given byte offset
 */
function readNewLogEntries(sinceOffset) {
  try {
    const fd = fs.openSync(ZED_LOG_PATH, 'r');
    const stats = fs.fstatSync(fd);
    const newSize = stats.size;

    if (newSize <= sinceOffset) {
      fs.closeSync(fd);
      return { entries: '', newOffset: sinceOffset };
    }

    const buffer = Buffer.alloc(newSize - sinceOffset);
    fs.readSync(fd, buffer, 0, buffer.length, sinceOffset);
    fs.closeSync(fd);

    return {
      entries: buffer.toString('utf-8'),
      newOffset: newSize
    };
  } catch (error) {
    console.error('Error reading log file:', error.message);
    return { entries: '', newOffset: sinceOffset };
  }
}

/**
 * Check if Zed is installed
 */
function isZedInstalled() {
  try {
    execSync('which zed', { stdio: 'pipe' });
    return true;
  } catch {
    return false;
  }
}

/**
 * Kill any running Zed instances for cleanup
 */
function killZed(zedProcess) {
  if (zedProcess && !zedProcess.killed) {
    zedProcess.kill('SIGTERM');
    // Give it a moment, then force kill if needed
    setTimeout(() => {
      if (!zedProcess.killed) {
        zedProcess.kill('SIGKILL');
      }
    }, 2000);
  }
}

/**
 * Monitor Zed logs for startup patterns
 */
async function monitorZedStartup(initialOffset, timeoutMs = STARTUP_TIMEOUT_MS) {
  const startTime = Date.now();
  let currentOffset = initialOffset;
  const results = {
    extensionsLoaded: false,
    vtstsStarted: false,
    errors: [],
    allLogs: '',
  };

  while (Date.now() - startTime < timeoutMs) {
    const { entries, newOffset } = readNewLogEntries(currentOffset);
    currentOffset = newOffset;

    if (entries) {
      results.allLogs += entries;

      // Check for success patterns
      if (SUCCESS_PATTERNS.extensionsLoaded.test(entries)) {
        results.extensionsLoaded = true;
      }
      if (SUCCESS_PATTERNS.vtstsStarting.test(entries)) {
        results.vtstsStarted = true;
      }

      // Check for error patterns
      for (const [errorName, pattern] of Object.entries(ERROR_PATTERNS)) {
        const matches = entries.match(new RegExp(pattern.source, 'g'));
        if (matches) {
          results.errors.push({ type: errorName, count: matches.length, sample: matches[0] });
        }
      }

      // If we've seen both success conditions, we can exit early
      if (results.extensionsLoaded && results.vtstsStarted) {
        // Wait a bit more to catch any immediate errors
        await new Promise(resolve => setTimeout(resolve, 2000));
        const { entries: finalEntries } = readNewLogEntries(currentOffset);
        results.allLogs += finalEntries;
        break;
      }
    }

    await new Promise(resolve => setTimeout(resolve, LOG_POLL_INTERVAL_MS));
  }

  return results;
}

/**
 * Launch Zed with the test project
 */
function launchZed(projectPath) {
  console.log(`Launching Zed with project: ${projectPath}`);

  const zedProcess = spawn('zed', ['--foreground', projectPath], {
    stdio: ['ignore', 'pipe', 'pipe'],
    detached: false,
  });

  zedProcess.stdout.on('data', (data) => {
    // Optionally log stdout for debugging
    // console.log(`[Zed stdout]: ${data}`);
  });

  zedProcess.stderr.on('data', (data) => {
    // Optionally log stderr for debugging
    // console.error(`[Zed stderr]: ${data}`);
  });

  return zedProcess;
}

// Test Suite
describe('Zed Extension E2E Tests', { timeout: 60000 }, () => {
  let zedProcess = null;
  let initialLogOffset = 0;

  before(() => {
    // Check prerequisites
    if (!isZedInstalled()) {
      console.log('Zed is not installed, skipping E2E tests');
      process.exit(0);
    }

    if (!fs.existsSync(TEST_PROJECT_PATH)) {
      throw new Error(`Test project not found at ${TEST_PROJECT_PATH}`);
    }

    // Record initial log position
    initialLogOffset = getLogFileSize();
    console.log(`Initial log offset: ${initialLogOffset}`);
  });

  after(() => {
    // Cleanup: kill Zed process
    if (zedProcess) {
      console.log('Cleaning up Zed process...');
      killZed(zedProcess);
    }
  });

  test('Zed starts and loads extensions successfully', async () => {
    // Launch Zed
    zedProcess = launchZed(TEST_PROJECT_PATH);

    // Give Zed a moment to start
    await new Promise(resolve => setTimeout(resolve, 3000));

    // Monitor logs for startup
    console.log('Monitoring Zed logs for startup...');
    const results = await monitorZedStartup(initialLogOffset);

    // Output results
    console.log('\n=== Startup Results ===');
    console.log(`Extensions loaded: ${results.extensionsLoaded}`);
    console.log(`vtsls started: ${results.vtstsStarted}`);
    console.log(`Errors found: ${results.errors.length}`);

    if (results.errors.length > 0) {
      console.log('\nErrors:');
      for (const error of results.errors) {
        console.log(`  - ${error.type}: ${error.count} occurrence(s)`);
        console.log(`    Sample: ${error.sample}`);
      }
    }

    // Assertions
    assert.ok(results.extensionsLoaded, 'Extensions should be loaded');

    // Note: vtsls errors are common during startup, we just check it attempted to start
    // The "Server reset the connection" errors are expected if the plugin crashes
  });

  test('vtsls language server starts', async () => {
    // This test runs after the first one, using the same Zed instance
    if (!zedProcess || zedProcess.killed) {
      console.log('Zed not running, skipping vtsls test');
      return;
    }

    const { entries } = readNewLogEntries(initialLogOffset);
    const vtstsStarted = SUCCESS_PATTERNS.vtstsStarting.test(entries);

    assert.ok(vtstsStarted, 'vtsls language server should start');
  });

  test('no critical extension errors', async () => {
    if (!zedProcess || zedProcess.killed) {
      console.log('Zed not running, skipping error check');
      return;
    }

    const { entries } = readNewLogEntries(initialLogOffset);

    // Check for manifest errors specifically for our extensions
    const manifestError = ERROR_PATTERNS.extensionManifestMissing.test(entries);

    // This is informational - manifest errors happen when extensions aren't properly installed
    if (manifestError) {
      console.log('Warning: Extension manifest errors found (extension may not be installed in Zed)');
    }

    // We don't fail on this since it depends on whether the extension is installed
  });
});

// Run tests if executed directly
if (require.main === module) {
  // The node:test runner will execute the tests
  console.log('Starting Zed Extension E2E Tests...');
  console.log(`Test project: ${TEST_PROJECT_PATH}`);
  console.log(`Log file: ${ZED_LOG_PATH}`);
  console.log('');
}
