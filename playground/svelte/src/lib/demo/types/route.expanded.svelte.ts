import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { Result } from 'macroforge/reexports';
import { Option } from 'macroforge/reexports';
import type { FieldController } from '@playground/macro/gigaform';
import type { ArrayFieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { Employee } from './employee.svelte';

export interface Route {
    id: string;
    techs: (string | Employee)[] | null;
    active: boolean;

    name: string;

    phone: string;

    position: string;
    serviceRoute: boolean;
    defaultDurationHours: number;
    tags: string[];
    icon: string | null;
    color: string | null;
}

export function routeDefaultValue(): Route {
    return {
        id: '',
        techs: null,
        active: false,
        name: '',
        phone: '',
        position: '',
        serviceRoute: false,
        defaultDurationHours: 0,
        tags: [],
        icon: null,
        color: null
    } as Route;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function routeSerialize(
    value: Route
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(routeSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function routeSerializeWithContext(
    value: Route,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Route', __id };
    result['id'] = value.id;
    if (value.techs !== null) {
        result['techs'] = value.techs;
    } else {
        result['techs'] = null;
    }
    result['active'] = value.active;
    result['name'] = value.name;
    result['phone'] = value.phone;
    result['position'] = value.position;
    result['serviceRoute'] = value.serviceRoute;
    result['defaultDurationHours'] = value.defaultDurationHours;
    result['tags'] = value.tags;
    result['icon'] = value.icon;
    result['color'] = value.color;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function routeDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Route> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = routeDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                { field: '_root', message: 'Route.deserialize: root cannot be a forward reference' }
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
export function routeDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Route | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Route.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('techs' in obj)) {
        errors.push({ field: 'techs', message: 'missing required field' });
    }
    if (!('active' in obj)) {
        errors.push({ field: 'active', message: 'missing required field' });
    }
    if (!('name' in obj)) {
        errors.push({ field: 'name', message: 'missing required field' });
    }
    if (!('phone' in obj)) {
        errors.push({ field: 'phone', message: 'missing required field' });
    }
    if (!('position' in obj)) {
        errors.push({ field: 'position', message: 'missing required field' });
    }
    if (!('serviceRoute' in obj)) {
        errors.push({ field: 'serviceRoute', message: 'missing required field' });
    }
    if (!('defaultDurationHours' in obj)) {
        errors.push({ field: 'defaultDurationHours', message: 'missing required field' });
    }
    if (!('tags' in obj)) {
        errors.push({ field: 'tags', message: 'missing required field' });
    }
    if (!('icon' in obj)) {
        errors.push({ field: 'icon', message: 'missing required field' });
    }
    if (!('color' in obj)) {
        errors.push({ field: 'color', message: 'missing required field' });
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
        const __raw_id = obj['id'] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_techs = obj['techs'] as (string | Employee)[] | null;
        if (__raw_techs === null) {
            instance.techs = null;
        } else {
            instance.techs = __raw_techs;
        }
    }
    {
        const __raw_active = obj['active'] as boolean;
        instance.active = __raw_active;
    }
    {
        const __raw_name = obj['name'] as string;
        if (__raw_name.length === 0) {
            errors.push({ field: 'name', message: 'must not be empty' });
        }
        instance.name = __raw_name;
    }
    {
        const __raw_phone = obj['phone'] as string;
        if (__raw_phone.length === 0) {
            errors.push({ field: 'phone', message: 'must not be empty' });
        }
        instance.phone = __raw_phone;
    }
    {
        const __raw_position = obj['position'] as string;
        if (__raw_position.length === 0) {
            errors.push({ field: 'position', message: 'must not be empty' });
        }
        instance.position = __raw_position;
    }
    {
        const __raw_serviceRoute = obj['serviceRoute'] as boolean;
        instance.serviceRoute = __raw_serviceRoute;
    }
    {
        const __raw_defaultDurationHours = obj['defaultDurationHours'] as number;
        instance.defaultDurationHours = __raw_defaultDurationHours;
    }
    {
        const __raw_tags = obj['tags'] as string[];
        if (Array.isArray(__raw_tags)) {
            instance.tags = __raw_tags as string[];
        }
    }
    {
        const __raw_icon = obj['icon'] as string | null;
        instance.icon = __raw_icon;
    }
    {
        const __raw_color = obj['color'] as string | null;
        instance.color = __raw_color;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Route;
}
export function routeValidateField<K extends keyof Route>(
    field: K,
    value: Route[K]
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
        case 'phone': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'phone', message: 'must not be empty' });
            }
            break;
        }
        case 'position': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'position', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function routeValidateFields(
    partial: Partial<Route>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('name' in partial && partial.name !== undefined) {
        const __val = partial.name as string;
        if (__val.length === 0) {
            errors.push({ field: 'name', message: 'must not be empty' });
        }
    }
    if ('phone' in partial && partial.phone !== undefined) {
        const __val = partial.phone as string;
        if (__val.length === 0) {
            errors.push({ field: 'phone', message: 'must not be empty' });
        }
    }
    if ('position' in partial && partial.position !== undefined) {
        const __val = partial.position as string;
        if (__val.length === 0) {
            errors.push({ field: 'position', message: 'must not be empty' });
        }
    }
    return errors;
}
export function routeHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'id' in o &&
        'techs' in o &&
        'active' in o &&
        'name' in o &&
        'phone' in o &&
        'position' in o &&
        'serviceRoute' in o &&
        'defaultDurationHours' in o &&
        'tags' in o &&
        'icon' in o &&
        'color' in o
    );
}
export function routeIs(obj: unknown): obj is Route {
    if (!routeHasShape(obj)) {
        return false;
    }
    const result = routeDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type RouteErrors = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    techs: Option<Array<string>>;
    active: Option<Array<string>>;
    name: Option<Array<string>>;
    phone: Option<Array<string>>;
    position: Option<Array<string>>;
    serviceRoute: Option<Array<string>>;
    defaultDurationHours: Option<Array<string>>;
    tags: Option<Array<string>>;
    icon: Option<Array<string>>;
    color: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type RouteTainted = {
    id: Option<boolean>;
    techs: Option<boolean>;
    active: Option<boolean>;
    name: Option<boolean>;
    phone: Option<boolean>;
    position: Option<boolean>;
    serviceRoute: Option<boolean>;
    defaultDurationHours: Option<boolean>;
    tags: Option<boolean>;
    icon: Option<boolean>;
    color: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface RouteFieldControllers {
    readonly id: FieldController<string>;
    readonly techs: FieldController<(string | Employee)[] | null>;
    readonly active: FieldController<boolean>;
    readonly name: FieldController<string>;
    readonly phone: FieldController<string>;
    readonly position: FieldController<string>;
    readonly serviceRoute: FieldController<boolean>;
    readonly defaultDurationHours: FieldController<number>;
    readonly tags: ArrayFieldController<string>;
    readonly icon: FieldController<string | null>;
    readonly color: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface RouteGigaform {
    readonly data: Route;
    readonly errors: RouteErrors;
    readonly tainted: RouteTainted;
    readonly fields: RouteFieldControllers;
    validate(): Result<Route, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Route>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function routeCreateForm(overrides?: Partial<Route>): RouteGigaform {
    let data = $state({ ...routeDefaultValue(), ...overrides });
    let errors = $state<RouteErrors>({
        _errors: Option.none(),
        id: Option.none(),
        techs: Option.none(),
        active: Option.none(),
        name: Option.none(),
        phone: Option.none(),
        position: Option.none(),
        serviceRoute: Option.none(),
        defaultDurationHours: Option.none(),
        tags: Option.none(),
        icon: Option.none(),
        color: Option.none()
    });
    let tainted = $state<RouteTainted>({
        id: Option.none(),
        techs: Option.none(),
        active: Option.none(),
        name: Option.none(),
        phone: Option.none(),
        position: Option.none(),
        serviceRoute: Option.none(),
        defaultDurationHours: Option.none(),
        tags: Option.none(),
        icon: Option.none(),
        color: Option.none()
    });
    const fields: RouteFieldControllers = {
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
                const fieldErrors = routeValidateField('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        techs: {
            path: ['techs'] as const,
            name: 'techs',
            constraints: { required: true },
            get: () => data.techs,
            set: (value: (string | Employee)[] | null) => {
                data.techs = value;
            },
            transform: (value: (string | Employee)[] | null): (string | Employee)[] | null => value,
            getError: () => errors.techs,
            setError: (value: Option<Array<string>>) => {
                errors.techs = value;
            },
            getTainted: () => tainted.techs,
            setTainted: (value: Option<boolean>) => {
                tainted.techs = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = routeValidateField('techs', data.techs);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        active: {
            path: ['active'] as const,
            name: 'active',
            constraints: { required: true },
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
                const fieldErrors = routeValidateField('active', data.active);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        name: {
            path: ['name'] as const,
            name: 'name',
            constraints: { required: true },
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
                const fieldErrors = routeValidateField('name', data.name);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        phone: {
            path: ['phone'] as const,
            name: 'phone',
            constraints: { required: true },
            get: () => data.phone,
            set: (value: string) => {
                data.phone = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.phone,
            setError: (value: Option<Array<string>>) => {
                errors.phone = value;
            },
            getTainted: () => tainted.phone,
            setTainted: (value: Option<boolean>) => {
                tainted.phone = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = routeValidateField('phone', data.phone);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        position: {
            path: ['position'] as const,
            name: 'position',
            constraints: { required: true },
            get: () => data.position,
            set: (value: string) => {
                data.position = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.position,
            setError: (value: Option<Array<string>>) => {
                errors.position = value;
            },
            getTainted: () => tainted.position,
            setTainted: (value: Option<boolean>) => {
                tainted.position = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = routeValidateField('position', data.position);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        serviceRoute: {
            path: ['serviceRoute'] as const,
            name: 'serviceRoute',
            constraints: { required: true },
            get: () => data.serviceRoute,
            set: (value: boolean) => {
                data.serviceRoute = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.serviceRoute,
            setError: (value: Option<Array<string>>) => {
                errors.serviceRoute = value;
            },
            getTainted: () => tainted.serviceRoute,
            setTainted: (value: Option<boolean>) => {
                tainted.serviceRoute = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = routeValidateField('serviceRoute', data.serviceRoute);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        defaultDurationHours: {
            path: ['defaultDurationHours'] as const,
            name: 'defaultDurationHours',
            constraints: { required: true },
            get: () => data.defaultDurationHours,
            set: (value: number) => {
                data.defaultDurationHours = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.defaultDurationHours,
            setError: (value: Option<Array<string>>) => {
                errors.defaultDurationHours = value;
            },
            getTainted: () => tainted.defaultDurationHours,
            setTainted: (value: Option<boolean>) => {
                tainted.defaultDurationHours = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = routeValidateField(
                    'defaultDurationHours',
                    data.defaultDurationHours
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        tags: {
            path: ['tags'] as const,
            name: 'tags',
            constraints: { required: true },
            get: () => data.tags,
            set: (value: string[]) => {
                data.tags = value;
            },
            transform: (value: string[]): string[] => value,
            getError: () => errors.tags,
            setError: (value: Option<Array<string>>) => {
                errors.tags = value;
            },
            getTainted: () => tainted.tags,
            setTainted: (value: Option<boolean>) => {
                tainted.tags = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = routeValidateField('tags', data.tags);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['tags', index] as const,
                name: `tags.${index}`,
                constraints: { required: true },
                get: () => data.tags[index]!,
                set: (value: string) => {
                    data.tags[index] = value;
                },
                transform: (value: string): string => value,
                getError: () => errors.tags,
                setError: (value: Option<Array<string>>) => {
                    errors.tags = value;
                },
                getTainted: () => tainted.tags,
                setTainted: (value: Option<boolean>) => {
                    tainted.tags = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: string) => {
                data.tags.push(item);
            },
            remove: (index: number) => {
                data.tags.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.tags[a]!;
                data.tags[a] = data.tags[b]!;
                data.tags[b] = tmp;
            }
        },
        icon: {
            path: ['icon'] as const,
            name: 'icon',
            constraints: { required: true },
            get: () => data.icon,
            set: (value: string | null) => {
                data.icon = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.icon,
            setError: (value: Option<Array<string>>) => {
                errors.icon = value;
            },
            getTainted: () => tainted.icon,
            setTainted: (value: Option<boolean>) => {
                tainted.icon = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = routeValidateField('icon', data.icon);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        color: {
            path: ['color'] as const,
            name: 'color',
            constraints: { required: true },
            get: () => data.color,
            set: (value: string | null) => {
                data.color = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.color,
            setError: (value: Option<Array<string>>) => {
                errors.color = value;
            },
            getTainted: () => tainted.color,
            setTainted: (value: Option<boolean>) => {
                tainted.color = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = routeValidateField('color', data.color);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Route, Array<{ field: string; message: string }>> {
        return routeDeserialize(data);
    }
    function reset(newOverrides?: Partial<Route>): void {
        data = { ...routeDefaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            id: Option.none(),
            techs: Option.none(),
            active: Option.none(),
            name: Option.none(),
            phone: Option.none(),
            position: Option.none(),
            serviceRoute: Option.none(),
            defaultDurationHours: Option.none(),
            tags: Option.none(),
            icon: Option.none(),
            color: Option.none()
        };
        tainted = {
            id: Option.none(),
            techs: Option.none(),
            active: Option.none(),
            name: Option.none(),
            phone: Option.none(),
            position: Option.none(),
            serviceRoute: Option.none(),
            defaultDurationHours: Option.none(),
            tags: Option.none(),
            icon: Option.none(),
            color: Option.none()
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */
export function routeFromFormData(
    formData: FormData
): Result<Route, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.techs = formData.get('techs') ?? '';
    {
        const activeVal = formData.get('active');
        obj.active = activeVal === 'true' || activeVal === 'on' || activeVal === '1';
    }
    obj.name = formData.get('name') ?? '';
    obj.phone = formData.get('phone') ?? '';
    obj.position = formData.get('position') ?? '';
    {
        const serviceRouteVal = formData.get('serviceRoute');
        obj.serviceRoute =
            serviceRouteVal === 'true' || serviceRouteVal === 'on' || serviceRouteVal === '1';
    }
    {
        const defaultDurationHoursStr = formData.get('defaultDurationHours');
        obj.defaultDurationHours = defaultDurationHoursStr
            ? parseFloat(defaultDurationHoursStr as string)
            : 0;
        if (obj.defaultDurationHours !== undefined && isNaN(obj.defaultDurationHours as number))
            obj.defaultDurationHours = 0;
    }
    obj.tags = formData.getAll('tags') as Array<string>;
    obj.icon = formData.get('icon') ?? '';
    obj.color = formData.get('color') ?? '';
    return routeDeserialize(obj);
}

export const Route = {
    defaultValue: routeDefaultValue,
    serialize: routeSerialize,
    serializeWithContext: routeSerializeWithContext,
    deserialize: routeDeserialize,
    deserializeWithContext: routeDeserializeWithContext,
    validateFields: routeValidateFields,
    hasShape: routeHasShape,
    is: routeIs,
    createForm: routeCreateForm,
    fromFormData: routeFromFormData
} as const;
