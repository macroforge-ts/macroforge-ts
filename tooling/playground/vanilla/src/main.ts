import { AllMacrosTestClass, testInstance } from './all-macros-test.ts';
import { User } from './user.ts';
import { runRunesTests } from './runes-test.ts';
import { runE2eHarness } from './e2e-harness.ts';
import { collectBuildtimeDemo } from './buildtime-demo.ts';
import * as attrs from './attributes-test.ts';
import { requireElement } from './dom.util.ts';
import {
    type AttributesResults,
    type MacroTestResults,
    playgroundResults
} from './playground-globals.ts';

/**
 * Calls an attributes-fixture export by name, or `null` when `@cfg` stripped
 * it. The stripped exports vanish at runtime, which is the evidence that the
 * pre-pass ran, so they are looked up dynamically rather than imported.
 */
function probeExport(name: string): string | null {
    const value: unknown = Reflect.get(attrs, name);
    return typeof value === 'function' ? String(value()) : null;
}

// `@deprecated` keeps the function and rewrites only its JSDoc, which
// `tests/expansion.test.mjs` covers. `@nonExhaustive` is type-only; the
// runtime value is just a string.
function collectAttributesResults(): AttributesResults {
    return {
        keptByFeature: probeExport('keptByPlayground'),
        strippedByFeature: probeExport('strippedByMissingFeature'),
        keptByTarget: probeExport('keptByWebTarget'),
        strippedByTarget: probeExport('strippedByNodeTarget'),
        deprecatedCall: attrs.renderV1(),
        nonExhaustiveValue: attrs.exampleStatus
    };
}

const playground = playgroundResults();
const macroTestResults: MacroTestResults = {};
playground.macroTests = macroTestResults;

function renderResult(id: string, html: string) {
    requireElement(id).innerHTML = html;
}

function runAllMacroTests() {
    // Test Debug macro -> static toString()
    const debugResult = AllMacrosTestClass.toString(testInstance);
    macroTestResults.debug = debugResult;
    renderResult('result-debug', `<strong>Debug (toString):</strong> <code>${debugResult}</code>`);

    // Test Clone macro -> static clone()
    if (typeof AllMacrosTestClass.clone === 'function') {
        const cloned = AllMacrosTestClass.clone(testInstance);
        macroTestResults.clone = cloned;
        renderResult(
            'result-clone',
            `<strong>Clone:</strong> <pre>${JSON.stringify(cloned, null, 2)}</pre>`
        );
    } else {
        renderResult('result-clone', `<strong>Clone:</strong> <em>Not available</em>`);
    }

    // Test PartialEq macro -> static equals()
    if (typeof AllMacrosTestClass.equals === 'function') {
        const equalsSelf = AllMacrosTestClass.equals(testInstance, testInstance);
        macroTestResults.equals = equalsSelf;
        renderResult('result-equals', `<strong>Equals (self):</strong> <code>${equalsSelf}</code>`);
    } else {
        renderResult('result-equals', `<strong>Equals:</strong> <em>Not available</em>`);
    }

    // Test Hash macro -> static hashCode()
    if (typeof AllMacrosTestClass.hashCode === 'function') {
        const hashCode = AllMacrosTestClass.hashCode(testInstance);
        macroTestResults.hashCode = hashCode;
        renderResult('result-hashcode', `<strong>HashCode:</strong> <code>${hashCode}</code>`);
    } else {
        renderResult('result-hashcode', `<strong>HashCode:</strong> <em>Not available</em>`);
    }

    // Test Serialize macro -> static serialize()
    const serialized = AllMacrosTestClass.serialize(testInstance);
    macroTestResults.serialize = serialized;
    renderResult('result-serialize', `<strong>Serialize:</strong> <pre>${serialized}</pre>`);

    // Test Deserialize macro -> deserialize()
    if (typeof AllMacrosTestClass.deserialize === 'function') {
        const testData = {
            id: 99,
            name: 'Deserialized User',
            email: 'deser@test.com',
            secretToken: 'token',
            isActive: false,
            score: 50
        };
        // deserialize returns a vanilla result { success: boolean, value/errors }
        const result = AllMacrosTestClass.deserialize(testData);
        if (result.success) {
            macroTestResults.deserialize = result.value;
            renderResult(
                'result-deserialize',
                `<strong>Deserialize:</strong> <pre>${JSON.stringify(result.value, null, 2)}</pre>`
            );
        } else {
            renderResult(
                'result-deserialize',
                `<strong>Deserialize Error:</strong> <pre>${
                    JSON.stringify(result.errors, null, 2)
                }</pre>`
            );
        }
    } else {
        renderResult('result-deserialize', `<strong>Deserialize:</strong> <em>Not available</em>`);
    }

    // Mark tests as complete
    document.getElementById('test-results')?.setAttribute(
        'data-tests-complete',
        'true'
    );
}

