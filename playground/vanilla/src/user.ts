import { Derive, Debug } from "@ts-macros/macros";

// Example of using Derive decorator and dynamic macro
@Derive("Debug", "JsonNative")
class User {
  @Debug({ rename: "identifier" })
  id: number;

  name: string;
  email: string;

  @Debug({ skip: true })
  authToken: string;

  constructor(id: number, name: string, email: string, authToken: string) {
    this.id = id;
    this.name = name;
    this.email = email;
    this.authToken = authToken;
  }
}

export { User };
