interface User {
    id: string;
    name: string;
    email: string;
}

type Role = 'admin' | 'user' | 'guest';

enum Status {
    Active,
    Inactive,
    Pending
}

function createUser(name: string): User {
    return {
        id: crypto.randomUUID(),
        name,
        email: `${name}@example.com`
    };
}

const users: Map<string, User> = new Map();

export { User, Role, Status, createUser, users };
