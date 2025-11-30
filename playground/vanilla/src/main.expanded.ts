// Import macro functions (these will be transformed by our Rust plugin)
import "./test-intellisense"; // Trigger type generation for Product class
import "./demo-intellisense"; // Trigger type generation for Customer class
import { User } from "./user";

// Test the generated methods
function testMacros() {
  const user = new User(1, "John Doe", "john@example.com", "tok_live_secret");
  const derivedSummary = user.toString();
  const derivedJson = user.toJSON();

  // These methods will be injected by the @Derive macro
  console.log("Derived summary:", derivedSummary);
  console.log("Derived JSON:", derivedJson);

  const app = document.getElementById("app");
  if (app) {
    app.innerHTML = `
      <h1>TS Macros Playground</h1>
      <p>This playground demonstrates Rust-powered macros for TypeScript.</p>

      <h2>Features:</h2>
      <ul>
        <li><code>@Derive</code> - Auto-generate methods like toString() and toJSON()</li>
        <li><code>@Debug(...)</code> - Per-field rename / skip controls inside derives</li>
      </ul>

      <h2>Derived Summary (Debug)</h2>
      <pre>${derivedSummary}</pre>

      <h2>Derived JSON (JsonNative)</h2>
      <pre>${JSON.stringify(derivedJson, null, 2)}</pre>

      <p>
        Notice how the summary uses <code>identifier</code> instead of <code>id</code>, while the
        <code>authToken</code> field is skipped entirely in <code>toString()</code> but still present in the JSON payload.
      </p>

      <p>Check the console for more examples!</p>
    `;
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
