/** import macro { Inspect } from "@playground/macro"; */

/**
 * Example class using the Inspect macro to test macroforge edge cases.
 *
 * Tests:
 * - Generic type parameters (T | null for optional fields)
 * - Optional fields
 * - Array types
 * - Conditional logic in templates
 * - Field decorators with arguments (@inspect, @label)
 * - Static methods (fieldMetadata)
 * - Instance methods (getInspectableFields, cloneArrayFields, countPopulatedFields)
 */
/** @derive(Inspect) */
export class FormModel {
    /** @inspect @label("Memo Notes") */
    memo: string | null;

    /** @label("User Name") */
    username: string;

    /** @inspect @label("Description Text") */
    description: string;

    /** @inspect */
    tags: string[];

    metadata: Record<string, unknown> | null;

    constructor(
        memo: string | null,
        username: string,
        description: string,
        tags: string[] = [],
        metadata: Record<string, unknown> | null = null
    ) {
        this.memo = memo;
        this.username = username;
        this.description = description;
        this.tags = tags;
        this.metadata = metadata;
    }
}

// Test the generated methods
const model = new FormModel('Test memo', 'johndoe', 'A test description', ['tag1', 'tag2']);

// Test static method - fieldMetadata()
const metadata = FormModel.fieldMetadata();
console.log('Field metadata:', metadata);

// Test instance method - getInspectableFields()
const inspectable = model.getInspectableFields();
console.log('Inspectable fields:', inspectable);

// Test instance method - cloneArrayFields()
const clonedArrays = model.cloneArrayFields();
console.log('Cloned arrays:', clonedArrays);

// Test instance method - countPopulatedFields()
const populatedCount = model.countPopulatedFields();
console.log('Populated field count:', populatedCount);
