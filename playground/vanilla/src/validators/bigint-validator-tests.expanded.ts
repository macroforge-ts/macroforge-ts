import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
/**
 * BigInt validator test classes for comprehensive deserializer validation testing.
 */

// GreaterThanBigInt validator
/** @derive(Deserialize) */
export class GreaterThanBigIntValidator {
    /** @serde({ validate: ["greaterThanBigInt(0)"] }) */
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        GreaterThanBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return GreaterThanBigIntValidator.fromObject(raw, opts);
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
        GreaterThanBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = GreaterThanBigIntValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'GreaterThanBigIntValidator.fromObject: root cannot be a forward reference'
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
    ): GreaterThanBigIntValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'GreaterThanBigIntValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(
            GreaterThanBigIntValidator.prototype
        ) as GreaterThanBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'];
            if (__raw_value <= BigInt(0)) {
                errors.push({
                    field: 'value',
                    message: 'must be greater than 0'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof GreaterThanBigIntValidator>(
        field: K,
        value: GreaterThanBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'value': {
                const __val = value as bigint;
                if (__val <= BigInt(0)) {
                    errors.push({
                        field: 'value',
                        message: 'must be greater than 0'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<GreaterThanBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in partial && partial.value !== undefined) {
            const __val = partial.value as bigint;
            if (__val <= BigInt(0)) {
                errors.push({
                    field: 'value',
                    message: 'must be greater than 0'
                });
            }
        }
        return errors;
    }
}

// GreaterThanOrEqualToBigInt validator
/** @derive(Deserialize) */
export class GreaterThanOrEqualToBigIntValidator {
    /** @serde({ validate: ["greaterThanOrEqualToBigInt(0)"] }) */
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        GreaterThanOrEqualToBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return GreaterThanOrEqualToBigIntValidator.fromObject(raw, opts);
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
        GreaterThanOrEqualToBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = GreaterThanOrEqualToBigIntValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'GreaterThanOrEqualToBigIntValidator.fromObject: root cannot be a forward reference'
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
    ): GreaterThanOrEqualToBigIntValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'GreaterThanOrEqualToBigIntValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(
            GreaterThanOrEqualToBigIntValidator.prototype
        ) as GreaterThanOrEqualToBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'];
            if (__raw_value < BigInt(0)) {
                errors.push({
                    field: 'value',
                    message: 'must be greater than or equal to 0'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof GreaterThanOrEqualToBigIntValidator>(
        field: K,
        value: GreaterThanOrEqualToBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'value': {
                const __val = value as bigint;
                if (__val < BigInt(0)) {
                    errors.push({
                        field: 'value',
                        message: 'must be greater than or equal to 0'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<GreaterThanOrEqualToBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in partial && partial.value !== undefined) {
            const __val = partial.value as bigint;
            if (__val < BigInt(0)) {
                errors.push({
                    field: 'value',
                    message: 'must be greater than or equal to 0'
                });
            }
        }
        return errors;
    }
}

// LessThanBigInt validator
/** @derive(Deserialize) */
export class LessThanBigIntValidator {
    /** @serde({ validate: ["lessThanBigInt(1000)"] }) */
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        LessThanBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return LessThanBigIntValidator.fromObject(raw, opts);
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
        LessThanBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = LessThanBigIntValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'LessThanBigIntValidator.fromObject: root cannot be a forward reference'
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
    ): LessThanBigIntValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'LessThanBigIntValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(
            LessThanBigIntValidator.prototype
        ) as LessThanBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'];
            if (__raw_value >= BigInt(1000)) {
                errors.push({
                    field: 'value',
                    message: 'must be less than 1000'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof LessThanBigIntValidator>(
        field: K,
        value: LessThanBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'value': {
                const __val = value as bigint;
                if (__val >= BigInt(1000)) {
                    errors.push({
                        field: 'value',
                        message: 'must be less than 1000'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<LessThanBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in partial && partial.value !== undefined) {
            const __val = partial.value as bigint;
            if (__val >= BigInt(1000)) {
                errors.push({
                    field: 'value',
                    message: 'must be less than 1000'
                });
            }
        }
        return errors;
    }
}

// LessThanOrEqualToBigInt validator
/** @derive(Deserialize) */
export class LessThanOrEqualToBigIntValidator {
    /** @serde({ validate: ["lessThanOrEqualToBigInt(1000)"] }) */
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        LessThanOrEqualToBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return LessThanOrEqualToBigIntValidator.fromObject(raw, opts);
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
        LessThanOrEqualToBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = LessThanOrEqualToBigIntValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'LessThanOrEqualToBigIntValidator.fromObject: root cannot be a forward reference'
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
    ): LessThanOrEqualToBigIntValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'LessThanOrEqualToBigIntValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(
            LessThanOrEqualToBigIntValidator.prototype
        ) as LessThanOrEqualToBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'];
            if (__raw_value > BigInt(1000)) {
                errors.push({
                    field: 'value',
                    message: 'must be less than or equal to 1000'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof LessThanOrEqualToBigIntValidator>(
        field: K,
        value: LessThanOrEqualToBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'value': {
                const __val = value as bigint;
                if (__val > BigInt(1000)) {
                    errors.push({
                        field: 'value',
                        message: 'must be less than or equal to 1000'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<LessThanOrEqualToBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in partial && partial.value !== undefined) {
            const __val = partial.value as bigint;
            if (__val > BigInt(1000)) {
                errors.push({
                    field: 'value',
                    message: 'must be less than or equal to 1000'
                });
            }
        }
        return errors;
    }
}

// BetweenBigInt validator
/** @derive(Deserialize) */
export class BetweenBigIntValidator {
    /** @serde({ validate: ["betweenBigInt(0, 1000)"] }) */
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        BetweenBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return BetweenBigIntValidator.fromObject(raw, opts);
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
        BetweenBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = BetweenBigIntValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'BetweenBigIntValidator.fromObject: root cannot be a forward reference'
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

    static __deserialize(value: any, ctx: DeserializeContext): BetweenBigIntValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'BetweenBigIntValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(BetweenBigIntValidator.prototype) as BetweenBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'];
            if (__raw_value < BigInt(0) || __raw_value > BigInt(1000)) {
                errors.push({
                    field: 'value',
                    message: 'must be between 0 and 1000'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof BetweenBigIntValidator>(
        field: K,
        value: BetweenBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'value': {
                const __val = value as bigint;
                if (__val < BigInt(0) || __val > BigInt(1000)) {
                    errors.push({
                        field: 'value',
                        message: 'must be between 0 and 1000'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<BetweenBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in partial && partial.value !== undefined) {
            const __val = partial.value as bigint;
            if (__val < BigInt(0) || __val > BigInt(1000)) {
                errors.push({
                    field: 'value',
                    message: 'must be between 0 and 1000'
                });
            }
        }
        return errors;
    }
}

// PositiveBigInt validator
/** @derive(Deserialize) */
export class PositiveBigIntValidator {
    /** @serde({ validate: ["positiveBigInt"] }) */
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        PositiveBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return PositiveBigIntValidator.fromObject(raw, opts);
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
        PositiveBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = PositiveBigIntValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'PositiveBigIntValidator.fromObject: root cannot be a forward reference'
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
    ): PositiveBigIntValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'PositiveBigIntValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(
            PositiveBigIntValidator.prototype
        ) as PositiveBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'];
            if (__raw_value <= 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be positive'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof PositiveBigIntValidator>(
        field: K,
        value: PositiveBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'value': {
                const __val = value as bigint;
                if (__val <= 0n) {
                    errors.push({
                        field: 'value',
                        message: 'must be positive'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<PositiveBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in partial && partial.value !== undefined) {
            const __val = partial.value as bigint;
            if (__val <= 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be positive'
                });
            }
        }
        return errors;
    }
}

// NonNegativeBigInt validator
/** @derive(Deserialize) */
export class NonNegativeBigIntValidator {
    /** @serde({ validate: ["nonNegativeBigInt"] }) */
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        NonNegativeBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return NonNegativeBigIntValidator.fromObject(raw, opts);
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
        NonNegativeBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = NonNegativeBigIntValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'NonNegativeBigIntValidator.fromObject: root cannot be a forward reference'
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
    ): NonNegativeBigIntValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'NonNegativeBigIntValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(
            NonNegativeBigIntValidator.prototype
        ) as NonNegativeBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'];
            if (__raw_value < 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be non-negative'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof NonNegativeBigIntValidator>(
        field: K,
        value: NonNegativeBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'value': {
                const __val = value as bigint;
                if (__val < 0n) {
                    errors.push({
                        field: 'value',
                        message: 'must be non-negative'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<NonNegativeBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in partial && partial.value !== undefined) {
            const __val = partial.value as bigint;
            if (__val < 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be non-negative'
                });
            }
        }
        return errors;
    }
}

// NegativeBigInt validator
/** @derive(Deserialize) */
export class NegativeBigIntValidator {
    /** @serde({ validate: ["negativeBigInt"] }) */
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        NegativeBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return NegativeBigIntValidator.fromObject(raw, opts);
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
        NegativeBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = NegativeBigIntValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'NegativeBigIntValidator.fromObject: root cannot be a forward reference'
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
    ): NegativeBigIntValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'NegativeBigIntValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(
            NegativeBigIntValidator.prototype
        ) as NegativeBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'];
            if (__raw_value >= 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be negative'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof NegativeBigIntValidator>(
        field: K,
        value: NegativeBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'value': {
                const __val = value as bigint;
                if (__val >= 0n) {
                    errors.push({
                        field: 'value',
                        message: 'must be negative'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<NegativeBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in partial && partial.value !== undefined) {
            const __val = partial.value as bigint;
            if (__val >= 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be negative'
                });
            }
        }
        return errors;
    }
}

// NonPositiveBigInt validator
/** @derive(Deserialize) */
export class NonPositiveBigIntValidator {
    /** @serde({ validate: ["nonPositiveBigInt"] }) */
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        NonPositiveBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return NonPositiveBigIntValidator.fromObject(raw, opts);
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
        NonPositiveBigIntValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = NonPositiveBigIntValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'NonPositiveBigIntValidator.fromObject: root cannot be a forward reference'
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
    ): NonPositiveBigIntValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'NonPositiveBigIntValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(
            NonPositiveBigIntValidator.prototype
        ) as NonPositiveBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'];
            if (__raw_value > 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be non-positive'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof NonPositiveBigIntValidator>(
        field: K,
        value: NonPositiveBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'value': {
                const __val = value as bigint;
                if (__val > 0n) {
                    errors.push({
                        field: 'value',
                        message: 'must be non-positive'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<NonPositiveBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in partial && partial.value !== undefined) {
            const __val = partial.value as bigint;
            if (__val > 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be non-positive'
                });
            }
        }
        return errors;
    }
}
