/**
 * Array validator test classes for comprehensive decoder validation testing.
 */

// MaxItems validator
/** @derive(Decode) */
export class MaxItemsValidator {
    /** @endec({ validate: ["maxItems(5)"] }) */
    items: Array<string>;
}

// MinItems validator
/** @derive(Decode) */
export class MinItemsValidator {
    /** @endec({ validate: ["minItems(2)"] }) */
    items: Array<string>;
}

// ItemsCount validator
/** @derive(Decode) */
export class ItemsCountValidator {
    /** @endec({ validate: ["itemsCount(3)"] }) */
    items: Array<string>;
}
