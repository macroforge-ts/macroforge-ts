import { Derive, Debug } from "./macros";

/** @derive(Debug) */
class User {
  id: string;
}

const obj = new User();
obj.
