import { Derive } from "@macro/derive";

/** @derive(Debug) */
class ValidationExample {
    /** @debug({ rename: "userId" }) */
    id: string;

    name: string;

    /** @debug({ skip: true }) */
    internalFlag: boolean;

    constructor(id: string, name: string, internalFlag: boolean) {
        this.id = id;
        this.name = name;
        this.internalFlag = internalFlag;
    }
}
