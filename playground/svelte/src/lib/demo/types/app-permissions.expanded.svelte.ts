import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { applicationsSerializeWithContext } from './applications.svelte';
import { pageSerializeWithContext } from './page.svelte';
import { tableSerializeWithContext } from './table.svelte';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { Result } from '@playground/macro/gigaform';
import { Option } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
import type { ArrayFieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { Page } from './page.svelte';
import type { Applications } from './applications.svelte';
import type { Table } from './table.svelte';

export interface AppPermissions {
    applications: Applications[];
    pages: Page[];
    data: Table[];
}

export function appPermissionsDefaultValue(): AppPermissions {
    return { applications: [], pages: [], data: [] } as AppPermissions;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function appPermissionsSerialize(
    value: AppPermissions
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(appPermissionsSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function appPermissionsSerializeWithContext(
    value: AppPermissions,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'AppPermissions', __id };
    result['applications'] = value.applications.map((item) =>
        applicationsSerializeWithContext(item, ctx)
    );
    result['pages'] = value.pages.map((item) => pageSerializeWithContext(item, ctx));
    result['data'] = value.data.map((item) => tableSerializeWithContext(item, ctx));
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function appPermissionsDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, AppPermissions> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = appPermissionsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'AppPermissions.deserialize: root cannot be a forward reference'
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
export function appPermissionsDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): AppPermissions | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'AppPermissions.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('applications' in obj)) {
        errors.push({ field: 'applications', message: 'missing required field' });
    }
    if (!('pages' in obj)) {
        errors.push({ field: 'pages', message: 'missing required field' });
    }
    if (!('data' in obj)) {
        errors.push({ field: 'data', message: 'missing required field' });
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
        const __raw_applications = obj['applications'] as Applications[];
        if (Array.isArray(__raw_applications)) {
            instance.applications = __raw_applications as Applications[];
        }
    }
    {
        const __raw_pages = obj['pages'] as Page[];
        if (Array.isArray(__raw_pages)) {
            instance.pages = __raw_pages as Page[];
        }
    }
    {
        const __raw_data = obj['data'] as Table[];
        if (Array.isArray(__raw_data)) {
            instance.data = __raw_data as Table[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as AppPermissions;
}
export function appPermissionsValidateField<K extends keyof AppPermissions>(
    field: K,
    value: AppPermissions[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function appPermissionsValidateFields(
    partial: Partial<AppPermissions>
): Array<{ field: string; message: string }> {
    return [];
}
export function appPermissionsHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'applications' in o && 'pages' in o && 'data' in o;
}
export function appPermissionsIs(obj: unknown): obj is AppPermissions {
    if (!appPermissionsHasShape(obj)) {
        return false;
    }
    const result = appPermissionsDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type AppPermissionsErrors = {
    _errors: Option<Array<string>>;
    applications: Option<Array<string>>;
    pages: Option<Array<string>>;
    data: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type AppPermissionsTainted = {
    applications: Option<boolean>;
    pages: Option<boolean>;
    data: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface AppPermissionsFieldControllers {
    readonly applications: ArrayFieldController<Applications>;
    readonly pages: ArrayFieldController<Page>;
    readonly data: ArrayFieldController<Table>;
} /** Gigaform instance containing reactive state and field controllers */
export interface AppPermissionsGigaform {
    readonly data: AppPermissions;
    readonly errors: AppPermissionsErrors;
    readonly tainted: AppPermissionsTainted;
    readonly fields: AppPermissionsFieldControllers;
    validate(): Result<AppPermissions, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<AppPermissions>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function appPermissionsCreateForm(
    overrides?: Partial<AppPermissions>
): AppPermissionsGigaform {
    let data = $state({ ...appPermissionsDefaultValue(), ...overrides });
    let errors = $state<AppPermissionsErrors>({
        _errors: Option.none(),
        applications: Option.none(),
        pages: Option.none(),
        data: Option.none()
    });
    let tainted = $state<AppPermissionsTainted>({
        applications: Option.none(),
        pages: Option.none(),
        data: Option.none()
    });
    const fields: AppPermissionsFieldControllers = {
        applications: {
            path: ['applications'] as const,
            name: 'applications',
            constraints: { required: true },
            get: () => data.applications,
            set: (value: Applications[]) => {
                data.applications = value;
            },
            transform: (value: Applications[]): Applications[] => value,
            getError: () => errors.applications,
            setError: (value: Option<Array<string>>) => {
                errors.applications = value;
            },
            getTainted: () => tainted.applications,
            setTainted: (value: Option<boolean>) => {
                tainted.applications = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = appPermissionsValidateField('applications', data.applications);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['applications', index] as const,
                name: `applications.${index}`,
                constraints: { required: true },
                get: () => data.applications[index]!,
                set: (value: Applications) => {
                    data.applications[index] = value;
                },
                transform: (value: Applications): Applications => value,
                getError: () => errors.applications,
                setError: (value: Option<Array<string>>) => {
                    errors.applications = value;
                },
                getTainted: () => tainted.applications,
                setTainted: (value: Option<boolean>) => {
                    tainted.applications = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: Applications) => {
                data.applications.push(item);
            },
            remove: (index: number) => {
                data.applications.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.applications[a]!;
                data.applications[a] = data.applications[b]!;
                data.applications[b] = tmp;
            }
        },
        pages: {
            path: ['pages'] as const,
            name: 'pages',
            constraints: { required: true },
            get: () => data.pages,
            set: (value: Page[]) => {
                data.pages = value;
            },
            transform: (value: Page[]): Page[] => value,
            getError: () => errors.pages,
            setError: (value: Option<Array<string>>) => {
                errors.pages = value;
            },
            getTainted: () => tainted.pages,
            setTainted: (value: Option<boolean>) => {
                tainted.pages = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = appPermissionsValidateField('pages', data.pages);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['pages', index] as const,
                name: `pages.${index}`,
                constraints: { required: true },
                get: () => data.pages[index]!,
                set: (value: Page) => {
                    data.pages[index] = value;
                },
                transform: (value: Page): Page => value,
                getError: () => errors.pages,
                setError: (value: Option<Array<string>>) => {
                    errors.pages = value;
                },
                getTainted: () => tainted.pages,
                setTainted: (value: Option<boolean>) => {
                    tainted.pages = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: Page) => {
                data.pages.push(item);
            },
            remove: (index: number) => {
                data.pages.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.pages[a]!;
                data.pages[a] = data.pages[b]!;
                data.pages[b] = tmp;
            }
        },
        data: {
            path: ['data'] as const,
            name: 'data',
            constraints: { required: true },
            get: () => data.data,
            set: (value: Table[]) => {
                data.data = value;
            },
            transform: (value: Table[]): Table[] => value,
            getError: () => errors.data,
            setError: (value: Option<Array<string>>) => {
                errors.data = value;
            },
            getTainted: () => tainted.data,
            setTainted: (value: Option<boolean>) => {
                tainted.data = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = appPermissionsValidateField('data', data.data);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['data', index] as const,
                name: `data.${index}`,
                constraints: { required: true },
                get: () => data.data[index]!,
                set: (value: Table) => {
                    data.data[index] = value;
                },
                transform: (value: Table): Table => value,
                getError: () => errors.data,
                setError: (value: Option<Array<string>>) => {
                    errors.data = value;
                },
                getTainted: () => tainted.data,
                setTainted: (value: Option<boolean>) => {
                    tainted.data = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: Table) => {
                data.data.push(item);
            },
            remove: (index: number) => {
                data.data.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.data[a]!;
                data.data[a] = data.data[b]!;
                data.data[b] = tmp;
            }
        }
    };
    function validate(): Result<AppPermissions, Array<{ field: string; message: string }>> {
        return appPermissionsDeserialize(data);
    }
    function reset(newOverrides?: Partial<AppPermissions>): void {
        data = { ...appPermissionsDefaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            applications: Option.none(),
            pages: Option.none(),
            data: Option.none()
        };
        tainted = { applications: Option.none(), pages: Option.none(), data: Option.none() };
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
export function appPermissionsFromFormData(
    formData: FormData
): Result<AppPermissions, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const applicationsItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('applications.' + idx + '.') || idx === 0) {
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('applications.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('applications.' + idx + '.')) {
                        const fieldName = key.slice(
                            'applications.'.length + String(idx).length + 1
                        );
                        item[fieldName] = value;
                    }
                }
                applicationsItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.applications = applicationsItems;
    }
    {
        const pagesItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('pages.' + idx + '.') || idx === 0) {
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('pages.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('pages.' + idx + '.')) {
                        const fieldName = key.slice('pages.'.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                pagesItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.pages = pagesItems;
    }
    {
        const dataItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('data.' + idx + '.') || idx === 0) {
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('data.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('data.' + idx + '.')) {
                        const fieldName = key.slice('data.'.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                dataItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.data = dataItems;
    }
    return appPermissionsDeserialize(obj);
}

export const AppPermissions = {
    defaultValue: appPermissionsDefaultValue,
    serialize: appPermissionsSerialize,
    serializeWithContext: appPermissionsSerializeWithContext,
    deserialize: appPermissionsDeserialize,
    deserializeWithContext: appPermissionsDeserializeWithContext,
    validateFields: appPermissionsValidateFields,
    hasShape: appPermissionsHasShape,
    is: appPermissionsIs,
    createForm: appPermissionsCreateForm,
    fromFormData: appPermissionsFromFormData
} as const;
