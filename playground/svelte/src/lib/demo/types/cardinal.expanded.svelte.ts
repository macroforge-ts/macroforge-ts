import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Cardinal {
    north: number;
    east: number;
    south: number;
    west: number;
}

export function defaultValueCardinal(): Cardinal {
    return { north: 0, east: 0, south: 0, west: 0 } as Cardinal;
}

export function toStringifiedJSONCardinal(value: Cardinal): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeCardinal(value, ctx));
}
export function toObjectCardinal(value: Cardinal): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeCardinal(value, ctx);
}
export function __serializeCardinal(
    value: Cardinal,
    ctx: SerializeContext
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

export function fromStringifiedJSONCardinal(
    json: string,
    opts?: DeserializeOptions
): Result<Cardinal, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectCardinal(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectCardinal(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Cardinal, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeCardinal(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Cardinal.fromObject: root cannot be a forward reference'
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
export function __deserializeCardinal(value: any, ctx: DeserializeContext): Cardinal | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Cardinal.__deserialize: expected an object' }
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
        throw new DeserializeError(errors);
    }
    return instance as Cardinal;
}
export function validateFieldCardinal<K extends keyof Cardinal>(
    field: K,
    value: Cardinal[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsCardinal(
    partial: Partial<Cardinal>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeCardinal(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'north' in o && 'east' in o && 'south' in o && 'west' in o;
}
export function isCardinal(obj: unknown): obj is Cardinal {
    if (!hasShapeCardinal(obj)) {
        return false;
    }
    const result = fromObjectCardinal(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsCardinal = {
    _errors: Option<Array<string>>;
    north: Option<Array<string>>;
    east: Option<Array<string>>;
    south: Option<Array<string>>;
    west: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedCardinal = {
    north: Option<boolean>;
    east: Option<boolean>;
    south: Option<boolean>;
    west: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersCardinal {
    readonly north: FieldController<number>;
    readonly east: FieldController<number>;
    readonly south: FieldController<number>;
    readonly west: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformCardinal {
    readonly data: Cardinal;
    readonly errors: ErrorsCardinal;
    readonly tainted: TaintedCardinal;
    readonly fields: FieldControllersCardinal;
    validate(): Result<Cardinal, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Cardinal>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormCardinal(overrides?: Partial<Cardinal>): GigaformCardinal {
    let data = $state({ ...Cardinal.defaultValue(), ...overrides });
    let errors = $state<ErrorsCardinal>({
        _errors: Option.none(),
        north: Option.none(),
        east: Option.none(),
        south: Option.none(),
        west: Option.none()
    });
    let tainted = $state<TaintedCardinal>({
        north: Option.none(),
        east: Option.none(),
        south: Option.none(),
        west: Option.none()
    });
    const fields: FieldControllersCardinal = {
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
                const fieldErrors = Cardinal.validateField('north', data.north);
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
                const fieldErrors = Cardinal.validateField('east', data.east);
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
                const fieldErrors = Cardinal.validateField('south', data.south);
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
                const fieldErrors = Cardinal.validateField('west', data.west);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Cardinal, Array<{ field: string; message: string }>> {
        return Cardinal.fromObject(data);
    }
    function reset(newOverrides?: Partial<Cardinal>): void {
        data = { ...Cardinal.defaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            north: Option.none(),
            east: Option.none(),
            south: Option.none(),
            west: Option.none()
        };
        tainted = {
            north: Option.none(),
            east: Option.none(),
            south: Option.none(),
            west: Option.none()
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function fromFormDataCardinal(
    formData: FormData
): Result<Cardinal, Array<{ field: string; message: string }>> {
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
    return Cardinal.fromStringifiedJSON(JSON.stringify(obj));
}
