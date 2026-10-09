/** import macro { stringify, concat_names, twice } from "@macroforge/test-macros" */

export const quotedCall = $stringify($concat_names(a, b));
export const doubled = $twice(ab);
