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

export class FormModel {
    memo: string | null;

    username: string;

    description: string;

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
    /** Returns field metadata for inspection  */

    static fieldMetadata(): Array<{
        name: string;
        label: string;
        optional: boolean;
        array: boolean;
        type: string;
    }> {
        return [
            {
                name: 'memo',
                label: 'Memo Notes',
                optional: true,
                array: false,
                type: 'string | null'
            },
            {
                name: 'username',
                label: 'User Name',
                optional: false,
                array: false,
                type: 'string'
            },
            {
                name: 'description',
                label: 'Description Text',
                optional: false,
                array: false,
                type: 'string'
            },
            {
                name: 'tags',
                label: 'Tags',
                optional: false,
                array: true,
                type: 'string[]'
            },
            {
                name: 'metadata',
                label: 'Metadata',
                optional: true,
                array: false,
                type: 'Record<string, unknown> | null'
            }
        ];
    }
    /** Returns only the inspectable fields  */

    getInspectableFields(): Record<string, unknown> {
        const result: Record<string, unknown> = {};
        result['memo'] = this.memo;
        result['description'] = this.description;
        result['tags'] = this.tags;
        return result;
    }
    /** Deep clones only array fields  */

    cloneArrayFields(): Partial<FormModel> {
        const result: Partial<FormModel> = {};
        result.tags = [...(this.tags ?? [])] as typeof this.tags;
        return result;
    }
    /** Returns non-null field count  */

    countPopulatedFields(): number {
        let count = 0;
        if (this.memo != null) count++;
        count++;
        count++;
        count++;
        if (this.metadata != null) count++;
        return count;
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
