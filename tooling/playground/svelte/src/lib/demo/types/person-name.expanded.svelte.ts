import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */


export interface PersonName {
    
    
    firstName: string;
    
    
    lastName: string;
}

export function personNameDefaultValue(): PersonName {
    return {
        firstName: "",
        lastName: ""
    } as PersonName;
}

export function personNameSerialize(value: PersonName): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(personNameSerializeWithContext(value, ctx));
}
export function personNameSerializeWithContext(value: PersonName, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: "PersonName",
        __id
    };
    result.firstName = value.firstName;
    result.lastName = value.lastName;
    return result;
}

export function personNameDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: PersonName } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = personNameDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "PersonName.deserialize: root cannot be a forward reference"
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return {
            success: true,
            value: resultOrRef
        };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return {
                success: false,
                errors: e.errors
            };
        }
        const message = e instanceof Error ? e.message : String(e);
        return {
            success: false,
            errors: [
                {
                    field: "_root",
                    message
                }
            ]
        };
    }
}
export function personNameDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): PersonName | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"PersonName"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("firstName" in obj)) {
        errors.push({
            field: "firstName",
            message: "missing required field"
        });
    }
    if (!("lastName" in obj)) {
        errors.push({
            field: "lastName",
            message: "missing required field"
        });
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
        const __raw_firstName = obj["firstName"] as string;
        if (__raw_firstName.trim().length === 0) {
            errors.push({
                field: "firstName",
                message: "PersonName.firstName must not be empty"
            });
        }
        instance.firstName = __raw_firstName;
    }
    {
        const __raw_lastName = obj["lastName"] as string;
        if (__raw_lastName.trim().length === 0) {
            errors.push({
                field: "lastName",
                message: "PersonName.lastName must not be empty"
            });
        }
        instance.lastName = __raw_lastName;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as PersonName;
}
export function personNameValidateField<K extends keyof PersonName>(_field: K, _value: PersonName[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === "firstName") {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "firstName",
                message: "PersonName.firstName must not be empty"
            });
        }
    }
    if (_field === "lastName") {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "lastName",
                message: "PersonName.lastName must not be empty"
            });
        }
    }
    return errors;
}
export function personNameValidateFields(_partial: Partial<PersonName>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("firstName" in _partial && _partial.firstName !== undefined) {
        const __val = _partial.firstName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "firstName",
                message: "PersonName.firstName must not be empty"
            });
        }
    }
    if ("lastName" in _partial && _partial.lastName !== undefined) {
        const __val = _partial.lastName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "lastName",
                message: "PersonName.lastName must not be empty"
            });
        }
    }
    return errors;
}
export function personNameHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"firstName" in o && "lastName" in o';
}
export function personNameIs(obj: unknown): obj is PersonName {
    if (!personNameHasShape(obj)) {
        return false;
    }
    const result = personNameDeserialize(obj);
    return result.success;
}

export type PersonNameErrors = {
    _errors: __gf_Option<Array<string>>;
    firstName: __gf_Option<Array<string>>;
    lastName: __gf_Option<Array<string>>;
};
export type PersonNameTainted = {
    firstName: __gf_Option<boolean>;
    lastName: __gf_Option<boolean>;
};
export interface PersonNameFieldControllers {
    readonly firstName: FieldController<string>;
    readonly lastName: FieldController<string>;
}
export interface PersonNameGigaform {
    readonly data: PersonName;
    readonly errors: PersonNameErrors;
    readonly tainted: PersonNameTainted;
    readonly fields: PersonNameFieldControllers;
    validate(): Exit<PersonName, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<PersonName>): void;
}
export function personNameCreateForm(overrides?: Partial<PersonName>): PersonNameGigaform {
    let data = $state({
        ...personNameDefaultValue(),
        ...overrides
    });
    let errors = $state<PersonNameErrors>({
        _errors: optionNone(),
        firstName: optionNone(),
        lastName: optionNone()
    } as PersonNameErrors);
    let tainted = $state<PersonNameTainted>({
        firstName: optionNone(),
        lastName: optionNone()
    } as PersonNameTainted);
    const fields = {
        firstName: {
            path: [
                "firstName"
            ] as const,
            name: "firstName",
            constraints: {
                required: true
            },
            label: "First Name",
            get: ()=>data.firstName,
            set: (value: string)=>{
                data.firstName = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.firstName,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.firstName = value;
            },
            getTainted: ()=>tainted.firstName,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.firstName = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = personNameValidateField("firstName", data.firstName);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        lastName: {
            path: [
                "lastName"
            ] as const,
            name: "lastName",
            constraints: {
                required: true
            },
            label: "Last Name",
            get: ()=>data.lastName,
            set: (value: string)=>{
                data.lastName = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.lastName,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.lastName = value;
            },
            getTainted: ()=>tainted.lastName,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.lastName = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = personNameValidateField("lastName", data.lastName);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        }
    } as PersonNameFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<PersonName, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(personNameDeserialize(data));
    }
    function reset(newOverrides?: Partial<PersonName>): void {
        data = {
            ...personNameDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            firstName: optionNone(),
            lastName: optionNone()
        };
        tainted = {
            firstName: optionNone(),
            lastName: optionNone()
        };
    }
    return {
        get data () {
            return data;
        },
        set data (v){
            data = v;
        },
        get errors () {
            return errors;
        },
        set errors (v){
            errors = v;
        },
        get tainted () {
            return tainted;
        },
        set tainted (v){
            tainted = v;
        },
        fields,
        validate,
        reset
    };
}
export function personNameFromFormData(formData: FormData): Exit<PersonName, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<PersonName, Array<{ field: string; message: string }>>";
    obj.firstName = formData.get(`${"firstName"}`) ?? "";
    obj.lastName = formData.get(`${"lastName"}`) ?? "";
    return toExit(personNameDeserialize(obj));
}

export const PersonName = {
  defaultValue: personNameDefaultValue,
  serialize: personNameSerialize,
  serializeWithContext: personNameSerializeWithContext,
  deserialize: personNameDeserialize,
  deserializeWithContext: personNameDeserializeWithContext,
  validateFields: personNameValidateFields,
  hasShape: personNameHasShape,
  is: personNameIs,
  createForm: personNameCreateForm,
  fromFormData: personNameFromFormData
} as const;