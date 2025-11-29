import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
}

const obj = new User();
obj.
