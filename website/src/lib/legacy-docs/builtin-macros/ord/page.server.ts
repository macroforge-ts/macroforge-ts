import { expandExample } from '$lib/server/macroforge';
import { getBuiltinMacro, getVersion } from '$lib/server/api-docs';

export async function load() {
	return {
		version: getVersion('rust', 'macroforge_ts'),
		macro: getBuiltinMacro('ord'),
		examples: {
			basic: await expandExample(`/** @derive(Ord) */
class Version {
  major: number;
  minor: number;
  patch: number;

  constructor(major: number, minor: number, patch: number) {
    this.major = major;
    this.minor = minor;
    this.patch = patch;
  }
}`),
			skip: await expandExample(`/** @derive(Ord) */
class Task {
  priority: number;
  name: string;

  /** @ord(skip) */
  createdAt: Date;

  constructor(priority: number, name: string, createdAt: Date) {
    this.priority = priority;
    this.name = name;
    this.createdAt = createdAt;
  }
}`),
			interface: await expandExample(`/** @derive(Ord) */
interface Point {
  x: number;
  y: number;
}`),
			enum: await expandExample(`/** @derive(Ord) */
enum Priority {
  Low = 0,
  Medium = 1,
  High = 2,
  Critical = 3
}`),
			typeAlias: await expandExample(`/** @derive(Ord) */
type Coordinate = {
  x: number;
  y: number;
};`)
		}
	};
}
