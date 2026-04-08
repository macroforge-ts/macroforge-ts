import { Derive } from "@macro/derive";

/** @derive(Debug, PartialEq, Hash) */
class Config {
    readonly id: string;
    name: string;
    description?: string;
    readonly createdAt: Date;
    updatedAt?: Date;

    constructor(id: string, name: string, createdAt: Date) {
        this.id = id;
        this.name = name;
        this.createdAt = createdAt;
    }

    update(name: string, description?: string): void {
        this.name = name;
        this.description = description;
        this.updatedAt = new Date();
    }
}
