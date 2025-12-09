import { expandExample } from '$lib/server/macroforge';

export function load() {
	return {
		examples: {
			basic: expandExample(`import { PartialEq } from "macroforge";

/** @derive(PartialEq) */
class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}`),
			skip: expandExample(`import { PartialEq } from "macroforge";

/** @derive(PartialEq) */
class User {
  id: number;
  name: string;

  /** @partialEq(skip) */
  createdAt: Date;

  constructor(id: number, name: string, createdAt: Date) {
    this.id = id;
    this.name = name;
    this.createdAt = createdAt;
  }
}`),
			interface: expandExample(`import { PartialEq } from "macroforge";

/** @derive(PartialEq) */
interface Point {
  x: number;
  y: number;
}`),
			enum: expandExample(`import { PartialEq } from "macroforge";

/** @derive(PartialEq) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}`),
			typeAlias: expandExample(`import { PartialEq } from "macroforge";

/** @derive(PartialEq) */
type Point = {
  x: number;
  y: number;
};`)
		}
	};
}
