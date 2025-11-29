export declare class MacroUser {
    id: string;
    name: string;
    role: string;
    favoriteMacro: "Derive" | "JsonNative";
    since: string;
    apiToken: string;
    constructor(id: string, name: string, role: string, favoriteMacro: "Derive" | "JsonNative", since: string, apiToken: string);
    toString(): string;
    toJSON(): Record<string, unknown>;
}
export declare const showcaseUserSummary: string;
export declare const showcaseUserJson: Record<string, unknown>;
//# sourceMappingURL=macro-user.test.d.ts.map