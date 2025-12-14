import { expandExample } from '$lib/server/macroforge';
import { getBuiltinMacro, getVersion } from '$lib/server/api-docs';

export function load() {
	const macro = getBuiltinMacro('debug');

	// Use example from Rust doc comments if available, otherwise use fallback
	const docExample = macro?.exampleCode?.[0]?.code;

	return {
		version: getVersion('rust', 'macroforge_ts'),
		macro,
		examples: {
			// Use the example from Rust doc comments (includes rename and skip)
			fromDocs: docExample ? expandExample(docExample) : null,
			// Hand-written examples for specific variations
			basic: expandExample(`/** @derive(Debug) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}`),
			rename: expandExample(`/** @derive(Debug) */
class User {
  /** @debug({ rename: "userId" }) */
  id: number;

  name: string;

  constructor(id: number, name: string) {
    this.id = id;
    this.name = name;
  }
}`),
			skip: expandExample(`/** @derive(Debug) */
class User {
  name: string;
  email: string;

  /** @debug({ skip: true }) */
  password: string;

  /** @debug({ skip: true }) */
  authToken: string;

  constructor(name: string, email: string, password: string, authToken: string) {
    this.name = name;
    this.email = email;
    this.password = password;
    this.authToken = authToken;
  }
}`),
			interface: expandExample(`/** @derive(Debug) */
interface Status {
  active: boolean;
  message: string;
}`),
			enum: expandExample(`/** @derive(Debug) */
enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
}`),
			typeAlias: expandExample(`/** @derive(Debug) */
type Point = {
  x: number;
  y: number;
};`)
		}
	};
}
