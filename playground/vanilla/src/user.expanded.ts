import { Derive, Debug, debug } from "@ts-macros/swc-napi";
import { JSON } from "@playground/macro";

// Example of using Derive decorator and dynamic macro
export class User {
  id: number;

  name: string;
  email: string;

  authToken: string;

  constructor(id: number, name: string, email: string, authToken: string) {
    this.id = id;
    this.name = name;
    this.email = email;
    this.authToken = authToken;
  }

  toString(): string {
    const parts: string[] = [];
    parts.push("identifier: " + this.id);
    parts.push("name: " + this.name);
    parts.push("email: " + this.email);
    return "User { " + parts.join(", ") + " }";
}

  toJSON(): Record<string, unknown> {
    const result: Record<string, unknown> = {};
    result.id = this.id;
    result.name = this.name;
    result.email = this.email;
    result.authToken = this.authToken;
    return result;
}
}

const user = new User(1, "John Doe", "john@example.com", "tok_live_secret");

const derivedSummary = user.toString();
const derivedJson = user.toJSON();