function testMacros() {
    const user = new User(1, 'John Doe', 'john@example.com', 'tok_live_secret');
    const derivedSummary = User.toString(user);
    const derivedJson = user.toJSON();

    // Pull spliced-at-build-time values out of buildtime-demo.ts. If the
    // Vite plugin's macroforge pre-pass didn't run, this will throw
    // because the `@macroforge/core/buildtime` runtime stubs fire.
    const buildtimeResult = collectBuildtimeDemo();
    playground.buildtime = buildtimeResult;

    // Probe the attribute-pre-pass fixture. Stripped exports show up as
    // `null`; kept ones return their value.
    const attributeResults = collectAttributesResults();
    playground.attributes = attributeResults;

    const app = document.getElementById('app');
    if (app) {
        app.innerHTML = `
      <h1>TS Macros Playground</h1>
      <p>This playground demonstrates Rust-powered macros for TypeScript.</p>

      <h2>Macro Test Panel</h2>
      <div id="test-controls">
        <button id="btn-test-all" data-testid="test-all-macros">
          Run All Macro Tests
        </button>
      </div>

      <div id="test-results" data-testid="test-results">
        <h3>Test Results</h3>
        <div id="result-debug" data-testid="result-debug"><em>Click button to run tests</em></div>
        <div id="result-clone" data-testid="result-clone"></div>
        <div id="result-equals" data-testid="result-equals"></div>
        <div id="result-hashcode" data-testid="result-hashcode"></div>
        <div id="result-serialize" data-testid="result-serialize"></div>
        <div id="result-deserialize" data-testid="result-deserialize"></div>
      </div>

      <h2>Features:</h2>
      <ul>
        <li><code>@derive</code> - Auto-generate methods like toString() and toJSON()</li>
        <li><code>@Debug(...)</code> - Per-field rename / skip controls inside derives</li>
      </ul>

      <h2>Derived Summary (Debug)</h2>
      <pre data-testid="user-summary">${derivedSummary}</pre>

      <h2>Derived JSON (JsonNative)</h2>
      <pre data-testid="user-json">${JSON.stringify(derivedJson, null, 2)}</pre>

      <p>
        Notice how the summary uses <code>identifier</code> instead of <code>id</code>, while the
        <code>authToken</code> field is skipped entirely in <code>toString()</code> but still present in the JSON payload.
      </p>

      <h2>@buildtime evaluation</h2>
      <p>
        Every value below was computed at compile time by macroforge and
        spliced into the module as a TS literal. The runtime stub
        imported from <code>@macroforge/core/buildtime</code> still throws if
        called — proving the plugin did the work, not the browser.
      </p>
      <div id="buildtime-results" data-testid="buildtime-results">
        <div><strong>answer (6 * 7):</strong> <code data-testid="bt-answer">${buildtimeResult.answer}</code></div>
        <div><strong>schema sha256:</strong> <code data-testid="bt-hash">${buildtimeResult.schemaHash}</code></div>
        <div><strong>app name (from JSON):</strong> <code data-testid="bt-app-name">${buildtimeResult.appName}</code></div>
        <div><strong>app version (from JSON):</strong> <code data-testid="bt-app-version">${buildtimeResult.appVersion}</code></div>
        <div><strong>routes (from JSON):</strong> <code data-testid="bt-routes">${
            buildtimeResult.routes.join(',')
        }</code></div>
        <div><strong>derived summary:</strong> <code data-testid="bt-summary">${buildtimeResult.derivedSummary}</code></div>
        <div><strong>greeting for alice:</strong> <code data-testid="bt-greet-alice">${buildtimeResult.greetingAlice}</code></div>
        <div><strong>greeting keys:</strong> <code data-testid="bt-greet-keys">${
            buildtimeResult.greetingKeys.join(',')
        }</code></div>
        <div><strong>constant object thirteen:</strong> <code data-testid="bt-const-thirteen">${buildtimeResult.constantObject.thirteen}</code></div>
        <div><strong>runtime stub throws:</strong> <code data-testid="bt-stub-throws">${
            String(buildtimeResult.runtimeStubThrows)
        }</code></div>
      </div>

      <h2>Attribute macros</h2>
      <p>
        Each row reflects an annotation on a declaration in
        <code>attributes-test.ts</code>. Stripped exports show as
        <code>(stripped)</code>; kept ones show the function's return value.
      </p>
      <div id="attributes-results" data-testid="attributes-results">
        <div><strong>@cfg feature kept:</strong> <code data-testid="attr-kept-feature">${
            attributeResults.keptByFeature ?? '(stripped)'
        }</code></div>
        <div><strong>@cfg feature stripped:</strong> <code data-testid="attr-stripped-feature">${
            attributeResults.strippedByFeature ?? '(stripped)'
        }</code></div>
        <div><strong>@cfg target kept:</strong> <code data-testid="attr-kept-target">${
            attributeResults.keptByTarget ?? '(stripped)'
        }</code></div>
        <div><strong>@cfg target stripped:</strong> <code data-testid="attr-stripped-target">${
            attributeResults.strippedByTarget ?? '(stripped)'
        }</code></div>
        <div><strong>@deprecated call result:</strong> <code data-testid="attr-deprecated-call">${attributeResults.deprecatedCall}</code></div>
        <div><strong>@nonExhaustive value:</strong> <code data-testid="attr-non-exhaustive">${attributeResults.nonExhaustiveValue}</code></div>
      </div>

      <p>Check the console for more examples!</p>
    `;

        // Attach test button handler
        document.getElementById('btn-test-all')?.addEventListener(
            'click',
            runAllMacroTests
        );
    }

    // Run e2e harness for all macro types and expose on globalThis
    try {
        playground.e2e = runE2eHarness();
        console.log('E2e harness collected successfully');
    } catch (error) {
        console.error('E2e harness failed:', error);
    }

    // Run runes reactivity tests automatically and expose on globalThis
    try {
        const runes = runRunesTests();
        playground.runes = runes;
        console.log(`Runes tests: ${runes.passed} passed, ${runes.failed} failed`);
        for (const detail of runes.details) console.log(detail);
    } catch (error) {
        console.error('Runes tests failed:', error);
        playground.runes = {
            passed: 0,
            failed: 1,
            details: [`ERROR: ${error}`]
        };
    }

    console.log('User object:', user);
    console.log('Macros playground loaded successfully!');
}

// Run tests when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', testMacros);
} else {
    testMacros();
}
