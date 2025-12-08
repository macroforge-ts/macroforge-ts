/** import macro { JSON } from "@playground/macro"; */

/** @derive(Debug, Serialize, Deserialize) */
export class MacroUser {
  /** @debug({ rename: "userId" }) */
  id: string;
  name: string;
  role: string;
  favoriteMacro: "Derive" | "JsonNative";
  since: string;
  /** @debug({ skip: true }) */
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
