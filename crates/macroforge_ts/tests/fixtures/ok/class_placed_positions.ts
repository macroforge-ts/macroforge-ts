/** import macro { Placed } from "@macroforge/test-macros" */
import { helper } from "./helper";

/** @derive(Placed) */
export class Widget {
    id: string;
}

export const after = helper();
