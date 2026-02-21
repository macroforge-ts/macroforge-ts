/**
 * Custom validator test classes for comprehensive deserializer validation testing.
 */

// Custom validator function for even numbers
export function isEven(value: number): boolean {
    return value % 2 === 0;
}

// Custom validator function for valid usernames
export function isValidUsername(value: string): boolean {
    return /^[a-z][a-z0-9_]{2,15}$/.test(value);
}

// Custom number validator
/** @derive(Deserialize) */
export class CustomNumberValidator {
    /** @serde({ validate: ["custom(isEven)"] }) */
    evenNumber: number;
}

// Custom string validator
/** @derive(Deserialize) */
export class CustomStringValidator {
    /** @serde({ validate: ["custom(isValidUsername)"] }) */
    username: string;
}

// Custom validator with custom message
/** @derive(Deserialize) */
export class CustomWithMessageValidator {
    /** @serde({ validate: [{ validate: "custom(isEven)", message: "Number must be even" }] }) */
    evenNumber: number;
}
