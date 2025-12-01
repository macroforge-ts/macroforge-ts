import { Derive, Debug } from "@ts-macros/macros";

/** @derive(Debug) */
export class User {
  id: string;
  name: string;
  email: string;

  constructor(id: string, name: string, email: string) {
    this.id = id;
    this.name = name;
    this.email = email;
  }
}

// Test that macro-generated methods work
const user = new User("1", "Test User", "test@example.com");
console.log(user.toString());
