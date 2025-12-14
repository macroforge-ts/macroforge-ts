import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import { Company } from './company.svelte';

export interface CompanyName {
    companyName: string;
}

export function defaultValueCompanyName(): CompanyName {
    return { companyName: '' } as CompanyName;
}

export function toStringifiedJSONCompanyName(value: CompanyName): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeCompanyName(value, ctx));
}
export function toObjectCompanyName(value: CompanyName): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeCompanyName(value, ctx);
}
export function __serializeCompanyName(
    value: CompanyName,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'CompanyName', __id };
    result['companyName'] = value.companyName;
    return result;
}

export function fromStringifiedJSONCompanyName(
    json: string,
    opts?: DeserializeOptions
): Result<CompanyName, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectCompanyName(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectCompanyName(
    obj: unknown,
    opts?: DeserializeOptions
): Result<CompanyName, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeCompanyName(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'CompanyName.fromObject: root cannot be a forward reference'
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
export function __deserializeCompanyName(
    value: any,
    ctx: DeserializeContext
): CompanyName | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'CompanyName.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('companyName' in obj)) {
        errors.push({ field: 'companyName', message: 'missing required field' });
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
        const __raw_companyName = obj['companyName'] as string;
        if (__raw_companyName.length === 0) {
            errors.push({ field: 'companyName', message: 'must not be empty' });
        }
        instance.companyName = __raw_companyName;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as CompanyName;
}
export function validateFieldCompanyName<K extends keyof CompanyName>(
    field: K,
    value: CompanyName[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'companyName': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'companyName', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function validateFieldsCompanyName(
    partial: Partial<CompanyName>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('companyName' in partial && partial.companyName !== undefined) {
        const __val = partial.companyName as string;
        if (__val.length === 0) {
            errors.push({ field: 'companyName', message: 'must not be empty' });
        }
    }
    return errors;
}
export function hasShapeCompanyName(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'companyName' in o;
}
export function isCompanyName(obj: unknown): obj is CompanyName {
    if (!hasShapeCompanyName(obj)) {
        return false;
    }
    const result = fromObjectCompanyName(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsCompanyName = {
    _errors: Option<Array<string>>;
    companyName: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedCompanyName = {
    companyName: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersCompanyName {
    readonly companyName: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformCompanyName {
    readonly data: CompanyName;
    readonly errors: ErrorsCompanyName;
    readonly tainted: TaintedCompanyName;
    readonly fields: FieldControllersCompanyName;
    validate(): Result<CompanyName, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<CompanyName>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormCompanyName(overrides?: Partial<CompanyName>): GigaformCompanyName {
    let data = $state({ ...CompanyName.defaultValue(), ...overrides });
    let errors = $state<ErrorsCompanyName>({ _errors: Option.none(), companyName: Option.none() });
    let tainted = $state<TaintedCompanyName>({ companyName: Option.none() });
    const fields: FieldControllersCompanyName = {
        companyName: {
            path: ['companyName'] as const,
            name: 'companyName',
            constraints: { required: true },
            label: 'Company Name',
            get: () => data.companyName,
            set: (value: string) => {
                data.companyName = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.companyName,
            setError: (value: Option<Array<string>>) => {
                errors.companyName = value;
            },
            getTainted: () => tainted.companyName,
            setTainted: (value: Option<boolean>) => {
                tainted.companyName = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = CompanyName.validateField('companyName', data.companyName);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<CompanyName, Array<{ field: string; message: string }>> {
        return CompanyName.fromObject(data);
    }
    function reset(newOverrides?: Partial<CompanyName>): void {
        data = { ...CompanyName.defaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), companyName: Option.none() };
        tainted = { companyName: Option.none() };
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
export function fromFormDataCompanyName(
    formData: FormData
): Result<CompanyName, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.companyName = formData.get('companyName') ?? '';
    return CompanyName.fromStringifiedJSON(JSON.stringify(obj));
}
