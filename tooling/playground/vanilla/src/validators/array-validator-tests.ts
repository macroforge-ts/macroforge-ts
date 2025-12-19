/**
 * Array validator test classes for comprehensive deserializer validation testing.
 */

// MaxItems validator
/** @derive(Deserialize) */
export class MaxItemsValidator {
    /** @serde({ validate: ["maxItems(5)"] }) */
    items: string[];
}

// MinItems validator
/** @derive(Deserialize) */
export class MinItemsValidator {
    /** @serde({ validate: ["minItems(2)"] }) */
    items: string[];
}

// ItemsCount validator
/** @derive(Deserialize) */
export class ItemsCountValidator {
    /** @serde({ validate: ["itemsCount(3)"] }) */
    items: string[];
}
