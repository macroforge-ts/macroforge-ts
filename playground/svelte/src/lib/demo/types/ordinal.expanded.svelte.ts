import { SerializeContext } from 'macroforge/serde';
import { Exit } from 'macroforge/utils/effect';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Ordinal {
    north: number;
    northeast: number;
    east: number;
    southeast: number;
    south: number;
    southwest: number;
    west: number;
    northwest: number;
}

export function ordinalDefaultValue(): Ordinal {
    return {
        north: 0,
        northeast: 0,
        east: 0,
        southeast: 0,
        south: 0,
        southwest: 0,
        west: 0,
        northwest: 0
    } as Ordinal;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function ordinalSerialize(
    value: Ordinal
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(ordinalSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function ordinalSerializeWithContext(
    value: Ordinal,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Ordinal', __id };
    result['north'] = value.north;
    result['northeast'] = value.northeast;
    result['east'] = value.east;
    result['southeast'] = value.southeast;
    result['south'] = value.south;
    result['southwest'] = value.southwest;
    result['west'] = value.west;
    result['northwest'] = value.northwest;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function ordinalDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Exit.Exit<Array<{ field: string; message: string }>, Ordinal> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = ordinalDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Exit.fail([
                {
                    field: '_root',
                    message: 'Ordinal.deserialize: root cannot be a forward reference'
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Exit.succeed(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Exit.fail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Exit.fail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function ordinalDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): Ordinal | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Ordinal.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('north' in obj)) {
        errors.push({ field: 'north', message: 'missing required field' });
    }
    if (!('northeast' in obj)) {
        errors.push({ field: 'northeast', message: 'missing required field' });
    }
    if (!('east' in obj)) {
        errors.push({ field: 'east', message: 'missing required field' });
    }
    if (!('southeast' in obj)) {
        errors.push({ field: 'southeast', message: 'missing required field' });
    }
    if (!('south' in obj)) {
        errors.push({ field: 'south', message: 'missing required field' });
    }
    if (!('southwest' in obj)) {
        errors.push({ field: 'southwest', message: 'missing required field' });
    }
    if (!('west' in obj)) {
        errors.push({ field: 'west', message: 'missing required field' });
    }
    if (!('northwest' in obj)) {
        errors.push({ field: 'northwest', message: 'missing required field' });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
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
        const __raw_northeast = obj['northeast'] as number;
        instance.northeast = __raw_northeast;
    }
    {
        const __raw_east = obj['east'] as number;
        instance.east = __raw_east;
    }
    {
        const __raw_southeast = obj['southeast'] as number;
        instance.southeast = __raw_southeast;
    }
    {
        const __raw_south = obj['south'] as number;
        instance.south = __raw_south;
    }
    {
        const __raw_southwest = obj['southwest'] as number;
        instance.southwest = __raw_southwest;
    }
    {
        const __raw_west = obj['west'] as number;
        instance.west = __raw_west;
    }
    {
        const __raw_northwest = obj['northwest'] as number;
        instance.northwest = __raw_northwest;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Ordinal;
}
export function ordinalValidateField<K extends keyof Ordinal>(
    field: K,
    value: Ordinal[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function ordinalValidateFields(
    partial: Partial<Ordinal>
): Array<{ field: string; message: string }> {
    return [];
}
export function ordinalHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'north' in o &&
        'northeast' in o &&
        'east' in o &&
        'southeast' in o &&
        'south' in o &&
        'southwest' in o &&
        'west' in o &&
        'northwest' in o
    );
}
export function ordinalIs(obj: unknown): obj is Ordinal {
    if (!ordinalHasShape(obj)) {
        return false;
    }
    const result = ordinalDeserialize(obj);
    return Exit.isSuccess(result);
}

/** Nested error structure matching the data shape */ export type OrdinalErrors = {
    _errors: Option<Array<string>>;
    north: Option<Array<string>>;
    northeast: Option<Array<string>>;
    east: Option<Array<string>>;
    southeast: Option<Array<string>>;
    south: Option<Array<string>>;
    southwest: Option<Array<string>>;
    west: Option<Array<string>>;
    northwest: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type OrdinalTainted = {
    north: Option<boolean>;
    northeast: Option<boolean>;
    east: Option<boolean>;
    southeast: Option<boolean>;
    south: Option<boolean>;
    southwest: Option<boolean>;
    west: Option<boolean>;
    northwest: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface OrdinalFieldControllers {
    readonly north: FieldController<number>;
    readonly northeast: FieldController<number>;
    readonly east: FieldController<number>;
    readonly southeast: FieldController<number>;
    readonly south: FieldController<number>;
    readonly southwest: FieldController<number>;
    readonly west: FieldController<number>;
    readonly northwest: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface OrdinalGigaform {
    readonly data: Ordinal;
    readonly errors: OrdinalErrors;
    readonly tainted: OrdinalTainted;
    readonly fields: OrdinalFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, Ordinal>;
    reset(overrides?: Partial<Ordinal>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function ordinalCreateForm(overrides?: Partial<Ordinal>): OrdinalGigaform {
    let data = $state({ ...ordinalDefaultValue(), ...overrides });
    let errors = $state<OrdinalErrors>({
        _errors: optionNone(),
        north: optionNone(),
        northeast: optionNone(),
        east: optionNone(),
        southeast: optionNone(),
        south: optionNone(),
        southwest: optionNone(),
        west: optionNone(),
        northwest: optionNone()
    });
    let tainted = $state<OrdinalTainted>({
        north: optionNone(),
        northeast: optionNone(),
        east: optionNone(),
        southeast: optionNone(),
        south: optionNone(),
        southwest: optionNone(),
        west: optionNone(),
        northwest: optionNone()
    });
    const fields: OrdinalFieldControllers = {
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
            setError: (value: Option<Array<string>>) => {
                errors.north = value;
            },
            getTainted: () => tainted.north,
            setTainted: (value: Option<boolean>) => {
                tainted.north = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = ordinalValidateField('north', data.north);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        northeast: {
            path: ['northeast'] as const,
            name: 'northeast',
            constraints: { required: true },
            get: () => data.northeast,
            set: (value: number) => {
                data.northeast = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.northeast,
            setError: (value: Option<Array<string>>) => {
                errors.northeast = value;
            },
            getTainted: () => tainted.northeast,
            setTainted: (value: Option<boolean>) => {
                tainted.northeast = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = ordinalValidateField('northeast', data.northeast);
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
            setError: (value: Option<Array<string>>) => {
                errors.east = value;
            },
            getTainted: () => tainted.east,
            setTainted: (value: Option<boolean>) => {
                tainted.east = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = ordinalValidateField('east', data.east);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        southeast: {
            path: ['southeast'] as const,
            name: 'southeast',
            constraints: { required: true },
            get: () => data.southeast,
            set: (value: number) => {
                data.southeast = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.southeast,
            setError: (value: Option<Array<string>>) => {
                errors.southeast = value;
            },
            getTainted: () => tainted.southeast,
            setTainted: (value: Option<boolean>) => {
                tainted.southeast = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = ordinalValidateField('southeast', data.southeast);
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
            setError: (value: Option<Array<string>>) => {
                errors.south = value;
            },
            getTainted: () => tainted.south,
            setTainted: (value: Option<boolean>) => {
                tainted.south = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = ordinalValidateField('south', data.south);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        southwest: {
            path: ['southwest'] as const,
            name: 'southwest',
            constraints: { required: true },
            get: () => data.southwest,
            set: (value: number) => {
                data.southwest = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.southwest,
            setError: (value: Option<Array<string>>) => {
                errors.southwest = value;
            },
            getTainted: () => tainted.southwest,
            setTainted: (value: Option<boolean>) => {
                tainted.southwest = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = ordinalValidateField('southwest', data.southwest);
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
            setError: (value: Option<Array<string>>) => {
                errors.west = value;
            },
            getTainted: () => tainted.west,
            setTainted: (value: Option<boolean>) => {
                tainted.west = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = ordinalValidateField('west', data.west);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        northwest: {
            path: ['northwest'] as const,
            name: 'northwest',
            constraints: { required: true },
            get: () => data.northwest,
            set: (value: number) => {
                data.northwest = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.northwest,
            setError: (value: Option<Array<string>>) => {
                errors.northwest = value;
            },
            getTainted: () => tainted.northwest,
            setTainted: (value: Option<boolean>) => {
                tainted.northwest = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = ordinalValidateField('northwest', data.northwest);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, Ordinal> {
        return toExit(ordinalDeserialize(data));
    }
    function reset(newOverrides?: Partial<Ordinal>): void {
        data = { ...ordinalDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            north: optionNone(),
            northeast: optionNone(),
            east: optionNone(),
            southeast: optionNone(),
            south: optionNone(),
            southwest: optionNone(),
            west: optionNone(),
            northwest: optionNone()
        };
        tainted = {
            north: optionNone(),
            northeast: optionNone(),
            east: optionNone(),
            southeast: optionNone(),
            south: optionNone(),
            southwest: optionNone(),
            west: optionNone(),
            northwest: optionNone()
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
export function ordinalFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, Ordinal> {
    const obj: Record<string, unknown> = {};
    {
        const northStr = formData.get('north');
        obj.north = northStr ? parseFloat(northStr as string) : 0;
        if (obj.north !== undefined && isNaN(obj.north as number)) obj.north = 0;
    }
    {
        const northeastStr = formData.get('northeast');
        obj.northeast = northeastStr ? parseFloat(northeastStr as string) : 0;
        if (obj.northeast !== undefined && isNaN(obj.northeast as number)) obj.northeast = 0;
    }
    {
        const eastStr = formData.get('east');
        obj.east = eastStr ? parseFloat(eastStr as string) : 0;
        if (obj.east !== undefined && isNaN(obj.east as number)) obj.east = 0;
    }
    {
        const southeastStr = formData.get('southeast');
        obj.southeast = southeastStr ? parseFloat(southeastStr as string) : 0;
        if (obj.southeast !== undefined && isNaN(obj.southeast as number)) obj.southeast = 0;
    }
    {
        const southStr = formData.get('south');
        obj.south = southStr ? parseFloat(southStr as string) : 0;
        if (obj.south !== undefined && isNaN(obj.south as number)) obj.south = 0;
    }
    {
        const southwestStr = formData.get('southwest');
        obj.southwest = southwestStr ? parseFloat(southwestStr as string) : 0;
        if (obj.southwest !== undefined && isNaN(obj.southwest as number)) obj.southwest = 0;
    }
    {
        const westStr = formData.get('west');
        obj.west = westStr ? parseFloat(westStr as string) : 0;
        if (obj.west !== undefined && isNaN(obj.west as number)) obj.west = 0;
    }
    {
        const northwestStr = formData.get('northwest');
        obj.northwest = northwestStr ? parseFloat(northwestStr as string) : 0;
        if (obj.northwest !== undefined && isNaN(obj.northwest as number)) obj.northwest = 0;
    }
    return toExit(ordinalDeserialize(obj));
}

export const Ordinal = {
    defaultValue: ordinalDefaultValue,
    serialize: ordinalSerialize,
    serializeWithContext: ordinalSerializeWithContext,
    deserialize: ordinalDeserialize,
    deserializeWithContext: ordinalDeserializeWithContext,
    validateFields: ordinalValidateFields,
    hasShape: ordinalHasShape,
    is: ordinalIs,
    createForm: ordinalCreateForm,
    fromFormData: ordinalFromFormData
} as const;
