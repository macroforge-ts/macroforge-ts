import { assertEquals, assertExists } from 'jsr:@std/assert@1';
import { CONFIG_FILES, findConfigFile, loadMacroConfig } from '../src/config.ts';

Deno.test('CONFIG_FILES - contains expected config file names', () => {
    assertEquals(CONFIG_FILES.length, 5);
    assertEquals(CONFIG_FILES[0], 'macroforge.config.ts');
    assertEquals(CONFIG_FILES[1], 'macroforge.config.mts');
    assertEquals(CONFIG_FILES[2], 'macroforge.config.js');
    assertEquals(CONFIG_FILES[3], 'macroforge.config.mjs');
    assertEquals(CONFIG_FILES[4], 'macroforge.config.cjs');
});

Deno.test('CONFIG_FILES - TypeScript config takes precedence', () => {
    // Verify .ts is checked before .js
    const tsIndex = CONFIG_FILES.indexOf('macroforge.config.ts');
    const jsIndex = CONFIG_FILES.indexOf('macroforge.config.js');

    assertEquals(tsIndex < jsIndex, true);
});

Deno.test('findConfigFile - returns null for non-existent directory', () => {
    const result = findConfigFile('/non/existent/path/that/does/not/exist');
    assertEquals(result, null);
});

Deno.test('findConfigFile - returns null when no config exists', async () => {
    // Create a temp directory without any config
    const tempDir = await Deno.makeTempDir();
    try {
        // Create a package.json to act as boundary
        await Deno.writeTextFile(`${tempDir}/package.json`, '{}');

        const result = findConfigFile(tempDir);
        assertEquals(result, null);
    } finally {
        await Deno.remove(tempDir, { recursive: true });
    }
});

Deno.test('findConfigFile - finds config file in directory', async () => {
    const tempDir = await Deno.makeTempDir();
    try {
        // Create a package.json boundary and config file
        await Deno.writeTextFile(`${tempDir}/package.json`, '{}');
        await Deno.writeTextFile(
            `${tempDir}/macroforge.config.js`,
            'export default { keepDecorators: true };'
        );

        const result = findConfigFile(tempDir);
        assertExists(result);
        assertEquals(result.endsWith('macroforge.config.js'), true);
    } finally {
        await Deno.remove(tempDir, { recursive: true });
    }
});

Deno.test('findConfigFile - prefers .ts over .js', async () => {
    const tempDir = await Deno.makeTempDir();
    try {
        await Deno.writeTextFile(`${tempDir}/package.json`, '{}');
        await Deno.writeTextFile(`${tempDir}/macroforge.config.js`, 'export default {};');
        await Deno.writeTextFile(`${tempDir}/macroforge.config.ts`, 'export default {};');

        const result = findConfigFile(tempDir);
        assertExists(result);
        assertEquals(result.endsWith('macroforge.config.ts'), true);
    } finally {
        await Deno.remove(tempDir, { recursive: true });
    }
});

Deno.test('loadMacroConfig - returns defaults when no config found', async () => {
    const tempDir = await Deno.makeTempDir();
    try {
        await Deno.writeTextFile(`${tempDir}/package.json`, '{}');

        const config = loadMacroConfig(tempDir);
        assertEquals(config.keepDecorators, false);
        assertEquals(config.configPath, undefined);
    } finally {
        await Deno.remove(tempDir, { recursive: true });
    }
});

Deno.test('loadMacroConfig - sets configPath when config found', async () => {
    const tempDir = await Deno.makeTempDir();
    try {
        await Deno.writeTextFile(`${tempDir}/package.json`, '{}');
        await Deno.writeTextFile(`${tempDir}/macroforge.config.js`, 'export default {};');

        const config = loadMacroConfig(tempDir);
        assertEquals(config.keepDecorators, false);
        assertExists(config.configPath);
        assertEquals(config.configPath!.endsWith('macroforge.config.js'), true);
    } finally {
        await Deno.remove(tempDir, { recursive: true });
    }
});

Deno.test('loadMacroConfig - uses custom loader when provided', async () => {
    const tempDir = await Deno.makeTempDir();
    try {
        await Deno.writeTextFile(`${tempDir}/package.json`, '{}');
        await Deno.writeTextFile(`${tempDir}/macroforge.config.js`, 'keepDecorators: true');

        const customLoader = (_content: string, _filepath: string) => ({
            keepDecorators: true,
            generateConvenienceConst: true,
            hasForeignTypes: false,
            foreignTypeCount: 0,
            returnTypes: 'vanilla'
        });

        const config = loadMacroConfig(tempDir, customLoader);
        assertEquals(config.keepDecorators, true);
        assertEquals(config.generateConvenienceConst, true);
    } finally {
        await Deno.remove(tempDir, { recursive: true });
    }
});
