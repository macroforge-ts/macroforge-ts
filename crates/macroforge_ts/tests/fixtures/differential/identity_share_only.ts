// Differential fixture: $identity in `share-only` mode.
//
// Dev and prod should both emit the runtime helper and rewrite call
// sites to call it (share-only doesn't branch on build mode). The
// runtime is a plain identity; both expansions should log the same
// value.

import { macroRules } from "macroforge/rules";

const $identity = macroRules({
  mode: "share-only",
  expand: macroRules`
    ($x:Expr) => __inline_identity($x)
  `,
  runtime: "function __identity(v) { return v; }",
  call: macroRules`
    ($x:Expr) => __identity($x)
  `,
});

const a = $identity(42);
const b = $identity("hello");
const c = $identity([1, 2, 3]);

console.log(JSON.stringify({ a, b, c }));
