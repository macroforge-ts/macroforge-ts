/**
 * Get TypeScript diagnostics using the ts-derive-plugin
 * Usage: node scripts/get-diagnostics.cjs [file-path]
 */

const ts = require('typescript');
const path = require('path');
const fs = require('fs');

// Get the project root (where this script is located)
const scriptDir = __dirname;
const projectRoot = path.dirname(scriptDir);
const targetFile = process.argv[2] || 'playground/svelte/src/lib/demo/macro-user.ts';
const absoluteTargetFile = path.isAbsolute(targetFile) ? targetFile : path.join(projectRoot, targetFile);

// Find tsconfig.json
function findTsConfig(startDir) {
  let dir = startDir;
  while (dir !== path.dirname(dir)) {
    const configPath = path.join(dir, 'tsconfig.json');
    if (fs.existsSync(configPath)) {
      return configPath;
    }
    dir = path.dirname(dir);
  }
  return null;
}

const tsConfigPath = findTsConfig(path.dirname(absoluteTargetFile));
if (!tsConfigPath) {
  console.error('Could not find tsconfig.json');
  process.exit(1);
}

console.log('Using tsconfig:', tsConfigPath);
console.log('Target file:', absoluteTargetFile);
console.log('');

// Read tsconfig
const configFile = ts.readConfigFile(tsConfigPath, ts.sys.readFile);
if (configFile.error) {
  console.error('Error reading tsconfig:', configFile.error.messageText);
  process.exit(1);
}

const parsedConfig = ts.parseJsonConfigFileContent(
  configFile.config,
  ts.sys,
  path.dirname(tsConfigPath)
);

// Create language service host
const files = new Map();
const fileVersions = new Map();

const servicesHost = {
  getScriptFileNames: () => [absoluteTargetFile, ...parsedConfig.fileNames],
  getScriptVersion: (fileName) => fileVersions.get(fileName) || '0',
  getScriptSnapshot: (fileName) => {
    if (!fs.existsSync(fileName)) {
      return undefined;
    }
    return ts.ScriptSnapshot.fromString(fs.readFileSync(fileName, 'utf8'));
  },
  getCurrentDirectory: () => path.dirname(tsConfigPath),
  getCompilationSettings: () => parsedConfig.options,
  getDefaultLibFileName: (options) => ts.getDefaultLibFilePath(options),
  fileExists: ts.sys.fileExists,
  readFile: ts.sys.readFile,
  readDirectory: ts.sys.readDirectory,
  directoryExists: ts.sys.directoryExists,
  getDirectories: ts.sys.getDirectories,
};

// Create language service
const languageService = ts.createLanguageService(servicesHost, ts.createDocumentRegistry());

// Check if file exists
if (!fs.existsSync(absoluteTargetFile)) {
  console.error('File not found:', absoluteTargetFile);
  process.exit(1);
}

// Get diagnostics
console.log('=== Semantic Diagnostics ===');
let semanticDiagnostics = [];
try {
  semanticDiagnostics = languageService.getSemanticDiagnostics(absoluteTargetFile);
} catch (e) {
  console.log('Error getting semantic diagnostics:', e.message);
}

if (semanticDiagnostics.length === 0) {
  console.log('No semantic errors found!');
} else {
  semanticDiagnostics.forEach((diag) => {
    const message = ts.flattenDiagnosticMessageText(diag.messageText, '\n');
    if (diag.file && diag.start !== undefined) {
      const { line, character } = diag.file.getLineAndCharacterOfPosition(diag.start);
      console.log(`${diag.file.fileName}(${line + 1},${character + 1}): error TS${diag.code}: ${message}`);
    } else {
      console.log(`error TS${diag.code}: ${message}`);
    }
  });
}

console.log('');
console.log('=== Syntactic Diagnostics ===');
let syntacticDiagnostics = [];
try {
  syntacticDiagnostics = languageService.getSyntacticDiagnostics(absoluteTargetFile);
} catch (e) {
  console.log('Error getting syntactic diagnostics:', e.message);
}

if (syntacticDiagnostics.length === 0) {
  console.log('No syntactic errors found!');
} else {
  syntacticDiagnostics.forEach((diag) => {
    const message = ts.flattenDiagnosticMessageText(diag.messageText, '\n');
    if (diag.file && diag.start !== undefined) {
      const { line, character } = diag.file.getLineAndCharacterOfPosition(diag.start);
      console.log(`${diag.file.fileName}(${line + 1},${character + 1}): error TS${diag.code}: ${message}`);
    } else {
      console.log(`error TS${diag.code}: ${message}`);
    }
  });
}

// Output summary
console.log('');
console.log('=== Summary ===');
console.log(`Semantic errors: ${semanticDiagnostics.length}`);
console.log(`Syntactic errors: ${syntacticDiagnostics.length}`);
