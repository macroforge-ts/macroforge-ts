import { Result } from "macroforge/result";
/** import macro { JSON } from "@playground/macro"; */

/**  */
export class MacroUser {
  
  id: string;
  name: string;
  role: string;
  favoriteMacro: "Derive" | "JsonNative";
  since: string;
  
  apiToken: string;

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

  constructor(init: {
    id: string;
    name: string;
    role: string;
    favoriteMacro: "Derive" | "JsonNative";
    since: string;
    apiToken: string;
}){
    this.id = init.id;
    this.name = init.name;
    this.role = init.role;
    this.favoriteMacro = init.favoriteMacro;
    this.since = init.since;
    this.apiToken = init.apiToken;
}

  static fromJSON(data: unknown): Result<MacroUser, string[]> {
    if (typeof data !== "object" || data === null || Array.isArray(data)) {
        return Result.err([
            "MacroUser.fromJSON: expected an object, got " + (Array.isArray(data) ? "array" : typeof data)
        ]);
    }
    const obj = data as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
        errors.push("MacroUser.fromJSON: missing required field \"id\"");
    }
    if (!("name" in obj)) {
        errors.push("MacroUser.fromJSON: missing required field \"name\"");
    }
    if (!("role" in obj)) {
        errors.push("MacroUser.fromJSON: missing required field \"role\"");
    }
    if (!("favoriteMacro" in obj)) {
        errors.push("MacroUser.fromJSON: missing required field \"favoriteMacro\"");
    }
    if (!("since" in obj)) {
        errors.push("MacroUser.fromJSON: missing required field \"since\"");
    }
    if (!("apiToken" in obj)) {
        errors.push("MacroUser.fromJSON: missing required field \"apiToken\"");
    }
    const __raw_id = obj["id"];
    const __raw_name = obj["name"];
    const __raw_role = obj["role"];
    const __raw_favoriteMacro = obj["favoriteMacro"];
    const __raw_since = obj["since"];
    const __raw_apiToken = obj["apiToken"];
    if (errors.length > 0) {
        return Result.err(errors);
    }
    const init: Record<string, unknown> = {};
    init.id = __raw_id as string;
    init.name = __raw_name as string;
    init.role = __raw_role as string;
    init.favoriteMacro = __raw_favoriteMacro as "Derive" | "JsonNative";
    init.since = __raw_since as string;
    init.apiToken = __raw_apiToken as string;
    return Result.ok(new MacroUser(init as any));
}
}

const showcaseUser = new MacroUser({
  id: "usr_2626",
  name: "Alya Vector",
  role: "Macro Explorer",
  favoriteMacro: "Derive",
  since: "2024-09-12",
  apiToken: "svelte-secret-token",
});

export const showcaseUserSummary = showcaseUser.toString();
export const showcaseUserJson = showcaseUser.toJSON();