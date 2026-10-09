## When it is regenerated

`.macroforge/config/` is generated, like `.svelte-kit/`: don't edit it and don't commit it. It is
rewritten whenever macroforge reads the project: the Vite plugin's project scan, `macroforge watch`
(again on every config change), `macroforge cache` and `macroforge expand`. In CI, run
`macroforge sync` before a type check to write it without expanding anything. It is written only
when its contents change, and swapped in whole, so a reader never sees half of it.

Both `handlers.ts` and a type-stripped `handlers.js` are written, so the entry resolves in tools
that read types and in runtimes that don't. A config that extends a base config gets the base
expanded beside it under `modules/`, with its handlers re-exported from `handlers.ts`.
