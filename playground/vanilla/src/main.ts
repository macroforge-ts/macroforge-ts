// Import macro functions (these will be transformed by our Rust plugin)
import { Derive } from "./macros";
import "./test-intellisense"; // Trigger type generation for Product class
import "./demo-intellisense"; // Trigger type generation for Customer class

// Example of using IncludeStr macro (will be replaced with file content at compile time)
// const readme = IncludeStr('../README.md')

// Example of using Derive decorator
@Derive("Debug", "JSON")
class User {
  constructor(
    public id: number,
    public name: string,
    public email: string,
  ) {}
}

// Test the generated methods
function testMacros() {
  const user = new User(1, "John Doe", "john@example.com");
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
        <li><code>IncludeStr</code> - Include file contents at compile time</li>
        <li><code>@Derive</code> - Auto-generate methods like toString() and toJSON()</li>
      </ul>

      <h2>Example User Object:</h2>
      <pre>${JSON.stringify(user, null, 2)}</pre>

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
