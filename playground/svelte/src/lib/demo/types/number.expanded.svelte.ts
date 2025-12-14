import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Number {
    countryCode: string;

    areaCode: string;

    localNumber: string;
}

export function defaultValueNumber(): Number {
    return { countryCode: '', areaCode: '', localNumber: '' } as Number;
}

export function toStringifiedJSONNumber(value: Number): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeNumber(value, ctx));
}
export function toObjectNumber(value: Number): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeNumber(value, ctx);
}
export function __serializeNumber(value: Number, ctx: SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Number', __id };
    result['countryCode'] = value.countryCode;
    result['areaCode'] = value.areaCode;
    result['localNumber'] = value.localNumber;
    return result;
}

export function fromStringifiedJSONNumber(
    json: string,
    opts?: DeserializeOptions
): Result<Number, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectNumber(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectNumber(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Number, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeNumber(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Number.fromObject: root cannot be a forward reference' }
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
export function __deserializeNumber(value: any, ctx: DeserializeContext): Number | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Number.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('countryCode' in obj)) {
        errors.push({ field: 'countryCode', message: 'missing required field' });
    }
    if (!('areaCode' in obj)) {
        errors.push({ field: 'areaCode', message: 'missing required field' });
    }
    if (!('localNumber' in obj)) {
        errors.push({ field: 'localNumber', message: 'missing required field' });
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
        const __raw_countryCode = obj['countryCode'] as string;
        if (__raw_countryCode.length === 0) {
            errors.push({ field: 'countryCode', message: 'must not be empty' });
        }
        instance.countryCode = __raw_countryCode;
    }
    {
        const __raw_areaCode = obj['areaCode'] as string;
        if (__raw_areaCode.length === 0) {
            errors.push({ field: 'areaCode', message: 'must not be empty' });
        }
        instance.areaCode = __raw_areaCode;
    }
    {
        const __raw_localNumber = obj['localNumber'] as string;
        if (__raw_localNumber.length === 0) {
            errors.push({ field: 'localNumber', message: 'must not be empty' });
        }
        instance.localNumber = __raw_localNumber;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Number;
}
export function validateFieldNumber<K extends keyof Number>(
    field: K,
    value: Number[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'countryCode': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'countryCode', message: 'must not be empty' });
            }
            break;
        }
        case 'areaCode': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'areaCode', message: 'must not be empty' });
            }
            break;
        }
        case 'localNumber': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'localNumber', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function validateFieldsNumber(
    partial: Partial<Number>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('countryCode' in partial && partial.countryCode !== undefined) {
        const __val = partial.countryCode as string;
        if (__val.length === 0) {
            errors.push({ field: 'countryCode', message: 'must not be empty' });
        }
    }
    if ('areaCode' in partial && partial.areaCode !== undefined) {
        const __val = partial.areaCode as string;
        if (__val.length === 0) {
            errors.push({ field: 'areaCode', message: 'must not be empty' });
        }
    }
    if ('localNumber' in partial && partial.localNumber !== undefined) {
        const __val = partial.localNumber as string;
        if (__val.length === 0) {
            errors.push({ field: 'localNumber', message: 'must not be empty' });
        }
    }
    return errors;
}
export function hasShapeNumber(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'countryCode' in o && 'areaCode' in o && 'localNumber' in o;
}
export function isNumber(obj: unknown): obj is Number {
    if (!hasShapeNumber(obj)) {
        return false;
    }
    const result = fromObjectNumber(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsNumber = {
    _errors: Option<Array<string>>;
    countryCode: Option<Array<string>>;
    areaCode: Option<Array<string>>;
    localNumber: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedNumber = {
    countryCode: Option<boolean>;
    areaCode: Option<boolean>;
    localNumber: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersNumber {
    readonly countryCode: FieldController<string>;
    readonly areaCode: FieldController<string>;
    readonly localNumber: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformNumber {
    readonly data: Number;
    readonly errors: ErrorsNumber;
    readonly tainted: TaintedNumber;
    readonly fields: FieldControllersNumber;
    validate(): Result<Number, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Number>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormNumber(overrides?: Partial<Number>): GigaformNumber {
    let data = $state({ ...Number.defaultValue(), ...overrides });
    let errors = $state<ErrorsNumber>({
        _errors: Option.none(),
        countryCode: Option.none(),
        areaCode: Option.none(),
        localNumber: Option.none()
    });
    let tainted = $state<TaintedNumber>({
        countryCode: Option.none(),
        areaCode: Option.none(),
        localNumber: Option.none()
    });
    const fields: FieldControllersNumber = {
        countryCode: {
            path: ['countryCode'] as const,
            name: 'countryCode',
            constraints: { required: true },

            get: () => data.countryCode,
            set: (value: string) => {
                data.countryCode = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.countryCode,
            setError: (value: Option<Array<string>>) => {
                errors.countryCode = value;
            },
            getTainted: () => tainted.countryCode,
            setTainted: (value: Option<boolean>) => {
                tainted.countryCode = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = Number.validateField('countryCode', data.countryCode);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        areaCode: {
            path: ['areaCode'] as const,
            name: 'areaCode',
            constraints: { required: true },

            get: () => data.areaCode,
            set: (value: string) => {
                data.areaCode = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.areaCode,
            setError: (value: Option<Array<string>>) => {
                errors.areaCode = value;
            },
            getTainted: () => tainted.areaCode,
            setTainted: (value: Option<boolean>) => {
                tainted.areaCode = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = Number.validateField('areaCode', data.areaCode);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        localNumber: {
            path: ['localNumber'] as const,
            name: 'localNumber',
            constraints: { required: true },

            get: () => data.localNumber,
            set: (value: string) => {
                data.localNumber = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.localNumber,
            setError: (value: Option<Array<string>>) => {
                errors.localNumber = value;
            },
            getTainted: () => tainted.localNumber,
            setTainted: (value: Option<boolean>) => {
                tainted.localNumber = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = Number.validateField('localNumber', data.localNumber);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Number, Array<{ field: string; message: string }>> {
        return Number.fromObject(data);
    }
    function reset(newOverrides?: Partial<Number>): void {
        data = { ...Number.defaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            countryCode: Option.none(),
            areaCode: Option.none(),
            localNumber: Option.none()
        };
        tainted = {
            countryCode: Option.none(),
            areaCode: Option.none(),
            localNumber: Option.none()
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
export function fromFormDataNumber(
    formData: FormData
): Result<Number, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.countryCode = formData.get('countryCode') ?? '';
    obj.areaCode = formData.get('areaCode') ?? '';
    obj.localNumber = formData.get('localNumber') ?? '';
    return Number.fromStringifiedJSON(JSON.stringify(obj));
}
