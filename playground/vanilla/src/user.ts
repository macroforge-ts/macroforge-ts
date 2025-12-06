import { Derive, Debug, debug } from "macroforge";
import { JSON } from "@playground/macro";
/** import macro { JSON } from "@playground/macro"; */

// Example of using Derive decorator and dynamic macro
/** @derive(Debug, JSON) */
export class User {
  /** @debug({ rename: "identifier" }) */
  id: number;

  name: string;
  email: string;

  /** @debug({ skip: true }) */
  authToken: string;

  constructor(id: number, name: string, email: string, authToken: string) {
    this.id = id;
    this.name = name;
    this.email = email;
    this.authToken = authToken;
  }
}

const user = new User(1, "John Doe", "john@example.com", "tok_live_secret");

const derivedSummary = user.toString();
const derivedJson = user.toJSON();
