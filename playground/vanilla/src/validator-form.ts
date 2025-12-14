/**
 * Validator form model for E2E testing.
 * Tests string, number, array, and date validators with real form validation.
 */

import { Result } from 'macroforge/utils';

/** @derive(Deserialize) */
export class UserRegistrationForm {
    /** @serde({ validate: ["email"] }) */
    email: string;

    /** @serde({ validate: ["minLength(8)", "maxLength(50)"] }) */
    password: string;

    /** @serde({ validate: ["minLength(3)", "maxLength(20)", "lowercase", "pattern(\"^[a-z][a-z0-9_]+$\")"] }) */
    username: string;

    /** @serde({ validate: ["int", "between(18, 120)"] }) */
    age: number;

    /** @serde({ validate: ["url"] }) */
    website: string;
}

/** @derive(Deserialize) */
export class ProductForm {
    /** @serde({ validate: ["nonEmpty", "maxLength(100)"] }) */
    name: string;

    /** @serde({ validate: ["positive", "lessThan(1000000)"] }) */
    price: number;

    /** @serde({ validate: ["int", "nonNegative"] }) */
    quantity: number;

    /** @serde({ validate: ["minItems(1)", "maxItems(5)"] }) */
    tags: string[];

    /** @serde({ validate: ["uuid"] }) */
    sku: string;
}

/** @derive(Deserialize) */
export class EventForm {
    /** @serde({ validate: ["nonEmpty", "trimmed"] }) */
    title: string;

    /** @serde({ validate: ["validDate", "greaterThanDate(\"2020-01-01\")"] }) */
    startDate: Date;

    /** @serde({ validate: ["validDate"] }) */
    endDate: Date;

    /** @serde({ validate: ["int", "between(1, 1000)"] }) */
    maxAttendees: number;
}

// Type for validation result
export type ValidationResult<T> = {
    success: boolean;
    data?: T;
    errors?: string[];
};

// Helper to convert Result to ValidationResult
// The Result type uses static methods
export function toValidationResult<T>(result: any): ValidationResult<T> {
    if (Result.isOk(result)) {
        return { success: true, data: Result.unwrap(result) };
    } else {
        // Errors are now structured as {field, message} objects
        const errors = Result.unwrapErr(result);
        return {
            success: false,
            errors: errors.map((e: any) => (typeof e === 'string' ? e : e.message))
        };
    }
}

// Form validation functions
export function validateUserRegistration(data: unknown): ValidationResult<UserRegistrationForm> {
    const result = (UserRegistrationForm as any).fromStringifiedJSON(JSON.stringify(data));
    return toValidationResult(result);
}

export function validateProduct(data: unknown): ValidationResult<ProductForm> {
    const result = (ProductForm as any).fromStringifiedJSON(JSON.stringify(data));
    return toValidationResult(result);
}

export function validateEvent(data: unknown): ValidationResult<EventForm> {
    const result = (EventForm as any).fromStringifiedJSON(JSON.stringify(data));
    return toValidationResult(result);
}
