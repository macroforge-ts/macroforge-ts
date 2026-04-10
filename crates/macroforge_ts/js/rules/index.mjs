// js/rules/index.ts
function macroRules(_strings, ..._values) {
  throw new Error(
    "macroforge/rules: macros are compile-time only \u2014 they should have been erased by the macroforge build pass. If you're seeing this at runtime, the macroforge plugin is not installed or not running on this file."
  );
}
export {
  macroRules
};
