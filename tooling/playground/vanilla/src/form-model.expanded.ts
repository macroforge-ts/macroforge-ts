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

    
    tags: Array<string>;

    metadata: Record<string, unknown> | null;

    constructor(
        memo: string | null,
        username: string,
        description: string,
        tags: Array<string> = [],
        metadata: Record<string, unknown> | null = null
    ) {
        this.memo = memo;
        this.username = username;
        this.description = description;
        this.tags = tags;
        this.metadata = metadata;
    }

        static fieldMetadata(): Array<{
    name: string;
    label: string;
    optional: boolean;
    array: boolean;
    type: string;
}> {
    const arr: Array<{
        name: string;
        label: string;
        optional: boolean;
        array: boolean;
        type: string;
    }> = [];
    arr.push({
        name: `${"memo"}`,
        label: `${"Memo Notes"}`,
        optional: true,
        array: false,
        type: `${"string | null"}`
    });
    arr.push({
        name: `${"username"}`,
        label: `${"User Name"}`,
        optional: false,
        array: false,
        type: `${"string"}`
    });
    arr.push({
        name: `${"description"}`,
        label: `${"Description Text"}`,
        optional: false,
        array: false,
        type: `${"string"}`
    });
    arr.push({
        name: `${"tags"}`,
        label: `${"Tags"}`,
        optional: false,
        array: true,
        type: `${"Array<string>"}`
    });
    arr.push({
        name: `${"metadata"}`,
        label: `${"Metadata"}`,
        optional: true,
        array: false,
        type: `${"Record<string, unknown> | null"}`
    });
    return arr;
    result[`${"memo"}`] = this.memo;
    result[`${"description"}`] = this.description;
    result[`${"tags"}`] = this.tags;
    return result;
    result.tags = [
        ...(this.tags ?? [])
    ] as Partial<FormModel>[`${tags}`];
    return result;
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