import { expandExample } from '$lib/server/macroforge';

export function load() {
	return {
		examples: {
			basic: expandExample(`/** @derive(Ord) */
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
			skip: expandExample(`/** @derive(Ord) */
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
			interface: expandExample(`/** @derive(Ord) */
interface Point {
  x: number;
  y: number;
}`),
			enum: expandExample(`/** @derive(Ord) */
enum Priority {
  Low = 0,
  Medium = 1,
  High = 2,
  Critical = 3
}`),
			typeAlias: expandExample(`/** @derive(Ord) */
type Coordinate = {
  x: number;
  y: number;
};`)
		}
	};
}
