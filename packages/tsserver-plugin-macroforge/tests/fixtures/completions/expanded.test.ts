import { Derive, Debug } from "./macros";

/** @derive(Debug) */
class User {
  id: string;
  toString(): string { return this.id; }
}

const obj = new User();
obj.
