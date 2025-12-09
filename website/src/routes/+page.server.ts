import { expandExample } from '$lib/server/macroforge';

export function load() {
	const heroExample = expandExample(`import { Debug, Clone, PartialEq } from "macroforge";

/** @derive(Debug, Clone, PartialEq) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}`);

	return {
		heroExample
	};
}
