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

export interface Gradient {
    startHue: number;
}

export function gradientDefaultValue(): Gradient {
    return { startHue: 0 } as Gradient;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function gradientSerialize(
    value: Gradient
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(gradientSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function gradientSerializeWithContext(
    value: Gradient,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Gradient', __id };
    result['startHue'] = value.startHue;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function gradientDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Exit.Exit<Array<{ field: string; message: string }>, Gradient> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = gradientDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Exit.fail([
                {
                    field: '_root',
                    message: 'Gradient.deserialize: root cannot be a forward reference'
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
export function gradientDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): Gradient | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Gradient.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('startHue' in obj)) {
        errors.push({ field: 'startHue', message: 'missing required field' });
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
        const __raw_startHue = obj['startHue'] as number;
        instance.startHue = __raw_startHue;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Gradient;
}
export function gradientValidateField<K extends keyof Gradient>(
    field: K,
    value: Gradient[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function gradientValidateFields(
    partial: Partial<Gradient>
): Array<{ field: string; message: string }> {
    return [];
}
export function gradientHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'startHue' in o;
}
export function gradientIs(obj: unknown): obj is Gradient {
    if (!gradientHasShape(obj)) {
        return false;
    }
    const result = gradientDeserialize(obj);
    return Exit.isSuccess(result);
}

/** Nested error structure matching the data shape */ export type GradientErrors = {
    _errors: Option<Array<string>>;
    startHue: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type GradientTainted = {
    startHue: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface GradientFieldControllers {
    readonly startHue: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GradientGigaform {
    readonly data: Gradient;
    readonly errors: GradientErrors;
    readonly tainted: GradientTainted;
    readonly fields: GradientFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, Gradient>;
    reset(overrides?: Partial<Gradient>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function gradientCreateForm(overrides?: Partial<Gradient>): GradientGigaform {
    let data = $state({ ...gradientDefaultValue(), ...overrides });
    let errors = $state<GradientErrors>({ _errors: optionNone(), startHue: optionNone() });
    let tainted = $state<GradientTainted>({ startHue: optionNone() });
    const fields: GradientFieldControllers = {
        startHue: {
            path: ['startHue'] as const,
            name: 'startHue',
            constraints: { required: true },
            get: () => data.startHue,
            set: (value: number) => {
                data.startHue = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.startHue,
            setError: (value: Option<Array<string>>) => {
                errors.startHue = value;
            },
            getTainted: () => tainted.startHue,
            setTainted: (value: Option<boolean>) => {
                tainted.startHue = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = gradientValidateField('startHue', data.startHue);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, Gradient> {
        return toExit(gradientDeserialize(data));
    }
    function reset(newOverrides?: Partial<Gradient>): void {
        data = { ...gradientDefaultValue(), ...newOverrides };
        errors = { _errors: optionNone(), startHue: optionNone() };
        tainted = { startHue: optionNone() };
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
export function gradientFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, Gradient> {
    const obj: Record<string, unknown> = {};
    {
        const startHueStr = formData.get('startHue');
        obj.startHue = startHueStr ? parseFloat(startHueStr as string) : 0;
        if (obj.startHue !== undefined && isNaN(obj.startHue as number)) obj.startHue = 0;
    }
    return toExit(gradientDeserialize(obj));
}

export const Gradient = {
    defaultValue: gradientDefaultValue,
    serialize: gradientSerialize,
    serializeWithContext: gradientSerializeWithContext,
    deserialize: gradientDeserialize,
    deserializeWithContext: gradientDeserializeWithContext,
    validateFields: gradientValidateFields,
    hasShape: gradientHasShape,
    is: gradientIs,
    createForm: gradientCreateForm,
    fromFormData: gradientFromFormData
} as const;
