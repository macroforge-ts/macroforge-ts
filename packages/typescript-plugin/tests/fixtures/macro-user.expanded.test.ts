import { Schema } from "effect";
/** import macro { JSON } from "@playground/macro"; */

export class MacroUser {
  id: string;

  name: string;
  role: string;
  favoriteMacro: "Derive" | "JsonNative";
  since: string;

  apiToken: string;

  constructor(
    id: string,
    name: string,
    role: string,
    favoriteMacro: "Derive" | "JsonNative",
    since: string,
    apiToken: string,
  ) {
    this.id = id;
    this.name = name;
    this.role = role;
    this.favoriteMacro = favoriteMacro;
    this.since = since;
    this.apiToken = apiToken;
  }

  toString() {
    return `MacroUser { userId: ${this.id}, name: ${this.name} }`;
  }

  toJSON() {
    return {
      userId: this.id,
      name: this.name,
      role: this.role,
      favoriteMacro: this.favoriteMacro,
      since: this.since,
    };
  }
}

const showcaseUser = new MacroUser(
  "usr_2626",
  "Alya Vector",
  "Macro Explorer",
  "Derive",
  "2024-09-12",
  "svelte-secret-token",
);

export const showcaseUserSummary = showcaseUser.toString();
export const showcaseUserJson = showcaseUser.toJSON();
