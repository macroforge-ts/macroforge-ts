/** import macro { JSON } from "@playground/macro"; */

// Example of using Derive decorator and dynamic macro

export class User {
    
    id: number;

    name: string;
    email: string;

    
    authToken: string;

    constructor(id: number, name: string, email: string, authToken: string) {
        this.id = id;
        this.name = name;
        this.email = email;
        this.authToken = authToken;
    }

    static toString(value: User): string {
    return userToString(value);
}

    toJSON(): Record<string, unknown> {
    const result: Record<string, unknown> = {};
    result.id = this.id;
    result.name = this.name;
    result.email = this.email;
    result.authToken = this.authToken;
    return result;
}
}

export function userToString(value: User): string {const parts: string[]= []; parts.push("identifier: " + value.id); parts.push("name: " + value.name); parts.push("email: " + value.email); return "User { " + parts.join(", " )+ " }" ; }

const user = new User(1, 'John Doe', 'john@example.com', 'tok_live_secret');

export const derivedSummary = User.toString(user);
export const derivedJson = user.toJSON();