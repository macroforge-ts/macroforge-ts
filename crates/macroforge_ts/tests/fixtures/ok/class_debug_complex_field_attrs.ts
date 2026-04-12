/** @derive(Debug) */
class MacroUser {
    /** @debug({ rename: "userId" }) */
    id: string;

    name: string;
    role: string;
    favoriteMacro: 'Derive' | 'JsonNative';
    since: string;

    /** @debug({ skip: true }) */
    apiToken: string;

    constructor(
        id: string,
        name: string,
        role: string,
        favoriteMacro: 'Derive' | 'JsonNative',
        since: string,
        apiToken: string
    ) {
        this.id = id;
        this.name = name;
        this.role = role;
        this.favoriteMacro = favoriteMacro;
        this.since = since;
        this.apiToken = apiToken;
    }
}
