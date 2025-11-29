import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
}

const u = new User();
console.log(u.id);
