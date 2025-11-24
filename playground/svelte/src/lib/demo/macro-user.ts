import { Derive, IncludeStr } from "$lib/macros";

export const macroNote = IncludeStr("./macro-snippet.md").trim();

@Derive("Debug", "JSON")
export class MacroUser {
  constructor(
    public name: string,
    public role: string,
    public favoriteMacro: "IncludeStr" | "Derive",
    public since: string,
  ) {}
}

const showcaseUser = new MacroUser("Alya Vector", "Macro Explorer", "Derive", "2024-09-12");

export const showcaseUserSummary = showcaseUser.toString();
export const showcaseUserJson = showcaseUser.toJSON();
