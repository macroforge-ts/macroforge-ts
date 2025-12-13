import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
/**
 * Date validator test classes for comprehensive deserializer validation testing.
 */

// ValidDate validator
/** @derive(Deserialize) */
export class ValidDateValidator {
    /** @serde({ validate: ["validDate"] }) */
    date: Date;

    constructor(props: {
        date: Date;
    }) {
        this.date = props.date;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        ValidDateValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return ValidDateValidator.fromObject(raw, opts);
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
        ValidDateValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = ValidDateValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'ValidDateValidator.fromObject: root cannot be a forward reference'
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

    static __deserialize(value: any, ctx: DeserializeContext): ValidDateValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'ValidDateValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('date' in obj)) {
            errors.push({
                field: 'date',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(ValidDateValidator.prototype) as ValidDateValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_date = obj['date'];
            {
                const __dateVal =
                    typeof __raw_date === 'string' ? new Date(__raw_date) : (__raw_date as Date);
                if (__dateVal == null || isNaN(__dateVal.getTime())) {
                    errors.push({
                        field: 'date',
                        message: 'must be a valid date'
                    });
                }
                instance.date = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof ValidDateValidator>(
        field: K,
        value: ValidDateValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'date': {
                const __val = value as Date;
                if (__val == null || isNaN(__val.getTime())) {
                    errors.push({
                        field: 'date',
                        message: 'must be a valid date'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<ValidDateValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('date' in partial && partial.date !== undefined) {
            const __val = partial.date as Date;
            if (__val == null || isNaN(__val.getTime())) {
                errors.push({
                    field: 'date',
                    message: 'must be a valid date'
                });
            }
        }
        return errors;
    }
}

// GreaterThanDate validator
/** @derive(Deserialize) */
export class GreaterThanDateValidator {
    /** @serde({ validate: ['greaterThanDate("2020-01-01")'] }) */
    date: Date;

    constructor(props: {
        date: Date;
    }) {
        this.date = props.date;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        GreaterThanDateValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return GreaterThanDateValidator.fromObject(raw, opts);
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
        GreaterThanDateValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = GreaterThanDateValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'GreaterThanDateValidator.fromObject: root cannot be a forward reference'
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
    ): GreaterThanDateValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'GreaterThanDateValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('date' in obj)) {
            errors.push({
                field: 'date',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(
            GreaterThanDateValidator.prototype
        ) as GreaterThanDateValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_date = obj['date'];
            {
                const __dateVal =
                    typeof __raw_date === 'string' ? new Date(__raw_date) : (__raw_date as Date);
                if (__dateVal == null || __dateVal.getTime() <= new Date('2020-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be after 2020-01-01'
                    });
                }
                instance.date = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof GreaterThanDateValidator>(
        field: K,
        value: GreaterThanDateValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'date': {
                const __val = value as Date;
                if (__val == null || __val.getTime() <= new Date('2020-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be after 2020-01-01'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<GreaterThanDateValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('date' in partial && partial.date !== undefined) {
            const __val = partial.date as Date;
            if (__val == null || __val.getTime() <= new Date('2020-01-01').getTime()) {
                errors.push({
                    field: 'date',
                    message: 'must be after 2020-01-01'
                });
            }
        }
        return errors;
    }
}

// GreaterThanOrEqualToDate validator
/** @derive(Deserialize) */
export class GreaterThanOrEqualToDateValidator {
    /** @serde({ validate: ['greaterThanOrEqualToDate("2020-01-01")'] }) */
    date: Date;

    constructor(props: {
        date: Date;
    }) {
        this.date = props.date;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        GreaterThanOrEqualToDateValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return GreaterThanOrEqualToDateValidator.fromObject(raw, opts);
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
        GreaterThanOrEqualToDateValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = GreaterThanOrEqualToDateValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'GreaterThanOrEqualToDateValidator.fromObject: root cannot be a forward reference'
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
    ): GreaterThanOrEqualToDateValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'GreaterThanOrEqualToDateValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('date' in obj)) {
            errors.push({
                field: 'date',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(
            GreaterThanOrEqualToDateValidator.prototype
        ) as GreaterThanOrEqualToDateValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_date = obj['date'];
            {
                const __dateVal =
                    typeof __raw_date === 'string' ? new Date(__raw_date) : (__raw_date as Date);
                if (__dateVal == null || __dateVal.getTime() < new Date('2020-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be on or after 2020-01-01'
                    });
                }
                instance.date = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof GreaterThanOrEqualToDateValidator>(
        field: K,
        value: GreaterThanOrEqualToDateValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'date': {
                const __val = value as Date;
                if (__val == null || __val.getTime() < new Date('2020-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be on or after 2020-01-01'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<GreaterThanOrEqualToDateValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('date' in partial && partial.date !== undefined) {
            const __val = partial.date as Date;
            if (__val == null || __val.getTime() < new Date('2020-01-01').getTime()) {
                errors.push({
                    field: 'date',
                    message: 'must be on or after 2020-01-01'
                });
            }
        }
        return errors;
    }
}

// LessThanDate validator
/** @derive(Deserialize) */
export class LessThanDateValidator {
    /** @serde({ validate: ['lessThanDate("2030-01-01")'] }) */
    date: Date;

    constructor(props: {
        date: Date;
    }) {
        this.date = props.date;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        LessThanDateValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return LessThanDateValidator.fromObject(raw, opts);
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
        LessThanDateValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = LessThanDateValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'LessThanDateValidator.fromObject: root cannot be a forward reference'
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

    static __deserialize(value: any, ctx: DeserializeContext): LessThanDateValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'LessThanDateValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('date' in obj)) {
            errors.push({
                field: 'date',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(LessThanDateValidator.prototype) as LessThanDateValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_date = obj['date'];
            {
                const __dateVal =
                    typeof __raw_date === 'string' ? new Date(__raw_date) : (__raw_date as Date);
                if (__dateVal == null || __dateVal.getTime() >= new Date('2030-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be before 2030-01-01'
                    });
                }
                instance.date = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof LessThanDateValidator>(
        field: K,
        value: LessThanDateValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'date': {
                const __val = value as Date;
                if (__val == null || __val.getTime() >= new Date('2030-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be before 2030-01-01'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<LessThanDateValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('date' in partial && partial.date !== undefined) {
            const __val = partial.date as Date;
            if (__val == null || __val.getTime() >= new Date('2030-01-01').getTime()) {
                errors.push({
                    field: 'date',
                    message: 'must be before 2030-01-01'
                });
            }
        }
        return errors;
    }
}

// LessThanOrEqualToDate validator
/** @derive(Deserialize) */
export class LessThanOrEqualToDateValidator {
    /** @serde({ validate: ['lessThanOrEqualToDate("2030-01-01")'] }) */
    date: Date;

    constructor(props: {
        date: Date;
    }) {
        this.date = props.date;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        LessThanOrEqualToDateValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return LessThanOrEqualToDateValidator.fromObject(raw, opts);
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
        LessThanOrEqualToDateValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = LessThanOrEqualToDateValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'LessThanOrEqualToDateValidator.fromObject: root cannot be a forward reference'
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
    ): LessThanOrEqualToDateValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'LessThanOrEqualToDateValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('date' in obj)) {
            errors.push({
                field: 'date',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(
            LessThanOrEqualToDateValidator.prototype
        ) as LessThanOrEqualToDateValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_date = obj['date'];
            {
                const __dateVal =
                    typeof __raw_date === 'string' ? new Date(__raw_date) : (__raw_date as Date);
                if (__dateVal == null || __dateVal.getTime() > new Date('2030-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be on or before 2030-01-01'
                    });
                }
                instance.date = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof LessThanOrEqualToDateValidator>(
        field: K,
        value: LessThanOrEqualToDateValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'date': {
                const __val = value as Date;
                if (__val == null || __val.getTime() > new Date('2030-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be on or before 2030-01-01'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<LessThanOrEqualToDateValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('date' in partial && partial.date !== undefined) {
            const __val = partial.date as Date;
            if (__val == null || __val.getTime() > new Date('2030-01-01').getTime()) {
                errors.push({
                    field: 'date',
                    message: 'must be on or before 2030-01-01'
                });
            }
        }
        return errors;
    }
}

// BetweenDate validator
/** @derive(Deserialize) */
export class BetweenDateValidator {
    /** @serde({ validate: ['betweenDate("2020-01-01", "2030-01-01")'] }) */
    date: Date;

    constructor(props: {
        date: Date;
    }) {
        this.date = props.date;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        BetweenDateValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return BetweenDateValidator.fromObject(raw, opts);
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
        BetweenDateValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = BetweenDateValidator.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'BetweenDateValidator.fromObject: root cannot be a forward reference'
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

    static __deserialize(value: any, ctx: DeserializeContext): BetweenDateValidator | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'BetweenDateValidator.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('date' in obj)) {
            errors.push({
                field: 'date',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(BetweenDateValidator.prototype) as BetweenDateValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_date = obj['date'];
            {
                const __dateVal =
                    typeof __raw_date === 'string' ? new Date(__raw_date) : (__raw_date as Date);
                if (
                    __dateVal == null ||
                    __dateVal.getTime() < new Date('2020-01-01').getTime() ||
                    __dateVal.getTime() > new Date('2030-01-01').getTime()
                ) {
                    errors.push({
                        field: 'date',
                        message: 'must be between 2020-01-01 and 2030-01-01'
                    });
                }
                instance.date = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof BetweenDateValidator>(
        field: K,
        value: BetweenDateValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'date': {
                const __val = value as Date;
                if (
                    __val == null ||
                    __val.getTime() < new Date('2020-01-01').getTime() ||
                    __val.getTime() > new Date('2030-01-01').getTime()
                ) {
                    errors.push({
                        field: 'date',
                        message: 'must be between 2020-01-01 and 2030-01-01'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<BetweenDateValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('date' in partial && partial.date !== undefined) {
            const __val = partial.date as Date;
            if (
                __val == null ||
                __val.getTime() < new Date('2020-01-01').getTime() ||
                __val.getTime() > new Date('2030-01-01').getTime()
            ) {
                errors.push({
                    field: 'date',
                    message: 'must be between 2020-01-01 and 2030-01-01'
                });
            }
        }
        return errors;
    }
}
