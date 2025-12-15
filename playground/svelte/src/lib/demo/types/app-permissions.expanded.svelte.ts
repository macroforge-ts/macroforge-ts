import { SerializeContext } from 'macroforge/serde';
import { __serializeApplications } from './applications.svelte';
import { __serializePage } from './page.svelte';
import { __serializeTable } from './table.svelte';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
import type { ArrayFieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import { Page } from './page.svelte';
import { Applications } from './applications.svelte';
import { Table } from './table.svelte';

export interface AppPermissions {
    applications: Applications[];
    pages: Page[];
    data: Table[];
}

export function defaultValueAppPermissions(): AppPermissions {
    return { applications: [], pages: [], data: [] } as AppPermissions;
}

export function toStringifiedJSONAppPermissions(value: AppPermissions): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeAppPermissions(value, ctx));
}
export function toObjectAppPermissions(value: AppPermissions): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeAppPermissions(value, ctx);
}
export function __serializeAppPermissions(
    value: AppPermissions,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'AppPermissions', __id };
    result['applications'] = value.applications.map((item) => __serializeApplications(item, ctx));
    result['pages'] = value.pages.map((item) => __serializePage(item, ctx));
    result['data'] = value.data.map((item) => __serializeTable(item, ctx));
    return result;
}

export function fromStringifiedJSONAppPermissions(
    json: string,
    opts?: DeserializeOptions
): Result<AppPermissions, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectAppPermissions(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectAppPermissions(
    obj: unknown,
    opts?: DeserializeOptions
): Result<AppPermissions, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeAppPermissions(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'AppPermissions.fromObject: root cannot be a forward reference'
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
export function __deserializeAppPermissions(
    value: any,
    ctx: DeserializeContext
): AppPermissions | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'AppPermissions.__deserialize: expected an object' }
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
        throw new DeserializeError(errors);
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
        throw new DeserializeError(errors);
    }
    return instance as AppPermissions;
}
export function validateFieldAppPermissions<K extends keyof AppPermissions>(
    field: K,
    value: AppPermissions[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsAppPermissions(
    partial: Partial<AppPermissions>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeAppPermissions(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'applications' in o && 'pages' in o && 'data' in o;
}
export function isAppPermissions(obj: unknown): obj is AppPermissions {
    if (!hasShapeAppPermissions(obj)) {
        return false;
    }
    const result = fromObjectAppPermissions(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsAppPermissions = {
    _errors: Option<Array<string>>;
    applications: Option<Array<string>>;
    pages: Option<Array<string>>;
    data: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedAppPermissions = {
    applications: Option<boolean>;
    pages: Option<boolean>;
    data: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersAppPermissions {
    readonly applications: ArrayFieldController<Applications>;
    readonly pages: ArrayFieldController<Page>;
    readonly data: ArrayFieldController<Table>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformAppPermissions {
    readonly data: AppPermissions;
    readonly errors: ErrorsAppPermissions;
    readonly tainted: TaintedAppPermissions;
    readonly fields: FieldControllersAppPermissions;
    validate(): Result<AppPermissions, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<AppPermissions>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormAppPermissions(
    overrides?: Partial<AppPermissions>
): GigaformAppPermissions {
    let data = $state({ ...defaultValueAppPermissions(), ...overrides });
    let errors = $state<ErrorsAppPermissions>({
        _errors: Option.none(),
        applications: Option.none(),
        pages: Option.none(),
        data: Option.none()
    });
    let tainted = $state<TaintedAppPermissions>({
        applications: Option.none(),
        pages: Option.none(),
        data: Option.none()
    });
    const fields: FieldControllersAppPermissions = {
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
                const fieldErrors = validateFieldAppPermissions('applications', data.applications);
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
                const fieldErrors = validateFieldAppPermissions('pages', data.pages);
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
                const fieldErrors = validateFieldAppPermissions('data', data.data);
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
        return fromObjectAppPermissions(data);
    }
    function reset(newOverrides?: Partial<AppPermissions>): void {
        data = { ...defaultValueAppPermissions(), ...newOverrides };
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function fromFormDataAppPermissions(
    formData: FormData
): Result<AppPermissions, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        // Collect array items from indexed form fields
        const applicationsItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('applications.' + idx + '.') || idx === 0) {
            // Check if any field with this index exists
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
            if (idx > 1000) break; // Safety limit
        }
        obj.applications = applicationsItems;
    }
    {
        // Collect array items from indexed form fields
        const pagesItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('pages.' + idx + '.') || idx === 0) {
            // Check if any field with this index exists
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
            if (idx > 1000) break; // Safety limit
        }
        obj.pages = pagesItems;
    }
    {
        // Collect array items from indexed form fields
        const dataItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('data.' + idx + '.') || idx === 0) {
            // Check if any field with this index exists
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
            if (idx > 1000) break; // Safety limit
        }
        obj.data = dataItems;
    }
    return fromStringifiedJSONAppPermissions(JSON.stringify(obj));
}
