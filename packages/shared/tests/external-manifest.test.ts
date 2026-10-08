import { assertEquals } from 'jsr:@std/assert@1';
import {
    clearExternalManifestCache,
    getExternalDecoratorInfo,
    getExternalMacroInfo,
    getExternalManifest
} from '../src/external-manifest.ts';

// A manifest shaped the way the engine's `MacroManifest` encodes.
const mockManifest = {
    version: 1,
    macros: [
        {
            name: 'JSON',
            kind: 'derive',
            description: 'JSON encoding macro',
            package: 'test-package'
        },
        {
            name: 'Debug',
            kind: 'derive',
            description: 'Debug implementation macro',
            package: 'test-package'
        }
    ],
    decorators: [
        {
            module: 'hiddenController',
            export: 'hiddenController',
            kind: 'property',
            docs: 'Hidden field'
        },
        {
            module: 'fieldController',
            export: 'fieldController',
            kind: 'property',
            docs: 'Field controller'
        }
    ]
};

// Mock require function that returns a package with manifest
function createMockRequire(manifest: unknown) {
    return (_id: string) => ({
        __macroforgeGetManifest: () => manifest
    });
}

// Mock require that throws (package not found)
function failingRequire(_id: string): unknown {
    throw new Error('Module not found');
}

// Mock require that returns package without manifest
function noManifestRequire(_id: string): unknown {
    return { someOtherExport: true };
}

Deno.test('clearExternalManifestCache - clears the cache', () => {
    // First, populate cache
    const mockRequire = createMockRequire(mockManifest);
    getExternalManifest('test-package', mockRequire);

    // Clear and verify by using a failing require (would use cache if present)
    clearExternalManifestCache();

    // Now it should try to require again (and fail)
    const result = getExternalManifest('test-package', failingRequire);
    assertEquals(result, null);
});

Deno.test('getExternalManifest - returns manifest from package', () => {
    clearExternalManifestCache();
    const mockRequire = createMockRequire(mockManifest);

    const result = getExternalManifest('test-package', mockRequire);

    assertEquals(result?.macros.length, 2);
    assertEquals(result?.decorators.length, 2);
});

Deno.test('getExternalManifest - caches result', () => {
    clearExternalManifestCache();

    let callCount = 0;
    const countingRequire = (_id: string) => {
        callCount++;
        return { __macroforgeGetManifest: () => mockManifest };
    };

    // Call twice
    getExternalManifest('cached-package', countingRequire);
    getExternalManifest('cached-package', countingRequire);

    // Should only have called require once
    assertEquals(callCount, 1);
});

Deno.test('getExternalManifest - rejects a malformed manifest', () => {
    clearExternalManifestCache();
    const malformed = createMockRequire({ macros: [{ name: 'JSON' }], decorators: [] });

    assertEquals(getExternalManifest('malformed-package', malformed), null);
});

Deno.test('getExternalManifest - returns null for missing package', () => {
    clearExternalManifestCache();

    const result = getExternalManifest('missing-package', failingRequire);
    assertEquals(result, null);
});

Deno.test('getExternalManifest - returns null for package without manifest', () => {
    clearExternalManifestCache();

    const result = getExternalManifest('no-manifest-package', noManifestRequire);
    assertEquals(result, null);
});

Deno.test('getExternalManifest - caches null result', () => {
    clearExternalManifestCache();

    let callCount = 0;
    const countingFailingRequire = (_id: string) => {
        callCount++;
        throw new Error('Not found');
    };

    // Call twice
    getExternalManifest('failing-package', countingFailingRequire);
    getExternalManifest('failing-package', countingFailingRequire);

    // Should only have tried once, then cached null
    assertEquals(callCount, 1);
});

Deno.test('getExternalMacroInfo - finds macro by name', () => {
    clearExternalManifestCache();
    const mockRequire = createMockRequire(mockManifest);

    const result = getExternalMacroInfo('JSON', 'test-package', mockRequire);

    assertEquals(result?.name, 'JSON');
    assertEquals(result?.description, 'JSON encoding macro');
});

Deno.test('getExternalMacroInfo - case insensitive lookup', () => {
    clearExternalManifestCache();
    const mockRequire = createMockRequire(mockManifest);

    const result = getExternalMacroInfo('json', 'test-package', mockRequire);

    assertEquals(result?.name, 'JSON');
});

Deno.test('getExternalMacroInfo - returns null for unknown macro', () => {
    clearExternalManifestCache();
    const mockRequire = createMockRequire(mockManifest);

    const result = getExternalMacroInfo('Unknown', 'test-package', mockRequire);
    assertEquals(result, null);
});

Deno.test('getExternalMacroInfo - returns null for missing package', () => {
    clearExternalManifestCache();

    const result = getExternalMacroInfo(
        'JSON',
        'missing-package',
        failingRequire
    );
    assertEquals(result, null);
});

Deno.test('getExternalDecoratorInfo - finds decorator by name', () => {
    clearExternalManifestCache();
    const mockRequire = createMockRequire(mockManifest);

    const result = getExternalDecoratorInfo(
        'hiddenController',
        'test-package',
        mockRequire
    );

    assertEquals(result?.export, 'hiddenController');
    assertEquals(result?.module, 'hiddenController');
});

Deno.test('getExternalDecoratorInfo - case insensitive lookup', () => {
    clearExternalManifestCache();
    const mockRequire = createMockRequire(mockManifest);

    const result = getExternalDecoratorInfo(
        'HIDDENCONTROLLER',
        'test-package',
        mockRequire
    );

    assertEquals(result?.export, 'hiddenController');
});

Deno.test('getExternalDecoratorInfo - returns null for unknown decorator', () => {
    clearExternalManifestCache();
    const mockRequire = createMockRequire(mockManifest);

    const result = getExternalDecoratorInfo(
        'unknownDecorator',
        'test-package',
        mockRequire
    );
    assertEquals(result, null);
});

Deno.test('getExternalDecoratorInfo - returns null for missing package', () => {
    clearExternalManifestCache();

    const result = getExternalDecoratorInfo(
        'hiddenController',
        'missing-package',
        failingRequire
    );
    assertEquals(result, null);
});
