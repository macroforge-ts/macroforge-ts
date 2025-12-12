import { Result } from "macroforge/utils";
import { DeserializeContext } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
/**
 * Date validator test classes for comprehensive deserializer validation testing.
 */

// ValidDate validator
/**  */
export class ValidDateValidator {
  date: Date;

  constructor(props: { date: Date }) {
    this.date = props.date;
  }

  static fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<ValidDateValidator, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = ValidDateValidator.__deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "ValidDateValidator.fromStringifiedJSON: root cannot be a forward reference",
        ]);
      }
      ctx.applyPatches();
      if (opts?.freeze) {
        ctx.freezeAll();
      }
      return Result.ok(resultOrRef);
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      return Result.err(message.split("; "));
    }
  }

  static __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): ValidDateValidator | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("ValidDateValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("date" in obj)) {
      errors.push(
        'ValidDateValidator.__deserialize: missing required field "date"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance = Object.create(
      ValidDateValidator.prototype,
    ) as ValidDateValidator;
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_date = obj["date"];
      {
        const __dateVal =
          typeof __raw_date === "string"
            ? new Date(__raw_date)
            : (__raw_date as Date);
        if (__dateVal == null || isNaN(__dateVal.getTime())) {
          errors.push(
            "ValidDateValidator.fromStringifiedJSON: field 'date' must be a valid date",
          );
        }
        (instance as any).date = __dateVal;
      }
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance;
  }
}

// GreaterThanDate validator
/**  */
export class GreaterThanDateValidator {
  date: Date;

  constructor(props: { date: Date }) {
    this.date = props.date;
  }

  static fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<GreaterThanDateValidator, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = GreaterThanDateValidator.__deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "GreaterThanDateValidator.fromStringifiedJSON: root cannot be a forward reference",
        ]);
      }
      ctx.applyPatches();
      if (opts?.freeze) {
        ctx.freezeAll();
      }
      return Result.ok(resultOrRef);
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      return Result.err(message.split("; "));
    }
  }

  static __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): GreaterThanDateValidator | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error(
        "GreaterThanDateValidator.__deserialize: expected an object",
      );
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("date" in obj)) {
      errors.push(
        'GreaterThanDateValidator.__deserialize: missing required field "date"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance = Object.create(
      GreaterThanDateValidator.prototype,
    ) as GreaterThanDateValidator;
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_date = obj["date"];
      {
        const __dateVal =
          typeof __raw_date === "string"
            ? new Date(__raw_date)
            : (__raw_date as Date);
        if (
          __dateVal == null ||
          __dateVal.getTime() <= new Date("2020-01-01").getTime()
        ) {
          errors.push(
            "GreaterThanDateValidator.fromStringifiedJSON: field 'date' must be after 2020-01-01",
          );
        }
        (instance as any).date = __dateVal;
      }
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance;
  }
}

// GreaterThanOrEqualToDate validator
/**  */
export class GreaterThanOrEqualToDateValidator {
  date: Date;

  constructor(props: { date: Date }) {
    this.date = props.date;
  }

  static fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<GreaterThanOrEqualToDateValidator, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = GreaterThanOrEqualToDateValidator.__deserialize(
        raw,
        ctx,
      );
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "GreaterThanOrEqualToDateValidator.fromStringifiedJSON: root cannot be a forward reference",
        ]);
      }
      ctx.applyPatches();
      if (opts?.freeze) {
        ctx.freezeAll();
      }
      return Result.ok(resultOrRef);
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      return Result.err(message.split("; "));
    }
  }

  static __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): GreaterThanOrEqualToDateValidator | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error(
        "GreaterThanOrEqualToDateValidator.__deserialize: expected an object",
      );
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("date" in obj)) {
      errors.push(
        'GreaterThanOrEqualToDateValidator.__deserialize: missing required field "date"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance = Object.create(
      GreaterThanOrEqualToDateValidator.prototype,
    ) as GreaterThanOrEqualToDateValidator;
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_date = obj["date"];
      {
        const __dateVal =
          typeof __raw_date === "string"
            ? new Date(__raw_date)
            : (__raw_date as Date);
        if (
          __dateVal == null ||
          __dateVal.getTime() < new Date("2020-01-01").getTime()
        ) {
          errors.push(
            "GreaterThanOrEqualToDateValidator.fromStringifiedJSON: field 'date' must be on or after 2020-01-01",
          );
        }
        (instance as any).date = __dateVal;
      }
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance;
  }
}

