import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
  toString(): string {
    return "User { id: " + this.id + " }";
  }
  clone(): User {
    return new User();
  }
}

const x: string = 123;
