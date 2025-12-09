import type { PageServerLoad } from './$types';
import { expandExample } from '$lib/server/macroforge';

export const load: PageServerLoad = async () => {
	return {
		examples: {
			fieldAttributes: expandExample(`/** @derive(Debug, Serialize) */
class User {
  /** @debug({ rename: "userId" }) */
  /** @serde({ rename: "user_id" }) */
  id: number;

  name: string;

  /** @debug({ skip: true }) */
  /** @serde({ skip: true }) */
  password: string;

  /** @serde({ flatten: true }) */
  metadata: Record<string, unknown>;
}`)
		}
	};
};
