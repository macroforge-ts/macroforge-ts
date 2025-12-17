import { expandExample } from '$lib/server/macroforge';
import { getBuiltinMacro, getVersion } from '$lib/server/api-docs';

export async function load() {
	return {
		version: getVersion('rust', 'macroforge_ts'),
		macro: getBuiltinMacro('hash'),
		examples: {
			basic: await expandExample(`/** @derive(Hash) */
class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}`),
			skip: await expandExample(`/** @derive(Hash) */
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
			interface: await expandExample(`/** @derive(Hash) */
interface Point {
  x: number;
  y: number;
}`),
			enum: await expandExample(`/** @derive(Hash) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}`),
			typeAlias: await expandExample(`/** @derive(Hash) */
type Coordinates = {
  lat: number;
  lng: number;
};`)
		}
	};
}
