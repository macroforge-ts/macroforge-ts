/** import macro { JSON } from "@playground/macro"; */

/**  */
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

    toString(): string {
    const parts: string[] = [];
    parts.push("userId: " + this.id);
    parts.push("name: " + this.name);
    parts.push("role: " + this.role);
    parts.push("favoriteMacro: " + this.favoriteMacro);
    parts.push("since: " + this.since);
    return "MacroUser { " + parts.join(", ") + " }";
}

    toJSON(): Record<string, unknown> {
    const result: Record<string, unknown> = {};
    result["id"] = this.id;
    result["name"] = this.name;
    result["role"] = this.role;
    result["favoriteMacro"] = this.favoriteMacro;
    result["since"] = this.since;
    result["apiToken"] = this.apiToken;
    return result;
}

    static fromJSON(data: unknown): MacroUser {
    if (typeof data !== "object" || data === null || Array.isArray(data)) {
        throw new Error("MacroUser.fromJSON: expected an object, got " + (Array.isArray(data) ? "array" : typeof data));
    }
    const obj = data as Record<string, unknown>;
    if (!("id" in obj)) {
        throw new Error("MacroUser.fromJSON: missing required field \"id\"");
    }
    if (!("name" in obj)) {
        throw new Error("MacroUser.fromJSON: missing required field \"name\"");
    }
    if (!("role" in obj)) {
        throw new Error("MacroUser.fromJSON: missing required field \"role\"");
    }
    if (!("favoriteMacro" in obj)) {
        throw new Error("MacroUser.fromJSON: missing required field \"favoriteMacro\"");
    }
    if (!("since" in obj)) {
        throw new Error("MacroUser.fromJSON: missing required field \"since\"");
    }
    if (!("apiToken" in obj)) {
        throw new Error("MacroUser.fromJSON: missing required field \"apiToken\"");
    }
    const instance = new MacroUser();
    const __raw_id = obj["id"];
    instance.id = __raw_id as string;
    const __raw_name = obj["name"];
    instance.name = __raw_name as string;
    const __raw_role = obj["role"];
    instance.role = __raw_role as string;
    const __raw_favoriteMacro = obj["favoriteMacro"];
    instance.favoriteMacro = __raw_favoriteMacro as "Derive" | "JsonNative";
    const __raw_since = obj["since"];
    instance.since = __raw_since as string;
    const __raw_apiToken = obj["apiToken"];
    instance.apiToken = __raw_apiToken as string;
    return instance;
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