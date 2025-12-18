import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Color {
    red: number;
    green: number;
    blue: number;
}

export function colorDefaultValue(): Color {
    return { red: 0, green: 0, blue: 0 } as Color;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function colorSerialize(
    value: Color
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(colorSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function colorSerializeWithContext(
    value: Color,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Color', __id };
    result['red'] = value.red;
    result['green'] = value.green;
    result['blue'] = value.blue;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function colorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Color> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = colorDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                { field: '_root', message: 'Color.deserialize: root cannot be a forward reference' }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return __mf_exitSucceed(resultOrRef);
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return __mf_exitFail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return __mf_exitFail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function colorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Color | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Color.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('red' in obj)) {
        errors.push({ field: 'red', message: 'missing required field' });
    }
    if (!('green' in obj)) {
        errors.push({ field: 'green', message: 'missing required field' });
    }
    if (!('blue' in obj)) {
        errors.push({ field: 'blue', message: 'missing required field' });
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
        const __raw_red = obj['red'] as number;
        instance.red = __raw_red;
    }
    {
        const __raw_green = obj['green'] as number;
        instance.green = __raw_green;
    }
    {
        const __raw_blue = obj['blue'] as number;
        instance.blue = __raw_blue;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Color;
}
export function colorValidateField<K extends keyof Color>(
    field: K,
    value: Color[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function colorValidateFields(
    partial: Partial<Color>
): Array<{ field: string; message: string }> {
    return [];
}
export function colorHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'red' in o && 'green' in o && 'blue' in o;
}
export function colorIs(obj: unknown): obj is Color {
    if (!colorHasShape(obj)) {
        return false;
    }
    const result = colorDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type ColorErrors = {
    _errors: Option<Array<string>>;
    red: Option<Array<string>>;
    green: Option<Array<string>>;
    blue: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type ColorTainted = {
    red: Option<boolean>;
    green: Option<boolean>;
    blue: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface ColorFieldControllers {
    readonly red: FieldController<number>;
    readonly green: FieldController<number>;
    readonly blue: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface ColorGigaform {
    readonly data: Color;
    readonly errors: ColorErrors;
    readonly tainted: ColorTainted;
    readonly fields: ColorFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, Color>;
    reset(overrides?: Partial<Color>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function colorCreateForm(overrides?: Partial<Color>): ColorGigaform {
    let data = $state({ ...colorDefaultValue(), ...overrides });
    let errors = $state<ColorErrors>({
        _errors: optionNone(),
        red: optionNone(),
        green: optionNone(),
        blue: optionNone()
    });
    let tainted = $state<ColorTainted>({
        red: optionNone(),
        green: optionNone(),
        blue: optionNone()
    });
    const fields: ColorFieldControllers = {
        red: {
            path: ['red'] as const,
            name: 'red',
            constraints: { required: true },
            get: () => data.red,
            set: (value: number) => {
                data.red = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.red,
            setError: (value: Option<Array<string>>) => {
                errors.red = value;
            },
            getTainted: () => tainted.red,
            setTainted: (value: Option<boolean>) => {
                tainted.red = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = colorValidateField('red', data.red);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        green: {
            path: ['green'] as const,
            name: 'green',
            constraints: { required: true },
            get: () => data.green,
            set: (value: number) => {
                data.green = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.green,
            setError: (value: Option<Array<string>>) => {
                errors.green = value;
            },
            getTainted: () => tainted.green,
            setTainted: (value: Option<boolean>) => {
                tainted.green = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = colorValidateField('green', data.green);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        blue: {
            path: ['blue'] as const,
            name: 'blue',
            constraints: { required: true },
            get: () => data.blue,
            set: (value: number) => {
                data.blue = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.blue,
            setError: (value: Option<Array<string>>) => {
                errors.blue = value;
            },
            getTainted: () => tainted.blue,
            setTainted: (value: Option<boolean>) => {
                tainted.blue = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = colorValidateField('blue', data.blue);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, Color> {
        return colorDeserialize(data);
    }
    function reset(newOverrides?: Partial<Color>): void {
        data = { ...colorDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            red: optionNone(),
            green: optionNone(),
            blue: optionNone()
        };
        tainted = { red: optionNone(), green: optionNone(), blue: optionNone() };
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
export function colorFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, Color> {
    const obj: Record<string, unknown> = {};
    {
        const redStr = formData.get('red');
        obj.red = redStr ? parseFloat(redStr as string) : 0;
        if (obj.red !== undefined && isNaN(obj.red as number)) obj.red = 0;
    }
    {
        const greenStr = formData.get('green');
        obj.green = greenStr ? parseFloat(greenStr as string) : 0;
        if (obj.green !== undefined && isNaN(obj.green as number)) obj.green = 0;
    }
    {
        const blueStr = formData.get('blue');
        obj.blue = blueStr ? parseFloat(blueStr as string) : 0;
        if (obj.blue !== undefined && isNaN(obj.blue as number)) obj.blue = 0;
    }
    return colorDeserialize(obj);
}

export const Color = {
    defaultValue: colorDefaultValue,
    serialize: colorSerialize,
    serializeWithContext: colorSerializeWithContext,
    deserialize: colorDeserialize,
    deserializeWithContext: colorDeserializeWithContext,
    validateFields: colorValidateFields,
    hasShape: colorHasShape,
    is: colorIs,
    createForm: colorCreateForm,
    fromFormData: colorFromFormData
} as const;
