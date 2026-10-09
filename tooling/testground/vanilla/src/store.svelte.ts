/** @derive(Debug, Clone) */
export class Store {
    items: string[];
    count: number;

    constructor(items: string[], count: number) {
        this.items = items;
        this.count = count;
    }
}
