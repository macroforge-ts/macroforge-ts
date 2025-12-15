import { defaultValueServiceDefaults } from './service-defaults.svelte';
import { SerializeContext } from 'macroforge/serde';
import { __serializeServiceDefaults } from './service-defaults.svelte';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { __deserializeServiceDefaults } from './service-defaults.svelte';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import { ServiceDefaults } from './service-defaults.svelte';

export interface Service {
    id: string;

    name: string;

    quickCode: string;

    group: string | null;

    subgroup: string | null;

    unit: string | null;

    active: boolean;

    commission: boolean;

    favorite: boolean;

    averageTime: string | null;
    defaults: ServiceDefaults;
}

export function defaultValueService(): Service {
    return {
        id: '',
        name: '',
        quickCode: '',
        group: null,
        subgroup: null,
        unit: null,
        active: false,
        commission: false,
        favorite: false,
        averageTime: null,
        defaults: defaultValueServiceDefaults()
    } as Service;
}

export function toStringifiedJSONService(value: Service): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeService(value, ctx));
}
export function toObjectService(value: Service): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeService(value, ctx);
}
export function __serializeService(value: Service, ctx: SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Service', __id };
    result['id'] = value.id;
    result['name'] = value.name;
    result['quickCode'] = value.quickCode;
    result['group'] = value.group;
    result['subgroup'] = value.subgroup;
    result['unit'] = value.unit;
    result['active'] = value.active;
    result['commission'] = value.commission;
    result['favorite'] = value.favorite;
    result['averageTime'] = value.averageTime;
    result['defaults'] = __serializeServiceDefaults(value.defaults, ctx);
    return result;
}

