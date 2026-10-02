## Sharing a Base Config

The config is read statically, never run, and the reader follows what a config inherits. Name a base
with `extends`, a path or an array of them with later ones winning:

apps/web/macroforge.config.ts

```
export default {
  extends: "../../macroforge.base.ts",
  foreignTypes: {
    "Option.Option": { from: ["effect"] },
  },
};
```

Fields the config sets replace the base's, except `foreignTypes`, which merge by type name: an entry
the config names again replaces the base's, and the rest are added.

A config can also export, spread or wrap a config it imports, with JavaScript's spread semantics, so
a field set after a spread replaces the spread one:

macroforge.config.ts

```
import base from "@acme/macroforge-config";

export default defineConfig({
  ...base,
  keepDecorators: true,
  foreignTypes: { ...base.foreignTypes, "Option.Option": { from: ["effect"] } },
});
```

Relative specifiers resolve against the importing file, and package specifiers from the nearest
`node_modules`, honouring its `exports` and `main`. A reference the reader cannot follow, such as a
config a function builds, is an error rather than an empty config. Editing a base config reaches
every config built on it.
