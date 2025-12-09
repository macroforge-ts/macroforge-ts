import type { PageServerLoad } from './$types';
import { expandExample } from '$lib/server/macroforge';

export const load: PageServerLoad = async () => {
	return {
		examples: {
			basic: expandExample(`import { Debug } from "macroforge";

/** @derive(Debug) */
class User {
  name: string;
}`)
		}
	};
};