export function fromStringifiedJSONService(
    json: string,
    opts?: DeserializeOptions
): Result<Service, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectService(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectService(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Service, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeService(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Service.fromObject: root cannot be a forward reference'
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
export function __deserializeService(value: any, ctx: DeserializeContext): Service | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Service.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('name' in obj)) {
        errors.push({ field: 'name', message: 'missing required field' });
    }
    if (!('quickCode' in obj)) {
        errors.push({ field: 'quickCode', message: 'missing required field' });
    }
    if (!('group' in obj)) {
        errors.push({ field: 'group', message: 'missing required field' });
    }
    if (!('subgroup' in obj)) {
        errors.push({ field: 'subgroup', message: 'missing required field' });
    }
    if (!('unit' in obj)) {
        errors.push({ field: 'unit', message: 'missing required field' });
    }
    if (!('active' in obj)) {
        errors.push({ field: 'active', message: 'missing required field' });
    }
    if (!('commission' in obj)) {
        errors.push({ field: 'commission', message: 'missing required field' });
    }
    if (!('favorite' in obj)) {
        errors.push({ field: 'favorite', message: 'missing required field' });
    }
    if (!('averageTime' in obj)) {
        errors.push({ field: 'averageTime', message: 'missing required field' });
    }
    if (!('defaults' in obj)) {
        errors.push({ field: 'defaults', message: 'missing required field' });
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
        const __raw_id = obj['id'] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_name = obj['name'] as string;
        if (__raw_name.length === 0) {
            errors.push({ field: 'name', message: 'must not be empty' });
        }
        instance.name = __raw_name;
    }
    {
        const __raw_quickCode = obj['quickCode'] as string;
        if (__raw_quickCode.length === 0) {
            errors.push({ field: 'quickCode', message: 'must not be empty' });
        }
        instance.quickCode = __raw_quickCode;
    }
    {
        const __raw_group = obj['group'] as string | null;
        instance.group = __raw_group;
    }
    {
        const __raw_subgroup = obj['subgroup'] as string | null;
        instance.subgroup = __raw_subgroup;
    }
    {
        const __raw_unit = obj['unit'] as string | null;
        instance.unit = __raw_unit;
    }
    {
        const __raw_active = obj['active'] as boolean;
        instance.active = __raw_active;
    }
    {
        const __raw_commission = obj['commission'] as boolean;
        instance.commission = __raw_commission;
    }
    {
        const __raw_favorite = obj['favorite'] as boolean;
        instance.favorite = __raw_favorite;
    }
    {
        const __raw_averageTime = obj['averageTime'] as string | null;
        instance.averageTime = __raw_averageTime;
    }
    {
        const __raw_defaults = obj['defaults'] as ServiceDefaults;
        {
            const __result = __deserializeServiceDefaults(__raw_defaults, ctx);
            ctx.assignOrDefer(instance, 'defaults', __result);
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Service;
}
export function validateFieldService<K extends keyof Service>(
    field: K,
    value: Service[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'name': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'name', message: 'must not be empty' });
            }
            break;
        }
        case 'quickCode': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'quickCode', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function validateFieldsService(
    partial: Partial<Service>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('name' in partial && partial.name !== undefined) {
        const __val = partial.name as string;
        if (__val.length === 0) {
            errors.push({ field: 'name', message: 'must not be empty' });
        }
    }
    if ('quickCode' in partial && partial.quickCode !== undefined) {
        const __val = partial.quickCode as string;
        if (__val.length === 0) {
            errors.push({ field: 'quickCode', message: 'must not be empty' });
        }
    }
    return errors;
}
export function hasShapeService(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'id' in o &&
        'name' in o &&
        'quickCode' in o &&
        'group' in o &&
        'subgroup' in o &&
        'unit' in o &&
        'active' in o &&
        'commission' in o &&
        'favorite' in o &&
        'averageTime' in o &&
        'defaults' in o
    );
}
export function isService(obj: unknown): obj is Service {
    if (!hasShapeService(obj)) {
        return false;
    }
    const result = fromObjectService(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsService = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    name: Option<Array<string>>;
    quickCode: Option<Array<string>>;
    group: Option<Array<string>>;
    subgroup: Option<Array<string>>;
    unit: Option<Array<string>>;
    active: Option<Array<string>>;
    commission: Option<Array<string>>;
    favorite: Option<Array<string>>;
    averageTime: Option<Array<string>>;
    defaults: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedService = {
    id: Option<boolean>;
    name: Option<boolean>;
    quickCode: Option<boolean>;
    group: Option<boolean>;
    subgroup: Option<boolean>;
    unit: Option<boolean>;
    active: Option<boolean>;
    commission: Option<boolean>;
    favorite: Option<boolean>;
    averageTime: Option<boolean>;
    defaults: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersService {
    readonly id: FieldController<string>;
    readonly name: FieldController<string>;
    readonly quickCode: FieldController<string>;
    readonly group: FieldController<string | null>;
    readonly subgroup: FieldController<string | null>;
    readonly unit: FieldController<string | null>;
    readonly active: FieldController<boolean>;
    readonly commission: FieldController<boolean>;
    readonly favorite: FieldController<boolean>;
    readonly averageTime: FieldController<string | null>;
    readonly defaults: FieldController<ServiceDefaults>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformService {
    readonly data: Service;
    readonly errors: ErrorsService;
    readonly tainted: TaintedService;
    readonly fields: FieldControllersService;
    validate(): Result<Service, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Service>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormService(overrides?: Partial<Service>): GigaformService {
    let data = $state({ ...defaultValueService(), ...overrides });
    let errors = $state<ErrorsService>({
        _errors: Option.none(),
        id: Option.none(),
        name: Option.none(),
        quickCode: Option.none(),
        group: Option.none(),
        subgroup: Option.none(),
        unit: Option.none(),
        active: Option.none(),
        commission: Option.none(),
        favorite: Option.none(),
        averageTime: Option.none(),
        defaults: Option.none()
    });
    let tainted = $state<TaintedService>({
        id: Option.none(),
        name: Option.none(),
        quickCode: Option.none(),
        group: Option.none(),
        subgroup: Option.none(),
        unit: Option.none(),
        active: Option.none(),
        commission: Option.none(),
        favorite: Option.none(),
        averageTime: Option.none(),
        defaults: Option.none()
    });
    const fields: FieldControllersService = {
        id: {
            path: ['id'] as const,
            name: 'id',
            constraints: { required: true },

            get: () => data.id,
            set: (value: string) => {
                data.id = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.id,
            setError: (value: Option<Array<string>>) => {
                errors.id = value;
            },
            getTainted: () => tainted.id,
            setTainted: (value: Option<boolean>) => {
                tainted.id = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldService('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        name: {
            path: ['name'] as const,
            name: 'name',
            constraints: { required: true },
            label: 'Name',
            get: () => data.name,
            set: (value: string) => {
                data.name = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.name,
            setError: (value: Option<Array<string>>) => {
                errors.name = value;
            },
            getTainted: () => tainted.name,
            setTainted: (value: Option<boolean>) => {
                tainted.name = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldService('name', data.name);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        quickCode: {
            path: ['quickCode'] as const,
            name: 'quickCode',
            constraints: { required: true },
            label: 'Quick Code',
            get: () => data.quickCode,
            set: (value: string) => {
                data.quickCode = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.quickCode,
            setError: (value: Option<Array<string>>) => {
                errors.quickCode = value;
            },
            getTainted: () => tainted.quickCode,
            setTainted: (value: Option<boolean>) => {
                tainted.quickCode = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldService('quickCode', data.quickCode);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        group: {
            path: ['group'] as const,
            name: 'group',
            constraints: { required: true },
            label: 'Group',
            get: () => data.group,
            set: (value: string | null) => {
                data.group = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.group,
            setError: (value: Option<Array<string>>) => {
                errors.group = value;
            },
            getTainted: () => tainted.group,
            setTainted: (value: Option<boolean>) => {
                tainted.group = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldService('group', data.group);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        subgroup: {
            path: ['subgroup'] as const,
            name: 'subgroup',
            constraints: { required: true },
            label: 'Subgroup',
            get: () => data.subgroup,
            set: (value: string | null) => {
                data.subgroup = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.subgroup,
            setError: (value: Option<Array<string>>) => {
                errors.subgroup = value;
            },
            getTainted: () => tainted.subgroup,
            setTainted: (value: Option<boolean>) => {
                tainted.subgroup = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldService('subgroup', data.subgroup);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        unit: {
            path: ['unit'] as const,
            name: 'unit',
            constraints: { required: true },
            label: 'Unit',
            get: () => data.unit,
            set: (value: string | null) => {
                data.unit = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.unit,
            setError: (value: Option<Array<string>>) => {
                errors.unit = value;
            },
            getTainted: () => tainted.unit,
            setTainted: (value: Option<boolean>) => {
                tainted.unit = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldService('unit', data.unit);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        active: {
            path: ['active'] as const,
            name: 'active',
            constraints: { required: true },
            label: 'Active',
            get: () => data.active,
            set: (value: boolean) => {
                data.active = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.active,
            setError: (value: Option<Array<string>>) => {
                errors.active = value;
            },
            getTainted: () => tainted.active,
            setTainted: (value: Option<boolean>) => {
                tainted.active = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldService('active', data.active);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        commission: {
            path: ['commission'] as const,
            name: 'commission',
            constraints: { required: true },
            label: 'Commission',
            get: () => data.commission,
            set: (value: boolean) => {
                data.commission = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.commission,
            setError: (value: Option<Array<string>>) => {
                errors.commission = value;
            },
            getTainted: () => tainted.commission,
            setTainted: (value: Option<boolean>) => {
                tainted.commission = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldService('commission', data.commission);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        favorite: {
            path: ['favorite'] as const,
            name: 'favorite',
            constraints: { required: true },
            label: 'Favorite',
            get: () => data.favorite,
            set: (value: boolean) => {
                data.favorite = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.favorite,
            setError: (value: Option<Array<string>>) => {
                errors.favorite = value;
            },
            getTainted: () => tainted.favorite,
            setTainted: (value: Option<boolean>) => {
                tainted.favorite = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldService('favorite', data.favorite);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        averageTime: {
            path: ['averageTime'] as const,
            name: 'averageTime',
            constraints: { required: true },
            label: 'Average Time',
            get: () => data.averageTime,
            set: (value: string | null) => {
                data.averageTime = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.averageTime,
            setError: (value: Option<Array<string>>) => {
                errors.averageTime = value;
            },
            getTainted: () => tainted.averageTime,
            setTainted: (value: Option<boolean>) => {
                tainted.averageTime = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldService('averageTime', data.averageTime);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        defaults: {
            path: ['defaults'] as const,
            name: 'defaults',
            constraints: { required: true },

            get: () => data.defaults,
            set: (value: ServiceDefaults) => {
                data.defaults = value;
            },
            transform: (value: ServiceDefaults): ServiceDefaults => value,
            getError: () => errors.defaults,
            setError: (value: Option<Array<string>>) => {
                errors.defaults = value;
            },
            getTainted: () => tainted.defaults,
            setTainted: (value: Option<boolean>) => {
                tainted.defaults = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldService('defaults', data.defaults);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Service, Array<{ field: string; message: string }>> {
        return fromObjectService(data);
    }
    function reset(newOverrides?: Partial<Service>): void {
        data = { ...defaultValueService(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            id: Option.none(),
            name: Option.none(),
            quickCode: Option.none(),
            group: Option.none(),
            subgroup: Option.none(),
            unit: Option.none(),
            active: Option.none(),
            commission: Option.none(),
            favorite: Option.none(),
            averageTime: Option.none(),
            defaults: Option.none()
        };
        tainted = {
            id: Option.none(),
            name: Option.none(),
            quickCode: Option.none(),
            group: Option.none(),
            subgroup: Option.none(),
            unit: Option.none(),
            active: Option.none(),
            commission: Option.none(),
            favorite: Option.none(),
            averageTime: Option.none(),
            defaults: Option.none()
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
export function fromFormDataService(
    formData: FormData
): Result<Service, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.name = formData.get('name') ?? '';
    obj.quickCode = formData.get('quickCode') ?? '';
    obj.group = formData.get('group') ?? '';
    obj.subgroup = formData.get('subgroup') ?? '';
    obj.unit = formData.get('unit') ?? '';
    {
        const activeVal = formData.get('active');
        obj.active = activeVal === 'true' || activeVal === 'on' || activeVal === '1';
    }
    {
        const commissionVal = formData.get('commission');
        obj.commission =
            commissionVal === 'true' || commissionVal === 'on' || commissionVal === '1';
    }
    {
        const favoriteVal = formData.get('favorite');
        obj.favorite = favoriteVal === 'true' || favoriteVal === 'on' || favoriteVal === '1';
    }
    obj.averageTime = formData.get('averageTime') ?? '';
    {
        // Collect nested object fields with prefix "defaults."
        const defaultsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('defaults.')) {
                const fieldName = key.slice('defaults.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = defaultsObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.defaults = defaultsObj;
    }
    return fromStringifiedJSONService(JSON.stringify(obj));
}
