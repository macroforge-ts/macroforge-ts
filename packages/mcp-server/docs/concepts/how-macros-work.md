# How Macros Work
 *Macroforge performs compile-time code generation by parsing your TypeScript, expanding macros, and outputting transformed code. This happens before your code runs, resulting in zero runtime overhead.*
 ## Compile-Time Expansion
 Unlike runtime solutions that use reflection or proxies, Macroforge expands macros at compile time:
 1. **Parse**: Your TypeScript code is parsed into an AST using SWC
 2. **Find**: Macroforge finds <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorators and their associated items
 3. **Expand**: Each macro generates new code based on the class structure
 4. **Output**: The transformed TypeScript is written out, ready for normal compilation
 <div><div class="flex items-center justify-between gap-2 px-4 py-2 bg-muted rounded-t-lg border border-b-0 border-border"> 
**Before:**
```
/** @derive(Debug) */
class User &#123;
    name: string;
&#125;
``` <div class="flex items-center justify-between gap-2 px-4 py-2 bg-muted rounded-t-lg border border-b-0 border-border"> 
**After:**
```
class User &#123;
    name: string;

    static toString(value: User): string &#123;
        return userToString(value);
    &#125;
&#125;

export function userToString(value: User): string &#123;
    const parts: string[] = [];
    parts.push('name: ' + value.name);
    return 'User &#123; ' + parts.join(', ') + ' &#125;';
&#125;
``` ## Zero Runtime Overhead
 Because code generation happens at compile time, there's no:
 - Runtime reflection or metadata
 - Proxy objects or wrappers
 - Additional dependencies in your bundle
 - Performance cost at runtime
 The generated code is plain TypeScript that compiles to efficient JavaScript.
 ## Source Mapping
 Macroforge tracks the relationship between your source code and the expanded output. This means:
 - Errors in generated code point back to your source
 - Debugging works correctly
 - IDE features like "go to definition" work as expected
 > with @derive decorators  <div class="font-semibold text-foreground">SWC Parser TypeScript → AST  <div class="font-semibold text-foreground">Macro Expansion Engine Finds @derive decorators, runs macros, generates new AST nodes  <div class="font-semibold text-foreground">Code Generator AST → TypeScript  <div class="font-semibold text-foreground">Expanded TypeScript ready for normal compilation  ## Integration Points
 Macroforge integrates at two key points:
 ### IDE (TypeScript Plugin)
 The TypeScript plugin intercepts language server calls to provide:
 - Diagnostics that reference your source, not expanded code
 - Completions for generated methods
 - Hover information showing what macros generate
 ### Build (Vite Plugin)
 The Vite plugin runs macro expansion during the build process:
 - Transforms files before they reach the TypeScript compiler
 - Generates type declaration files (.d.ts)
 - Produces metadata for debugging
 ## Next Steps
 - [Learn about the derive system](../docs/concepts/derive-system)
 - [Explore the architecture](../docs/concepts/architecture)