import { Derive, Debug } from "./macros";

/** @derive(Debug) */
class User {
  id: string;
}

const u = new User();
console.log(u.id);
