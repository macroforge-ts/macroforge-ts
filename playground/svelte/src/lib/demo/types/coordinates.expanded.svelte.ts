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

export interface Coordinates {
    lat: number;
    lng: number;
}

export function coordinatesDefaultValue(): Coordinates {
    return { lat: 0, lng: 0 } as Coordinates;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function coordinatesSerialize(
    value: Coordinates
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(coordinatesSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function coordinatesSerializeWithContext(
    value: Coordinates,
    ctx: __mf_SerializeContext
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

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function coordinatesDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Coordinates> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = coordinatesDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'Coordinates.deserialize: root cannot be a forward reference'
                }
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
export function coordinatesDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Coordinates | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Coordinates.deserializeWithContext: expected an object' }
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
        throw new __mf_DeserializeError(errors);
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
        throw new __mf_DeserializeError(errors);
    }
    return instance as Coordinates;
}
export function coordinatesValidateField<K extends keyof Coordinates>(
    field: K,
    value: Coordinates[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function coordinatesValidateFields(
    partial: Partial<Coordinates>
): Array<{ field: string; message: string }> {
    return [];
}
export function coordinatesHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'lat' in o && 'lng' in o;
}
export function coordinatesIs(obj: unknown): obj is Coordinates {
    if (!coordinatesHasShape(obj)) {
        return false;
    }
    const result = coordinatesDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type CoordinatesErrors = {
    _errors: Option<Array<string>>;
    lat: Option<Array<string>>;
    lng: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type CoordinatesTainted = {
    lat: Option<boolean>;
    lng: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface CoordinatesFieldControllers {
    readonly lat: FieldController<number>;
    readonly lng: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface CoordinatesGigaform {
    readonly data: Coordinates;
    readonly errors: CoordinatesErrors;
    readonly tainted: CoordinatesTainted;
    readonly fields: CoordinatesFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, Coordinates>;
    reset(overrides?: Partial<Coordinates>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function coordinatesCreateForm(overrides?: Partial<Coordinates>): CoordinatesGigaform {
    let data = $state({ ...coordinatesDefaultValue(), ...overrides });
    let errors = $state<CoordinatesErrors>({
        _errors: optionNone(),
        lat: optionNone(),
        lng: optionNone()
    });
    let tainted = $state<CoordinatesTainted>({ lat: optionNone(), lng: optionNone() });
    const fields: CoordinatesFieldControllers = {
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
                const fieldErrors = coordinatesValidateField('lat', data.lat);
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
                const fieldErrors = coordinatesValidateField('lng', data.lng);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, Coordinates> {
        return coordinatesDeserialize(data);
    }
    function reset(newOverrides?: Partial<Coordinates>): void {
        data = { ...coordinatesDefaultValue(), ...newOverrides };
        errors = { _errors: optionNone(), lat: optionNone(), lng: optionNone() };
        tainted = { lat: optionNone(), lng: optionNone() };
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
export function coordinatesFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, Coordinates> {
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
    return coordinatesDeserialize(obj);
}

export const Coordinates = {
    defaultValue: coordinatesDefaultValue,
    serialize: coordinatesSerialize,
    serializeWithContext: coordinatesSerializeWithContext,
    deserialize: coordinatesDeserialize,
    deserializeWithContext: coordinatesDeserializeWithContext,
    validateFields: coordinatesValidateFields,
    hasShape: coordinatesHasShape,
    is: coordinatesIs,
    createForm: coordinatesCreateForm,
    fromFormData: coordinatesFromFormData
} as const;
