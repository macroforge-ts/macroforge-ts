import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
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

export function defaultValueOrdinal(): Ordinal {
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

export function toStringifiedJSONOrdinal(value: Ordinal): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeOrdinal(value, ctx));
}
export function toObjectOrdinal(value: Ordinal): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeOrdinal(value, ctx);
}
export function __serializeOrdinal(value: Ordinal, ctx: SerializeContext): Record<string, unknown> {
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

export function fromStringifiedJSONOrdinal(
    json: string,
    opts?: DeserializeOptions
): Result<Ordinal, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectOrdinal(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectOrdinal(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Ordinal, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeOrdinal(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Ordinal.fromObject: root cannot be a forward reference'
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
export function __deserializeOrdinal(value: any, ctx: DeserializeContext): Ordinal | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Ordinal.__deserialize: expected an object' }
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
export function validateFieldOrdinal<K extends keyof Ordinal>(
    field: K,
    value: Ordinal[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsOrdinal(
    partial: Partial<Ordinal>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeOrdinal(obj: unknown): boolean {
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
export function isOrdinal(obj: unknown): obj is Ordinal {
    if (!hasShapeOrdinal(obj)) {
        return false;
    }
    const result = fromObjectOrdinal(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsOrdinal = {
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
export type TaintedOrdinal = {
    north: Option<boolean>;
    northeast: Option<boolean>;
    east: Option<boolean>;
    southeast: Option<boolean>;
    south: Option<boolean>;
    southwest: Option<boolean>;
    west: Option<boolean>;
    northwest: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersOrdinal {
    readonly north: FieldController<number>;
    readonly northeast: FieldController<number>;
    readonly east: FieldController<number>;
    readonly southeast: FieldController<number>;
    readonly south: FieldController<number>;
    readonly southwest: FieldController<number>;
    readonly west: FieldController<number>;
    readonly northwest: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformOrdinal {
    readonly data: Ordinal;
    readonly errors: ErrorsOrdinal;
    readonly tainted: TaintedOrdinal;
    readonly fields: FieldControllersOrdinal;
    validate(): Result<Ordinal, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Ordinal>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormOrdinal(overrides?: Partial<Ordinal>): GigaformOrdinal {
    let data = $state({ ...defaultValueOrdinal(), ...overrides });
    let errors = $state<ErrorsOrdinal>({
        _errors: Option.none(),
        north: Option.none(),
        northeast: Option.none(),
        east: Option.none(),
        southeast: Option.none(),
        south: Option.none(),
        southwest: Option.none(),
        west: Option.none(),
        northwest: Option.none()
    });
    let tainted = $state<TaintedOrdinal>({
        north: Option.none(),
        northeast: Option.none(),
        east: Option.none(),
        southeast: Option.none(),
        south: Option.none(),
        southwest: Option.none(),
        west: Option.none(),
        northwest: Option.none()
    });
    const fields: FieldControllersOrdinal = {
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
                const fieldErrors = validateFieldOrdinal('north', data.north);
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
                const fieldErrors = validateFieldOrdinal('northeast', data.northeast);
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
                const fieldErrors = validateFieldOrdinal('east', data.east);
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
                const fieldErrors = validateFieldOrdinal('southeast', data.southeast);
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
                const fieldErrors = validateFieldOrdinal('south', data.south);
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
                const fieldErrors = validateFieldOrdinal('southwest', data.southwest);
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
                const fieldErrors = validateFieldOrdinal('west', data.west);
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
                const fieldErrors = validateFieldOrdinal('northwest', data.northwest);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Ordinal, Array<{ field: string; message: string }>> {
        return fromObjectOrdinal(data);
    }
    function reset(newOverrides?: Partial<Ordinal>): void {
        data = { ...defaultValueOrdinal(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            north: Option.none(),
            northeast: Option.none(),
            east: Option.none(),
            southeast: Option.none(),
            south: Option.none(),
            southwest: Option.none(),
            west: Option.none(),
            northwest: Option.none()
        };
        tainted = {
            north: Option.none(),
            northeast: Option.none(),
            east: Option.none(),
            southeast: Option.none(),
            south: Option.none(),
            southwest: Option.none(),
            west: Option.none(),
            northwest: Option.none()
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
export function fromFormDataOrdinal(
    formData: FormData
): Result<Ordinal, Array<{ field: string; message: string }>> {
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
    return fromStringifiedJSONOrdinal(JSON.stringify(obj));
}
