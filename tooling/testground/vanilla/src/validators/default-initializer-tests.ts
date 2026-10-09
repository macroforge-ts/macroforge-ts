// Default on classes whose fields have initializers, derived beside Decode,
// whose constructor takes arguments.

/** @derive(Default, Decode) */
export class Settings {
    theme: string = 'dark';
    retries?: number = 3;
    label?: string;
    tags: string[] = ['general'];
    /** @default(7) */
    limit: number = 5;
    createdAt: number = Date.now();
    static instances: number = 0;
}

/** @derive(Default) */
export class Greeter {
    name: string = 'world';
    greet: () => string = function (this: Greeter): string {
        return `hello ${this.name}`;
    };
}

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Encode, Decode, Default) */
export class Counter {
    static created: number = 0;
    count: number = 1;
}
