import { expandExample } from '$lib/server/macroforge';

export function load() {
	return {
		examples: {
			basic: expandExample(`import { Hash } from "macroforge";

/** @derive(Hash) */
class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}`),
			skip: expandExample(`import { Hash } from "macroforge";

/** @derive(Hash) */
class User {
  id: number;
  name: string;

  /** @hash(skip) */
  lastLogin: Date;

  constructor(id: number, name: string, lastLogin: Date) {
    this.id = id;
    this.name = name;
    this.lastLogin = lastLogin;
  }
}`),
			interface: expandExample(`import { Hash } from "macroforge";

/** @derive(Hash) */
interface Point {
  x: number;
  y: number;
}`),
			enum: expandExample(`import { Hash } from "macroforge";

/** @derive(Hash) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}`),
			typeAlias: expandExample(`import { Hash } from "macroforge";

/** @derive(Hash) */
type Coordinates = {
  lat: number;
  lng: number;
};`)
		}
	};
}
