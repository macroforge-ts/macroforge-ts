import { Derive } from '@macro/derive';

/** @derive(Debug, Clone, PartialEq, Hash) */
class Product {
    id: string;
    name: string;
    price: number;
    private secret: string;

    constructor(id: string, name: string, price: number, secret: string) {
        this.id = id;
        this.name = name;
        this.price = price;
        this.secret = secret;
    }

    getDisplayName(): string {
        return `${this.name} - $${this.price}`;
    }

    static fromJSON(json: any): Product {
        return new Product(json.id, json.name, json.price, json.secret);
    }
}
