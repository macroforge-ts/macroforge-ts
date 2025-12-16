/** import macro { JSON } from "@playground/macro"; */

/** @derive(Debug, Serialize, Deserialize) */
export class MacroUser {
    /** @debug({ rename: "userId" }) */
    id: string;
    name: string;
    role: string;
    favoriteMacro: 'Derive' | 'JsonNative';
    since: string;
    /** @debug({ skip: true }) */
    apiToken: string;
}

const showcaseUser = new MacroUser({
    id: 'usr_2626',
    name: 'Alya Vector',
    role: 'Macro Explorer',
    favoriteMacro: 'Derive',
    since: '2024-09-12',
    apiToken: 'svelte-secret-token'
});

export const showcaseUserSummary = MacroUser.toString(showcaseUser);
export const showcaseUserJson = JSON.parse(MacroUser.serialize(showcaseUser));
