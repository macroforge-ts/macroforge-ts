## Setup

`#macroforge/config` is a [subpath import](https://nodejs.org/api/packages.html#subpath-imports),
declared in your manifest the way SvelteKit declares `#lib`. Add it once:

```bash
macroforge init
```

which writes this into `package.json` (or the `imports` map of `deno.json` when there is no
`package.json`):

```json
{
    "imports": {
        "#macroforge/config": {
            "types": "./.macroforge/config/handlers.ts",
            "default": "./.macroforge/config/handlers.js"
        }
    }
}
```

TypeScript, bundlers, Node and Deno all resolve it natively. A project whose config declares foreign
types but whose manifest does not map `#macroforge/config` fails with an error that says what to
add. `macroforge init` does not rewrite a `deno.jsonc`, since that would drop its comments; add the
entry by hand there.
