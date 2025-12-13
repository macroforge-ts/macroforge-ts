import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
/**
 * Edge case test classes for comprehensive deserializer validation testing.
 */

// Multiple validators on single field
/** @derive(Deserialize) */
export class MultipleValidatorsTest {
    /** @serde({ validate: ["nonEmpty", "maxLength(100)", "trimmed"] }) */
    text: string;

    constructor(props: {
        text: string;
    }) {
        this.text = props.text;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        MultipleValidatorsTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return MultipleValidatorsTest.fromObject(raw, opts);
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
        MultipleValidatorsTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = MultipleValidatorsTest.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'MultipleValidatorsTest.fromObject: root cannot be a forward reference'
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

    static __deserialize(value: any, ctx: DeserializeContext): MultipleValidatorsTest | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'MultipleValidatorsTest.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('text' in obj)) {
            errors.push({
                field: 'text',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(MultipleValidatorsTest.prototype) as MultipleValidatorsTest;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_text = obj['text'];
            if (__raw_text.length === 0) {
                errors.push({
                    field: 'text',
                    message: 'must not be empty'
                });
            }
            if (__raw_text.length > 100) {
                errors.push({
                    field: 'text',
                    message: 'must have at most 100 characters'
                });
            }
            if (__raw_text !== __raw_text.trim()) {
                errors.push({
                    field: 'text',
                    message: 'must be trimmed (no leading/trailing whitespace)'
                });
            }
            instance.text = __raw_text;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof MultipleValidatorsTest>(
        field: K,
        value: MultipleValidatorsTest[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'text': {
                const __val = value as string;
                if (__val.length === 0) {
                    errors.push({
                        field: 'text',
                        message: 'must not be empty'
                    });
                }
                if (__val.length > 100) {
                    errors.push({
                        field: 'text',
                        message: 'must have at most 100 characters'
                    });
                }
                if (__val !== __val.trim()) {
                    errors.push({
                        field: 'text',
                        message: 'must be trimmed (no leading/trailing whitespace)'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<MultipleValidatorsTest>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('text' in partial && partial.text !== undefined) {
            const __val = partial.text as string;
            if (__val.length === 0) {
                errors.push({
                    field: 'text',
                    message: 'must not be empty'
                });
            }
            if (__val.length > 100) {
                errors.push({
                    field: 'text',
                    message: 'must have at most 100 characters'
                });
            }
            if (__val !== __val.trim()) {
                errors.push({
                    field: 'text',
                    message: 'must be trimmed (no leading/trailing whitespace)'
                });
            }
        }
        return errors;
    }
}

// Custom error message
/** @derive(Deserialize) */
export class CustomMessageTest {
    /** @serde({ validate: [{ validate: "email", message: "Please enter a valid email address" }] }) */
    email: string;

    constructor(props: {
        email: string;
    }) {
        this.email = props.email;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        CustomMessageTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return CustomMessageTest.fromObject(raw, opts);
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
        CustomMessageTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = CustomMessageTest.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'CustomMessageTest.fromObject: root cannot be a forward reference'
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

    static __deserialize(value: any, ctx: DeserializeContext): CustomMessageTest | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'CustomMessageTest.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(CustomMessageTest.prototype) as CustomMessageTest;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_email = obj['email'];
            if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__raw_email)) {
                errors.push({
                    field: 'email',
                    message: 'Please enter a valid email address'
                });
            }
            instance.email = __raw_email;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof CustomMessageTest>(
        field: K,
        value: CustomMessageTest[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'email': {
                const __val = value as string;
                if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
                    errors.push({
                        field: 'email',
                        message: 'Please enter a valid email address'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<CustomMessageTest>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('email' in partial && partial.email !== undefined) {
            const __val = partial.email as string;
            if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
                errors.push({
                    field: 'email',
                    message: 'Please enter a valid email address'
                });
            }
        }
        return errors;
    }
}

// Mixed validators with custom message
/** @derive(Deserialize) */
export class MixedValidatorsTest {
    /** @serde({ validate: ["nonEmpty", { validate: "email", message: "Invalid email format" }] }) */
    email: string;

    constructor(props: {
        email: string;
    }) {
        this.email = props.email;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        MixedValidatorsTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return MixedValidatorsTest.fromObject(raw, opts);
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
        MixedValidatorsTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = MixedValidatorsTest.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'MixedValidatorsTest.fromObject: root cannot be a forward reference'
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

    static __deserialize(value: any, ctx: DeserializeContext): MixedValidatorsTest | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'MixedValidatorsTest.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(MixedValidatorsTest.prototype) as MixedValidatorsTest;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_email = obj['email'];
            if (__raw_email.length === 0) {
                errors.push({
                    field: 'email',
                    message: 'must not be empty'
                });
            }
            if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__raw_email)) {
                errors.push({
                    field: 'email',
                    message: 'Invalid email format'
                });
            }
            instance.email = __raw_email;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof MixedValidatorsTest>(
        field: K,
        value: MixedValidatorsTest[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'email': {
                const __val = value as string;
                if (__val.length === 0) {
                    errors.push({
                        field: 'email',
                        message: 'must not be empty'
                    });
                }
                if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
                    errors.push({
                        field: 'email',
                        message: 'Invalid email format'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<MixedValidatorsTest>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('email' in partial && partial.email !== undefined) {
            const __val = partial.email as string;
            if (__val.length === 0) {
                errors.push({
                    field: 'email',
                    message: 'must not be empty'
                });
            }
            if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
                errors.push({
                    field: 'email',
                    message: 'Invalid email format'
                });
            }
        }
        return errors;
    }
}

// Combined string validators
/** @derive(Deserialize) */
export class CombinedStringValidatorsTest {
    /** @serde({ validate: ["minLength(3)", "maxLength(20)", "lowercase"] }) */
    username: string;

    constructor(props: {
        username: string;
    }) {
        this.username = props.username;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        CombinedStringValidatorsTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return CombinedStringValidatorsTest.fromObject(raw, opts);
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
        CombinedStringValidatorsTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = CombinedStringValidatorsTest.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'CombinedStringValidatorsTest.fromObject: root cannot be a forward reference'
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

    static __deserialize(
        value: any,
        ctx: DeserializeContext
    ): CombinedStringValidatorsTest | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'CombinedStringValidatorsTest.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('username' in obj)) {
            errors.push({
                field: 'username',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(
            CombinedStringValidatorsTest.prototype
        ) as CombinedStringValidatorsTest;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_username = obj['username'];
            if (__raw_username.length < 3) {
                errors.push({
                    field: 'username',
                    message: 'must have at least 3 characters'
                });
            }
            if (__raw_username.length > 20) {
                errors.push({
                    field: 'username',
                    message: 'must have at most 20 characters'
                });
            }
            if (__raw_username !== __raw_username.toLowerCase()) {
                errors.push({
                    field: 'username',
                    message: 'must be lowercase'
                });
            }
            instance.username = __raw_username;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof CombinedStringValidatorsTest>(
        field: K,
        value: CombinedStringValidatorsTest[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'username': {
                const __val = value as string;
                if (__val.length < 3) {
                    errors.push({
                        field: 'username',
                        message: 'must have at least 3 characters'
                    });
                }
                if (__val.length > 20) {
                    errors.push({
                        field: 'username',
                        message: 'must have at most 20 characters'
                    });
                }
                if (__val !== __val.toLowerCase()) {
                    errors.push({
                        field: 'username',
                        message: 'must be lowercase'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<CombinedStringValidatorsTest>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('username' in partial && partial.username !== undefined) {
            const __val = partial.username as string;
            if (__val.length < 3) {
                errors.push({
                    field: 'username',
                    message: 'must have at least 3 characters'
                });
            }
            if (__val.length > 20) {
                errors.push({
                    field: 'username',
                    message: 'must have at most 20 characters'
                });
            }
            if (__val !== __val.toLowerCase()) {
                errors.push({
                    field: 'username',
                    message: 'must be lowercase'
                });
            }
        }
        return errors;
    }
}

// Combined number validators
/** @derive(Deserialize) */
export class CombinedNumberValidatorsTest {
    /** @serde({ validate: ["int", "positive", "lessThan(1000)"] }) */
    score: number;

    constructor(props: {
        score: number;
    }) {
        this.score = props.score;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        CombinedNumberValidatorsTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return CombinedNumberValidatorsTest.fromObject(raw, opts);
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
        CombinedNumberValidatorsTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = CombinedNumberValidatorsTest.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'CombinedNumberValidatorsTest.fromObject: root cannot be a forward reference'
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

    static __deserialize(
        value: any,
        ctx: DeserializeContext
    ): CombinedNumberValidatorsTest | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'CombinedNumberValidatorsTest.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('score' in obj)) {
            errors.push({
                field: 'score',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(
            CombinedNumberValidatorsTest.prototype
        ) as CombinedNumberValidatorsTest;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_score = obj['score'];
            if (!Number.isInteger(__raw_score)) {
                errors.push({
                    field: 'score',
                    message: 'must be an integer'
                });
            }
            if (__raw_score <= 0) {
                errors.push({
                    field: 'score',
                    message: 'must be positive'
                });
            }
            if (__raw_score >= 1000) {
                errors.push({
                    field: 'score',
                    message: 'must be less than 1000'
                });
            }
            instance.score = __raw_score;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof CombinedNumberValidatorsTest>(
        field: K,
        value: CombinedNumberValidatorsTest[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'score': {
                const __val = value as number;
                if (!Number.isInteger(__val)) {
                    errors.push({
                        field: 'score',
                        message: 'must be an integer'
                    });
                }
                if (__val <= 0) {
                    errors.push({
                        field: 'score',
                        message: 'must be positive'
                    });
                }
                if (__val >= 1000) {
                    errors.push({
                        field: 'score',
                        message: 'must be less than 1000'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<CombinedNumberValidatorsTest>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('score' in partial && partial.score !== undefined) {
            const __val = partial.score as number;
            if (!Number.isInteger(__val)) {
                errors.push({
                    field: 'score',
                    message: 'must be an integer'
                });
            }
            if (__val <= 0) {
                errors.push({
                    field: 'score',
                    message: 'must be positive'
                });
            }
            if (__val >= 1000) {
                errors.push({
                    field: 'score',
                    message: 'must be less than 1000'
                });
            }
        }
        return errors;
    }
}
