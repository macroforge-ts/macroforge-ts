import { User } from "./user";
import { AllMacrosTestClass, testInstance } from "./all-macros-test";

// Global results object for Playwright assertions
declare global {
  interface Window {
    macroTestResults: {
      debug?: string;
      clone?: object;
      equals?: boolean;
      hashCode?: number;
      serialize?: object;
      deserialize?: object;
    };
  }
}

window.macroTestResults = {};

function runAllMacroTests() {
  const results = window.macroTestResults;

  // Test Debug macro -> toString()
  const debugResult = testInstance.toString();
  results.debug = debugResult;
  document.getElementById("result-debug")!.innerHTML =
    `<strong>Debug (toString):</strong> <code>${debugResult}</code>`;

  // Test Clone macro -> clone()
  if (typeof (testInstance as any).clone === "function") {
    const cloned = (testInstance as any).clone();
    results.clone = cloned;
    document.getElementById("result-clone")!.innerHTML =
      `<strong>Clone:</strong> <pre>${JSON.stringify(cloned, null, 2)}</pre>`;
  } else {
    document.getElementById("result-clone")!.innerHTML =
      `<strong>Clone:</strong> <em>Not available</em>`;
  }

  // Test Eq macro -> equals()
  if (typeof (testInstance as any).equals === "function") {
    const equalsSelf = (testInstance as any).equals(testInstance);
    results.equals = equalsSelf;
    document.getElementById("result-equals")!.innerHTML =
      `<strong>Equals (self):</strong> <code>${equalsSelf}</code>`;
  } else {
    document.getElementById("result-equals")!.innerHTML =
      `<strong>Equals:</strong> <em>Not available</em>`;
  }

  // Test Eq macro -> hashCode()
  if (typeof (testInstance as any).hashCode === "function") {
    const hashCode = (testInstance as any).hashCode();
    results.hashCode = hashCode;
    document.getElementById("result-hashcode")!.innerHTML =
      `<strong>HashCode:</strong> <code>${hashCode}</code>`;
  } else {
    document.getElementById("result-hashcode")!.innerHTML =
      `<strong>HashCode:</strong> <em>Not available</em>`;
  }

  // Test Serialize macro -> toJSON()
  const serialized = testInstance.toJSON();
  results.serialize = serialized;
  document.getElementById("result-serialize")!.innerHTML =
    `<strong>Serialize (toJSON):</strong> <pre>${JSON.stringify(serialized, null, 2)}</pre>`;

  // Test Deserialize macro -> fromJSON()
  if (typeof (AllMacrosTestClass as any).fromJSON === "function") {
    const testData = {
      id: 99,
      name: "Deserialized User",
      email: "deser@test.com",
      secretToken: "token",
      isActive: false,
      score: 50,
    };
    const deserialized = (AllMacrosTestClass as any).fromJSON(testData);
    results.deserialize = deserialized;
    document.getElementById("result-deserialize")!.innerHTML =
      `<strong>Deserialize (fromJSON):</strong> <pre>${JSON.stringify(deserialized, null, 2)}</pre>`;
  } else {
    document.getElementById("result-deserialize")!.innerHTML =
      `<strong>Deserialize:</strong> <em>Not available</em>`;
  }

  // Mark tests as complete
  document.getElementById("test-results")?.setAttribute("data-tests-complete", "true");
}

function testMacros() {
  const user = new User(1, "John Doe", "john@example.com", "tok_live_secret");
  const derivedSummary = user.toString();
  const derivedJson = user.toJSON();

  const app = document.getElementById("app");
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

      <p>Check the console for more examples!</p>
    `;

    // Attach test button handler
    document.getElementById("btn-test-all")?.addEventListener("click", runAllMacroTests);
  }

  console.log("User object:", user);
  console.log("Macros playground loaded successfully!");
}

// Run tests when DOM is ready
if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", testMacros);
} else {
  testMacros();
}

export {};
