// Test script to monitor for crashes
// Run this before starting your editor to see if the native module is crashing

const fs = require('fs');
const logFile = '/tmp/ts-macros-crash.log';

function log(msg) {
  const timestamp = new Date().toISOString();
  fs.appendFileSync(logFile, `${timestamp}: ${msg}\n`);
  console.error(msg);
}

// Clear log file
fs.writeFileSync(logFile, '');

// Monitor for crashes
process.on('uncaughtException', (err) => {
  log(`UNCAUGHT EXCEPTION: ${err.stack || err.message || err}`);
  process.exit(1);
});

process.on('unhandledRejection', (reason, promise) => {
  log(`UNHANDLED REJECTION at ${promise}: ${reason}`);
});

process.on('SIGABRT', () => {
  log('SIGABRT - Process aborted');
  process.exit(1);
});

process.on('SIGSEGV', () => {
  log('SIGSEGV - Segmentation fault');
  process.exit(1);
});

log('Loading native module...');

try {
  const { expandSync } = require('@ts-macros/swc-napi');
  log('Native module loaded successfully');

  // Test basic expansion
  const testCode1 = `
import { Derive } from "@macro/derive";

@Derive(Debug)
class User {
  name: string;
}
`;

  log('Testing first expansion...');
  const result1 = expandSync(testCode1, 'test1.ts', { keepDecorators: true });
  log(`First expansion succeeded: ${result1.code.length} chars`);

  // Test second expansion (simulating file switch)
  const testCode2 = `
import { Derive } from "@macro/derive";

@Derive(Clone)
class Product {
  id: number;
}
`;

  log('Testing second expansion (file switch simulation)...');
  const result2 = expandSync(testCode2, 'test2.ts', { keepDecorators: true });
  log(`Second expansion succeeded: ${result2.code.length} chars`);

  // Test switching back
  log('Testing third expansion (switch back to first file)...');
  const result3 = expandSync(testCode1, 'test1.ts', { keepDecorators: true });
  log(`Third expansion succeeded: ${result3.code.length} chars`);

  log('All expansions completed successfully! The issue might be in the TS plugin, not the native module.');

} catch (e) {
  log(`ERROR: ${e.stack || e.message || e}`);
  process.exit(1);
}

console.log(`\nCheck ${logFile} for detailed logs`);
