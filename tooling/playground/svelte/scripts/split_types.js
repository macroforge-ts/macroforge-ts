import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const SOURCE_FILE = path.join(__dirname, '../src/lib/demo/types/types.svelte.ts');
const OUTPUT_DIR = path.dirname(SOURCE_FILE);

// Read source file
const source = fs.readFileSync(SOURCE_FILE, 'utf-8');

// 1. Identify all defined types to help with import generation
const typeNames = new Set();
const typeRegex = /export\s+(?:interface|type)\s+(\w+)/g;
let match;
while ((match = typeRegex.exec(source)) !== null) {
    typeNames.add(match[1]);
}

console.log(`Found ${typeNames.size} types.`);

// 2. Split into blocks
// We look for the start of a JSDoc comment or an export statement
const lines = source.split('\n');
const blocks = [];
let currentBlockLines = [];
let currentTypeName = null;

// Helper to check if a line is the start of a new declaration
function isStartOfDeclaration(line) {
    return line.trim().startsWith('/** @derive') || 
           (line.trim().startsWith('export ') && !currentBlockLines.some(l => l.trim().startsWith('/** @derive')));
}

// Global imports/macros that need to be in every file or handled specially
const globalImports = [
    '/** import macro {Gigaform} from "@playground/macro"; */'
];

for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    
    // Check if this line marks the start of a new type definition
    // We assume mostly standard formatting where @derive starts the block
    if (isStartOfDeclaration(line) && currentBlockLines.length > 0 && currentTypeName) {
        // Save previous block
        blocks.push({
            name: currentTypeName,
            content: currentBlockLines.join('\n')
        });
        currentBlockLines = [];
        currentTypeName = null;
    }

    if (line.trim().startsWith('/** import macro')) {
        // Skip, we handle this globally
        continue;
    }

    // Try to capture the type name if we find the export line
    const typeMatch = line.match(/export\s+(?:interface|type)\s+(\w+)/);
    if (typeMatch) {
        currentTypeName = typeMatch[1];
    }

    if (line.trim() !== '') {
        currentBlockLines.push(line);
    } else if (currentBlockLines.length > 0) {
        // Keep empty lines inside blocks, but maybe trim leading/trailing later
        currentBlockLines.push(line);
    }
}

// Push last block
if (currentBlockLines.length > 0 && currentTypeName) {
    blocks.push({
        name: currentTypeName,
        content: currentBlockLines.join('\n')
    });
}

// Helper to convert PascalCase to kebab-case
function toKebabCase(str) {
    return str.replace(/([a-z0-9])([A-Z])/g, '$1-$2').toLowerCase();
}

// 3. Process and write files
blocks.forEach(block => {
    const filename = `${toKebabCase(block.name)}.svelte.ts`;
    const filepath = path.join(OUTPUT_DIR, filename);
    
    let fileContent = block.content.trim() + '\n';
    
    // Detect dependencies
    const imports = new Set();
    
    typeNames.forEach(type => {
        if (type === block.name) return; // Don't import self
        
        // Use word boundary to ensure we don't match substrings (e.g. "User" in "UserRole")
        const regex = new RegExp('\\b' + type + '\\b');
        if (regex.test(fileContent)) {
            imports.add(`import { ${type} } from './${toKebabCase(type)}.svelte';`);
        }
    });

    // Construct final content
    let finalContent = '';
    
    // Add macro import if Gigaform is used (which is in the @derive)
    if (fileContent.includes('Gigaform')) {
        finalContent += globalImports[0] + '\n\n';
    }

    if (imports.size > 0) {
        finalContent += Array.from(imports).join('\n') + '\n\n';
    }

    finalContent += fileContent;

    fs.writeFileSync(filepath, finalContent);
    console.log(`Wrote ${filename}`);
});

console.log('Done splitting types.');