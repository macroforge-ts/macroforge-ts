import { expandExample } from '$lib/server/macroforge';

export async function load() {
    const heroExample = await expandExample(
        `
/** @derive(Debug, Clone, PartialEq) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}`,
        'hero-example.ts'
    );

    return {
        heroExample
    };
}
