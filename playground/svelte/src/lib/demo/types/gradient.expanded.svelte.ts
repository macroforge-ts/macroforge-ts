import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Gradient {
    startHue: number;
}

export function defaultValueGradient(): Gradient {
    return { startHue: 0 } as Gradient;
}

export function toStringifiedJSONGradient(value: Gradient): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeGradient(value, ctx));
}
export function toObjectGradient(value: Gradient): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeGradient(value, ctx);
}
export function __serializeGradient(
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

export function fromStringifiedJSONGradient(
    json: string,
    opts?: DeserializeOptions
): Result<Gradient, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectGradient(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectGradient(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Gradient, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeGradient(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Gradient.fromObject: root cannot be a forward reference'
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function __deserializeGradient(value: any, ctx: DeserializeContext): Gradient | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Gradient.__deserialize: expected an object' }
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
export function validateFieldGradient<K extends keyof Gradient>(
    field: K,
    value: Gradient[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsGradient(
    partial: Partial<Gradient>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeGradient(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'startHue' in o;
}
export function isGradient(obj: unknown): obj is Gradient {
    if (!hasShapeGradient(obj)) {
        return false;
    }
    const result = fromObjectGradient(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsGradient = {
    _errors: Option<Array<string>>;
    startHue: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedGradient = {
    startHue: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersGradient {
    readonly startHue: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformGradient {
    readonly data: Gradient;
    readonly errors: ErrorsGradient;
    readonly tainted: TaintedGradient;
    readonly fields: FieldControllersGradient;
    validate(): Result<Gradient, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Gradient>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormGradient(overrides?: Partial<Gradient>): GigaformGradient {
    let data = $state({ ...Gradient.defaultValue(), ...overrides });
    let errors = $state<ErrorsGradient>({ _errors: Option.none(), startHue: Option.none() });
    let tainted = $state<TaintedGradient>({ startHue: Option.none() });
    const fields: FieldControllersGradient = {
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
                const fieldErrors = Gradient.validateField('startHue', data.startHue);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Gradient, Array<{ field: string; message: string }>> {
        return Gradient.fromObject(data);
    }
    function reset(newOverrides?: Partial<Gradient>): void {
        data = { ...Gradient.defaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), startHue: Option.none() };
        tainted = { startHue: Option.none() };
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function fromFormDataGradient(
    formData: FormData
): Result<Gradient, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const startHueStr = formData.get('startHue');
        obj.startHue = startHueStr ? parseFloat(startHueStr as string) : 0;
        if (obj.startHue !== undefined && isNaN(obj.startHue as number)) obj.startHue = 0;
    }
    return Gradient.fromStringifiedJSON(JSON.stringify(obj));
}
