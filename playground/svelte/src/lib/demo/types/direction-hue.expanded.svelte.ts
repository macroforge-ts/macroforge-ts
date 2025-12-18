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

export interface DirectionHue {
    bearing: number;
    hue: number;
}

export function directionHueDefaultValue(): DirectionHue {
    return { bearing: 0, hue: 0 } as DirectionHue;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function directionHueSerialize(
    value: DirectionHue
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(directionHueSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function directionHueSerializeWithContext(
    value: DirectionHue,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'DirectionHue', __id };
    result['bearing'] = value.bearing;
    result['hue'] = value.hue;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function directionHueDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Exit.Exit<Array<{ field: string; message: string }>, DirectionHue> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = directionHueDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Exit.fail([
                {
                    field: '_root',
                    message: 'DirectionHue.deserialize: root cannot be a forward reference'
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
export function directionHueDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): DirectionHue | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'DirectionHue.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('bearing' in obj)) {
        errors.push({ field: 'bearing', message: 'missing required field' });
    }
    if (!('hue' in obj)) {
        errors.push({ field: 'hue', message: 'missing required field' });
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
        const __raw_bearing = obj['bearing'] as number;
        instance.bearing = __raw_bearing;
    }
    {
        const __raw_hue = obj['hue'] as number;
        instance.hue = __raw_hue;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as DirectionHue;
}
export function directionHueValidateField<K extends keyof DirectionHue>(
    field: K,
    value: DirectionHue[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function directionHueValidateFields(
    partial: Partial<DirectionHue>
): Array<{ field: string; message: string }> {
    return [];
}
export function directionHueHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'bearing' in o && 'hue' in o;
}
export function directionHueIs(obj: unknown): obj is DirectionHue {
    if (!directionHueHasShape(obj)) {
        return false;
    }
    const result = directionHueDeserialize(obj);
    return Exit.isSuccess(result);
}

/** Nested error structure matching the data shape */ export type DirectionHueErrors = {
    _errors: Option<Array<string>>;
    bearing: Option<Array<string>>;
    hue: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type DirectionHueTainted = {
    bearing: Option<boolean>;
    hue: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface DirectionHueFieldControllers {
    readonly bearing: FieldController<number>;
    readonly hue: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface DirectionHueGigaform {
    readonly data: DirectionHue;
    readonly errors: DirectionHueErrors;
    readonly tainted: DirectionHueTainted;
    readonly fields: DirectionHueFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, DirectionHue>;
    reset(overrides?: Partial<DirectionHue>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function directionHueCreateForm(overrides?: Partial<DirectionHue>): DirectionHueGigaform {
    let data = $state({ ...directionHueDefaultValue(), ...overrides });
    let errors = $state<DirectionHueErrors>({
        _errors: optionNone(),
        bearing: optionNone(),
        hue: optionNone()
    });
    let tainted = $state<DirectionHueTainted>({ bearing: optionNone(), hue: optionNone() });
    const fields: DirectionHueFieldControllers = {
        bearing: {
            path: ['bearing'] as const,
            name: 'bearing',
            constraints: { required: true },
            get: () => data.bearing,
            set: (value: number) => {
                data.bearing = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.bearing,
            setError: (value: Option<Array<string>>) => {
                errors.bearing = value;
            },
            getTainted: () => tainted.bearing,
            setTainted: (value: Option<boolean>) => {
                tainted.bearing = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = directionHueValidateField('bearing', data.bearing);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        hue: {
            path: ['hue'] as const,
            name: 'hue',
            constraints: { required: true },
            get: () => data.hue,
            set: (value: number) => {
                data.hue = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.hue,
            setError: (value: Option<Array<string>>) => {
                errors.hue = value;
            },
            getTainted: () => tainted.hue,
            setTainted: (value: Option<boolean>) => {
                tainted.hue = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = directionHueValidateField('hue', data.hue);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, DirectionHue> {
        return toExit(directionHueDeserialize(data));
    }
    function reset(newOverrides?: Partial<DirectionHue>): void {
        data = { ...directionHueDefaultValue(), ...newOverrides };
        errors = { _errors: optionNone(), bearing: optionNone(), hue: optionNone() };
        tainted = { bearing: optionNone(), hue: optionNone() };
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
export function directionHueFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, DirectionHue> {
    const obj: Record<string, unknown> = {};
    {
        const bearingStr = formData.get('bearing');
        obj.bearing = bearingStr ? parseFloat(bearingStr as string) : 0;
        if (obj.bearing !== undefined && isNaN(obj.bearing as number)) obj.bearing = 0;
    }
    {
        const hueStr = formData.get('hue');
        obj.hue = hueStr ? parseFloat(hueStr as string) : 0;
        if (obj.hue !== undefined && isNaN(obj.hue as number)) obj.hue = 0;
    }
    return toExit(directionHueDeserialize(obj));
}

export const DirectionHue = {
    defaultValue: directionHueDefaultValue,
    serialize: directionHueSerialize,
    serializeWithContext: directionHueSerializeWithContext,
    deserialize: directionHueDeserialize,
    deserializeWithContext: directionHueDeserializeWithContext,
    validateFields: directionHueValidateFields,
    hasShape: directionHueHasShape,
    is: directionHueIs,
    createForm: directionHueCreateForm,
    fromFormData: directionHueFromFormData
} as const;
