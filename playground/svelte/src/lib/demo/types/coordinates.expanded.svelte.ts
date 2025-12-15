import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Coordinates {
    lat: number;
    lng: number;
}

export function defaultValueCoordinates(): Coordinates {
    return { lat: 0, lng: 0 } as Coordinates;
}

export function toStringifiedJSONCoordinates(value: Coordinates): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeCoordinates(value, ctx));
}
export function toObjectCoordinates(value: Coordinates): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeCoordinates(value, ctx);
}
export function __serializeCoordinates(
    value: Coordinates,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Coordinates', __id };
    result['lat'] = value.lat;
    result['lng'] = value.lng;
    return result;
}

export function fromStringifiedJSONCoordinates(
    json: string,
    opts?: DeserializeOptions
): Result<Coordinates, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectCoordinates(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectCoordinates(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Coordinates, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeCoordinates(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Coordinates.fromObject: root cannot be a forward reference'
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
export function __deserializeCoordinates(
    value: any,
    ctx: DeserializeContext
): Coordinates | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Coordinates.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('lat' in obj)) {
        errors.push({ field: 'lat', message: 'missing required field' });
    }
    if (!('lng' in obj)) {
        errors.push({ field: 'lng', message: 'missing required field' });
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
        const __raw_lat = obj['lat'] as number;
        instance.lat = __raw_lat;
    }
    {
        const __raw_lng = obj['lng'] as number;
        instance.lng = __raw_lng;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Coordinates;
}
export function validateFieldCoordinates<K extends keyof Coordinates>(
    field: K,
    value: Coordinates[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsCoordinates(
    partial: Partial<Coordinates>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeCoordinates(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'lat' in o && 'lng' in o;
}
export function isCoordinates(obj: unknown): obj is Coordinates {
    if (!hasShapeCoordinates(obj)) {
        return false;
    }
    const result = fromObjectCoordinates(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsCoordinates = {
    _errors: Option<Array<string>>;
    lat: Option<Array<string>>;
    lng: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedCoordinates = {
    lat: Option<boolean>;
    lng: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersCoordinates {
    readonly lat: FieldController<number>;
    readonly lng: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformCoordinates {
    readonly data: Coordinates;
    readonly errors: ErrorsCoordinates;
    readonly tainted: TaintedCoordinates;
    readonly fields: FieldControllersCoordinates;
    validate(): Result<Coordinates, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Coordinates>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormCoordinates(overrides?: Partial<Coordinates>): GigaformCoordinates {
    let data = $state({ ...defaultValueCoordinates(), ...overrides });
    let errors = $state<ErrorsCoordinates>({
        _errors: Option.none(),
        lat: Option.none(),
        lng: Option.none()
    });
    let tainted = $state<TaintedCoordinates>({ lat: Option.none(), lng: Option.none() });
    const fields: FieldControllersCoordinates = {
        lat: {
            path: ['lat'] as const,
            name: 'lat',
            constraints: { required: true },

            get: () => data.lat,
            set: (value: number) => {
                data.lat = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.lat,
            setError: (value: Option<Array<string>>) => {
                errors.lat = value;
            },
            getTainted: () => tainted.lat,
            setTainted: (value: Option<boolean>) => {
                tainted.lat = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldCoordinates('lat', data.lat);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        lng: {
            path: ['lng'] as const,
            name: 'lng',
            constraints: { required: true },

            get: () => data.lng,
            set: (value: number) => {
                data.lng = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.lng,
            setError: (value: Option<Array<string>>) => {
                errors.lng = value;
            },
            getTainted: () => tainted.lng,
            setTainted: (value: Option<boolean>) => {
                tainted.lng = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldCoordinates('lng', data.lng);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Coordinates, Array<{ field: string; message: string }>> {
        return fromObjectCoordinates(data);
    }
    function reset(newOverrides?: Partial<Coordinates>): void {
        data = { ...defaultValueCoordinates(), ...newOverrides };
        errors = { _errors: Option.none(), lat: Option.none(), lng: Option.none() };
        tainted = { lat: Option.none(), lng: Option.none() };
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
export function fromFormDataCoordinates(
    formData: FormData
): Result<Coordinates, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const latStr = formData.get('lat');
        obj.lat = latStr ? parseFloat(latStr as string) : 0;
        if (obj.lat !== undefined && isNaN(obj.lat as number)) obj.lat = 0;
    }
    {
        const lngStr = formData.get('lng');
        obj.lng = lngStr ? parseFloat(lngStr as string) : 0;
        if (obj.lng !== undefined && isNaN(obj.lng as number)) obj.lng = 0;
    }
    return fromStringifiedJSONCoordinates(JSON.stringify(obj));
}
