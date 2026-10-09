/**
 * Validator form model for E2E testing in Svelte.
 * Tests string, number, array, and date validators with real form validation.
 */

import type { ValidationOutcome } from '../validation-outcome';

/** @derive(Decode) */
export class UserRegistrationForm {
    /** @endec({ validate: ["email"] }) */
    email: string;

    /** @endec({ validate: ["minLength(8)", "maxLength(50)"] }) */
    password: string;

    /** @endec({ validate: ["minLength(3)", "maxLength(20)", "lowercase", "pattern(\"^[a-z][a-z0-9_]+$\")"] }) */
    username: string;

    /** @endec({ validate: ["int", "between(18, 120)"] }) */
    age: number;

    /** @endec({ validate: ["url"] }) */
    website: string;
}

/** @derive(Decode) */
export class ProductForm {
    /** @endec({ validate: ["nonEmpty", "maxLength(100)"] }) */
    name: string;

    /** @endec({ validate: ["positive", "lessThan(1000000)"] }) */
    price: number;

    /** @endec({ validate: ["int", "nonNegative"] }) */
    quantity: number;

    /** @endec({ validate: ["minItems(1)", "maxItems(5)"] }) */
    tags: Array<string>;

    /** @endec({ validate: ["uuid"] }) */
    sku: string;
}

/** @derive(Decode) */
export class EventForm {
    /** @endec({ validate: ["nonEmpty", "trimmed"] }) */
    title: string;

    /** @endec({ validate: ["validDate", "greaterThanDate(\"2020-01-01\")"] }) */
    startDate: Date;

    /** @endec({ validate: ["validDate"] }) */
    endDate: Date;

    /** @endec({ validate: ["int", "between(1, 1000)"] }) */
    maxAttendees: number;
}

/** A validator's result, with each field error rendered as `field: message`. */
export type ValidationResult<T> = ValidationOutcome<T, Array<string>>;

/** Converts what a derived `decode()` returns into a `ValidationResult`. */
export function toValidationResult<T>(
    result: { success: true; value: T } | {
        success: false;
        errors: Array<{ field: string; message: string }>;
    }
): ValidationResult<T> {
    if (result.success) {
        return { success: true, data: result.value };
    }
    return {
        success: false,
        errors: result.errors.map((error) => `${error.field}: ${error.message}`)
    };
}

// Form validation functions
export function validateUserRegistration(
    data: unknown
): ValidationResult<UserRegistrationForm> {
    const result = UserRegistrationForm.decode(data);
    return toValidationResult(result);
}

export function validateProduct(data: unknown): ValidationResult<ProductForm> {
    const result = ProductForm.decode(data);
    return toValidationResult(result);
}

export function validateEvent(data: unknown): ValidationResult<EventForm> {
    const result = EventForm.decode(data);
    return toValidationResult(result);
}
