import { expandExample } from '$lib/server/macroforge';

export function load() {
	return {
		examples: {
			basic: expandExample(`import { Clone } from "macroforge";

/** @derive(Clone) */
class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}`),
			nested: expandExample(`import { Clone } from "macroforge";

/** @derive(Clone) */
class User {
  name: string;
  address: { city: string; zip: string };

  constructor(name: string, address: { city: string; zip: string }) {
    this.name = name;
    this.address = address;
  }
}`),
			interface: expandExample(`import { Clone } from "macroforge";

/** @derive(Clone) */
interface Point {
  x: number;
  y: number;
}`),
			enum: expandExample(`import { Clone } from "macroforge";

/** @derive(Clone) */
enum Status {
  Active = "active",
  Inactive = "inactive",
}`),
			typeAlias: expandExample(`import { Clone } from "macroforge";

/** @derive(Clone) */
type Point = {
  x: number;
  y: number;
};`)
		}
	};
}
