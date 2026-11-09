/** @derive(Debug, Clone, Encode, Decode) */
interface HTTPResponse {
    status: number;
}

/** @derive(Debug, Clone, Encode, Decode) */
interface HttpResponse {
    code: number;
}

/** @derive(Encode) */
interface User {
    id: string;
}

/** @derive(Encode) */
interface UserEncode {
    raw: string;
}

export function userClone(): void {}

/** @derive(Clone) */
interface Account {
    id: string;
}

function accountClone(): void {}
