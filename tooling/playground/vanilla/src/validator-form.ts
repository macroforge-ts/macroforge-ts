/**
 * Validator form model for E2E testing.
 * Tests string, number, array, and date validators with real form validation,
 * and newtypes and a validated union arm as field types.
 */

import type { Contact } from './validators/union-arm-validator-tests.ts';
import type { Cents, Meters, Username } from './validators/newtype-validator-tests.ts';

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

/** @derive(Decode) */
export class MeasurementForm {
    runner: Username;

    distance: Meters;

    cents: Cents;

    contact: Contact;
}

// Type for validation result (matches macroforge's vanilla return type)
export type ValidationResult<T> =
    | {
        success: true;
        value: T;
    }
    | { success: false; errors: Array<{ field: string; message: string }> };

// Form validation functions
export function validateUserRegistration(
    data: unknown
): ValidationResult<UserRegistrationForm> {
    const result = UserRegistrationForm.decode(JSON.stringify(data));
    return result;
}

export function validateProduct(data: unknown): ValidationResult<ProductForm> {
    const result = ProductForm.decode(JSON.stringify(data));
    return result;
}

export function validateEvent(data: unknown): ValidationResult<EventForm> {
    const result = EventForm.decode(JSON.stringify(data));
    return result;
}

export function validateMeasurement(data: unknown): ValidationResult<MeasurementForm> {
    return MeasurementForm.decode(data);
}
