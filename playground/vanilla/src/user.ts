import { Derive, Debug, debug } from "@ts-macros/swc-napi";
import { JSON } from "@playground/macro";

// Example of using Derive decorator and dynamic macro
@Derive(Debug, JSON)
class User {
  @debug({ rename: "identifier" })
  id: number;

  name: string;
  email: string;

  @debug({ skip: true })
  authToken: string;

  constructor(id: number, name: string, email: string, authToken: string) {
    this.id = id;
    this.name = name;
    this.email = email;
    this.authToken = authToken;
  }
}

export { User };
