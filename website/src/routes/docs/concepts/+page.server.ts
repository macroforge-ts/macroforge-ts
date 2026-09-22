import { expandExample } from '$lib/server/macroforge.ts';

export const load = async () => {
    return {
        examples: {
            basic: await expandExample(`

/** @derive(Debug) */
class User {
  name: string;
}`)
        }
    };
};
