import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
/** import macro { JSON } from "@playground/macro"; */

export class MacroUser {
    id: string;
    name: string;
    role: string;
    favoriteMacro: 'Derive' | 'JsonNative';
    since: string;

    apiToken: string;

    toString(): string {
        const parts: string[] = [];
        parts.push('userId: ' + this.id);
        parts.push('name: ' + this.name);
        parts.push('role: ' + this.role);
        parts.push('favoriteMacro: ' + this.favoriteMacro);
        parts.push('since: ' + this.since);
        return 'MacroUser { ' + parts.join(', ') + ' }';
    }

    toStringifiedJSON(): string {
        const ctx = SerializeContext.create();
        return JSON.stringify(this.__serialize(ctx));
    }

    toObject(): Record<string, unknown> {
        const ctx = SerializeContext.create();
        return this.__serialize(ctx);
    }

    __serialize(ctx: SerializeContext): Record<string, unknown> {
        const existingId = ctx.getId(this);
        if (existingId !== undefined) {
            return {
                __ref: existingId
            };
        }
        const __id = ctx.register(this);
        const result: Record<string, unknown> = {
            __type: 'MacroUser',
            __id
        };
        result['id'] = this.id;
        result['name'] = this.name;
        result['role'] = this.role;
        result['favoriteMacro'] = this.favoriteMacro;
        result['since'] = this.since;
        result['apiToken'] = this.apiToken;
        return result;
    }

    constructor(props: {
        id: string;
        name: string;
        role: string;
        favoriteMacro: 'Derive' | 'JsonNative';
        since: string;
        apiToken: string;
    }) {
        this.id = props.id;
        this.name = props.name;
        this.role = props.role;
        this.favoriteMacro = props.favoriteMacro;
        this.since = props.since;
        this.apiToken = props.apiToken;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        MacroUser,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return MacroUser.fromObject(raw, opts);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }

    static fromObject(
        obj: unknown,
        opts?: DeserializeOptions
    ): Result<
        MacroUser,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = MacroUser.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'MacroUser.fromObject: root cannot be a forward reference'
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
            return Result.err([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }

    static __deserialize(value: any, ctx: DeserializeContext): MacroUser | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'MacroUser.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('name' in obj)) {
            errors.push({
                field: 'name',
                message: 'missing required field'
            });
        }
        if (!('role' in obj)) {
            errors.push({
                field: 'role',
                message: 'missing required field'
            });
        }
        if (!('favoriteMacro' in obj)) {
            errors.push({
                field: 'favoriteMacro',
                message: 'missing required field'
            });
        }
        if (!('since' in obj)) {
            errors.push({
                field: 'since',
                message: 'missing required field'
            });
        }
        if (!('apiToken' in obj)) {
            errors.push({
                field: 'apiToken',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(MacroUser.prototype) as MacroUser;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj['id'];
            (instance as any).id = __raw_id;
        }
        {
            const __raw_name = obj['name'];
            (instance as any).name = __raw_name;
        }
        {
            const __raw_role = obj['role'];
            (instance as any).role = __raw_role;
        }
        {
            const __raw_favoriteMacro = obj['favoriteMacro'];
            (instance as any).favoriteMacro = __raw_favoriteMacro;
        }
        {
            const __raw_since = obj['since'];
            (instance as any).since = __raw_since;
        }
        {
            const __raw_apiToken = obj['apiToken'];
            (instance as any).apiToken = __raw_apiToken;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof MacroUser>(
        field: K,
        value: MacroUser[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<MacroUser>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }
}

const showcaseUser = new MacroUser({
    id: 'usr_2626',
    name: 'Alya Vector',
    role: 'Macro Explorer',
    favoriteMacro: 'Derive',
    since: '2024-09-12',
    apiToken: 'svelte-secret-token'
});

export const showcaseUserSummary = showcaseUser.toString();
export const showcaseUserJson = showcaseUser.toObject();
