import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Viewed {
    durationSeconds: number | null;
    source: string | null;
}

export function defaultValueViewed(): Viewed {
    return { durationSeconds: null, source: null } as Viewed;
}

export function toStringifiedJSONViewed(value: Viewed): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeViewed(value, ctx));
}
export function toObjectViewed(value: Viewed): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeViewed(value, ctx);
}
export function __serializeViewed(value: Viewed, ctx: SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Viewed', __id };
    result['durationSeconds'] = value.durationSeconds;
    result['source'] = value.source;
    return result;
}

export function fromStringifiedJSONViewed(
    json: string,
    opts?: DeserializeOptions
): Result<Viewed, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectViewed(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectViewed(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Viewed, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeViewed(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Viewed.fromObject: root cannot be a forward reference' }
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
export function __deserializeViewed(value: any, ctx: DeserializeContext): Viewed | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Viewed.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('durationSeconds' in obj)) {
        errors.push({ field: 'durationSeconds', message: 'missing required field' });
    }
    if (!('source' in obj)) {
        errors.push({ field: 'source', message: 'missing required field' });
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
        const __raw_durationSeconds = obj['durationSeconds'] as number | null;
        instance.durationSeconds = __raw_durationSeconds;
    }
    {
        const __raw_source = obj['source'] as string | null;
        instance.source = __raw_source;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Viewed;
}
export function validateFieldViewed<K extends keyof Viewed>(
    field: K,
    value: Viewed[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsViewed(
    partial: Partial<Viewed>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeViewed(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'durationSeconds' in o && 'source' in o;
}
export function isViewed(obj: unknown): obj is Viewed {
    if (!hasShapeViewed(obj)) {
        return false;
    }
    const result = fromObjectViewed(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsViewed = {
    _errors: Option<Array<string>>;
    durationSeconds: Option<Array<string>>;
    source: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedViewed = {
    durationSeconds: Option<boolean>;
    source: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersViewed {
    readonly durationSeconds: FieldController<number | null>;
    readonly source: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformViewed {
    readonly data: Viewed;
    readonly errors: ErrorsViewed;
    readonly tainted: TaintedViewed;
    readonly fields: FieldControllersViewed;
    validate(): Result<Viewed, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Viewed>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormViewed(overrides?: Partial<Viewed>): GigaformViewed {
    let data = $state({ ...Viewed.defaultValue(), ...overrides });
    let errors = $state<ErrorsViewed>({
        _errors: Option.none(),
        durationSeconds: Option.none(),
        source: Option.none()
    });
    let tainted = $state<TaintedViewed>({ durationSeconds: Option.none(), source: Option.none() });
    const fields: FieldControllersViewed = {
        durationSeconds: {
            path: ['durationSeconds'] as const,
            name: 'durationSeconds',
            constraints: { required: true },

            get: () => data.durationSeconds,
            set: (value: number | null) => {
                data.durationSeconds = value;
            },
            transform: (value: number | null): number | null => value,
            getError: () => errors.durationSeconds,
            setError: (value: Option<Array<string>>) => {
                errors.durationSeconds = value;
            },
            getTainted: () => tainted.durationSeconds,
            setTainted: (value: Option<boolean>) => {
                tainted.durationSeconds = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = Viewed.validateField('durationSeconds', data.durationSeconds);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        source: {
            path: ['source'] as const,
            name: 'source',
            constraints: { required: true },

            get: () => data.source,
            set: (value: string | null) => {
                data.source = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.source,
            setError: (value: Option<Array<string>>) => {
                errors.source = value;
            },
            getTainted: () => tainted.source,
            setTainted: (value: Option<boolean>) => {
                tainted.source = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = Viewed.validateField('source', data.source);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Viewed, Array<{ field: string; message: string }>> {
        return Viewed.fromObject(data);
    }
    function reset(newOverrides?: Partial<Viewed>): void {
        data = { ...Viewed.defaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), durationSeconds: Option.none(), source: Option.none() };
        tainted = { durationSeconds: Option.none(), source: Option.none() };
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
export function fromFormDataViewed(
    formData: FormData
): Result<Viewed, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const durationSecondsStr = formData.get('durationSeconds');
        obj.durationSeconds = durationSecondsStr ? parseFloat(durationSecondsStr as string) : 0;
        if (obj.durationSeconds !== undefined && isNaN(obj.durationSeconds as number))
            obj.durationSeconds = 0;
    }
    obj.source = formData.get('source') ?? '';
    return Viewed.fromStringifiedJSON(JSON.stringify(obj));
}
