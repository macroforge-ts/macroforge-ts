import { expandExample } from '$lib/server/macroforge';
import { getBuiltinMacro, getVersion } from '$lib/server/api-docs';

export function load() {
	return {
		version: getVersion('rust', 'macroforge_ts'),
		macro: getBuiltinMacro('partial_ord'),
		examples: {
			basic: expandExample(`/** @derive(PartialOrd) */
class Temperature {
  celsius: number;

  constructor(celsius: number) {
    this.celsius = celsius;
  }
}`),
			skip: expandExample(`/** @derive(PartialOrd) */
class Item {
  price: number;
  name: string;

  /** @ord(skip) */
  description: string;

  constructor(price: number, name: string, description: string) {
    this.price = price;
    this.name = name;
    this.description = description;
  }
}`),
			interface: expandExample(`/** @derive(PartialOrd) */
interface Measurement {
  value: number;
  unit: string;
}`),
			enum: expandExample(`/** @derive(PartialOrd) */
enum Size {
  Small = 1,
  Medium = 2,
  Large = 3
}`),
			typeAlias: expandExample(`/** @derive(PartialOrd) */
type Interval = {
  start: number;
  end: number;
};`)
		}
	};
}
