import { Derive } from '@macro/derive';

/** @derive(Debug, Clone) */
class User {
    id: number;
    name: string;

    constructor(id: number, name: string) {
        this.id = id;
        this.name = name;
    }
}
