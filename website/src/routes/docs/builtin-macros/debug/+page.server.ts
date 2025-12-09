import { expandExample } from '$lib/server/macroforge';

export function load() {
	return {
		examples: {
			basic: expandExample(`import { Debug } from "macroforge";

/** @derive(Debug) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}`),
			rename: expandExample(`import { Debug } from "macroforge";

/** @derive(Debug) */
class User {
  /** @debug({ rename: "userId" }) */
  id: number;

  name: string;

  constructor(id: number, name: string) {
    this.id = id;
    this.name = name;
  }
}`),
			skip: expandExample(`import { Debug } from "macroforge";

/** @derive(Debug) */
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
			interface: expandExample(`import { Debug } from "macroforge";

/** @derive(Debug) */
interface Status {
  active: boolean;
  message: string;
}`),
			enum: expandExample(`import { Debug } from "macroforge";

/** @derive(Debug) */
enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
}`),
			typeAlias: expandExample(`import { Debug } from "macroforge";

/** @derive(Debug) */
type Point = {
  x: number;
  y: number;
};`)
		}
	};
}
