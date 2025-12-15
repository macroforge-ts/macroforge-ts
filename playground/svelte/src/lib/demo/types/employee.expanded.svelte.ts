import { defaultValueEmail } from './email.svelte';
import { defaultValueSettings } from './settings.svelte';
import { SerializeContext } from 'macroforge/serde';
import { __serializeEmail } from './email.svelte';
import { __serializeJobTitle } from './job-title.svelte';
import { __serializePhoneNumber } from './phone-number.svelte';
import { __serializeSettings } from './settings.svelte';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { __deserializeEmail } from './email.svelte';
import { __deserializeJobTitle } from './job-title.svelte';
import { __deserializeSettings } from './settings.svelte';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
import type { ArrayFieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import { PhoneNumber } from './phone-number.svelte';
import { Settings } from './settings.svelte';
import { Route } from './route.svelte';
import { Email } from './email.svelte';
import { JobTitle } from './job-title.svelte';

export interface Employee {
    id: string;
    imageUrl: string | null;

    name: string;
    phones: PhoneNumber[];

    role: string;

    title: JobTitle;
    email: Email;

    address: string;

    username: string;

    route: string | Route;
    ratePerHour: number;
    active: boolean;
    isTechnician: boolean;
    isSalesRep: boolean;
    description: string | null;
    linkedinUrl: string | null;
    attendance: string[];
    settings: Settings;
}

export function defaultValueEmployee(): Employee {
    return {
        id: '',
        imageUrl: null,
        name: '',
        phones: [],
        role: '',
        title: 'Technician',
        email: defaultValueEmail(),
        address: '',
        username: '',
        route: '',
        ratePerHour: 0,
        active: false,
        isTechnician: false,
        isSalesRep: false,
        description: null,
        linkedinUrl: null,
        attendance: [],
        settings: defaultValueSettings()
    } as Employee;
}

export function toStringifiedJSONEmployee(value: Employee): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeEmployee(value, ctx));
}
export function toObjectEmployee(value: Employee): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeEmployee(value, ctx);
}
export function __serializeEmployee(
    value: Employee,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Employee', __id };
    result['id'] = value.id;
    result['imageUrl'] = value.imageUrl;
    result['name'] = value.name;
    result['phones'] = value.phones.map((item) => __serializePhoneNumber(item, ctx));
    result['role'] = value.role;
    result['title'] = __serializeJobTitle(value.title, ctx);
    result['email'] = __serializeEmail(value.email, ctx);
    result['address'] = value.address;
    result['username'] = value.username;
    result['route'] = value.route;
    result['ratePerHour'] = value.ratePerHour;
    result['active'] = value.active;
    result['isTechnician'] = value.isTechnician;
    result['isSalesRep'] = value.isSalesRep;
    result['description'] = value.description;
    result['linkedinUrl'] = value.linkedinUrl;
    result['attendance'] = value.attendance;
    result['settings'] = __serializeSettings(value.settings, ctx);
    return result;
}

