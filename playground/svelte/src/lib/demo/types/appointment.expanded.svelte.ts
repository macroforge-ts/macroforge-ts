import { SerializeContext } from 'macroforge/serde';
import { __serializeColors } from './colors.svelte';
import { __serializeRecurrenceRule } from './recurrence-rule.svelte';
import { __serializeStatus } from './status.svelte';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { __deserializeColors } from './colors.svelte';
import { __deserializeRecurrenceRule } from './recurrence-rule.svelte';
import { __deserializeStatus } from './status.svelte';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
import type { ArrayFieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { Site } from './site.svelte';
import type { Colors } from './colors.svelte';
import type { Employee } from './employee.svelte';
import type { RecurrenceRule } from './recurrence-rule.svelte';
import type { Status } from './status.svelte';

export interface Appointment {
    id: string;

    title: string;

    status: Status;

    begins: string;

    duration: number;

    timeZone: string;

    offsetMs: number;

    allDay: boolean;

    multiDay: boolean;

    employees: (string | Employee)[];

    location: string | Site;

    description: string | null;

    colors: Colors;

    recurrenceRule: RecurrenceRule | null;
}

export function defaultValueAppointment(): Appointment {
    return {
        id: '',
        title: '',
        status: 'Scheduled',
        begins: '',
        duration: 0,
        timeZone: '',
        offsetMs: 0,
        allDay: false,
        multiDay: false,
        employees: [],
        location: '',
        description: null,
        colors: { main: '#000000', hover: '#333333', active: '#666666' },
        recurrenceRule: null
    } as Appointment;
}

export function toStringifiedJSONAppointment(value: Appointment): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeAppointment(value, ctx));
}
export function toObjectAppointment(value: Appointment): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeAppointment(value, ctx);
}
export function __serializeAppointment(
    value: Appointment,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Appointment', __id };
    result['id'] = value.id;
    result['title'] = value.title;
    result['status'] = __serializeStatus(value.status, ctx);
    result['begins'] = value.begins;
    result['duration'] = value.duration;
    result['timeZone'] = value.timeZone;
    result['offsetMs'] = value.offsetMs;
    result['allDay'] = value.allDay;
    result['multiDay'] = value.multiDay;
    result['employees'] = value.employees;
    result['location'] = value.location;
    result['description'] = value.description;
    result['colors'] = __serializeColors(value.colors, ctx);
    if (value.recurrenceRule !== null) {
        result['recurrenceRule'] = __serializeRecurrenceRule(value.recurrenceRule, ctx);
    } else {
        result['recurrenceRule'] = null;
    }
    return result;
}

