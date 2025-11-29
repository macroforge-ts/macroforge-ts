declare class User {
    id: number;
    name: string;
    email: string;
    authToken: string;
    constructor(id: number, name: string, email: string, authToken: string);
    toString(): string;
    toJSON(): Record<string, unknown>;
}
export { User };
