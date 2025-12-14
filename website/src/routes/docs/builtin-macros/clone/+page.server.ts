import { expandExample } from '$lib/server/macroforge';
import { getBuiltinMacro, getVersion } from '$lib/server/api-docs';

export function load() {
	return {
		version: getVersion('rust', 'macroforge_ts'),
		macro: getBuiltinMacro('clone'),
		examples: {
			basic: expandExample(`/** @derive(Clone) */
class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}`),
			nested: expandExample(`/** @derive(Clone) */
class User {
  name: string;
  address: { city: string; zip: string };

  constructor(name: string, address: { city: string; zip: string }) {
    this.name = name;
    this.address = address;
  }
}`),
			interface: expandExample(`/** @derive(Clone) */
interface Point {
  x: number;
  y: number;
}`),
			enum: expandExample(`/** @derive(Clone) */
enum Status {
  Active = "active",
  Inactive = "inactive",
}`),
			typeAlias: expandExample(`/** @derive(Clone) */
type Point = {
  x: number;
  y: number;
};`)
		}
	};
}
