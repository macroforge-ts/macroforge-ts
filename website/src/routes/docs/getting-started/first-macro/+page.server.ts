import type { PageServerLoad } from './$types';
import { expandExample } from '$lib/server/macroforge';

export const load: PageServerLoad = async () => {
	return {
		examples: {
			basic: expandExample(`/** @derive(Debug, Clone, Eq) */
export class User {
  name: string;
  age: number;
  email: string;

  constructor(name: string, age: number, email: string) {
    this.name = name;
    this.age = age;
    this.email = email;
  }
}`),
			customizing: expandExample(`/** @derive(Debug) */
export class User {
  /** @debug({ rename: "userId" }) */
  id: number;

  name: string;

  /** @debug({ skip: true }) */
  password: string;

  constructor(id: number, name: string, password: string) {
    this.id = id;
    this.name = name;
    this.password = password;
  }
}`)
		}
	};
};
