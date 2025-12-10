const fs = require('fs');
const path = require('path');
const vm = require('vm');
const { expandSync } = require('../../../crates/macroforge_ts/index.js');

const filePath = path.join(__dirname, '../../vanilla/src/validators/string-validator-tests.ts');
const sourceCode = fs.readFileSync(filePath, 'utf8');
const result = expandSync(sourceCode, path.basename(filePath));

let code = result.code;

// Strip TypeScript
code = code.replace(/import\s*\{\s*Result\s*\}\s*from\s*["']macroforge\/result["'];?/g, '');
code = code.replace(/:\s*(?:string|number|boolean|unknown|any|void|null|undefined|bigint|Date|never)(?:\[\])?(?=\s*[=;,)\]}])/g, '');
code = code.replace(/:\s*(?:Record|Result|Map|Set|Array)<[^>]+>(?:\[\])?(?=\s*[=;,)\]}])/g, '');
code = code.replace(/:\s*[A-Z]\w*(?:\[\])?(?=\s*[=;,)\]}])/g, '');
code = code.replace(/\s+as\s+(?:string|number|boolean|unknown|any|void|null|undefined|bigint|Date|never)(?:\[\])?/g, '');
code = code.replace(/\s+as\s+(?:Record|Result|Map|Set|Array)<[^>]+>(?:\[\])?/g, '');
code = code.replace(/\s+as\s+[A-Z]\w*(?:\[\])?/g, '');
code = code.replace(/\):\s*(?:string|number|boolean|unknown|any|void|null|undefined|bigint|Date|never)(?:\[\])?\s*\{/g, ') {');
code = code.replace(/\):\s*(?:Record|Result|Map|Set|Array)<[^>]+>(?:\[\])?\s*\{/g, ') {');
code = code.replace(/\):\s*[A-Z]\w*(?:\[\])?\s*\{/g, ') {');
code = code.replace(/class\s+(\w+)<[^>]+>/g, 'class $1');
code = code.replace(/constructor\s*\(\s*init\s*:\s*\{[^}]+\}\s*\)/g, 'constructor(init)');
code = code.replace(/export\s+class\s+(\w+)/g, 'class $1');
code = code.replace(/export\s+const\s+(\w+)/g, 'const $1');
code = code.replace(/export\s+function\s+(\w+)/g, 'function $1');
code = code.replace(/export\s+\{[^}]+\}/g, '');

// Find class names (strip comments first)
const codeNoComments = code.replace(/\/\*[\s\S]*?\*\//g, '').replace(/\/\/.*$/gm, '');
const classMatches = codeNoComments.matchAll(/class\s+(\w+)\s*(?:\{|extends)/g);
const classNames = [...classMatches].map(m => m[1]);
const exportStatements = classNames.map(name => `exports.${name} = ${name};`).join('\n');
code += '\n' + exportStatements;

class Result {
  constructor(ok, value) { this._ok = ok; this._value = value; }
  static ok(v) { return new Result(true, v); }
  static err(v) { return new Result(false, v); }
  isOk() { return this._ok; }
  isErr() { return !this._ok; }
  unwrap() { return this._value; }
  unwrapErr() { return this._value; }
}

try {
  const script = new vm.Script(code, { filename: 'test.js' });
  const exports = {};
  const context = vm.createContext({
    Result,
    exports,
    console,
    Object,
    Array,
    String,
    Number,
    Boolean,
    Date,
    Map,
    Set,
    BigInt,
    JSON,
    Error,
    TypeError,
    isNaN,
    parseFloat,
    parseInt,
    URL,
    RegExp
  });
  script.runInContext(context);
  console.log('Success! Exports:', Object.keys(exports));

  // Test one
  const r = exports.EmailValidator.fromJSON({ email: 'test@example.com' });
  console.log('Email validation result:', r.isOk() ? 'OK' : 'Error: ' + r.unwrapErr());
} catch (e) {
  console.error('Error:', e.message);
  console.error('Stack:', e.stack);
  // Write the code to a file for inspection
  fs.writeFileSync('/tmp/debug-expanded.js', code);
  console.log('Expanded code written to /tmp/debug-expanded.js');
}
