/** import macro { traced } from "@macroforge/test-macros" */

/** @traced */
export async function fetchUser(id: number): Promise<string> {
    return `user-${id}`;
}
