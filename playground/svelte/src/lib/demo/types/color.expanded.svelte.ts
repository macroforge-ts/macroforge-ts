import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Color {
    red: number;
    green: number;
    blue: number;
}

export function defaultValueColor(): Color {
    return { red: 0, green: 0, blue: 0 } as Color;
}

export function toStringifiedJSONColor(value: Color): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeColor(value, ctx));
}
export function toObjectColor(value: Color): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeColor(value, ctx);
}
export function __serializeColor(value: Color, ctx: SerializeContext): Record<string, unknown> {
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

export function fromStringifiedJSONColor(
    json: string,
    opts?: DeserializeOptions
): Result<Color, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectColor(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectColor(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Color, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeColor(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Color.fromObject: root cannot be a forward reference' }
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
export function __deserializeColor(value: any, ctx: DeserializeContext): Color | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Color.__deserialize: expected an object' }
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
        throw new DeserializeError(errors);
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
        throw new DeserializeError(errors);
    }
    return instance as Color;
}
export function validateFieldColor<K extends keyof Color>(
    field: K,
    value: Color[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsColor(
    partial: Partial<Color>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeColor(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'red' in o && 'green' in o && 'blue' in o;
}
export function isColor(obj: unknown): obj is Color {
    if (!hasShapeColor(obj)) {
        return false;
    }
    const result = fromObjectColor(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsColor = {
    _errors: Option<Array<string>>;
    red: Option<Array<string>>;
    green: Option<Array<string>>;
    blue: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedColor = {
    red: Option<boolean>;
    green: Option<boolean>;
    blue: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersColor {
    readonly red: FieldController<number>;
    readonly green: FieldController<number>;
    readonly blue: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformColor {
    readonly data: Color;
    readonly errors: ErrorsColor;
    readonly tainted: TaintedColor;
    readonly fields: FieldControllersColor;
    validate(): Result<Color, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Color>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormColor(overrides?: Partial<Color>): GigaformColor {
    let data = $state({ ...defaultValueColor(), ...overrides });
    let errors = $state<ErrorsColor>({
        _errors: Option.none(),
        red: Option.none(),
        green: Option.none(),
        blue: Option.none()
    });
    let tainted = $state<TaintedColor>({
        red: Option.none(),
        green: Option.none(),
        blue: Option.none()
    });
    const fields: FieldControllersColor = {
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
                const fieldErrors = validateFieldColor('red', data.red);
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
                const fieldErrors = validateFieldColor('green', data.green);
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
                const fieldErrors = validateFieldColor('blue', data.blue);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Color, Array<{ field: string; message: string }>> {
        return fromObjectColor(data);
    }
    function reset(newOverrides?: Partial<Color>): void {
        data = { ...defaultValueColor(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            red: Option.none(),
            green: Option.none(),
            blue: Option.none()
        };
        tainted = { red: Option.none(), green: Option.none(), blue: Option.none() };
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
export function fromFormDataColor(
    formData: FormData
): Result<Color, Array<{ field: string; message: string }>> {
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
    return fromStringifiedJSONColor(JSON.stringify(obj));
}