export function fromStringifiedJSONAppointment(
    json: string,
    opts?: DeserializeOptions
): Result<Appointment, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectAppointment(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectAppointment(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Appointment, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeAppointment(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Appointment.fromObject: root cannot be a forward reference'
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
export function __deserializeAppointment(
    value: any,
    ctx: DeserializeContext
): Appointment | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Appointment.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('title' in obj)) {
        errors.push({ field: 'title', message: 'missing required field' });
    }
    if (!('status' in obj)) {
        errors.push({ field: 'status', message: 'missing required field' });
    }
    if (!('begins' in obj)) {
        errors.push({ field: 'begins', message: 'missing required field' });
    }
    if (!('duration' in obj)) {
        errors.push({ field: 'duration', message: 'missing required field' });
    }
    if (!('timeZone' in obj)) {
        errors.push({ field: 'timeZone', message: 'missing required field' });
    }
    if (!('offsetMs' in obj)) {
        errors.push({ field: 'offsetMs', message: 'missing required field' });
    }
    if (!('allDay' in obj)) {
        errors.push({ field: 'allDay', message: 'missing required field' });
    }
    if (!('multiDay' in obj)) {
        errors.push({ field: 'multiDay', message: 'missing required field' });
    }
    if (!('employees' in obj)) {
        errors.push({ field: 'employees', message: 'missing required field' });
    }
    if (!('location' in obj)) {
        errors.push({ field: 'location', message: 'missing required field' });
    }
    if (!('description' in obj)) {
        errors.push({ field: 'description', message: 'missing required field' });
    }
    if (!('colors' in obj)) {
        errors.push({ field: 'colors', message: 'missing required field' });
    }
    if (!('recurrenceRule' in obj)) {
        errors.push({ field: 'recurrenceRule', message: 'missing required field' });
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
        const __raw_title = obj['title'] as string;
        if (__raw_title.length === 0) {
            errors.push({ field: 'title', message: 'must not be empty' });
        }
        instance.title = __raw_title;
    }
    {
        const __raw_status = obj['status'] as Status;
        {
            const __result = __deserializeStatus(__raw_status, ctx);
            ctx.assignOrDefer(instance, 'status', __result);
        }
    }
    {
        const __raw_begins = obj['begins'] as string;
        instance.begins = __raw_begins;
    }
    {
        const __raw_duration = obj['duration'] as number;
        instance.duration = __raw_duration;
    }
    {
        const __raw_timeZone = obj['timeZone'] as string;
        instance.timeZone = __raw_timeZone;
    }
    {
        const __raw_offsetMs = obj['offsetMs'] as number;
        instance.offsetMs = __raw_offsetMs;
    }
    {
        const __raw_allDay = obj['allDay'] as boolean;
        instance.allDay = __raw_allDay;
    }
    {
        const __raw_multiDay = obj['multiDay'] as boolean;
        instance.multiDay = __raw_multiDay;
    }
    {
        const __raw_employees = obj['employees'] as (string | Employee)[];
        if (Array.isArray(__raw_employees)) {
            instance.employees = __raw_employees as (string | Employee)[];
        }
    }
    {
        const __raw_location = obj['location'] as string | Site;
        instance.location = __raw_location;
    }
    {
        const __raw_description = obj['description'] as string | null;
        instance.description = __raw_description;
    }
    {
        const __raw_colors = obj['colors'] as Colors;
        {
            const __result = __deserializeColors(__raw_colors, ctx);
            ctx.assignOrDefer(instance, 'colors', __result);
        }
    }
    {
        const __raw_recurrenceRule = obj['recurrenceRule'] as RecurrenceRule | null;
        if (__raw_recurrenceRule === null) {
            instance.recurrenceRule = null;
        } else {
            const __result = __deserializeRecurrenceRule(__raw_recurrenceRule, ctx);
            ctx.assignOrDefer(instance, 'recurrenceRule', __result);
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Appointment;
}
export function validateFieldAppointment<K extends keyof Appointment>(
    field: K,
    value: Appointment[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'title': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'title', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function validateFieldsAppointment(
    partial: Partial<Appointment>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('title' in partial && partial.title !== undefined) {
        const __val = partial.title as string;
        if (__val.length === 0) {
            errors.push({ field: 'title', message: 'must not be empty' });
        }
    }
    return errors;
}
export function hasShapeAppointment(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'id' in o &&
        'title' in o &&
        'status' in o &&
        'begins' in o &&
        'duration' in o &&
        'timeZone' in o &&
        'offsetMs' in o &&
        'allDay' in o &&
        'multiDay' in o &&
        'employees' in o &&
        'location' in o &&
        'description' in o &&
        'colors' in o &&
        'recurrenceRule' in o
    );
}
export function isAppointment(obj: unknown): obj is Appointment {
    if (!hasShapeAppointment(obj)) {
        return false;
    }
    const result = fromObjectAppointment(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsAppointment = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    title: Option<Array<string>>;
    status: Option<Array<string>>;
    begins: Option<Array<string>>;
    duration: Option<Array<string>>;
    timeZone: Option<Array<string>>;
    offsetMs: Option<Array<string>>;
    allDay: Option<Array<string>>;
    multiDay: Option<Array<string>>;
    employees: Option<Array<string>>;
    location: Option<Array<string>>;
    description: Option<Array<string>>;
    colors: Option<Array<string>>;
    recurrenceRule: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedAppointment = {
    id: Option<boolean>;
    title: Option<boolean>;
    status: Option<boolean>;
    begins: Option<boolean>;
    duration: Option<boolean>;
    timeZone: Option<boolean>;
    offsetMs: Option<boolean>;
    allDay: Option<boolean>;
    multiDay: Option<boolean>;
    employees: Option<boolean>;
    location: Option<boolean>;
    description: Option<boolean>;
    colors: Option<boolean>;
    recurrenceRule: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersAppointment {
    readonly id: FieldController<string>;
    readonly title: FieldController<string>;
    readonly status: FieldController<Status>;
    readonly begins: FieldController<string>;
    readonly duration: FieldController<number>;
    readonly timeZone: FieldController<string>;
    readonly offsetMs: FieldController<number>;
    readonly allDay: FieldController<boolean>;
    readonly multiDay: FieldController<boolean>;
    readonly employees: ArrayFieldController<string | Employee>;
    readonly location: FieldController<string | Site>;
    readonly description: FieldController<string | null>;
    readonly colors: FieldController<Colors>;
    readonly recurrenceRule: FieldController<RecurrenceRule | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformAppointment {
    readonly data: Appointment;
    readonly errors: ErrorsAppointment;
    readonly tainted: TaintedAppointment;
    readonly fields: FieldControllersAppointment;
    validate(): Result<Appointment, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Appointment>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormAppointment(overrides?: Partial<Appointment>): GigaformAppointment {
    let data = $state({ ...defaultValueAppointment(), ...overrides });
    let errors = $state<ErrorsAppointment>({
        _errors: Option.none(),
        id: Option.none(),
        title: Option.none(),
        status: Option.none(),
        begins: Option.none(),
        duration: Option.none(),
        timeZone: Option.none(),
        offsetMs: Option.none(),
        allDay: Option.none(),
        multiDay: Option.none(),
        employees: Option.none(),
        location: Option.none(),
        description: Option.none(),
        colors: Option.none(),
        recurrenceRule: Option.none()
    });
    let tainted = $state<TaintedAppointment>({
        id: Option.none(),
        title: Option.none(),
        status: Option.none(),
        begins: Option.none(),
        duration: Option.none(),
        timeZone: Option.none(),
        offsetMs: Option.none(),
        allDay: Option.none(),
        multiDay: Option.none(),
        employees: Option.none(),
        location: Option.none(),
        description: Option.none(),
        colors: Option.none(),
        recurrenceRule: Option.none()
    });
    const fields: FieldControllersAppointment = {
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
                const fieldErrors = validateFieldAppointment('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        title: {
            path: ['title'] as const,
            name: 'title',
            constraints: { required: true },
            label: 'Title',
            get: () => data.title,
            set: (value: string) => {
                data.title = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.title,
            setError: (value: Option<Array<string>>) => {
                errors.title = value;
            },
            getTainted: () => tainted.title,
            setTainted: (value: Option<boolean>) => {
                tainted.title = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAppointment('title', data.title);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        status: {
            path: ['status'] as const,
            name: 'status',
            constraints: { required: true },
            label: 'Status',
            get: () => data.status,
            set: (value: Status) => {
                data.status = value;
            },
            transform: (value: Status): Status => value,
            getError: () => errors.status,
            setError: (value: Option<Array<string>>) => {
                errors.status = value;
            },
            getTainted: () => tainted.status,
            setTainted: (value: Option<boolean>) => {
                tainted.status = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAppointment('status', data.status);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        begins: {
            path: ['begins'] as const,
            name: 'begins',
            constraints: { required: true },
            label: 'Begins',
            get: () => data.begins,
            set: (value: string) => {
                data.begins = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.begins,
            setError: (value: Option<Array<string>>) => {
                errors.begins = value;
            },
            getTainted: () => tainted.begins,
            setTainted: (value: Option<boolean>) => {
                tainted.begins = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAppointment('begins', data.begins);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        duration: {
            path: ['duration'] as const,
            name: 'duration',
            constraints: { required: true },
            label: 'Duration',
            get: () => data.duration,
            set: (value: number) => {
                data.duration = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.duration,
            setError: (value: Option<Array<string>>) => {
                errors.duration = value;
            },
            getTainted: () => tainted.duration,
            setTainted: (value: Option<boolean>) => {
                tainted.duration = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAppointment('duration', data.duration);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        timeZone: {
            path: ['timeZone'] as const,
            name: 'timeZone',
            constraints: { required: true },
            label: 'Time Zone',
            get: () => data.timeZone,
            set: (value: string) => {
                data.timeZone = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.timeZone,
            setError: (value: Option<Array<string>>) => {
                errors.timeZone = value;
            },
            getTainted: () => tainted.timeZone,
            setTainted: (value: Option<boolean>) => {
                tainted.timeZone = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAppointment('timeZone', data.timeZone);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        offsetMs: {
            path: ['offsetMs'] as const,
            name: 'offsetMs',
            constraints: { required: true },

            get: () => data.offsetMs,
            set: (value: number) => {
                data.offsetMs = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.offsetMs,
            setError: (value: Option<Array<string>>) => {
                errors.offsetMs = value;
            },
            getTainted: () => tainted.offsetMs,
            setTainted: (value: Option<boolean>) => {
                tainted.offsetMs = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAppointment('offsetMs', data.offsetMs);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        allDay: {
            path: ['allDay'] as const,
            name: 'allDay',
            constraints: { required: true },
            label: 'All Day',
            get: () => data.allDay,
            set: (value: boolean) => {
                data.allDay = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.allDay,
            setError: (value: Option<Array<string>>) => {
                errors.allDay = value;
            },
            getTainted: () => tainted.allDay,
            setTainted: (value: Option<boolean>) => {
                tainted.allDay = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAppointment('allDay', data.allDay);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        multiDay: {
            path: ['multiDay'] as const,
            name: 'multiDay',
            constraints: { required: true },
            label: 'Multi Day',
            get: () => data.multiDay,
            set: (value: boolean) => {
                data.multiDay = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.multiDay,
            setError: (value: Option<Array<string>>) => {
                errors.multiDay = value;
            },
            getTainted: () => tainted.multiDay,
            setTainted: (value: Option<boolean>) => {
                tainted.multiDay = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAppointment('multiDay', data.multiDay);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        employees: {
            path: ['employees'] as const,
            name: 'employees',
            constraints: { required: true },
            label: 'Employees',
            get: () => data.employees,
            set: (value: (string | Employee)[]) => {
                data.employees = value;
            },
            transform: (value: (string | Employee)[]): (string | Employee)[] => value,
            getError: () => errors.employees,
            setError: (value: Option<Array<string>>) => {
                errors.employees = value;
            },
            getTainted: () => tainted.employees,
            setTainted: (value: Option<boolean>) => {
                tainted.employees = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAppointment('employees', data.employees);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['employees', index] as const,
                name: `employees.${index}`,
                constraints: { required: true },
                get: () => data.employees[index]!,
                set: (value: string | Employee) => {
                    data.employees[index] = value;
                },
                transform: (value: string | Employee): string | Employee => value,
                getError: () => errors.employees,
                setError: (value: Option<Array<string>>) => {
                    errors.employees = value;
                },
                getTainted: () => tainted.employees,
                setTainted: (value: Option<boolean>) => {
                    tainted.employees = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: string | Employee) => {
                data.employees.push(item);
            },
            remove: (index: number) => {
                data.employees.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.employees[a]!;
                data.employees[a] = data.employees[b]!;
                data.employees[b] = tmp;
            }
        },
        location: {
            path: ['location'] as const,
            name: 'location',
            constraints: { required: true },
            label: 'Location',
            get: () => data.location,
            set: (value: string | Site) => {
                data.location = value;
            },
            transform: (value: string | Site): string | Site => value,
            getError: () => errors.location,
            setError: (value: Option<Array<string>>) => {
                errors.location = value;
            },
            getTainted: () => tainted.location,
            setTainted: (value: Option<boolean>) => {
                tainted.location = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAppointment('location', data.location);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        description: {
            path: ['description'] as const,
            name: 'description',
            constraints: { required: true },
            label: 'Description',
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
                const fieldErrors = validateFieldAppointment('description', data.description);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        colors: {
            path: ['colors'] as const,
            name: 'colors',
            constraints: { required: true },

            get: () => data.colors,
            set: (value: Colors) => {
                data.colors = value;
            },
            transform: (value: Colors): Colors => value,
            getError: () => errors.colors,
            setError: (value: Option<Array<string>>) => {
                errors.colors = value;
            },
            getTainted: () => tainted.colors,
            setTainted: (value: Option<boolean>) => {
                tainted.colors = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAppointment('colors', data.colors);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        recurrenceRule: {
            path: ['recurrenceRule'] as const,
            name: 'recurrenceRule',
            constraints: { required: true },

            get: () => data.recurrenceRule,
            set: (value: RecurrenceRule | null) => {
                data.recurrenceRule = value;
            },
            transform: (value: RecurrenceRule | null): RecurrenceRule | null => value,
            getError: () => errors.recurrenceRule,
            setError: (value: Option<Array<string>>) => {
                errors.recurrenceRule = value;
            },
            getTainted: () => tainted.recurrenceRule,
            setTainted: (value: Option<boolean>) => {
                tainted.recurrenceRule = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAppointment('recurrenceRule', data.recurrenceRule);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Appointment, Array<{ field: string; message: string }>> {
        return fromObjectAppointment(data);
    }
    function reset(newOverrides?: Partial<Appointment>): void {
        data = { ...defaultValueAppointment(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            id: Option.none(),
            title: Option.none(),
            status: Option.none(),
            begins: Option.none(),
            duration: Option.none(),
            timeZone: Option.none(),
            offsetMs: Option.none(),
            allDay: Option.none(),
            multiDay: Option.none(),
            employees: Option.none(),
            location: Option.none(),
            description: Option.none(),
            colors: Option.none(),
            recurrenceRule: Option.none()
        };
        tainted = {
            id: Option.none(),
            title: Option.none(),
            status: Option.none(),
            begins: Option.none(),
            duration: Option.none(),
            timeZone: Option.none(),
            offsetMs: Option.none(),
            allDay: Option.none(),
            multiDay: Option.none(),
            employees: Option.none(),
            location: Option.none(),
            description: Option.none(),
            colors: Option.none(),
            recurrenceRule: Option.none()
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
export function fromFormDataAppointment(
    formData: FormData
): Result<Appointment, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.title = formData.get('title') ?? '';
    {
        // Collect nested object fields with prefix "status."
        const statusObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('status.')) {
                const fieldName = key.slice('status.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = statusObj;
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
        obj.status = statusObj;
    }
    obj.begins = formData.get('begins') ?? '';
    {
        const durationStr = formData.get('duration');
        obj.duration = durationStr ? parseFloat(durationStr as string) : 0;
        if (obj.duration !== undefined && isNaN(obj.duration as number)) obj.duration = 0;
    }
    obj.timeZone = formData.get('timeZone') ?? '';
    {
        const offsetMsStr = formData.get('offsetMs');
        obj.offsetMs = offsetMsStr ? parseFloat(offsetMsStr as string) : 0;
        if (obj.offsetMs !== undefined && isNaN(obj.offsetMs as number)) obj.offsetMs = 0;
    }
    {
        const allDayVal = formData.get('allDay');
        obj.allDay = allDayVal === 'true' || allDayVal === 'on' || allDayVal === '1';
    }
    {
        const multiDayVal = formData.get('multiDay');
        obj.multiDay = multiDayVal === 'true' || multiDayVal === 'on' || multiDayVal === '1';
    }
    {
        // Collect array items from indexed form fields
        const employeesItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('employees.' + idx + '.') || idx === 0) {
            // Check if any field with this index exists
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('employees.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('employees.' + idx + '.')) {
                        const fieldName = key.slice('employees.'.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                employeesItems.push(item);
            }
            idx++;
            if (idx > 1000) break; // Safety limit
        }
        obj.employees = employeesItems;
    }
    obj.location = formData.get('location') ?? '';
    obj.description = formData.get('description') ?? '';
    {
        // Collect nested object fields with prefix "colors."
        const colorsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('colors.')) {
                const fieldName = key.slice('colors.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = colorsObj;
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
        obj.colors = colorsObj;
    }
    obj.recurrenceRule = formData.get('recurrenceRule') ?? '';
    return fromStringifiedJSONAppointment(JSON.stringify(obj));
}
