import { expandExample } from '$lib/server/macroforge';

export function load() {
	return {
		examples: {
			basic: expandExample(`import { PartialOrd } from "macroforge";

/** @derive(PartialOrd) */
class Temperature {
  celsius: number;

  constructor(celsius: number) {
    this.celsius = celsius;
  }
}`),
			skip: expandExample(`import { PartialOrd } from "macroforge";

/** @derive(PartialOrd) */
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
			interface: expandExample(`import { PartialOrd } from "macroforge";

/** @derive(PartialOrd) */
interface Measurement {
  value: number;
  unit: string;
}`),
			enum: expandExample(`import { PartialOrd } from "macroforge";

/** @derive(PartialOrd) */
enum Size {
  Small = 1,
  Medium = 2,
  Large = 3
}`),
			typeAlias: expandExample(`import { PartialOrd } from "macroforge";

/** @derive(PartialOrd) */
type Interval = {
  start: number;
  end: number;
};`)
		}
	};
}
