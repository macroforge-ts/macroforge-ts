#!/usr/bin/env node
const fs = require('node:fs');
const path = require('node:path');
const { expandSync } = require('../crates/swc-napi-macros');

const roots = [
  path.join(__dirname, '..', 'playground', 'svelte', 'src', 'lib', 'demo'),
  path.join(__dirname, '..', 'playground', 'vanilla', 'src'),
];

function isSourceFile(file) {
  return file.endsWith('.ts') && !file.endsWith('.expanded.ts');
}

function expandFile(file) {
  const code = fs.readFileSync(file, 'utf8');
  const outPath = file.replace(/\.ts$/, '.expanded.ts');
  try {
    const res = expandSync(code, file, { keepDecorators: false });
    fs.writeFileSync(outPath, res.code);
    console.log(`expanded ${path.relative(process.cwd(), file)} -> ${path.relative(process.cwd(), outPath)}`);
  } catch (err) {
    console.error(`failed to expand ${file}: ${err.message || err}`);
    process.exitCode = 1;
  }
}

for (const root of roots) {
  if (!fs.existsSync(root)) continue;
  for (const entry of fs.readdirSync(root)) {
    const full = path.join(root, entry);
    if (fs.statSync(full).isFile() && isSourceFile(entry)) {
      expandFile(full);
    }
  }
}
