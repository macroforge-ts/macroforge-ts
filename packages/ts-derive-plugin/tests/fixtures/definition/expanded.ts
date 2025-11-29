import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
  toString(): string { return this.id; }
}

const u = new User();
console.log(u.id);