export function fromStringifiedJSONEmployee(
    json: string,
    opts?: DeserializeOptions
): Result<Employee, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectEmployee(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectEmployee(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Employee, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeEmployee(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Employee.fromObject: root cannot be a forward reference'
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
export function __deserializeEmployee(value: any, ctx: DeserializeContext): Employee | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Employee.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('imageUrl' in obj)) {
        errors.push({ field: 'imageUrl', message: 'missing required field' });
    }
    if (!('name' in obj)) {
        errors.push({ field: 'name', message: 'missing required field' });
    }
    if (!('phones' in obj)) {
        errors.push({ field: 'phones', message: 'missing required field' });
    }
    if (!('role' in obj)) {
        errors.push({ field: 'role', message: 'missing required field' });
    }
    if (!('title' in obj)) {
        errors.push({ field: 'title', message: 'missing required field' });
    }
    if (!('email' in obj)) {
        errors.push({ field: 'email', message: 'missing required field' });
    }
    if (!('address' in obj)) {
        errors.push({ field: 'address', message: 'missing required field' });
    }
    if (!('username' in obj)) {
        errors.push({ field: 'username', message: 'missing required field' });
    }
    if (!('route' in obj)) {
        errors.push({ field: 'route', message: 'missing required field' });
    }
    if (!('ratePerHour' in obj)) {
        errors.push({ field: 'ratePerHour', message: 'missing required field' });
    }
    if (!('active' in obj)) {
        errors.push({ field: 'active', message: 'missing required field' });
    }
    if (!('isTechnician' in obj)) {
        errors.push({ field: 'isTechnician', message: 'missing required field' });
    }
    if (!('isSalesRep' in obj)) {
        errors.push({ field: 'isSalesRep', message: 'missing required field' });
    }
    if (!('description' in obj)) {
        errors.push({ field: 'description', message: 'missing required field' });
    }
    if (!('linkedinUrl' in obj)) {
        errors.push({ field: 'linkedinUrl', message: 'missing required field' });
    }
    if (!('attendance' in obj)) {
        errors.push({ field: 'attendance', message: 'missing required field' });
    }
    if (!('settings' in obj)) {
        errors.push({ field: 'settings', message: 'missing required field' });
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
        const __raw_imageUrl = obj['imageUrl'] as string | null;
        instance.imageUrl = __raw_imageUrl;
    }
    {
        const __raw_name = obj['name'] as string;
        if (__raw_name.length === 0) {
            errors.push({ field: 'name', message: 'must not be empty' });
        }
        instance.name = __raw_name;
    }
    {
        const __raw_phones = obj['phones'] as PhoneNumber[];
        if (Array.isArray(__raw_phones)) {
            instance.phones = __raw_phones as PhoneNumber[];
        }
    }
    {
        const __raw_role = obj['role'] as string;
        if (__raw_role.length === 0) {
            errors.push({ field: 'role', message: 'must not be empty' });
        }
        instance.role = __raw_role;
    }
    {
        const __raw_title = obj['title'] as JobTitle;
        {
            const __result = __deserializeJobTitle(__raw_title, ctx);
            ctx.assignOrDefer(instance, 'title', __result);
        }
    }
    {
        const __raw_email = obj['email'] as Email;
        {
            const __result = __deserializeEmail(__raw_email, ctx);
            ctx.assignOrDefer(instance, 'email', __result);
        }
    }
    {
        const __raw_address = obj['address'] as string;
        if (__raw_address.length === 0) {
            errors.push({ field: 'address', message: 'must not be empty' });
        }
        instance.address = __raw_address;
    }
    {
        const __raw_username = obj['username'] as string;
        if (__raw_username.length === 0) {
            errors.push({ field: 'username', message: 'must not be empty' });
        }
        instance.username = __raw_username;
    }
    {
        const __raw_route = obj['route'] as string | Route;
        instance.route = __raw_route;
    }
    {
        const __raw_ratePerHour = obj['ratePerHour'] as number;
        instance.ratePerHour = __raw_ratePerHour;
    }
    {
        const __raw_active = obj['active'] as boolean;
        instance.active = __raw_active;
    }
    {
        const __raw_isTechnician = obj['isTechnician'] as boolean;
        instance.isTechnician = __raw_isTechnician;
    }
    {
        const __raw_isSalesRep = obj['isSalesRep'] as boolean;
        instance.isSalesRep = __raw_isSalesRep;
    }
    {
        const __raw_description = obj['description'] as string | null;
        instance.description = __raw_description;
    }
    {
        const __raw_linkedinUrl = obj['linkedinUrl'] as string | null;
        instance.linkedinUrl = __raw_linkedinUrl;
    }
    {
        const __raw_attendance = obj['attendance'] as string[];
        if (Array.isArray(__raw_attendance)) {
            instance.attendance = __raw_attendance as string[];
        }
    }
    {
        const __raw_settings = obj['settings'] as Settings;
        {
            const __result = __deserializeSettings(__raw_settings, ctx);
            ctx.assignOrDefer(instance, 'settings', __result);
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Employee;
}
export function validateFieldEmployee<K extends keyof Employee>(
    field: K,
    value: Employee[K]
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
        case 'role': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'role', message: 'must not be empty' });
            }
            break;
        }
        case 'address': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'address', message: 'must not be empty' });
            }
            break;
        }
        case 'username': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'username', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function validateFieldsEmployee(
    partial: Partial<Employee>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('name' in partial && partial.name !== undefined) {
        const __val = partial.name as string;
        if (__val.length === 0) {
            errors.push({ field: 'name', message: 'must not be empty' });
        }
    }
    if ('role' in partial && partial.role !== undefined) {
        const __val = partial.role as string;
        if (__val.length === 0) {
            errors.push({ field: 'role', message: 'must not be empty' });
        }
    }
    if ('address' in partial && partial.address !== undefined) {
        const __val = partial.address as string;
        if (__val.length === 0) {
            errors.push({ field: 'address', message: 'must not be empty' });
        }
    }
    if ('username' in partial && partial.username !== undefined) {
        const __val = partial.username as string;
        if (__val.length === 0) {
            errors.push({ field: 'username', message: 'must not be empty' });
        }
    }
    return errors;
}
export function hasShapeEmployee(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'id' in o &&
        'imageUrl' in o &&
        'name' in o &&
        'phones' in o &&
        'role' in o &&
        'title' in o &&
        'email' in o &&
        'address' in o &&
        'username' in o &&
        'route' in o &&
        'ratePerHour' in o &&
        'active' in o &&
        'isTechnician' in o &&
        'isSalesRep' in o &&
        'description' in o &&
        'linkedinUrl' in o &&
        'attendance' in o &&
        'settings' in o
    );
}
export function isEmployee(obj: unknown): obj is Employee {
    if (!hasShapeEmployee(obj)) {
        return false;
    }
    const result = fromObjectEmployee(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsEmployee = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    imageUrl: Option<Array<string>>;
    name: Option<Array<string>>;
    phones: Option<Array<string>>;
    role: Option<Array<string>>;
    title: Option<Array<string>>;
    email: Option<Array<string>>;
    address: Option<Array<string>>;
    username: Option<Array<string>>;
    route: Option<Array<string>>;
    ratePerHour: Option<Array<string>>;
    active: Option<Array<string>>;
    isTechnician: Option<Array<string>>;
    isSalesRep: Option<Array<string>>;
    description: Option<Array<string>>;
    linkedinUrl: Option<Array<string>>;
    attendance: Option<Array<string>>;
    settings: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedEmployee = {
    id: Option<boolean>;
    imageUrl: Option<boolean>;
    name: Option<boolean>;
    phones: Option<boolean>;
    role: Option<boolean>;
    title: Option<boolean>;
    email: Option<boolean>;
    address: Option<boolean>;
    username: Option<boolean>;
    route: Option<boolean>;
    ratePerHour: Option<boolean>;
    active: Option<boolean>;
    isTechnician: Option<boolean>;
    isSalesRep: Option<boolean>;
    description: Option<boolean>;
    linkedinUrl: Option<boolean>;
    attendance: Option<boolean>;
    settings: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersEmployee {
    readonly id: FieldController<string>;
    readonly imageUrl: FieldController<string | null>;
    readonly name: FieldController<string>;
    readonly phones: ArrayFieldController<PhoneNumber>;
    readonly role: FieldController<string>;
    readonly title: FieldController<JobTitle>;
    readonly email: FieldController<Email>;
    readonly address: FieldController<string>;
    readonly username: FieldController<string>;
    readonly route: FieldController<string | Route>;
    readonly ratePerHour: FieldController<number>;
    readonly active: FieldController<boolean>;
    readonly isTechnician: FieldController<boolean>;
    readonly isSalesRep: FieldController<boolean>;
    readonly description: FieldController<string | null>;
    readonly linkedinUrl: FieldController<string | null>;
    readonly attendance: ArrayFieldController<string>;
    readonly settings: FieldController<Settings>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformEmployee {
    readonly data: Employee;
    readonly errors: ErrorsEmployee;
    readonly tainted: TaintedEmployee;
    readonly fields: FieldControllersEmployee;
    validate(): Result<Employee, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Employee>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormEmployee(overrides?: Partial<Employee>): GigaformEmployee {
    let data = $state({ ...defaultValueEmployee(), ...overrides });
    let errors = $state<ErrorsEmployee>({
        _errors: Option.none(),
        id: Option.none(),
        imageUrl: Option.none(),
        name: Option.none(),
        phones: Option.none(),
        role: Option.none(),
        title: Option.none(),
        email: Option.none(),
        address: Option.none(),
        username: Option.none(),
        route: Option.none(),
        ratePerHour: Option.none(),
        active: Option.none(),
        isTechnician: Option.none(),
        isSalesRep: Option.none(),
        description: Option.none(),
        linkedinUrl: Option.none(),
        attendance: Option.none(),
        settings: Option.none()
    });
    let tainted = $state<TaintedEmployee>({
        id: Option.none(),
        imageUrl: Option.none(),
        name: Option.none(),
        phones: Option.none(),
        role: Option.none(),
        title: Option.none(),
        email: Option.none(),
        address: Option.none(),
        username: Option.none(),
        route: Option.none(),
        ratePerHour: Option.none(),
        active: Option.none(),
        isTechnician: Option.none(),
        isSalesRep: Option.none(),
        description: Option.none(),
        linkedinUrl: Option.none(),
        attendance: Option.none(),
        settings: Option.none()
    });
    const fields: FieldControllersEmployee = {
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
                const fieldErrors = validateFieldEmployee('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        imageUrl: {
            path: ['imageUrl'] as const,
            name: 'imageUrl',
            constraints: { required: true },

            get: () => data.imageUrl,
            set: (value: string | null) => {
                data.imageUrl = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.imageUrl,
            setError: (value: Option<Array<string>>) => {
                errors.imageUrl = value;
            },
            getTainted: () => tainted.imageUrl,
            setTainted: (value: Option<boolean>) => {
                tainted.imageUrl = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('imageUrl', data.imageUrl);
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
                const fieldErrors = validateFieldEmployee('name', data.name);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        phones: {
            path: ['phones'] as const,
            name: 'phones',
            constraints: { required: true },

            get: () => data.phones,
            set: (value: PhoneNumber[]) => {
                data.phones = value;
            },
            transform: (value: PhoneNumber[]): PhoneNumber[] => value,
            getError: () => errors.phones,
            setError: (value: Option<Array<string>>) => {
                errors.phones = value;
            },
            getTainted: () => tainted.phones,
            setTainted: (value: Option<boolean>) => {
                tainted.phones = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('phones', data.phones);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['phones', index] as const,
                name: `phones.${index}`,
                constraints: { required: true },
                get: () => data.phones[index]!,
                set: (value: PhoneNumber) => {
                    data.phones[index] = value;
                },
                transform: (value: PhoneNumber): PhoneNumber => value,
                getError: () => errors.phones,
                setError: (value: Option<Array<string>>) => {
                    errors.phones = value;
                },
                getTainted: () => tainted.phones,
                setTainted: (value: Option<boolean>) => {
                    tainted.phones = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: PhoneNumber) => {
                data.phones.push(item);
            },
            remove: (index: number) => {
                data.phones.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.phones[a]!;
                data.phones[a] = data.phones[b]!;
                data.phones[b] = tmp;
            }
        },
        role: {
            path: ['role'] as const,
            name: 'role',
            constraints: { required: true },

            get: () => data.role,
            set: (value: string) => {
                data.role = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.role,
            setError: (value: Option<Array<string>>) => {
                errors.role = value;
            },
            getTainted: () => tainted.role,
            setTainted: (value: Option<boolean>) => {
                tainted.role = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('role', data.role);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        title: {
            path: ['title'] as const,
            name: 'title',
            constraints: { required: true },

            get: () => data.title,
            set: (value: JobTitle) => {
                data.title = value;
            },
            transform: (value: JobTitle): JobTitle => value,
            getError: () => errors.title,
            setError: (value: Option<Array<string>>) => {
                errors.title = value;
            },
            getTainted: () => tainted.title,
            setTainted: (value: Option<boolean>) => {
                tainted.title = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('title', data.title);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        email: {
            path: ['email'] as const,
            name: 'email',
            constraints: { required: true },

            get: () => data.email,
            set: (value: Email) => {
                data.email = value;
            },
            transform: (value: Email): Email => value,
            getError: () => errors.email,
            setError: (value: Option<Array<string>>) => {
                errors.email = value;
            },
            getTainted: () => tainted.email,
            setTainted: (value: Option<boolean>) => {
                tainted.email = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('email', data.email);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        address: {
            path: ['address'] as const,
            name: 'address',
            constraints: { required: true },

            get: () => data.address,
            set: (value: string) => {
                data.address = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.address,
            setError: (value: Option<Array<string>>) => {
                errors.address = value;
            },
            getTainted: () => tainted.address,
            setTainted: (value: Option<boolean>) => {
                tainted.address = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('address', data.address);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        username: {
            path: ['username'] as const,
            name: 'username',
            constraints: { required: true },

            get: () => data.username,
            set: (value: string) => {
                data.username = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.username,
            setError: (value: Option<Array<string>>) => {
                errors.username = value;
            },
            getTainted: () => tainted.username,
            setTainted: (value: Option<boolean>) => {
                tainted.username = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('username', data.username);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        route: {
            path: ['route'] as const,
            name: 'route',
            constraints: { required: true },

            get: () => data.route,
            set: (value: string | Route) => {
                data.route = value;
            },
            transform: (value: string | Route): string | Route => value,
            getError: () => errors.route,
            setError: (value: Option<Array<string>>) => {
                errors.route = value;
            },
            getTainted: () => tainted.route,
            setTainted: (value: Option<boolean>) => {
                tainted.route = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('route', data.route);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        ratePerHour: {
            path: ['ratePerHour'] as const,
            name: 'ratePerHour',
            constraints: { required: true },

            get: () => data.ratePerHour,
            set: (value: number) => {
                data.ratePerHour = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.ratePerHour,
            setError: (value: Option<Array<string>>) => {
                errors.ratePerHour = value;
            },
            getTainted: () => tainted.ratePerHour,
            setTainted: (value: Option<boolean>) => {
                tainted.ratePerHour = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('ratePerHour', data.ratePerHour);
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
                const fieldErrors = validateFieldEmployee('active', data.active);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        isTechnician: {
            path: ['isTechnician'] as const,
            name: 'isTechnician',
            constraints: { required: true },

            get: () => data.isTechnician,
            set: (value: boolean) => {
                data.isTechnician = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.isTechnician,
            setError: (value: Option<Array<string>>) => {
                errors.isTechnician = value;
            },
            getTainted: () => tainted.isTechnician,
            setTainted: (value: Option<boolean>) => {
                tainted.isTechnician = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('isTechnician', data.isTechnician);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        isSalesRep: {
            path: ['isSalesRep'] as const,
            name: 'isSalesRep',
            constraints: { required: true },

            get: () => data.isSalesRep,
            set: (value: boolean) => {
                data.isSalesRep = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.isSalesRep,
            setError: (value: Option<Array<string>>) => {
                errors.isSalesRep = value;
            },
            getTainted: () => tainted.isSalesRep,
            setTainted: (value: Option<boolean>) => {
                tainted.isSalesRep = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('isSalesRep', data.isSalesRep);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        description: {
            path: ['description'] as const,
            name: 'description',
            constraints: { required: true },

            get: () => data.description,
            set: (value: string | null) => {
                data.description = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.description,
            setError: (value: Option<Array<string>>) => {
                errors.description = value;
            },
            getTainted: () => tainted.description,
            setTainted: (value: Option<boolean>) => {
                tainted.description = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('description', data.description);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        linkedinUrl: {
            path: ['linkedinUrl'] as const,
            name: 'linkedinUrl',
            constraints: { required: true },

            get: () => data.linkedinUrl,
            set: (value: string | null) => {
                data.linkedinUrl = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.linkedinUrl,
            setError: (value: Option<Array<string>>) => {
                errors.linkedinUrl = value;
            },
            getTainted: () => tainted.linkedinUrl,
            setTainted: (value: Option<boolean>) => {
                tainted.linkedinUrl = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('linkedinUrl', data.linkedinUrl);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        attendance: {
            path: ['attendance'] as const,
            name: 'attendance',
            constraints: { required: true },

            get: () => data.attendance,
            set: (value: string[]) => {
                data.attendance = value;
            },
            transform: (value: string[]): string[] => value,
            getError: () => errors.attendance,
            setError: (value: Option<Array<string>>) => {
                errors.attendance = value;
            },
            getTainted: () => tainted.attendance,
            setTainted: (value: Option<boolean>) => {
                tainted.attendance = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('attendance', data.attendance);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['attendance', index] as const,
                name: `attendance.${index}`,
                constraints: { required: true },
                get: () => data.attendance[index]!,
                set: (value: string) => {
                    data.attendance[index] = value;
                },
                transform: (value: string): string => value,
                getError: () => errors.attendance,
                setError: (value: Option<Array<string>>) => {
                    errors.attendance = value;
                },
                getTainted: () => tainted.attendance,
                setTainted: (value: Option<boolean>) => {
                    tainted.attendance = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: string) => {
                data.attendance.push(item);
            },
            remove: (index: number) => {
                data.attendance.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.attendance[a]!;
                data.attendance[a] = data.attendance[b]!;
                data.attendance[b] = tmp;
            }
        },
        settings: {
            path: ['settings'] as const,
            name: 'settings',
            constraints: { required: true },

            get: () => data.settings,
            set: (value: Settings) => {
                data.settings = value;
            },
            transform: (value: Settings): Settings => value,
            getError: () => errors.settings,
            setError: (value: Option<Array<string>>) => {
                errors.settings = value;
            },
            getTainted: () => tainted.settings,
            setTainted: (value: Option<boolean>) => {
                tainted.settings = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldEmployee('settings', data.settings);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Employee, Array<{ field: string; message: string }>> {
        return fromObjectEmployee(data);
    }
    function reset(newOverrides?: Partial<Employee>): void {
        data = { ...defaultValueEmployee(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            id: Option.none(),
            imageUrl: Option.none(),
            name: Option.none(),
            phones: Option.none(),
            role: Option.none(),
            title: Option.none(),
            email: Option.none(),
            address: Option.none(),
            username: Option.none(),
            route: Option.none(),
            ratePerHour: Option.none(),
            active: Option.none(),
            isTechnician: Option.none(),
            isSalesRep: Option.none(),
            description: Option.none(),
            linkedinUrl: Option.none(),
            attendance: Option.none(),
            settings: Option.none()
        };
        tainted = {
            id: Option.none(),
            imageUrl: Option.none(),
            name: Option.none(),
            phones: Option.none(),
            role: Option.none(),
            title: Option.none(),
            email: Option.none(),
            address: Option.none(),
            username: Option.none(),
            route: Option.none(),
            ratePerHour: Option.none(),
            active: Option.none(),
            isTechnician: Option.none(),
            isSalesRep: Option.none(),
            description: Option.none(),
            linkedinUrl: Option.none(),
            attendance: Option.none(),
            settings: Option.none()
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
export function fromFormDataEmployee(
    formData: FormData
): Result<Employee, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.imageUrl = formData.get('imageUrl') ?? '';
    obj.name = formData.get('name') ?? '';
    {
        // Collect array items from indexed form fields
        const phonesItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('phones.' + idx + '.') || idx === 0) {
            // Check if any field with this index exists
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('phones.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('phones.' + idx + '.')) {
                        const fieldName = key.slice('phones.'.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                phonesItems.push(item);
            }
            idx++;
            if (idx > 1000) break; // Safety limit
        }
        obj.phones = phonesItems;
    }
    obj.role = formData.get('role') ?? '';
    {
        // Collect nested object fields with prefix "title."
        const titleObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('title.')) {
                const fieldName = key.slice('title.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = titleObj;
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
        obj.title = titleObj;
    }
    {
        // Collect nested object fields with prefix "email."
        const emailObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('email.')) {
                const fieldName = key.slice('email.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = emailObj;
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
        obj.email = emailObj;
    }
    obj.address = formData.get('address') ?? '';
    obj.username = formData.get('username') ?? '';
    obj.route = formData.get('route') ?? '';
    {
        const ratePerHourStr = formData.get('ratePerHour');
        obj.ratePerHour = ratePerHourStr ? parseFloat(ratePerHourStr as string) : 0;
        if (obj.ratePerHour !== undefined && isNaN(obj.ratePerHour as number)) obj.ratePerHour = 0;
    }
    {
        const activeVal = formData.get('active');
        obj.active = activeVal === 'true' || activeVal === 'on' || activeVal === '1';
    }
    {
        const isTechnicianVal = formData.get('isTechnician');
        obj.isTechnician =
            isTechnicianVal === 'true' || isTechnicianVal === 'on' || isTechnicianVal === '1';
    }
    {
        const isSalesRepVal = formData.get('isSalesRep');
        obj.isSalesRep =
            isSalesRepVal === 'true' || isSalesRepVal === 'on' || isSalesRepVal === '1';
    }
    obj.description = formData.get('description') ?? '';
    obj.linkedinUrl = formData.get('linkedinUrl') ?? '';
    obj.attendance = formData.getAll('attendance') as Array<string>;
    {
        // Collect nested object fields with prefix "settings."
        const settingsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('settings.')) {
                const fieldName = key.slice('settings.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = settingsObj;
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
        obj.settings = settingsObj;
    }
    return fromStringifiedJSONEmployee(JSON.stringify(obj));
}