// LessThanDate validator
/**  */
export class LessThanDateValidator {
  date: Date;

  constructor(props: { date: Date }) {
    this.date = props.date;
  }

  static fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<LessThanDateValidator, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = LessThanDateValidator.__deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "LessThanDateValidator.fromStringifiedJSON: root cannot be a forward reference",
        ]);
      }
      ctx.applyPatches();
      if (opts?.freeze) {
        ctx.freezeAll();
      }
      return Result.ok(resultOrRef);
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      return Result.err(message.split("; "));
    }
  }

  static __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): LessThanDateValidator | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error(
        "LessThanDateValidator.__deserialize: expected an object",
      );
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("date" in obj)) {
      errors.push(
        'LessThanDateValidator.__deserialize: missing required field "date"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance = Object.create(
      LessThanDateValidator.prototype,
    ) as LessThanDateValidator;
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_date = obj["date"];
      {
        const __dateVal =
          typeof __raw_date === "string"
            ? new Date(__raw_date)
            : (__raw_date as Date);
        if (
          __dateVal == null ||
          __dateVal.getTime() >= new Date("2030-01-01").getTime()
        ) {
          errors.push(
            "LessThanDateValidator.fromStringifiedJSON: field 'date' must be before 2030-01-01",
          );
        }
        (instance as any).date = __dateVal;
      }
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance;
  }
}

// LessThanOrEqualToDate validator
/**  */
export class LessThanOrEqualToDateValidator {
  date: Date;

  constructor(props: { date: Date }) {
    this.date = props.date;
  }

  static fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<LessThanOrEqualToDateValidator, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = LessThanOrEqualToDateValidator.__deserialize(
        raw,
        ctx,
      );
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "LessThanOrEqualToDateValidator.fromStringifiedJSON: root cannot be a forward reference",
        ]);
      }
      ctx.applyPatches();
      if (opts?.freeze) {
        ctx.freezeAll();
      }
      return Result.ok(resultOrRef);
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      return Result.err(message.split("; "));
    }
  }

  static __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): LessThanOrEqualToDateValidator | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error(
        "LessThanOrEqualToDateValidator.__deserialize: expected an object",
      );
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("date" in obj)) {
      errors.push(
        'LessThanOrEqualToDateValidator.__deserialize: missing required field "date"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance = Object.create(
      LessThanOrEqualToDateValidator.prototype,
    ) as LessThanOrEqualToDateValidator;
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_date = obj["date"];
      {
        const __dateVal =
          typeof __raw_date === "string"
            ? new Date(__raw_date)
            : (__raw_date as Date);
        if (
          __dateVal == null ||
          __dateVal.getTime() > new Date("2030-01-01").getTime()
        ) {
          errors.push(
            "LessThanOrEqualToDateValidator.fromStringifiedJSON: field 'date' must be on or before 2030-01-01",
          );
        }
        (instance as any).date = __dateVal;
      }
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance;
  }
}

// BetweenDate validator
/**  */
export class BetweenDateValidator {
  date: Date;

  constructor(props: { date: Date }) {
    this.date = props.date;
  }

  static fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<BetweenDateValidator, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = BetweenDateValidator.__deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "BetweenDateValidator.fromStringifiedJSON: root cannot be a forward reference",
        ]);
      }
      ctx.applyPatches();
      if (opts?.freeze) {
        ctx.freezeAll();
      }
      return Result.ok(resultOrRef);
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      return Result.err(message.split("; "));
    }
  }

  static __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): BetweenDateValidator | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("BetweenDateValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("date" in obj)) {
      errors.push(
        'BetweenDateValidator.__deserialize: missing required field "date"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance = Object.create(
      BetweenDateValidator.prototype,
    ) as BetweenDateValidator;
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_date = obj["date"];
      {
        const __dateVal =
          typeof __raw_date === "string"
            ? new Date(__raw_date)
            : (__raw_date as Date);
        if (
          __dateVal == null ||
          __dateVal.getTime() < new Date("2020-01-01").getTime() ||
          __dateVal.getTime() > new Date("2030-01-01").getTime()
        ) {
          errors.push(
            "BetweenDateValidator.fromStringifiedJSON: field 'date' must be between 2020-01-01 and 2030-01-01",
          );
        }
        (instance as any).date = __dateVal;
      }
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance;
  }
}
