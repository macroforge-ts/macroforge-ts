# TypeScript Plugin
 *The TypeScript plugin provides IDE integration for Macroforge, including error reporting, completions, and type checking for generated code.*
 ## Installation
 ```
npm install -D @macroforge/typescript-plugin
``` ## Configuration
 Add the plugin to your `tsconfig.json`:
 ```
{
  "compilerOptions": {
    "plugins": [
      {
        "name": "@macroforge/typescript-plugin"
      }
    ]
  }
}
``` ## VS Code Setup
 VS Code uses its own TypeScript version by default. To use the workspace version (which includes plugins):
 1. Open the Command Palette (`Cmd/Ctrl + Shift + P`)
 2. Search for "TypeScript: Select TypeScript Version"
 3. Choose "Use Workspace Version"
  **Tip Add this setting to your `.vscode/settings.json` to make it permanent: ```
{
  "typescript.tsdk": "node_modules/typescript/lib"
}
``` ## Features
 ### Error Reporting
 Errors in macro-generated code are reported at the `@derive` decorator position:
 ```
/** @derive(Debug) */  // **<- Errors appear here
class User {
  name: string;
}
``` ### Completions
 The plugin provides completions for generated methods:
 ```
const user = new User("Alice");
user.to  // Suggests: toString(), toJSON(), etc.
``` ### Type Information
 Hover over generated methods to see their types:
 ```
// Hover over 'clone' shows:
// (method) User.clone(): User
const copy = user.clone();
``` ## Troubleshooting
 ### Plugin Not Loading
 1. Ensure you're using the workspace TypeScript version
 2. Restart the TypeScript server (Command Palette → "TypeScript: Restart TS Server")
 3. Check that the plugin is listed in `tsconfig.json`
 ### Errors Not Showing
 If errors from macros aren't appearing:
 1. Make sure the Vite plugin is also installed (for source file watching)
 2. Check that your file is saved (plugins process on save)