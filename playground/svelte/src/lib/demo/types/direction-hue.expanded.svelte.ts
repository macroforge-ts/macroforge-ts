import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface DirectionHue {
    bearing: number;
    hue: number;
}

export function defaultValueDirectionHue(): DirectionHue {
    return { bearing: 0, hue: 0 } as DirectionHue;
}

export function toStringifiedJSONDirectionHue(value: DirectionHue): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeDirectionHue(value, ctx));
}
export function toObjectDirectionHue(value: DirectionHue): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeDirectionHue(value, ctx);
}
export function __serializeDirectionHue(
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

export function fromStringifiedJSONDirectionHue(
    json: string,
    opts?: DeserializeOptions
): Result<DirectionHue, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectDirectionHue(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectDirectionHue(
    obj: unknown,
    opts?: DeserializeOptions
): Result<DirectionHue, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeDirectionHue(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'DirectionHue.fromObject: root cannot be a forward reference'
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
export function __deserializeDirectionHue(
    value: any,
    ctx: DeserializeContext
): DirectionHue | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'DirectionHue.__deserialize: expected an object' }
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
export function validateFieldDirectionHue<K extends keyof DirectionHue>(
    field: K,
    value: DirectionHue[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsDirectionHue(
    partial: Partial<DirectionHue>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeDirectionHue(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'bearing' in o && 'hue' in o;
}
export function isDirectionHue(obj: unknown): obj is DirectionHue {
    if (!hasShapeDirectionHue(obj)) {
        return false;
    }
    const result = fromObjectDirectionHue(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsDirectionHue = {
    _errors: Option<Array<string>>;
    bearing: Option<Array<string>>;
    hue: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedDirectionHue = {
    bearing: Option<boolean>;
    hue: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersDirectionHue {
    readonly bearing: FieldController<number>;
    readonly hue: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformDirectionHue {
    readonly data: DirectionHue;
    readonly errors: ErrorsDirectionHue;
    readonly tainted: TaintedDirectionHue;
    readonly fields: FieldControllersDirectionHue;
    validate(): Result<DirectionHue, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<DirectionHue>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormDirectionHue(overrides?: Partial<DirectionHue>): GigaformDirectionHue {
    let data = $state({ ...DirectionHue.defaultValue(), ...overrides });
    let errors = $state<ErrorsDirectionHue>({
        _errors: Option.none(),
        bearing: Option.none(),
        hue: Option.none()
    });
    let tainted = $state<TaintedDirectionHue>({ bearing: Option.none(), hue: Option.none() });
    const fields: FieldControllersDirectionHue = {
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
                const fieldErrors = DirectionHue.validateField('bearing', data.bearing);
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
                const fieldErrors = DirectionHue.validateField('hue', data.hue);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<DirectionHue, Array<{ field: string; message: string }>> {
        return DirectionHue.fromObject(data);
    }
    function reset(newOverrides?: Partial<DirectionHue>): void {
        data = { ...DirectionHue.defaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), bearing: Option.none(), hue: Option.none() };
        tainted = { bearing: Option.none(), hue: Option.none() };
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
export function fromFormDataDirectionHue(
    formData: FormData
): Result<DirectionHue, Array<{ field: string; message: string }>> {
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
    return DirectionHue.fromStringifiedJSON(JSON.stringify(obj));
}
