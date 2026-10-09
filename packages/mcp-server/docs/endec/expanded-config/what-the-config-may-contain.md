## What the config may contain

The expanded config is imported by generated code, so it runs wherever your code runs. The expansion
therefore refuses a config that:

- runs a statement at its top level, such as a bare function call: the top level may only declare;
- uses `import.meta` or a dynamic `import()`, which read differently once the module is moved into
  `.macroforge/config/`;
- declares a foreign type inside a function, where its handlers cannot be exported;
- gives two foreign types the same export name.

Relative imports in the config are rewritten to reach the same files from `.macroforge/config/`.
Calls that initialise a top-level binding or the default export are marked `/*#__PURE__*/`, so a
bundler can still drop what is unused.

## Packaged libraries

A published package does not ship `.macroforge/`. When `macroforge svelte-package` packages a
library, it copies the expanded config into the output as `__macroforge/config/` and rewrites each
emitted `#macroforge/config` import to a relative path to that copy, so the package needs no
manifest entry of its own.
