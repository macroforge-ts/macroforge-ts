import { Derive } from '@macro/derive';

/** @derive(Clone) */
class Account {
    public username: string;
    protected password: string;
    private apiKey: string;

    constructor(username: string, password: string, apiKey: string) {
        this.username = username;
        this.password = password;
        this.apiKey = apiKey;
    }

    public login(): boolean {
        return true;
    }

    protected validatePassword(input: string): boolean {
        return this.password === input;
    }

    private getApiKey(): string {
        return this.apiKey;
    }
}
