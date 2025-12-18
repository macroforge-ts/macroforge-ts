import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option as __gf_Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Cardinal {
    north: number;
    east: number;
    south: number;
    west: number;
}

export function cardinalDefaultValue(): Cardinal {
    return { north: 0, east: 0, south: 0, west: 0 } as Cardinal;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function cardinalSerialize(
    value: Cardinal
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(cardinalSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function cardinalSerializeWithContext(
    value: Cardinal,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Cardinal', __id };
    result['north'] = value.north;
    result['east'] = value.east;
    result['south'] = value.south;
    result['west'] = value.west;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function cardinalDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: Cardinal }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = cardinalDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: '_root',
                        message: 'Cardinal.deserialize: root cannot be a forward reference'
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return { success: true, value: resultOrRef };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return { success: false, errors: e.errors };
        }
        const message = e instanceof Error ? e.message : String(e);
        return { success: false, errors: [{ field: '_root', message }] };
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function cardinalDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Cardinal | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Cardinal.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('north' in obj)) {
        errors.push({ field: 'north', message: 'missing required field' });
    }
    if (!('east' in obj)) {
        errors.push({ field: 'east', message: 'missing required field' });
    }
    if (!('south' in obj)) {
        errors.push({ field: 'south', message: 'missing required field' });
    }
    if (!('west' in obj)) {
        errors.push({ field: 'west', message: 'missing required field' });
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_north = obj['north'] as number;
        instance.north = __raw_north;
    }
    {
        const __raw_east = obj['east'] as number;
        instance.east = __raw_east;
    }
    {
        const __raw_south = obj['south'] as number;
        instance.south = __raw_south;
    }
    {
        const __raw_west = obj['west'] as number;
        instance.west = __raw_west;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Cardinal;
}
export function cardinalValidateField<K extends keyof Cardinal>(
    _field: K,
    _value: Cardinal[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function cardinalValidateFields(
    _partial: Partial<Cardinal>
): Array<{ field: string; message: string }> {
    return [];
}
export function cardinalHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'north' in o && 'east' in o && 'south' in o && 'west' in o;
}
export function cardinalIs(obj: unknown): obj is Cardinal {
    if (!cardinalHasShape(obj)) {
        return false;
    }
    const result = cardinalDeserialize(obj);
    return result.success;
}

/** Nested error structure matching the data shape */ export type CardinalErrors = {
    _errors: __gf_Option<Array<string>>;
    north: __gf_Option<Array<string>>;
    east: __gf_Option<Array<string>>;
    south: __gf_Option<Array<string>>;
    west: __gf_Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type CardinalTainted = {
    north: __gf_Option<boolean>;
    east: __gf_Option<boolean>;
    south: __gf_Option<boolean>;
    west: __gf_Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface CardinalFieldControllers {
    readonly north: FieldController<number>;
    readonly east: FieldController<number>;
    readonly south: FieldController<number>;
    readonly west: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface CardinalGigaform {
    readonly data: Cardinal;
    readonly errors: CardinalErrors;
    readonly tainted: CardinalTainted;
    readonly fields: CardinalFieldControllers;
    validate(): Exit<Cardinal, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Cardinal>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function cardinalCreateForm(overrides?: Partial<Cardinal>): CardinalGigaform {
    let data = $state({ ...cardinalDefaultValue(), ...overrides });
    let errors = $state<CardinalErrors>({
        _errors: optionNone(),
        north: optionNone(),
        east: optionNone(),
        south: optionNone(),
        west: optionNone()
    });
    let tainted = $state<CardinalTainted>({
        north: optionNone(),
        east: optionNone(),
        south: optionNone(),
        west: optionNone()
    });
    const fields: CardinalFieldControllers = {
        north: {
            path: ['north'] as const,
            name: 'north',
            constraints: { required: true },
            get: () => data.north,
            set: (value: number) => {
                data.north = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.north,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.north = value;
            },
            getTainted: () => tainted.north,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.north = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = cardinalValidateField('north', data.north);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        east: {
            path: ['east'] as const,
            name: 'east',
            constraints: { required: true },
            get: () => data.east,
            set: (value: number) => {
                data.east = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.east,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.east = value;
            },
            getTainted: () => tainted.east,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.east = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = cardinalValidateField('east', data.east);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        south: {
            path: ['south'] as const,
            name: 'south',
            constraints: { required: true },
            get: () => data.south,
            set: (value: number) => {
                data.south = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.south,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.south = value;
            },
            getTainted: () => tainted.south,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.south = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = cardinalValidateField('south', data.south);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        west: {
            path: ['west'] as const,
            name: 'west',
            constraints: { required: true },
            get: () => data.west,
            set: (value: number) => {
                data.west = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.west,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.west = value;
            },
            getTainted: () => tainted.west,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.west = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = cardinalValidateField('west', data.west);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Cardinal, Array<{ field: string; message: string }>> {
        return toExit(cardinalDeserialize(data));
    }
    function reset(newOverrides?: Partial<Cardinal>): void {
        data = { ...cardinalDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            north: optionNone(),
            east: optionNone(),
            south: optionNone(),
            west: optionNone()
        };
        tainted = {
            north: optionNone(),
            east: optionNone(),
            south: optionNone(),
            west: optionNone()
        };
    }
    return {
        get data() {
            return data;
        },
        set data(v) {
            data = v;
        },
        get errors() {
            return errors;
        },
        set errors(v) {
            errors = v;
        },
        get tainted() {
            return tainted;
        },
        set tainted(v) {
            tainted = v;
        },
        fields,
        validate,
        reset
    };
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */
export function cardinalFromFormData(
    formData: FormData
): Exit<Cardinal, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const northStr = formData.get('north');
        obj.north = northStr ? parseFloat(northStr as string) : 0;
        if (obj.north !== undefined && isNaN(obj.north as number)) obj.north = 0;
    }
    {
        const eastStr = formData.get('east');
        obj.east = eastStr ? parseFloat(eastStr as string) : 0;
        if (obj.east !== undefined && isNaN(obj.east as number)) obj.east = 0;
    }
    {
        const southStr = formData.get('south');
        obj.south = southStr ? parseFloat(southStr as string) : 0;
        if (obj.south !== undefined && isNaN(obj.south as number)) obj.south = 0;
    }
    {
        const westStr = formData.get('west');
        obj.west = westStr ? parseFloat(westStr as string) : 0;
        if (obj.west !== undefined && isNaN(obj.west as number)) obj.west = 0;
    }
    return toExit(cardinalDeserialize(obj));
}

export const Cardinal = {
    defaultValue: cardinalDefaultValue,
    serialize: cardinalSerialize,
    serializeWithContext: cardinalSerializeWithContext,
    deserialize: cardinalDeserialize,
    deserializeWithContext: cardinalDeserializeWithContext,
    validateFields: cardinalValidateFields,
    hasShape: cardinalHasShape,
    is: cardinalIs,
    createForm: cardinalCreateForm,
    fromFormData: cardinalFromFormData
} as const;
