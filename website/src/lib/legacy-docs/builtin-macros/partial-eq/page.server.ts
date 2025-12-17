import { expandExample } from '$lib/server/macroforge';
import { getBuiltinMacro, getVersion } from '$lib/server/api-docs';

export async function load() {
	return {
		version: getVersion('rust', 'macroforge_ts'),
		macro: getBuiltinMacro('partial_eq'),
		examples: {
			basic: await expandExample(`/** @derive(PartialEq) */
class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}`),
			skip: await expandExample(`/** @derive(PartialEq) */
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
			interface: await expandExample(`/** @derive(PartialEq) */
interface Point {
  x: number;
  y: number;
}`),
			enum: await expandExample(`/** @derive(PartialEq) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}`),
			typeAlias: await expandExample(`/** @derive(PartialEq) */
type Point = {
  x: number;
  y: number;
};`)
		}
	};
}
