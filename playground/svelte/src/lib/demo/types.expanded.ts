import { SerializeContext } from "macroforge/serde";
import { Result } from "macroforge/result";
import { DeserializeContext } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@dealdraft/macros"; */

/**  */
export interface User {
  id: string;
  email: string | null;

  firstName: string;

  lastName: string;
  password: string | null;
  metadata: Metadata | null;
  settings: Settings;
  role: UserRole;
  emailVerified: boolean;
  verificationToken: string | null;
  verificationExpires: string | null;
  passwordResetToken: string | null;
  passwordResetExpires: string | null;
  permissions: AppPermissions;
}

export namespace User {
  export function toJson(self: User): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: User,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "User", __id };
    result["id"] = self.id;
    if (self.email !== null) {
      result["email"] =
        typeof (self.email as any)?.__serialize === "function"
          ? (self.email as any).__serialize(ctx)
          : self.email;
    } else {
      result["email"] = null;
    }
    result["firstName"] = self.firstName;
    result["lastName"] = self.lastName;
    if (self.password !== null) {
      result["password"] =
        typeof (self.password as any)?.__serialize === "function"
          ? (self.password as any).__serialize(ctx)
          : self.password;
    } else {
      result["password"] = null;
    }
    if (self.metadata !== null) {
      result["metadata"] =
        typeof (self.metadata as any)?.__serialize === "function"
          ? (self.metadata as any).__serialize(ctx)
          : self.metadata;
    } else {
      result["metadata"] = null;
    }
    result["settings"] =
      typeof (self.settings as any)?.__serialize === "function"
        ? (self.settings as any).__serialize(ctx)
        : self.settings;
    result["role"] =
      typeof (self.role as any)?.__serialize === "function"
        ? (self.role as any).__serialize(ctx)
        : self.role;
    result["emailVerified"] = self.emailVerified;
    if (self.verificationToken !== null) {
      result["verificationToken"] =
        typeof (self.verificationToken as any)?.__serialize === "function"
          ? (self.verificationToken as any).__serialize(ctx)
          : self.verificationToken;
    } else {
      result["verificationToken"] = null;
    }
    if (self.verificationExpires !== null) {
      result["verificationExpires"] =
        typeof (self.verificationExpires as any)?.__serialize === "function"
          ? (self.verificationExpires as any).__serialize(ctx)
          : self.verificationExpires;
    } else {
      result["verificationExpires"] = null;
    }
    if (self.passwordResetToken !== null) {
      result["passwordResetToken"] =
        typeof (self.passwordResetToken as any)?.__serialize === "function"
          ? (self.passwordResetToken as any).__serialize(ctx)
          : self.passwordResetToken;
    } else {
      result["passwordResetToken"] = null;
    }
    if (self.passwordResetExpires !== null) {
      result["passwordResetExpires"] =
        typeof (self.passwordResetExpires as any)?.__serialize === "function"
          ? (self.passwordResetExpires as any).__serialize(ctx)
          : self.passwordResetExpires;
    } else {
      result["passwordResetExpires"] = null;
    }
    result["permissions"] =
      typeof (self.permissions as any)?.__serialize === "function"
        ? (self.permissions as any).__serialize(ctx)
        : self.permissions;
    return result;
  }
}

export namespace User {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<User, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err(["User.fromJson: root cannot be a forward reference"]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): User | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("User.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('User.__deserialize: missing required field "id"');
    }
    if (!("email" in obj)) {
      errors.push('User.__deserialize: missing required field "email"');
    }
    if (!("firstName" in obj)) {
      errors.push('User.__deserialize: missing required field "firstName"');
    }
    if (!("lastName" in obj)) {
      errors.push('User.__deserialize: missing required field "lastName"');
    }
    if (!("password" in obj)) {
      errors.push('User.__deserialize: missing required field "password"');
    }
    if (!("metadata" in obj)) {
      errors.push('User.__deserialize: missing required field "metadata"');
    }
    if (!("settings" in obj)) {
      errors.push('User.__deserialize: missing required field "settings"');
    }
    if (!("role" in obj)) {
      errors.push('User.__deserialize: missing required field "role"');
    }
    if (!("emailVerified" in obj)) {
      errors.push('User.__deserialize: missing required field "emailVerified"');
    }
    if (!("verificationToken" in obj)) {
      errors.push(
        'User.__deserialize: missing required field "verificationToken"',
      );
    }
    if (!("verificationExpires" in obj)) {
      errors.push(
        'User.__deserialize: missing required field "verificationExpires"',
      );
    }
    if (!("passwordResetToken" in obj)) {
      errors.push(
        'User.__deserialize: missing required field "passwordResetToken"',
      );
    }
    if (!("passwordResetExpires" in obj)) {
      errors.push(
        'User.__deserialize: missing required field "passwordResetExpires"',
      );
    }
    if (!("permissions" in obj)) {
      errors.push('User.__deserialize: missing required field "permissions"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_email = obj["email"];
      instance.email = __raw_email;
    }
    {
      const __raw_firstName = obj["firstName"];
      instance.firstName = __raw_firstName;
    }
    {
      const __raw_lastName = obj["lastName"];
      instance.lastName = __raw_lastName;
    }
    {
      const __raw_password = obj["password"];
      instance.password = __raw_password;
    }
    {
      const __raw_metadata = obj["metadata"];
      instance.metadata = __raw_metadata;
    }
    {
      const __raw_settings = obj["settings"];
      if (typeof (Settings as any)?.__deserialize === "function") {
        const __result = (Settings as any).__deserialize(__raw_settings, ctx);
        ctx.assignOrDefer(instance, "settings", __result);
      } else {
        instance.settings = __raw_settings;
      }
    }
    {
      const __raw_role = obj["role"];
      if (typeof (UserRole as any)?.__deserialize === "function") {
        const __result = (UserRole as any).__deserialize(__raw_role, ctx);
        ctx.assignOrDefer(instance, "role", __result);
      } else {
        instance.role = __raw_role;
      }
    }
    {
      const __raw_emailVerified = obj["emailVerified"];
      instance.emailVerified = __raw_emailVerified;
    }
    {
      const __raw_verificationToken = obj["verificationToken"];
      instance.verificationToken = __raw_verificationToken;
    }
    {
      const __raw_verificationExpires = obj["verificationExpires"];
      instance.verificationExpires = __raw_verificationExpires;
    }
    {
      const __raw_passwordResetToken = obj["passwordResetToken"];
      instance.passwordResetToken = __raw_passwordResetToken;
    }
    {
      const __raw_passwordResetExpires = obj["passwordResetExpires"];
      instance.passwordResetExpires = __raw_passwordResetExpires;
    }
    {
      const __raw_permissions = obj["permissions"];
      if (typeof (AppPermissions as any)?.__deserialize === "function") {
        const __result = (AppPermissions as any).__deserialize(
          __raw_permissions,
          ctx,
        );
        ctx.assignOrDefer(instance, "permissions", __result);
      } else {
        instance.permissions = __raw_permissions;
      }
    }
    return instance as User;
  }
}

/**  */
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

export namespace Service {
  export function toJson(self: Service): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Service,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Service", __id };
    result["id"] = self.id;
    result["name"] = self.name;
    result["quickCode"] = self.quickCode;
    if (self.group !== null) {
      result["group"] =
        typeof (self.group as any)?.__serialize === "function"
          ? (self.group as any).__serialize(ctx)
          : self.group;
    } else {
      result["group"] = null;
    }
    if (self.subgroup !== null) {
      result["subgroup"] =
        typeof (self.subgroup as any)?.__serialize === "function"
          ? (self.subgroup as any).__serialize(ctx)
          : self.subgroup;
    } else {
      result["subgroup"] = null;
    }
    if (self.unit !== null) {
      result["unit"] =
        typeof (self.unit as any)?.__serialize === "function"
          ? (self.unit as any).__serialize(ctx)
          : self.unit;
    } else {
      result["unit"] = null;
    }
    result["active"] = self.active;
    result["commission"] = self.commission;
    result["favorite"] = self.favorite;
    if (self.averageTime !== null) {
      result["averageTime"] =
        typeof (self.averageTime as any)?.__serialize === "function"
          ? (self.averageTime as any).__serialize(ctx)
          : self.averageTime;
    } else {
      result["averageTime"] = null;
    }
    result["defaults"] =
      typeof (self.defaults as any)?.__serialize === "function"
        ? (self.defaults as any).__serialize(ctx)
        : self.defaults;
    return result;
  }
}

export namespace Service {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Service, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Service.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Service | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Service.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('Service.__deserialize: missing required field "id"');
    }
    if (!("name" in obj)) {
      errors.push('Service.__deserialize: missing required field "name"');
    }
    if (!("quickCode" in obj)) {
      errors.push('Service.__deserialize: missing required field "quickCode"');
    }
    if (!("group" in obj)) {
      errors.push('Service.__deserialize: missing required field "group"');
    }
    if (!("subgroup" in obj)) {
      errors.push('Service.__deserialize: missing required field "subgroup"');
    }
    if (!("unit" in obj)) {
      errors.push('Service.__deserialize: missing required field "unit"');
    }
    if (!("active" in obj)) {
      errors.push('Service.__deserialize: missing required field "active"');
    }
    if (!("commission" in obj)) {
      errors.push('Service.__deserialize: missing required field "commission"');
    }
    if (!("favorite" in obj)) {
      errors.push('Service.__deserialize: missing required field "favorite"');
    }
    if (!("averageTime" in obj)) {
      errors.push(
        'Service.__deserialize: missing required field "averageTime"',
      );
    }
    if (!("defaults" in obj)) {
      errors.push('Service.__deserialize: missing required field "defaults"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_name = obj["name"];
      instance.name = __raw_name;
    }
    {
      const __raw_quickCode = obj["quickCode"];
      instance.quickCode = __raw_quickCode;
    }
    {
      const __raw_group = obj["group"];
      instance.group = __raw_group;
    }
    {
      const __raw_subgroup = obj["subgroup"];
      instance.subgroup = __raw_subgroup;
    }
    {
      const __raw_unit = obj["unit"];
      instance.unit = __raw_unit;
    }
    {
      const __raw_active = obj["active"];
      instance.active = __raw_active;
    }
    {
      const __raw_commission = obj["commission"];
      instance.commission = __raw_commission;
    }
    {
      const __raw_favorite = obj["favorite"];
      instance.favorite = __raw_favorite;
    }
    {
      const __raw_averageTime = obj["averageTime"];
      instance.averageTime = __raw_averageTime;
    }
    {
      const __raw_defaults = obj["defaults"];
      if (typeof (ServiceDefaults as any)?.__deserialize === "function") {
        const __result = (ServiceDefaults as any).__deserialize(
          __raw_defaults,
          ctx,
        );
        ctx.assignOrDefer(instance, "defaults", __result);
      } else {
        instance.defaults = __raw_defaults;
      }
    }
    return instance as Service;
  }
}

/**  */
export interface ServiceDefaults {
  price: number;

  description: string;
}

export namespace ServiceDefaults {
  export function defaultValue(): ServiceDefaults {
    return { price: 0, description: "" } as ServiceDefaults;
  }
}

export namespace ServiceDefaults {
  export function toJson(self: ServiceDefaults): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: ServiceDefaults,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "ServiceDefaults", __id };
    result["price"] = self.price;
    result["description"] = self.description;
    return result;
  }
}

export namespace ServiceDefaults {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<ServiceDefaults, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "ServiceDefaults.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): ServiceDefaults | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("ServiceDefaults.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("price" in obj)) {
      errors.push(
        'ServiceDefaults.__deserialize: missing required field "price"',
      );
    }
    if (!("description" in obj)) {
      errors.push(
        'ServiceDefaults.__deserialize: missing required field "description"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_price = obj["price"];
      instance.price = __raw_price;
    }
    {
      const __raw_description = obj["description"];
      instance.description = __raw_description;
    }
    return instance as ServiceDefaults;
  }
}

/**  */
export interface Did {
  in: string | Actor;
  out: string | Target;
  id: string;
  activityType: ActivityType;
  createdAt: string;
  metadata: string | null;
}

export namespace Did {
  export function toJson(self: Did): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Did,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Did", __id };
    result["in"] = self.in;
    result["out"] = self.out;
    result["id"] = self.id;
    result["activityType"] =
      typeof (self.activityType as any)?.__serialize === "function"
        ? (self.activityType as any).__serialize(ctx)
        : self.activityType;
    result["createdAt"] = self.createdAt;
    if (self.metadata !== null) {
      result["metadata"] =
        typeof (self.metadata as any)?.__serialize === "function"
          ? (self.metadata as any).__serialize(ctx)
          : self.metadata;
    } else {
      result["metadata"] = null;
    }
    return result;
  }
}

export namespace Did {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Did, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err(["Did.fromJson: root cannot be a forward reference"]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Did | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Did.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("in" in obj)) {
      errors.push('Did.__deserialize: missing required field "in"');
    }
    if (!("out" in obj)) {
      errors.push('Did.__deserialize: missing required field "out"');
    }
    if (!("id" in obj)) {
      errors.push('Did.__deserialize: missing required field "id"');
    }
    if (!("activityType" in obj)) {
      errors.push('Did.__deserialize: missing required field "activityType"');
    }
    if (!("createdAt" in obj)) {
      errors.push('Did.__deserialize: missing required field "createdAt"');
    }
    if (!("metadata" in obj)) {
      errors.push('Did.__deserialize: missing required field "metadata"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_in = obj["in"];
      instance.in = __raw_in;
    }
    {
      const __raw_out = obj["out"];
      instance.out = __raw_out;
    }
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_activityType = obj["activityType"];
      if (typeof (ActivityType as any)?.__deserialize === "function") {
        const __result = (ActivityType as any).__deserialize(
          __raw_activityType,
          ctx,
        );
        ctx.assignOrDefer(instance, "activityType", __result);
      } else {
        instance.activityType = __raw_activityType;
      }
    }
    {
      const __raw_createdAt = obj["createdAt"];
      instance.createdAt = __raw_createdAt;
    }
    {
      const __raw_metadata = obj["metadata"];
      instance.metadata = __raw_metadata;
    }
    return instance as Did;
  }
}

/**  */
export interface PersonName {
  firstName: string;

  lastName: string;
}

export namespace PersonName {
  export function defaultValue(): PersonName {
    return { firstName: "", lastName: "" } as PersonName;
  }
}

export namespace PersonName {
  export function toJson(self: PersonName): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: PersonName,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "PersonName", __id };
    result["firstName"] = self.firstName;
    result["lastName"] = self.lastName;
    return result;
  }
}

export namespace PersonName {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<PersonName, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "PersonName.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): PersonName | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("PersonName.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("firstName" in obj)) {
      errors.push(
        'PersonName.__deserialize: missing required field "firstName"',
      );
    }
    if (!("lastName" in obj)) {
      errors.push(
        'PersonName.__deserialize: missing required field "lastName"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_firstName = obj["firstName"];
      instance.firstName = __raw_firstName;
    }
    {
      const __raw_lastName = obj["lastName"];
      instance.lastName = __raw_lastName;
    }
    return instance as PersonName;
  }
}

/**  */
export interface Promotion {
  id: string;
  date: string;
}

export namespace Promotion {
  export function defaultValue(): Promotion {
    return { id: "", date: "" } as Promotion;
  }
}

export namespace Promotion {
  export function toJson(self: Promotion): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Promotion,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Promotion", __id };
    result["id"] = self.id;
    result["date"] = self.date;
    return result;
  }
}

export namespace Promotion {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Promotion, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Promotion.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Promotion | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Promotion.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('Promotion.__deserialize: missing required field "id"');
    }
    if (!("date" in obj)) {
      errors.push('Promotion.__deserialize: missing required field "date"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_date = obj["date"];
      instance.date = __raw_date;
    }
    return instance as Promotion;
  }
}

/**  */
export interface Site {
  id: string;

  addressLine1: string;
  addressLine2: string | null;
  sublocalityLevel1: string | null;

  locality: string;
  administrativeAreaLevel3: string | null;
  administrativeAreaLevel2: string | null;

  administrativeAreaLevel1: string;

  country: string;

  postalCode: string;
  postalCodeSuffix: string | null;
  coordinates: Coordinates;
}

export namespace Site {
  export function toJson(self: Site): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Site,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Site", __id };
    result["id"] = self.id;
    result["addressLine1"] = self.addressLine1;
    if (self.addressLine2 !== null) {
      result["addressLine2"] =
        typeof (self.addressLine2 as any)?.__serialize === "function"
          ? (self.addressLine2 as any).__serialize(ctx)
          : self.addressLine2;
    } else {
      result["addressLine2"] = null;
    }
    if (self.sublocalityLevel1 !== null) {
      result["sublocalityLevel1"] =
        typeof (self.sublocalityLevel1 as any)?.__serialize === "function"
          ? (self.sublocalityLevel1 as any).__serialize(ctx)
          : self.sublocalityLevel1;
    } else {
      result["sublocalityLevel1"] = null;
    }
    result["locality"] = self.locality;
    if (self.administrativeAreaLevel3 !== null) {
      result["administrativeAreaLevel3"] =
        typeof (self.administrativeAreaLevel3 as any)?.__serialize ===
        "function"
          ? (self.administrativeAreaLevel3 as any).__serialize(ctx)
          : self.administrativeAreaLevel3;
    } else {
      result["administrativeAreaLevel3"] = null;
    }
    if (self.administrativeAreaLevel2 !== null) {
      result["administrativeAreaLevel2"] =
        typeof (self.administrativeAreaLevel2 as any)?.__serialize ===
        "function"
          ? (self.administrativeAreaLevel2 as any).__serialize(ctx)
          : self.administrativeAreaLevel2;
    } else {
      result["administrativeAreaLevel2"] = null;
    }
    result["administrativeAreaLevel1"] = self.administrativeAreaLevel1;
    result["country"] = self.country;
    result["postalCode"] = self.postalCode;
    if (self.postalCodeSuffix !== null) {
      result["postalCodeSuffix"] =
        typeof (self.postalCodeSuffix as any)?.__serialize === "function"
          ? (self.postalCodeSuffix as any).__serialize(ctx)
          : self.postalCodeSuffix;
    } else {
      result["postalCodeSuffix"] = null;
    }
    result["coordinates"] =
      typeof (self.coordinates as any)?.__serialize === "function"
        ? (self.coordinates as any).__serialize(ctx)
        : self.coordinates;
    return result;
  }
}

export namespace Site {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Site, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err(["Site.fromJson: root cannot be a forward reference"]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Site | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Site.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('Site.__deserialize: missing required field "id"');
    }
    if (!("addressLine1" in obj)) {
      errors.push('Site.__deserialize: missing required field "addressLine1"');
    }
    if (!("addressLine2" in obj)) {
      errors.push('Site.__deserialize: missing required field "addressLine2"');
    }
    if (!("sublocalityLevel1" in obj)) {
      errors.push(
        'Site.__deserialize: missing required field "sublocalityLevel1"',
      );
    }
    if (!("locality" in obj)) {
      errors.push('Site.__deserialize: missing required field "locality"');
    }
    if (!("administrativeAreaLevel3" in obj)) {
      errors.push(
        'Site.__deserialize: missing required field "administrativeAreaLevel3"',
      );
    }
    if (!("administrativeAreaLevel2" in obj)) {
      errors.push(
        'Site.__deserialize: missing required field "administrativeAreaLevel2"',
      );
    }
    if (!("administrativeAreaLevel1" in obj)) {
      errors.push(
        'Site.__deserialize: missing required field "administrativeAreaLevel1"',
      );
    }
    if (!("country" in obj)) {
      errors.push('Site.__deserialize: missing required field "country"');
    }
    if (!("postalCode" in obj)) {
      errors.push('Site.__deserialize: missing required field "postalCode"');
    }
    if (!("postalCodeSuffix" in obj)) {
      errors.push(
        'Site.__deserialize: missing required field "postalCodeSuffix"',
      );
    }
    if (!("coordinates" in obj)) {
      errors.push('Site.__deserialize: missing required field "coordinates"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_addressLine1 = obj["addressLine1"];
      instance.addressLine1 = __raw_addressLine1;
    }
    {
      const __raw_addressLine2 = obj["addressLine2"];
      instance.addressLine2 = __raw_addressLine2;
    }
    {
      const __raw_sublocalityLevel1 = obj["sublocalityLevel1"];
      instance.sublocalityLevel1 = __raw_sublocalityLevel1;
    }
    {
      const __raw_locality = obj["locality"];
      instance.locality = __raw_locality;
    }
    {
      const __raw_administrativeAreaLevel3 = obj["administrativeAreaLevel3"];
      instance.administrativeAreaLevel3 = __raw_administrativeAreaLevel3;
    }
    {
      const __raw_administrativeAreaLevel2 = obj["administrativeAreaLevel2"];
      instance.administrativeAreaLevel2 = __raw_administrativeAreaLevel2;
    }
    {
      const __raw_administrativeAreaLevel1 = obj["administrativeAreaLevel1"];
      instance.administrativeAreaLevel1 = __raw_administrativeAreaLevel1;
    }
    {
      const __raw_country = obj["country"];
      instance.country = __raw_country;
    }
    {
      const __raw_postalCode = obj["postalCode"];
      instance.postalCode = __raw_postalCode;
    }
    {
      const __raw_postalCodeSuffix = obj["postalCodeSuffix"];
      instance.postalCodeSuffix = __raw_postalCodeSuffix;
    }
    {
      const __raw_coordinates = obj["coordinates"];
      if (typeof (Coordinates as any)?.__deserialize === "function") {
        const __result = (Coordinates as any).__deserialize(
          __raw_coordinates,
          ctx,
        );
        ctx.assignOrDefer(instance, "coordinates", __result);
      } else {
        instance.coordinates = __raw_coordinates;
      }
    }
    return instance as Site;
  }
}

/**  */
export interface Metadata {
  createdAt: string;
  lastLogin: string | null;
  isActive: boolean;
  roles: string[];
}

export namespace Metadata {
  export function defaultValue(): Metadata {
    return {
      createdAt: "",
      lastLogin: null,
      isActive: false,
      roles: [],
    } as Metadata;
  }
}

export namespace Metadata {
  export function toJson(self: Metadata): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Metadata,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Metadata", __id };
    result["createdAt"] = self.createdAt;
    if (self.lastLogin !== null) {
      result["lastLogin"] =
        typeof (self.lastLogin as any)?.__serialize === "function"
          ? (self.lastLogin as any).__serialize(ctx)
          : self.lastLogin;
    } else {
      result["lastLogin"] = null;
    }
    result["isActive"] = self.isActive;
    result["roles"] = self.roles.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    return result;
  }
}

export namespace Metadata {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Metadata, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Metadata.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Metadata | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Metadata.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("createdAt" in obj)) {
      errors.push('Metadata.__deserialize: missing required field "createdAt"');
    }
    if (!("lastLogin" in obj)) {
      errors.push('Metadata.__deserialize: missing required field "lastLogin"');
    }
    if (!("isActive" in obj)) {
      errors.push('Metadata.__deserialize: missing required field "isActive"');
    }
    if (!("roles" in obj)) {
      errors.push('Metadata.__deserialize: missing required field "roles"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_createdAt = obj["createdAt"];
      instance.createdAt = __raw_createdAt;
    }
    {
      const __raw_lastLogin = obj["lastLogin"];
      instance.lastLogin = __raw_lastLogin;
    }
    {
      const __raw_isActive = obj["isActive"];
      instance.isActive = __raw_isActive;
    }
    {
      const __raw_roles = obj["roles"];
      instance.roles = __raw_roles;
    }
    return instance as Metadata;
  }
}

/**  */
export interface ColumnConfig {
  heading: string;
  dataPath: DataPath;
}

export namespace ColumnConfig {
  export function toJson(self: ColumnConfig): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: ColumnConfig,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "ColumnConfig", __id };
    result["heading"] = self.heading;
    result["dataPath"] =
      typeof (self.dataPath as any)?.__serialize === "function"
        ? (self.dataPath as any).__serialize(ctx)
        : self.dataPath;
    return result;
  }
}

export namespace ColumnConfig {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<ColumnConfig, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "ColumnConfig.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): ColumnConfig | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("ColumnConfig.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("heading" in obj)) {
      errors.push(
        'ColumnConfig.__deserialize: missing required field "heading"',
      );
    }
    if (!("dataPath" in obj)) {
      errors.push(
        'ColumnConfig.__deserialize: missing required field "dataPath"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_heading = obj["heading"];
      instance.heading = __raw_heading;
    }
    {
      const __raw_dataPath = obj["dataPath"];
      if (typeof (DataPath as any)?.__deserialize === "function") {
        const __result = (DataPath as any).__deserialize(__raw_dataPath, ctx);
        ctx.assignOrDefer(instance, "dataPath", __result);
      } else {
        instance.dataPath = __raw_dataPath;
      }
    }
    return instance as ColumnConfig;
  }
}

/**  */
export interface PhoneNumber {
  main: boolean;

  phoneType: string;

  number: string;
  canText: boolean;
  canCall: boolean;
}

export namespace PhoneNumber {
  export function defaultValue(): PhoneNumber {
    return {
      main: false,
      phoneType: "",
      number: "",
      canText: false,
      canCall: false,
    } as PhoneNumber;
  }
}

export namespace PhoneNumber {
  export function toJson(self: PhoneNumber): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: PhoneNumber,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "PhoneNumber", __id };
    result["main"] = self.main;
    result["phoneType"] = self.phoneType;
    result["number"] = self.number;
    result["canText"] = self.canText;
    result["canCall"] = self.canCall;
    return result;
  }
}

export namespace PhoneNumber {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<PhoneNumber, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "PhoneNumber.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): PhoneNumber | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("PhoneNumber.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("main" in obj)) {
      errors.push('PhoneNumber.__deserialize: missing required field "main"');
    }
    if (!("phoneType" in obj)) {
      errors.push(
        'PhoneNumber.__deserialize: missing required field "phoneType"',
      );
    }
    if (!("number" in obj)) {
      errors.push('PhoneNumber.__deserialize: missing required field "number"');
    }
    if (!("canText" in obj)) {
      errors.push(
        'PhoneNumber.__deserialize: missing required field "canText"',
      );
    }
    if (!("canCall" in obj)) {
      errors.push(
        'PhoneNumber.__deserialize: missing required field "canCall"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_main = obj["main"];
      instance.main = __raw_main;
    }
    {
      const __raw_phoneType = obj["phoneType"];
      instance.phoneType = __raw_phoneType;
    }
    {
      const __raw_number = obj["number"];
      instance.number = __raw_number;
    }
    {
      const __raw_canText = obj["canText"];
      instance.canText = __raw_canText;
    }
    {
      const __raw_canCall = obj["canCall"];
      instance.canCall = __raw_canCall;
    }
    return instance as PhoneNumber;
  }
}

/**  */
export interface Gradient {
  startHue: number;
}

export namespace Gradient {
  export function defaultValue(): Gradient {
    return { startHue: 0 } as Gradient;
  }
}

export namespace Gradient {
  export function toJson(self: Gradient): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Gradient,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Gradient", __id };
    result["startHue"] = self.startHue;
    return result;
  }
}

export namespace Gradient {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Gradient, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Gradient.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Gradient | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Gradient.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("startHue" in obj)) {
      errors.push('Gradient.__deserialize: missing required field "startHue"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_startHue = obj["startHue"];
      instance.startHue = __raw_startHue;
    }
    return instance as Gradient;
  }
}

/**  */
export interface Product {
  id: string;

  name: string;

  quickCode: string;
  group: string | null;
  subgroup: string | null;
  unit: string | null;
  active: boolean;
  commission: boolean;
  favorite: boolean;
  defaults: ProductDefaults;
}

export namespace Product {
  export function toJson(self: Product): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Product,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Product", __id };
    result["id"] = self.id;
    result["name"] = self.name;
    result["quickCode"] = self.quickCode;
    if (self.group !== null) {
      result["group"] =
        typeof (self.group as any)?.__serialize === "function"
          ? (self.group as any).__serialize(ctx)
          : self.group;
    } else {
      result["group"] = null;
    }
    if (self.subgroup !== null) {
      result["subgroup"] =
        typeof (self.subgroup as any)?.__serialize === "function"
          ? (self.subgroup as any).__serialize(ctx)
          : self.subgroup;
    } else {
      result["subgroup"] = null;
    }
    if (self.unit !== null) {
      result["unit"] =
        typeof (self.unit as any)?.__serialize === "function"
          ? (self.unit as any).__serialize(ctx)
          : self.unit;
    } else {
      result["unit"] = null;
    }
    result["active"] = self.active;
    result["commission"] = self.commission;
    result["favorite"] = self.favorite;
    result["defaults"] =
      typeof (self.defaults as any)?.__serialize === "function"
        ? (self.defaults as any).__serialize(ctx)
        : self.defaults;
    return result;
  }
}

export namespace Product {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Product, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Product.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Product | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Product.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('Product.__deserialize: missing required field "id"');
    }
    if (!("name" in obj)) {
      errors.push('Product.__deserialize: missing required field "name"');
    }
    if (!("quickCode" in obj)) {
      errors.push('Product.__deserialize: missing required field "quickCode"');
    }
    if (!("group" in obj)) {
      errors.push('Product.__deserialize: missing required field "group"');
    }
    if (!("subgroup" in obj)) {
      errors.push('Product.__deserialize: missing required field "subgroup"');
    }
    if (!("unit" in obj)) {
      errors.push('Product.__deserialize: missing required field "unit"');
    }
    if (!("active" in obj)) {
      errors.push('Product.__deserialize: missing required field "active"');
    }
    if (!("commission" in obj)) {
      errors.push('Product.__deserialize: missing required field "commission"');
    }
    if (!("favorite" in obj)) {
      errors.push('Product.__deserialize: missing required field "favorite"');
    }
    if (!("defaults" in obj)) {
      errors.push('Product.__deserialize: missing required field "defaults"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_name = obj["name"];
      instance.name = __raw_name;
    }
    {
      const __raw_quickCode = obj["quickCode"];
      instance.quickCode = __raw_quickCode;
    }
    {
      const __raw_group = obj["group"];
      instance.group = __raw_group;
    }
    {
      const __raw_subgroup = obj["subgroup"];
      instance.subgroup = __raw_subgroup;
    }
    {
      const __raw_unit = obj["unit"];
      instance.unit = __raw_unit;
    }
    {
      const __raw_active = obj["active"];
      instance.active = __raw_active;
    }
    {
      const __raw_commission = obj["commission"];
      instance.commission = __raw_commission;
    }
    {
      const __raw_favorite = obj["favorite"];
      instance.favorite = __raw_favorite;
    }
    {
      const __raw_defaults = obj["defaults"];
      if (typeof (ProductDefaults as any)?.__deserialize === "function") {
        const __result = (ProductDefaults as any).__deserialize(
          __raw_defaults,
          ctx,
        );
        ctx.assignOrDefer(instance, "defaults", __result);
      } else {
        instance.defaults = __raw_defaults;
      }
    }
    return instance as Product;
  }
}

/**  */
export interface YearlyRecurrenceRule {
  quantityOfYears: number;
}

export namespace YearlyRecurrenceRule {
  export function defaultValue(): YearlyRecurrenceRule {
    return { quantityOfYears: 0 } as YearlyRecurrenceRule;
  }
}

export namespace YearlyRecurrenceRule {
  export function toJson(self: YearlyRecurrenceRule): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: YearlyRecurrenceRule,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = {
      __type: "YearlyRecurrenceRule",
      __id,
    };
    result["quantityOfYears"] = self.quantityOfYears;
    return result;
  }
}

export namespace YearlyRecurrenceRule {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<YearlyRecurrenceRule, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "YearlyRecurrenceRule.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): YearlyRecurrenceRule | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("YearlyRecurrenceRule.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("quantityOfYears" in obj)) {
      errors.push(
        'YearlyRecurrenceRule.__deserialize: missing required field "quantityOfYears"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_quantityOfYears = obj["quantityOfYears"];
      instance.quantityOfYears = __raw_quantityOfYears;
    }
    return instance as YearlyRecurrenceRule;
  }
}

/**  */
export interface AppointmentNotifications {
  personalScheduleChangeNotifications: string;

  allScheduleChangeNotifications: string;
}

export namespace AppointmentNotifications {
  export function defaultValue(): AppointmentNotifications {
    return {
      personalScheduleChangeNotifications: "",
      allScheduleChangeNotifications: "",
    } as AppointmentNotifications;
  }
}

export namespace AppointmentNotifications {
  export function toJson(self: AppointmentNotifications): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: AppointmentNotifications,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = {
      __type: "AppointmentNotifications",
      __id,
    };
    result["personalScheduleChangeNotifications"] =
      self.personalScheduleChangeNotifications;
    result["allScheduleChangeNotifications"] =
      self.allScheduleChangeNotifications;
    return result;
  }
}

export namespace AppointmentNotifications {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<AppointmentNotifications, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "AppointmentNotifications.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): AppointmentNotifications | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error(
        "AppointmentNotifications.__deserialize: expected an object",
      );
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("personalScheduleChangeNotifications" in obj)) {
      errors.push(
        'AppointmentNotifications.__deserialize: missing required field "personalScheduleChangeNotifications"',
      );
    }
    if (!("allScheduleChangeNotifications" in obj)) {
      errors.push(
        'AppointmentNotifications.__deserialize: missing required field "allScheduleChangeNotifications"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_personalScheduleChangeNotifications =
        obj["personalScheduleChangeNotifications"];
      instance.personalScheduleChangeNotifications =
        __raw_personalScheduleChangeNotifications;
    }
    {
      const __raw_allScheduleChangeNotifications =
        obj["allScheduleChangeNotifications"];
      instance.allScheduleChangeNotifications =
        __raw_allScheduleChangeNotifications;
    }
    return instance as AppointmentNotifications;
  }
}

/**  */
export interface DirectionHue {
  bearing: number;
  hue: number;
}

export namespace DirectionHue {
  export function defaultValue(): DirectionHue {
    return { bearing: 0, hue: 0 } as DirectionHue;
  }
}

export namespace DirectionHue {
  export function toJson(self: DirectionHue): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: DirectionHue,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "DirectionHue", __id };
    result["bearing"] = self.bearing;
    result["hue"] = self.hue;
    return result;
  }
}

export namespace DirectionHue {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<DirectionHue, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "DirectionHue.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): DirectionHue | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("DirectionHue.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("bearing" in obj)) {
      errors.push(
        'DirectionHue.__deserialize: missing required field "bearing"',
      );
    }
    if (!("hue" in obj)) {
      errors.push('DirectionHue.__deserialize: missing required field "hue"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_bearing = obj["bearing"];
      instance.bearing = __raw_bearing;
    }
    {
      const __raw_hue = obj["hue"];
      instance.hue = __raw_hue;
    }
    return instance as DirectionHue;
  }
}

/**  */
export interface MonthlyRecurrenceRule {
  quantityOfMonths: number;
  day: number;

  name: string;
}

export namespace MonthlyRecurrenceRule {
  export function defaultValue(): MonthlyRecurrenceRule {
    return { quantityOfMonths: 0, day: 0, name: "" } as MonthlyRecurrenceRule;
  }
}

export namespace MonthlyRecurrenceRule {
  export function toJson(self: MonthlyRecurrenceRule): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: MonthlyRecurrenceRule,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = {
      __type: "MonthlyRecurrenceRule",
      __id,
    };
    result["quantityOfMonths"] = self.quantityOfMonths;
    result["day"] = self.day;
    result["name"] = self.name;
    return result;
  }
}

export namespace MonthlyRecurrenceRule {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<MonthlyRecurrenceRule, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "MonthlyRecurrenceRule.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): MonthlyRecurrenceRule | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error(
        "MonthlyRecurrenceRule.__deserialize: expected an object",
      );
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("quantityOfMonths" in obj)) {
      errors.push(
        'MonthlyRecurrenceRule.__deserialize: missing required field "quantityOfMonths"',
      );
    }
    if (!("day" in obj)) {
      errors.push(
        'MonthlyRecurrenceRule.__deserialize: missing required field "day"',
      );
    }
    if (!("name" in obj)) {
      errors.push(
        'MonthlyRecurrenceRule.__deserialize: missing required field "name"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_quantityOfMonths = obj["quantityOfMonths"];
      instance.quantityOfMonths = __raw_quantityOfMonths;
    }
    {
      const __raw_day = obj["day"];
      instance.day = __raw_day;
    }
    {
      const __raw_name = obj["name"];
      instance.name = __raw_name;
    }
    return instance as MonthlyRecurrenceRule;
  }
}

/**  */
export interface Represents {
  in: string | Employee;
  out: string | Account;
  id: string;
  dateStarted: string;
}

export namespace Represents {
  export function toJson(self: Represents): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Represents,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Represents", __id };
    result["in"] = self.in;
    result["out"] = self.out;
    result["id"] = self.id;
    result["dateStarted"] = self.dateStarted;
    return result;
  }
}

export namespace Represents {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Represents, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Represents.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Represents | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Represents.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("in" in obj)) {
      errors.push('Represents.__deserialize: missing required field "in"');
    }
    if (!("out" in obj)) {
      errors.push('Represents.__deserialize: missing required field "out"');
    }
    if (!("id" in obj)) {
      errors.push('Represents.__deserialize: missing required field "id"');
    }
    if (!("dateStarted" in obj)) {
      errors.push(
        'Represents.__deserialize: missing required field "dateStarted"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_in = obj["in"];
      instance.in = __raw_in;
    }
    {
      const __raw_out = obj["out"];
      instance.out = __raw_out;
    }
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_dateStarted = obj["dateStarted"];
      instance.dateStarted = __raw_dateStarted;
    }
    return instance as Represents;
  }
}

/**  */
export interface Payment {
  id: string;
  date: string;
}

export namespace Payment {
  export function defaultValue(): Payment {
    return { id: "", date: "" } as Payment;
  }
}

export namespace Payment {
  export function toJson(self: Payment): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Payment,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Payment", __id };
    result["id"] = self.id;
    result["date"] = self.date;
    return result;
  }
}

export namespace Payment {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Payment, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Payment.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Payment | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Payment.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('Payment.__deserialize: missing required field "id"');
    }
    if (!("date" in obj)) {
      errors.push('Payment.__deserialize: missing required field "date"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_date = obj["date"];
      instance.date = __raw_date;
    }
    return instance as Payment;
  }
}

/**  */
export interface Settings {
  appointmentNotifications: AppointmentNotifications | null;
  commissions: Commissions | null;
  scheduleSettings: ScheduleSettings;
  accountOverviewSettings: OverviewSettings;
  serviceOverviewSettings: OverviewSettings;
  appointmentOverviewSettings: OverviewSettings;
  leadOverviewSettings: OverviewSettings;
  packageOverviewSettings: OverviewSettings;
  productOverviewSettings: OverviewSettings;
  orderOverviewSettings: OverviewSettings;
  taxRateOverviewSettings: OverviewSettings;
  homePage: Page;
}

export namespace Settings {
  export function toJson(self: Settings): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Settings,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Settings", __id };
    if (self.appointmentNotifications !== null) {
      result["appointmentNotifications"] =
        typeof (self.appointmentNotifications as any)?.__serialize ===
        "function"
          ? (self.appointmentNotifications as any).__serialize(ctx)
          : self.appointmentNotifications;
    } else {
      result["appointmentNotifications"] = null;
    }
    if (self.commissions !== null) {
      result["commissions"] =
        typeof (self.commissions as any)?.__serialize === "function"
          ? (self.commissions as any).__serialize(ctx)
          : self.commissions;
    } else {
      result["commissions"] = null;
    }
    result["scheduleSettings"] =
      typeof (self.scheduleSettings as any)?.__serialize === "function"
        ? (self.scheduleSettings as any).__serialize(ctx)
        : self.scheduleSettings;
    result["accountOverviewSettings"] =
      typeof (self.accountOverviewSettings as any)?.__serialize === "function"
        ? (self.accountOverviewSettings as any).__serialize(ctx)
        : self.accountOverviewSettings;
    result["serviceOverviewSettings"] =
      typeof (self.serviceOverviewSettings as any)?.__serialize === "function"
        ? (self.serviceOverviewSettings as any).__serialize(ctx)
        : self.serviceOverviewSettings;
    result["appointmentOverviewSettings"] =
      typeof (self.appointmentOverviewSettings as any)?.__serialize ===
      "function"
        ? (self.appointmentOverviewSettings as any).__serialize(ctx)
        : self.appointmentOverviewSettings;
    result["leadOverviewSettings"] =
      typeof (self.leadOverviewSettings as any)?.__serialize === "function"
        ? (self.leadOverviewSettings as any).__serialize(ctx)
        : self.leadOverviewSettings;
    result["packageOverviewSettings"] =
      typeof (self.packageOverviewSettings as any)?.__serialize === "function"
        ? (self.packageOverviewSettings as any).__serialize(ctx)
        : self.packageOverviewSettings;
    result["productOverviewSettings"] =
      typeof (self.productOverviewSettings as any)?.__serialize === "function"
        ? (self.productOverviewSettings as any).__serialize(ctx)
        : self.productOverviewSettings;
    result["orderOverviewSettings"] =
      typeof (self.orderOverviewSettings as any)?.__serialize === "function"
        ? (self.orderOverviewSettings as any).__serialize(ctx)
        : self.orderOverviewSettings;
    result["taxRateOverviewSettings"] =
      typeof (self.taxRateOverviewSettings as any)?.__serialize === "function"
        ? (self.taxRateOverviewSettings as any).__serialize(ctx)
        : self.taxRateOverviewSettings;
    result["homePage"] =
      typeof (self.homePage as any)?.__serialize === "function"
        ? (self.homePage as any).__serialize(ctx)
        : self.homePage;
    return result;
  }
}

export namespace Settings {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Settings, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Settings.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Settings | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Settings.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("appointmentNotifications" in obj)) {
      errors.push(
        'Settings.__deserialize: missing required field "appointmentNotifications"',
      );
    }
    if (!("commissions" in obj)) {
      errors.push(
        'Settings.__deserialize: missing required field "commissions"',
      );
    }
    if (!("scheduleSettings" in obj)) {
      errors.push(
        'Settings.__deserialize: missing required field "scheduleSettings"',
      );
    }
    if (!("accountOverviewSettings" in obj)) {
      errors.push(
        'Settings.__deserialize: missing required field "accountOverviewSettings"',
      );
    }
    if (!("serviceOverviewSettings" in obj)) {
      errors.push(
        'Settings.__deserialize: missing required field "serviceOverviewSettings"',
      );
    }
    if (!("appointmentOverviewSettings" in obj)) {
      errors.push(
        'Settings.__deserialize: missing required field "appointmentOverviewSettings"',
      );
    }
    if (!("leadOverviewSettings" in obj)) {
      errors.push(
        'Settings.__deserialize: missing required field "leadOverviewSettings"',
      );
    }
    if (!("packageOverviewSettings" in obj)) {
      errors.push(
        'Settings.__deserialize: missing required field "packageOverviewSettings"',
      );
    }
    if (!("productOverviewSettings" in obj)) {
      errors.push(
        'Settings.__deserialize: missing required field "productOverviewSettings"',
      );
    }
    if (!("orderOverviewSettings" in obj)) {
      errors.push(
        'Settings.__deserialize: missing required field "orderOverviewSettings"',
      );
    }
    if (!("taxRateOverviewSettings" in obj)) {
      errors.push(
        'Settings.__deserialize: missing required field "taxRateOverviewSettings"',
      );
    }
    if (!("homePage" in obj)) {
      errors.push('Settings.__deserialize: missing required field "homePage"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_appointmentNotifications = obj["appointmentNotifications"];
      instance.appointmentNotifications = __raw_appointmentNotifications;
    }
    {
      const __raw_commissions = obj["commissions"];
      instance.commissions = __raw_commissions;
    }
    {
      const __raw_scheduleSettings = obj["scheduleSettings"];
      if (typeof (ScheduleSettings as any)?.__deserialize === "function") {
        const __result = (ScheduleSettings as any).__deserialize(
          __raw_scheduleSettings,
          ctx,
        );
        ctx.assignOrDefer(instance, "scheduleSettings", __result);
      } else {
        instance.scheduleSettings = __raw_scheduleSettings;
      }
    }
    {
      const __raw_accountOverviewSettings = obj["accountOverviewSettings"];
      if (typeof (OverviewSettings as any)?.__deserialize === "function") {
        const __result = (OverviewSettings as any).__deserialize(
          __raw_accountOverviewSettings,
          ctx,
        );
        ctx.assignOrDefer(instance, "accountOverviewSettings", __result);
      } else {
        instance.accountOverviewSettings = __raw_accountOverviewSettings;
      }
    }
    {
      const __raw_serviceOverviewSettings = obj["serviceOverviewSettings"];
      if (typeof (OverviewSettings as any)?.__deserialize === "function") {
        const __result = (OverviewSettings as any).__deserialize(
          __raw_serviceOverviewSettings,
          ctx,
        );
        ctx.assignOrDefer(instance, "serviceOverviewSettings", __result);
      } else {
        instance.serviceOverviewSettings = __raw_serviceOverviewSettings;
      }
    }
    {
      const __raw_appointmentOverviewSettings =
        obj["appointmentOverviewSettings"];
      if (typeof (OverviewSettings as any)?.__deserialize === "function") {
        const __result = (OverviewSettings as any).__deserialize(
          __raw_appointmentOverviewSettings,
          ctx,
        );
        ctx.assignOrDefer(instance, "appointmentOverviewSettings", __result);
      } else {
        instance.appointmentOverviewSettings =
          __raw_appointmentOverviewSettings;
      }
    }
    {
      const __raw_leadOverviewSettings = obj["leadOverviewSettings"];
      if (typeof (OverviewSettings as any)?.__deserialize === "function") {
        const __result = (OverviewSettings as any).__deserialize(
          __raw_leadOverviewSettings,
          ctx,
        );
        ctx.assignOrDefer(instance, "leadOverviewSettings", __result);
      } else {
        instance.leadOverviewSettings = __raw_leadOverviewSettings;
      }
    }
    {
      const __raw_packageOverviewSettings = obj["packageOverviewSettings"];
      if (typeof (OverviewSettings as any)?.__deserialize === "function") {
        const __result = (OverviewSettings as any).__deserialize(
          __raw_packageOverviewSettings,
          ctx,
        );
        ctx.assignOrDefer(instance, "packageOverviewSettings", __result);
      } else {
        instance.packageOverviewSettings = __raw_packageOverviewSettings;
      }
    }
    {
      const __raw_productOverviewSettings = obj["productOverviewSettings"];
      if (typeof (OverviewSettings as any)?.__deserialize === "function") {
        const __result = (OverviewSettings as any).__deserialize(
          __raw_productOverviewSettings,
          ctx,
        );
        ctx.assignOrDefer(instance, "productOverviewSettings", __result);
      } else {
        instance.productOverviewSettings = __raw_productOverviewSettings;
      }
    }
    {
      const __raw_orderOverviewSettings = obj["orderOverviewSettings"];
      if (typeof (OverviewSettings as any)?.__deserialize === "function") {
        const __result = (OverviewSettings as any).__deserialize(
          __raw_orderOverviewSettings,
          ctx,
        );
        ctx.assignOrDefer(instance, "orderOverviewSettings", __result);
      } else {
        instance.orderOverviewSettings = __raw_orderOverviewSettings;
      }
    }
    {
      const __raw_taxRateOverviewSettings = obj["taxRateOverviewSettings"];
      if (typeof (OverviewSettings as any)?.__deserialize === "function") {
        const __result = (OverviewSettings as any).__deserialize(
          __raw_taxRateOverviewSettings,
          ctx,
        );
        ctx.assignOrDefer(instance, "taxRateOverviewSettings", __result);
      } else {
        instance.taxRateOverviewSettings = __raw_taxRateOverviewSettings;
      }
    }
    {
      const __raw_homePage = obj["homePage"];
      if (typeof (Page as any)?.__deserialize === "function") {
        const __result = (Page as any).__deserialize(__raw_homePage, ctx);
        ctx.assignOrDefer(instance, "homePage", __result);
      } else {
        instance.homePage = __raw_homePage;
      }
    }
    return instance as Settings;
  }
}

/**  */
export interface Color {
  red: number;
  green: number;
  blue: number;
}

export namespace Color {
  export function defaultValue(): Color {
    return { red: 0, green: 0, blue: 0 } as Color;
  }
}

export namespace Color {
  export function toJson(self: Color): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Color,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Color", __id };
    result["red"] = self.red;
    result["green"] = self.green;
    result["blue"] = self.blue;
    return result;
  }
}

export namespace Color {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Color, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err(["Color.fromJson: root cannot be a forward reference"]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Color | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Color.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("red" in obj)) {
      errors.push('Color.__deserialize: missing required field "red"');
    }
    if (!("green" in obj)) {
      errors.push('Color.__deserialize: missing required field "green"');
    }
    if (!("blue" in obj)) {
      errors.push('Color.__deserialize: missing required field "blue"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_red = obj["red"];
      instance.red = __raw_red;
    }
    {
      const __raw_green = obj["green"];
      instance.green = __raw_green;
    }
    {
      const __raw_blue = obj["blue"];
      instance.blue = __raw_blue;
    }
    return instance as Color;
  }
}

/**  */
export interface CompanyName {
  companyName: string;
}

export namespace CompanyName {
  export function defaultValue(): CompanyName {
    return { companyName: "" } as CompanyName;
  }
}

export namespace CompanyName {
  export function toJson(self: CompanyName): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: CompanyName,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "CompanyName", __id };
    result["companyName"] = self.companyName;
    return result;
  }
}

export namespace CompanyName {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<CompanyName, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "CompanyName.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): CompanyName | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("CompanyName.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("companyName" in obj)) {
      errors.push(
        'CompanyName.__deserialize: missing required field "companyName"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_companyName = obj["companyName"];
      instance.companyName = __raw_companyName;
    }
    return instance as CompanyName;
  }
}

/**  */
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

export namespace Appointment {
  export function toJson(self: Appointment): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Appointment,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Appointment", __id };
    result["id"] = self.id;
    result["title"] = self.title;
    result["status"] =
      typeof (self.status as any)?.__serialize === "function"
        ? (self.status as any).__serialize(ctx)
        : self.status;
    result["begins"] = self.begins;
    result["duration"] = self.duration;
    result["timeZone"] = self.timeZone;
    result["offsetMs"] = self.offsetMs;
    result["allDay"] = self.allDay;
    result["multiDay"] = self.multiDay;
    result["employees"] = self.employees.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["location"] = self.location;
    if (self.description !== null) {
      result["description"] =
        typeof (self.description as any)?.__serialize === "function"
          ? (self.description as any).__serialize(ctx)
          : self.description;
    } else {
      result["description"] = null;
    }
    result["colors"] =
      typeof (self.colors as any)?.__serialize === "function"
        ? (self.colors as any).__serialize(ctx)
        : self.colors;
    if (self.recurrenceRule !== null) {
      result["recurrenceRule"] =
        typeof (self.recurrenceRule as any)?.__serialize === "function"
          ? (self.recurrenceRule as any).__serialize(ctx)
          : self.recurrenceRule;
    } else {
      result["recurrenceRule"] = null;
    }
    return result;
  }
}

export namespace Appointment {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Appointment, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Appointment.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Appointment | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Appointment.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('Appointment.__deserialize: missing required field "id"');
    }
    if (!("title" in obj)) {
      errors.push('Appointment.__deserialize: missing required field "title"');
    }
    if (!("status" in obj)) {
      errors.push('Appointment.__deserialize: missing required field "status"');
    }
    if (!("begins" in obj)) {
      errors.push('Appointment.__deserialize: missing required field "begins"');
    }
    if (!("duration" in obj)) {
      errors.push(
        'Appointment.__deserialize: missing required field "duration"',
      );
    }
    if (!("timeZone" in obj)) {
      errors.push(
        'Appointment.__deserialize: missing required field "timeZone"',
      );
    }
    if (!("offsetMs" in obj)) {
      errors.push(
        'Appointment.__deserialize: missing required field "offsetMs"',
      );
    }
    if (!("allDay" in obj)) {
      errors.push('Appointment.__deserialize: missing required field "allDay"');
    }
    if (!("multiDay" in obj)) {
      errors.push(
        'Appointment.__deserialize: missing required field "multiDay"',
      );
    }
    if (!("employees" in obj)) {
      errors.push(
        'Appointment.__deserialize: missing required field "employees"',
      );
    }
    if (!("location" in obj)) {
      errors.push(
        'Appointment.__deserialize: missing required field "location"',
      );
    }
    if (!("description" in obj)) {
      errors.push(
        'Appointment.__deserialize: missing required field "description"',
      );
    }
    if (!("colors" in obj)) {
      errors.push('Appointment.__deserialize: missing required field "colors"');
    }
    if (!("recurrenceRule" in obj)) {
      errors.push(
        'Appointment.__deserialize: missing required field "recurrenceRule"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_title = obj["title"];
      instance.title = __raw_title;
    }
    {
      const __raw_status = obj["status"];
      if (typeof (Status as any)?.__deserialize === "function") {
        const __result = (Status as any).__deserialize(__raw_status, ctx);
        ctx.assignOrDefer(instance, "status", __result);
      } else {
        instance.status = __raw_status;
      }
    }
    {
      const __raw_begins = obj["begins"];
      instance.begins = __raw_begins;
    }
    {
      const __raw_duration = obj["duration"];
      instance.duration = __raw_duration;
    }
    {
      const __raw_timeZone = obj["timeZone"];
      instance.timeZone = __raw_timeZone;
    }
    {
      const __raw_offsetMs = obj["offsetMs"];
      instance.offsetMs = __raw_offsetMs;
    }
    {
      const __raw_allDay = obj["allDay"];
      instance.allDay = __raw_allDay;
    }
    {
      const __raw_multiDay = obj["multiDay"];
      instance.multiDay = __raw_multiDay;
    }
    {
      const __raw_employees = obj["employees"];
      instance.employees = __raw_employees;
    }
    {
      const __raw_location = obj["location"];
      instance.location = __raw_location;
    }
    {
      const __raw_description = obj["description"];
      instance.description = __raw_description;
    }
    {
      const __raw_colors = obj["colors"];
      if (typeof (Colors as any)?.__deserialize === "function") {
        const __result = (Colors as any).__deserialize(__raw_colors, ctx);
        ctx.assignOrDefer(instance, "colors", __result);
      } else {
        instance.colors = __raw_colors;
      }
    }
    {
      const __raw_recurrenceRule = obj["recurrenceRule"];
      instance.recurrenceRule = __raw_recurrenceRule;
    }
    return instance as Appointment;
  }
}

/**  */
export interface Package {
  id: string;
  date: string;
}

export namespace Package {
  export function defaultValue(): Package {
    return { id: "", date: "" } as Package;
  }
}

export namespace Package {
  export function toJson(self: Package): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Package,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Package", __id };
    result["id"] = self.id;
    result["date"] = self.date;
    return result;
  }
}

export namespace Package {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Package, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Package.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Package | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Package.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('Package.__deserialize: missing required field "id"');
    }
    if (!("date" in obj)) {
      errors.push('Package.__deserialize: missing required field "date"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_date = obj["date"];
      instance.date = __raw_date;
    }
    return instance as Package;
  }
}

/**  */
export interface ScheduleSettings {
  daysPerWeek: number;
  rowHeight: RowHeight;
  visibleRoutes: string[];
  detailedCards: boolean;
}

export namespace ScheduleSettings {
  export function toJson(self: ScheduleSettings): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: ScheduleSettings,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = {
      __type: "ScheduleSettings",
      __id,
    };
    result["daysPerWeek"] = self.daysPerWeek;
    result["rowHeight"] =
      typeof (self.rowHeight as any)?.__serialize === "function"
        ? (self.rowHeight as any).__serialize(ctx)
        : self.rowHeight;
    result["visibleRoutes"] = self.visibleRoutes.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["detailedCards"] = self.detailedCards;
    return result;
  }
}

export namespace ScheduleSettings {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<ScheduleSettings, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "ScheduleSettings.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): ScheduleSettings | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("ScheduleSettings.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("daysPerWeek" in obj)) {
      errors.push(
        'ScheduleSettings.__deserialize: missing required field "daysPerWeek"',
      );
    }
    if (!("rowHeight" in obj)) {
      errors.push(
        'ScheduleSettings.__deserialize: missing required field "rowHeight"',
      );
    }
    if (!("visibleRoutes" in obj)) {
      errors.push(
        'ScheduleSettings.__deserialize: missing required field "visibleRoutes"',
      );
    }
    if (!("detailedCards" in obj)) {
      errors.push(
        'ScheduleSettings.__deserialize: missing required field "detailedCards"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_daysPerWeek = obj["daysPerWeek"];
      instance.daysPerWeek = __raw_daysPerWeek;
    }
    {
      const __raw_rowHeight = obj["rowHeight"];
      if (typeof (RowHeight as any)?.__deserialize === "function") {
        const __result = (RowHeight as any).__deserialize(__raw_rowHeight, ctx);
        ctx.assignOrDefer(instance, "rowHeight", __result);
      } else {
        instance.rowHeight = __raw_rowHeight;
      }
    }
    {
      const __raw_visibleRoutes = obj["visibleRoutes"];
      instance.visibleRoutes = __raw_visibleRoutes;
    }
    {
      const __raw_detailedCards = obj["detailedCards"];
      instance.detailedCards = __raw_detailedCards;
    }
    return instance as ScheduleSettings;
  }
}

/**  */
export interface DailyRecurrenceRule {
  quantityOfDays: number;
}

export namespace DailyRecurrenceRule {
  export function defaultValue(): DailyRecurrenceRule {
    return { quantityOfDays: 0 } as DailyRecurrenceRule;
  }
}

export namespace DailyRecurrenceRule {
  export function toJson(self: DailyRecurrenceRule): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: DailyRecurrenceRule,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = {
      __type: "DailyRecurrenceRule",
      __id,
    };
    result["quantityOfDays"] = self.quantityOfDays;
    return result;
  }
}

export namespace DailyRecurrenceRule {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<DailyRecurrenceRule, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "DailyRecurrenceRule.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): DailyRecurrenceRule | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("DailyRecurrenceRule.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("quantityOfDays" in obj)) {
      errors.push(
        'DailyRecurrenceRule.__deserialize: missing required field "quantityOfDays"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_quantityOfDays = obj["quantityOfDays"];
      instance.quantityOfDays = __raw_quantityOfDays;
    }
    return instance as DailyRecurrenceRule;
  }
}

/**  */
export interface SignUpCredentials {
  firstName: FirstName;
  lastName: LastName;
  email: EmailParts;
  password: Password;
  rememberMe: boolean;
}

export namespace SignUpCredentials {
  export function toJson(self: SignUpCredentials): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: SignUpCredentials,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = {
      __type: "SignUpCredentials",
      __id,
    };
    result["firstName"] =
      typeof (self.firstName as any)?.__serialize === "function"
        ? (self.firstName as any).__serialize(ctx)
        : self.firstName;
    result["lastName"] =
      typeof (self.lastName as any)?.__serialize === "function"
        ? (self.lastName as any).__serialize(ctx)
        : self.lastName;
    result["email"] =
      typeof (self.email as any)?.__serialize === "function"
        ? (self.email as any).__serialize(ctx)
        : self.email;
    result["password"] =
      typeof (self.password as any)?.__serialize === "function"
        ? (self.password as any).__serialize(ctx)
        : self.password;
    result["rememberMe"] = self.rememberMe;
    return result;
  }
}

export namespace SignUpCredentials {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<SignUpCredentials, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "SignUpCredentials.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): SignUpCredentials | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("SignUpCredentials.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("firstName" in obj)) {
      errors.push(
        'SignUpCredentials.__deserialize: missing required field "firstName"',
      );
    }
    if (!("lastName" in obj)) {
      errors.push(
        'SignUpCredentials.__deserialize: missing required field "lastName"',
      );
    }
    if (!("email" in obj)) {
      errors.push(
        'SignUpCredentials.__deserialize: missing required field "email"',
      );
    }
    if (!("password" in obj)) {
      errors.push(
        'SignUpCredentials.__deserialize: missing required field "password"',
      );
    }
    if (!("rememberMe" in obj)) {
      errors.push(
        'SignUpCredentials.__deserialize: missing required field "rememberMe"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_firstName = obj["firstName"];
      if (typeof (FirstName as any)?.__deserialize === "function") {
        const __result = (FirstName as any).__deserialize(__raw_firstName, ctx);
        ctx.assignOrDefer(instance, "firstName", __result);
      } else {
        instance.firstName = __raw_firstName;
      }
    }
    {
      const __raw_lastName = obj["lastName"];
      if (typeof (LastName as any)?.__deserialize === "function") {
        const __result = (LastName as any).__deserialize(__raw_lastName, ctx);
        ctx.assignOrDefer(instance, "lastName", __result);
      } else {
        instance.lastName = __raw_lastName;
      }
    }
    {
      const __raw_email = obj["email"];
      if (typeof (EmailParts as any)?.__deserialize === "function") {
        const __result = (EmailParts as any).__deserialize(__raw_email, ctx);
        ctx.assignOrDefer(instance, "email", __result);
      } else {
        instance.email = __raw_email;
      }
    }
    {
      const __raw_password = obj["password"];
      if (typeof (Password as any)?.__deserialize === "function") {
        const __result = (Password as any).__deserialize(__raw_password, ctx);
        ctx.assignOrDefer(instance, "password", __result);
      } else {
        instance.password = __raw_password;
      }
    }
    {
      const __raw_rememberMe = obj["rememberMe"];
      instance.rememberMe = __raw_rememberMe;
    }
    return instance as SignUpCredentials;
  }
}

/**  */
export interface OverviewSettings {
  rowHeight: RowHeight;
  cardOrRow: OverviewDisplay;
  perPage: number;
  columnConfigs: ColumnConfig[];
}

export namespace OverviewSettings {
  export function toJson(self: OverviewSettings): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: OverviewSettings,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = {
      __type: "OverviewSettings",
      __id,
    };
    result["rowHeight"] =
      typeof (self.rowHeight as any)?.__serialize === "function"
        ? (self.rowHeight as any).__serialize(ctx)
        : self.rowHeight;
    result["cardOrRow"] =
      typeof (self.cardOrRow as any)?.__serialize === "function"
        ? (self.cardOrRow as any).__serialize(ctx)
        : self.cardOrRow;
    result["perPage"] = self.perPage;
    result["columnConfigs"] = self.columnConfigs.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    return result;
  }
}

export namespace OverviewSettings {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<OverviewSettings, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "OverviewSettings.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): OverviewSettings | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("OverviewSettings.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("rowHeight" in obj)) {
      errors.push(
        'OverviewSettings.__deserialize: missing required field "rowHeight"',
      );
    }
    if (!("cardOrRow" in obj)) {
      errors.push(
        'OverviewSettings.__deserialize: missing required field "cardOrRow"',
      );
    }
    if (!("perPage" in obj)) {
      errors.push(
        'OverviewSettings.__deserialize: missing required field "perPage"',
      );
    }
    if (!("columnConfigs" in obj)) {
      errors.push(
        'OverviewSettings.__deserialize: missing required field "columnConfigs"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_rowHeight = obj["rowHeight"];
      if (typeof (RowHeight as any)?.__deserialize === "function") {
        const __result = (RowHeight as any).__deserialize(__raw_rowHeight, ctx);
        ctx.assignOrDefer(instance, "rowHeight", __result);
      } else {
        instance.rowHeight = __raw_rowHeight;
      }
    }
    {
      const __raw_cardOrRow = obj["cardOrRow"];
      if (typeof (OverviewDisplay as any)?.__deserialize === "function") {
        const __result = (OverviewDisplay as any).__deserialize(
          __raw_cardOrRow,
          ctx,
        );
        ctx.assignOrDefer(instance, "cardOrRow", __result);
      } else {
        instance.cardOrRow = __raw_cardOrRow;
      }
    }
    {
      const __raw_perPage = obj["perPage"];
      instance.perPage = __raw_perPage;
    }
    {
      const __raw_columnConfigs = obj["columnConfigs"];
      instance.columnConfigs = __raw_columnConfigs;
    }
    return instance as OverviewSettings;
  }
}

/**  */
export interface FirstName {
  name: string;
}

export namespace FirstName {
  export function defaultValue(): FirstName {
    return { name: "" } as FirstName;
  }
}

export namespace FirstName {
  export function toJson(self: FirstName): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: FirstName,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "FirstName", __id };
    result["name"] = self.name;
    return result;
  }
}

export namespace FirstName {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<FirstName, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "FirstName.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): FirstName | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("FirstName.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("name" in obj)) {
      errors.push('FirstName.__deserialize: missing required field "name"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_name = obj["name"];
      instance.name = __raw_name;
    }
    return instance as FirstName;
  }
}

/**  */
export interface Account {
  id: string;
  taxRate: string | TaxRate;
  site: string | Site;
  salesRep: Represents[] | null;
  orders: Ordered[];
  activity: Did[];
  customFields: [string, string][];
  accountName: AccountName;
  sector: Sector;
  memo: string | null;
  phones: PhoneNumber[];
  email: Email;

  leadSource: string;
  colors: Colors;
  needsReview: boolean;
  hasAlert: boolean;

  accountType: string;

  subtype: string;
  isTaxExempt: boolean;

  paymentTerms: string;
  tags: string[];
  dateAdded: string;
}

export namespace Account {
  export function toJson(self: Account): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Account,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Account", __id };
    result["id"] = self.id;
    result["taxRate"] = self.taxRate;
    result["site"] = self.site;
    if (self.salesRep !== null) {
      result["salesRep"] =
        typeof (self.salesRep as any)?.__serialize === "function"
          ? (self.salesRep as any).__serialize(ctx)
          : self.salesRep;
    } else {
      result["salesRep"] = null;
    }
    result["orders"] = self.orders.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["activity"] = self.activity.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["customFields"] = self.customFields.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["accountName"] =
      typeof (self.accountName as any)?.__serialize === "function"
        ? (self.accountName as any).__serialize(ctx)
        : self.accountName;
    result["sector"] =
      typeof (self.sector as any)?.__serialize === "function"
        ? (self.sector as any).__serialize(ctx)
        : self.sector;
    if (self.memo !== null) {
      result["memo"] =
        typeof (self.memo as any)?.__serialize === "function"
          ? (self.memo as any).__serialize(ctx)
          : self.memo;
    } else {
      result["memo"] = null;
    }
    result["phones"] = self.phones.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["email"] =
      typeof (self.email as any)?.__serialize === "function"
        ? (self.email as any).__serialize(ctx)
        : self.email;
    result["leadSource"] = self.leadSource;
    result["colors"] =
      typeof (self.colors as any)?.__serialize === "function"
        ? (self.colors as any).__serialize(ctx)
        : self.colors;
    result["needsReview"] = self.needsReview;
    result["hasAlert"] = self.hasAlert;
    result["accountType"] = self.accountType;
    result["subtype"] = self.subtype;
    result["isTaxExempt"] = self.isTaxExempt;
    result["paymentTerms"] = self.paymentTerms;
    result["tags"] = self.tags.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["dateAdded"] = self.dateAdded;
    return result;
  }
}

export namespace Account {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Account, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Account.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Account | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Account.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('Account.__deserialize: missing required field "id"');
    }
    if (!("taxRate" in obj)) {
      errors.push('Account.__deserialize: missing required field "taxRate"');
    }
    if (!("site" in obj)) {
      errors.push('Account.__deserialize: missing required field "site"');
    }
    if (!("salesRep" in obj)) {
      errors.push('Account.__deserialize: missing required field "salesRep"');
    }
    if (!("orders" in obj)) {
      errors.push('Account.__deserialize: missing required field "orders"');
    }
    if (!("activity" in obj)) {
      errors.push('Account.__deserialize: missing required field "activity"');
    }
    if (!("customFields" in obj)) {
      errors.push(
        'Account.__deserialize: missing required field "customFields"',
      );
    }
    if (!("accountName" in obj)) {
      errors.push(
        'Account.__deserialize: missing required field "accountName"',
      );
    }
    if (!("sector" in obj)) {
      errors.push('Account.__deserialize: missing required field "sector"');
    }
    if (!("memo" in obj)) {
      errors.push('Account.__deserialize: missing required field "memo"');
    }
    if (!("phones" in obj)) {
      errors.push('Account.__deserialize: missing required field "phones"');
    }
    if (!("email" in obj)) {
      errors.push('Account.__deserialize: missing required field "email"');
    }
    if (!("leadSource" in obj)) {
      errors.push('Account.__deserialize: missing required field "leadSource"');
    }
    if (!("colors" in obj)) {
      errors.push('Account.__deserialize: missing required field "colors"');
    }
    if (!("needsReview" in obj)) {
      errors.push(
        'Account.__deserialize: missing required field "needsReview"',
      );
    }
    if (!("hasAlert" in obj)) {
      errors.push('Account.__deserialize: missing required field "hasAlert"');
    }
    if (!("accountType" in obj)) {
      errors.push(
        'Account.__deserialize: missing required field "accountType"',
      );
    }
    if (!("subtype" in obj)) {
      errors.push('Account.__deserialize: missing required field "subtype"');
    }
    if (!("isTaxExempt" in obj)) {
      errors.push(
        'Account.__deserialize: missing required field "isTaxExempt"',
      );
    }
    if (!("paymentTerms" in obj)) {
      errors.push(
        'Account.__deserialize: missing required field "paymentTerms"',
      );
    }
    if (!("tags" in obj)) {
      errors.push('Account.__deserialize: missing required field "tags"');
    }
    if (!("dateAdded" in obj)) {
      errors.push('Account.__deserialize: missing required field "dateAdded"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_taxRate = obj["taxRate"];
      instance.taxRate = __raw_taxRate;
    }
    {
      const __raw_site = obj["site"];
      instance.site = __raw_site;
    }
    {
      const __raw_salesRep = obj["salesRep"];
      instance.salesRep = __raw_salesRep;
    }
    {
      const __raw_orders = obj["orders"];
      instance.orders = __raw_orders;
    }
    {
      const __raw_activity = obj["activity"];
      instance.activity = __raw_activity;
    }
    {
      const __raw_customFields = obj["customFields"];
      instance.customFields = __raw_customFields;
    }
    {
      const __raw_accountName = obj["accountName"];
      if (typeof (AccountName as any)?.__deserialize === "function") {
        const __result = (AccountName as any).__deserialize(
          __raw_accountName,
          ctx,
        );
        ctx.assignOrDefer(instance, "accountName", __result);
      } else {
        instance.accountName = __raw_accountName;
      }
    }
    {
      const __raw_sector = obj["sector"];
      if (typeof (Sector as any)?.__deserialize === "function") {
        const __result = (Sector as any).__deserialize(__raw_sector, ctx);
        ctx.assignOrDefer(instance, "sector", __result);
      } else {
        instance.sector = __raw_sector;
      }
    }
    {
      const __raw_memo = obj["memo"];
      instance.memo = __raw_memo;
    }
    {
      const __raw_phones = obj["phones"];
      instance.phones = __raw_phones;
    }
    {
      const __raw_email = obj["email"];
      if (typeof (Email as any)?.__deserialize === "function") {
        const __result = (Email as any).__deserialize(__raw_email, ctx);
        ctx.assignOrDefer(instance, "email", __result);
      } else {
        instance.email = __raw_email;
      }
    }
    {
      const __raw_leadSource = obj["leadSource"];
      instance.leadSource = __raw_leadSource;
    }
    {
      const __raw_colors = obj["colors"];
      if (typeof (Colors as any)?.__deserialize === "function") {
        const __result = (Colors as any).__deserialize(__raw_colors, ctx);
        ctx.assignOrDefer(instance, "colors", __result);
      } else {
        instance.colors = __raw_colors;
      }
    }
    {
      const __raw_needsReview = obj["needsReview"];
      instance.needsReview = __raw_needsReview;
    }
    {
      const __raw_hasAlert = obj["hasAlert"];
      instance.hasAlert = __raw_hasAlert;
    }
    {
      const __raw_accountType = obj["accountType"];
      instance.accountType = __raw_accountType;
    }
    {
      const __raw_subtype = obj["subtype"];
      instance.subtype = __raw_subtype;
    }
    {
      const __raw_isTaxExempt = obj["isTaxExempt"];
      instance.isTaxExempt = __raw_isTaxExempt;
    }
    {
      const __raw_paymentTerms = obj["paymentTerms"];
      instance.paymentTerms = __raw_paymentTerms;
    }
    {
      const __raw_tags = obj["tags"];
      instance.tags = __raw_tags;
    }
    {
      const __raw_dateAdded = obj["dateAdded"];
      instance.dateAdded = __raw_dateAdded;
    }
    return instance as Account;
  }
}

/**  */
export interface Edited {
  fieldName: string;
  oldValue: string | null;
  newValue: string | null;
}

export namespace Edited {
  export function defaultValue(): Edited {
    return { fieldName: "", oldValue: null, newValue: null } as Edited;
  }
}

export namespace Edited {
  export function toJson(self: Edited): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Edited,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Edited", __id };
    result["fieldName"] = self.fieldName;
    if (self.oldValue !== null) {
      result["oldValue"] =
        typeof (self.oldValue as any)?.__serialize === "function"
          ? (self.oldValue as any).__serialize(ctx)
          : self.oldValue;
    } else {
      result["oldValue"] = null;
    }
    if (self.newValue !== null) {
      result["newValue"] =
        typeof (self.newValue as any)?.__serialize === "function"
          ? (self.newValue as any).__serialize(ctx)
          : self.newValue;
    } else {
      result["newValue"] = null;
    }
    return result;
  }
}

export namespace Edited {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Edited, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Edited.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Edited | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Edited.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("fieldName" in obj)) {
      errors.push('Edited.__deserialize: missing required field "fieldName"');
    }
    if (!("oldValue" in obj)) {
      errors.push('Edited.__deserialize: missing required field "oldValue"');
    }
    if (!("newValue" in obj)) {
      errors.push('Edited.__deserialize: missing required field "newValue"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_fieldName = obj["fieldName"];
      instance.fieldName = __raw_fieldName;
    }
    {
      const __raw_oldValue = obj["oldValue"];
      instance.oldValue = __raw_oldValue;
    }
    {
      const __raw_newValue = obj["newValue"];
      instance.newValue = __raw_newValue;
    }
    return instance as Edited;
  }
}

/**  */
export interface Order {
  id: string;
  account: string | Account;
  stage: OrderStage;
  number: number;
  payments: (string | Payment)[];

  opportunity: string;

  reference: string;

  leadSource: string;
  salesRep: string | Employee;

  group: string;

  subgroup: string;
  isPosted: boolean;
  needsReview: boolean;

  actionItem: string;
  upsale: number;
  dateCreated: string;
  appointment: string | Appointment;
  lastTechs: (string | Employee)[];
  package: (string | Package)[] | null;
  promotion: (string | Promotion)[] | null;
  balance: number;
  due: string;
  total: number;
  site: string | Site;
  billedItems: BilledItem[];

  memo: string;
  discount: number;
  tip: number;
  commissions: number[];
}

export namespace Order {
  export function toJson(self: Order): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Order,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Order", __id };
    result["id"] = self.id;
    result["account"] = self.account;
    result["stage"] =
      typeof (self.stage as any)?.__serialize === "function"
        ? (self.stage as any).__serialize(ctx)
        : self.stage;
    result["number"] = self.number;
    result["payments"] = self.payments.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["opportunity"] = self.opportunity;
    result["reference"] = self.reference;
    result["leadSource"] = self.leadSource;
    result["salesRep"] = self.salesRep;
    result["group"] = self.group;
    result["subgroup"] = self.subgroup;
    result["isPosted"] = self.isPosted;
    result["needsReview"] = self.needsReview;
    result["actionItem"] = self.actionItem;
    result["upsale"] = self.upsale;
    result["dateCreated"] = self.dateCreated;
    result["appointment"] = self.appointment;
    result["lastTechs"] = self.lastTechs.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    if (self.package !== null) {
      result["package"] =
        typeof (self.package as any)?.__serialize === "function"
          ? (self.package as any).__serialize(ctx)
          : self.package;
    } else {
      result["package"] = null;
    }
    if (self.promotion !== null) {
      result["promotion"] =
        typeof (self.promotion as any)?.__serialize === "function"
          ? (self.promotion as any).__serialize(ctx)
          : self.promotion;
    } else {
      result["promotion"] = null;
    }
    result["balance"] = self.balance;
    result["due"] = self.due;
    result["total"] = self.total;
    result["site"] = self.site;
    result["billedItems"] = self.billedItems.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["memo"] = self.memo;
    result["discount"] = self.discount;
    result["tip"] = self.tip;
    result["commissions"] = self.commissions.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    return result;
  }
}

export namespace Order {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Order, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err(["Order.fromJson: root cannot be a forward reference"]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Order | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Order.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('Order.__deserialize: missing required field "id"');
    }
    if (!("account" in obj)) {
      errors.push('Order.__deserialize: missing required field "account"');
    }
    if (!("stage" in obj)) {
      errors.push('Order.__deserialize: missing required field "stage"');
    }
    if (!("number" in obj)) {
      errors.push('Order.__deserialize: missing required field "number"');
    }
    if (!("payments" in obj)) {
      errors.push('Order.__deserialize: missing required field "payments"');
    }
    if (!("opportunity" in obj)) {
      errors.push('Order.__deserialize: missing required field "opportunity"');
    }
    if (!("reference" in obj)) {
      errors.push('Order.__deserialize: missing required field "reference"');
    }
    if (!("leadSource" in obj)) {
      errors.push('Order.__deserialize: missing required field "leadSource"');
    }
    if (!("salesRep" in obj)) {
      errors.push('Order.__deserialize: missing required field "salesRep"');
    }
    if (!("group" in obj)) {
      errors.push('Order.__deserialize: missing required field "group"');
    }
    if (!("subgroup" in obj)) {
      errors.push('Order.__deserialize: missing required field "subgroup"');
    }
    if (!("isPosted" in obj)) {
      errors.push('Order.__deserialize: missing required field "isPosted"');
    }
    if (!("needsReview" in obj)) {
      errors.push('Order.__deserialize: missing required field "needsReview"');
    }
    if (!("actionItem" in obj)) {
      errors.push('Order.__deserialize: missing required field "actionItem"');
    }
    if (!("upsale" in obj)) {
      errors.push('Order.__deserialize: missing required field "upsale"');
    }
    if (!("dateCreated" in obj)) {
      errors.push('Order.__deserialize: missing required field "dateCreated"');
    }
    if (!("appointment" in obj)) {
      errors.push('Order.__deserialize: missing required field "appointment"');
    }
    if (!("lastTechs" in obj)) {
      errors.push('Order.__deserialize: missing required field "lastTechs"');
    }
    if (!("package" in obj)) {
      errors.push('Order.__deserialize: missing required field "package"');
    }
    if (!("promotion" in obj)) {
      errors.push('Order.__deserialize: missing required field "promotion"');
    }
    if (!("balance" in obj)) {
      errors.push('Order.__deserialize: missing required field "balance"');
    }
    if (!("due" in obj)) {
      errors.push('Order.__deserialize: missing required field "due"');
    }
    if (!("total" in obj)) {
      errors.push('Order.__deserialize: missing required field "total"');
    }
    if (!("site" in obj)) {
      errors.push('Order.__deserialize: missing required field "site"');
    }
    if (!("billedItems" in obj)) {
      errors.push('Order.__deserialize: missing required field "billedItems"');
    }
    if (!("memo" in obj)) {
      errors.push('Order.__deserialize: missing required field "memo"');
    }
    if (!("discount" in obj)) {
      errors.push('Order.__deserialize: missing required field "discount"');
    }
    if (!("tip" in obj)) {
      errors.push('Order.__deserialize: missing required field "tip"');
    }
    if (!("commissions" in obj)) {
      errors.push('Order.__deserialize: missing required field "commissions"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_account = obj["account"];
      instance.account = __raw_account;
    }
    {
      const __raw_stage = obj["stage"];
      if (typeof (OrderStage as any)?.__deserialize === "function") {
        const __result = (OrderStage as any).__deserialize(__raw_stage, ctx);
        ctx.assignOrDefer(instance, "stage", __result);
      } else {
        instance.stage = __raw_stage;
      }
    }
    {
      const __raw_number = obj["number"];
      instance.number = __raw_number;
    }
    {
      const __raw_payments = obj["payments"];
      instance.payments = __raw_payments;
    }
    {
      const __raw_opportunity = obj["opportunity"];
      instance.opportunity = __raw_opportunity;
    }
    {
      const __raw_reference = obj["reference"];
      instance.reference = __raw_reference;
    }
    {
      const __raw_leadSource = obj["leadSource"];
      instance.leadSource = __raw_leadSource;
    }
    {
      const __raw_salesRep = obj["salesRep"];
      instance.salesRep = __raw_salesRep;
    }
    {
      const __raw_group = obj["group"];
      instance.group = __raw_group;
    }
    {
      const __raw_subgroup = obj["subgroup"];
      instance.subgroup = __raw_subgroup;
    }
    {
      const __raw_isPosted = obj["isPosted"];
      instance.isPosted = __raw_isPosted;
    }
    {
      const __raw_needsReview = obj["needsReview"];
      instance.needsReview = __raw_needsReview;
    }
    {
      const __raw_actionItem = obj["actionItem"];
      instance.actionItem = __raw_actionItem;
    }
    {
      const __raw_upsale = obj["upsale"];
      instance.upsale = __raw_upsale;
    }
    {
      const __raw_dateCreated = obj["dateCreated"];
      instance.dateCreated = __raw_dateCreated;
    }
    {
      const __raw_appointment = obj["appointment"];
      instance.appointment = __raw_appointment;
    }
    {
      const __raw_lastTechs = obj["lastTechs"];
      instance.lastTechs = __raw_lastTechs;
    }
    {
      const __raw_package = obj["package"];
      instance.package = __raw_package;
    }
    {
      const __raw_promotion = obj["promotion"];
      instance.promotion = __raw_promotion;
    }
    {
      const __raw_balance = obj["balance"];
      instance.balance = __raw_balance;
    }
    {
      const __raw_due = obj["due"];
      instance.due = __raw_due;
    }
    {
      const __raw_total = obj["total"];
      instance.total = __raw_total;
    }
    {
      const __raw_site = obj["site"];
      instance.site = __raw_site;
    }
    {
      const __raw_billedItems = obj["billedItems"];
      instance.billedItems = __raw_billedItems;
    }
    {
      const __raw_memo = obj["memo"];
      instance.memo = __raw_memo;
    }
    {
      const __raw_discount = obj["discount"];
      instance.discount = __raw_discount;
    }
    {
      const __raw_tip = obj["tip"];
      instance.tip = __raw_tip;
    }
    {
      const __raw_commissions = obj["commissions"];
      instance.commissions = __raw_commissions;
    }
    return instance as Order;
  }
}

/**  */
export interface Commented {
  comment: string;
  replyTo: string | null;
}

export namespace Commented {
  export function defaultValue(): Commented {
    return { comment: "", replyTo: null } as Commented;
  }
}

export namespace Commented {
  export function toJson(self: Commented): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Commented,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Commented", __id };
    result["comment"] = self.comment;
    if (self.replyTo !== null) {
      result["replyTo"] =
        typeof (self.replyTo as any)?.__serialize === "function"
          ? (self.replyTo as any).__serialize(ctx)
          : self.replyTo;
    } else {
      result["replyTo"] = null;
    }
    return result;
  }
}

export namespace Commented {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Commented, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Commented.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Commented | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Commented.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("comment" in obj)) {
      errors.push('Commented.__deserialize: missing required field "comment"');
    }
    if (!("replyTo" in obj)) {
      errors.push('Commented.__deserialize: missing required field "replyTo"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_comment = obj["comment"];
      instance.comment = __raw_comment;
    }
    {
      const __raw_replyTo = obj["replyTo"];
      instance.replyTo = __raw_replyTo;
    }
    return instance as Commented;
  }
}

/**  */
export interface Custom {
  mappings: DirectionHue[];
}

export namespace Custom {
  export function defaultValue(): Custom {
    return { mappings: [] } as Custom;
  }
}

export namespace Custom {
  export function toJson(self: Custom): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Custom,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Custom", __id };
    result["mappings"] = self.mappings.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    return result;
  }
}

export namespace Custom {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Custom, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Custom.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Custom | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Custom.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("mappings" in obj)) {
      errors.push('Custom.__deserialize: missing required field "mappings"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_mappings = obj["mappings"];
      instance.mappings = __raw_mappings;
    }
    return instance as Custom;
  }
}

/**  */
export interface Colors {
  main: string;

  hover: string;

  active: string;
}

export namespace Colors {
  export function defaultValue(): Colors {
    return { main: "", hover: "", active: "" } as Colors;
  }
}

export namespace Colors {
  export function toJson(self: Colors): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Colors,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Colors", __id };
    result["main"] = self.main;
    result["hover"] = self.hover;
    result["active"] = self.active;
    return result;
  }
}

export namespace Colors {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Colors, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Colors.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Colors | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Colors.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("main" in obj)) {
      errors.push('Colors.__deserialize: missing required field "main"');
    }
    if (!("hover" in obj)) {
      errors.push('Colors.__deserialize: missing required field "hover"');
    }
    if (!("active" in obj)) {
      errors.push('Colors.__deserialize: missing required field "active"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_main = obj["main"];
      instance.main = __raw_main;
    }
    {
      const __raw_hover = obj["hover"];
      instance.hover = __raw_hover;
    }
    {
      const __raw_active = obj["active"];
      instance.active = __raw_active;
    }
    return instance as Colors;
  }
}

/**  */
export interface ProductDefaults {
  price: number;

  description: string;
}

export namespace ProductDefaults {
  export function defaultValue(): ProductDefaults {
    return { price: 0, description: "" } as ProductDefaults;
  }
}

export namespace ProductDefaults {
  export function toJson(self: ProductDefaults): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: ProductDefaults,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "ProductDefaults", __id };
    result["price"] = self.price;
    result["description"] = self.description;
    return result;
  }
}

export namespace ProductDefaults {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<ProductDefaults, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "ProductDefaults.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): ProductDefaults | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("ProductDefaults.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("price" in obj)) {
      errors.push(
        'ProductDefaults.__deserialize: missing required field "price"',
      );
    }
    if (!("description" in obj)) {
      errors.push(
        'ProductDefaults.__deserialize: missing required field "description"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_price = obj["price"];
      instance.price = __raw_price;
    }
    {
      const __raw_description = obj["description"];
      instance.description = __raw_description;
    }
    return instance as ProductDefaults;
  }
}

/**  */
export interface Viewed {
  durationSeconds: number | null;
  source: string | null;
}

export namespace Viewed {
  export function defaultValue(): Viewed {
    return { durationSeconds: null, source: null } as Viewed;
  }
}

export namespace Viewed {
  export function toJson(self: Viewed): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Viewed,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Viewed", __id };
    if (self.durationSeconds !== null) {
      result["durationSeconds"] =
        typeof (self.durationSeconds as any)?.__serialize === "function"
          ? (self.durationSeconds as any).__serialize(ctx)
          : self.durationSeconds;
    } else {
      result["durationSeconds"] = null;
    }
    if (self.source !== null) {
      result["source"] =
        typeof (self.source as any)?.__serialize === "function"
          ? (self.source as any).__serialize(ctx)
          : self.source;
    } else {
      result["source"] = null;
    }
    return result;
  }
}

export namespace Viewed {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Viewed, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Viewed.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Viewed | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Viewed.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("durationSeconds" in obj)) {
      errors.push(
        'Viewed.__deserialize: missing required field "durationSeconds"',
      );
    }
    if (!("source" in obj)) {
      errors.push('Viewed.__deserialize: missing required field "source"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_durationSeconds = obj["durationSeconds"];
      instance.durationSeconds = __raw_durationSeconds;
    }
    {
      const __raw_source = obj["source"];
      instance.source = __raw_source;
    }
    return instance as Viewed;
  }
}

/**  */
export interface WeeklyRecurrenceRule {
  quantityOfWeeks: number;
  weekdays: Weekday[];
}

export namespace WeeklyRecurrenceRule {
  export function defaultValue(): WeeklyRecurrenceRule {
    return { quantityOfWeeks: 0, weekdays: [] } as WeeklyRecurrenceRule;
  }
}

export namespace WeeklyRecurrenceRule {
  export function toJson(self: WeeklyRecurrenceRule): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: WeeklyRecurrenceRule,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = {
      __type: "WeeklyRecurrenceRule",
      __id,
    };
    result["quantityOfWeeks"] = self.quantityOfWeeks;
    result["weekdays"] = self.weekdays.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    return result;
  }
}

export namespace WeeklyRecurrenceRule {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<WeeklyRecurrenceRule, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "WeeklyRecurrenceRule.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): WeeklyRecurrenceRule | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("WeeklyRecurrenceRule.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("quantityOfWeeks" in obj)) {
      errors.push(
        'WeeklyRecurrenceRule.__deserialize: missing required field "quantityOfWeeks"',
      );
    }
    if (!("weekdays" in obj)) {
      errors.push(
        'WeeklyRecurrenceRule.__deserialize: missing required field "weekdays"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_quantityOfWeeks = obj["quantityOfWeeks"];
      instance.quantityOfWeeks = __raw_quantityOfWeeks;
    }
    {
      const __raw_weekdays = obj["weekdays"];
      instance.weekdays = __raw_weekdays;
    }
    return instance as WeeklyRecurrenceRule;
  }
}

/**  */
export interface Paid {
  amount: number | null;
  currency: string | null;
  paymentMethod: string | null;
}

export namespace Paid {
  export function defaultValue(): Paid {
    return { amount: null, currency: null, paymentMethod: null } as Paid;
  }
}

export namespace Paid {
  export function toJson(self: Paid): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Paid,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Paid", __id };
    if (self.amount !== null) {
      result["amount"] =
        typeof (self.amount as any)?.__serialize === "function"
          ? (self.amount as any).__serialize(ctx)
          : self.amount;
    } else {
      result["amount"] = null;
    }
    if (self.currency !== null) {
      result["currency"] =
        typeof (self.currency as any)?.__serialize === "function"
          ? (self.currency as any).__serialize(ctx)
          : self.currency;
    } else {
      result["currency"] = null;
    }
    if (self.paymentMethod !== null) {
      result["paymentMethod"] =
        typeof (self.paymentMethod as any)?.__serialize === "function"
          ? (self.paymentMethod as any).__serialize(ctx)
          : self.paymentMethod;
    } else {
      result["paymentMethod"] = null;
    }
    return result;
  }
}

export namespace Paid {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Paid, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err(["Paid.fromJson: root cannot be a forward reference"]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Paid | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Paid.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("amount" in obj)) {
      errors.push('Paid.__deserialize: missing required field "amount"');
    }
    if (!("currency" in obj)) {
      errors.push('Paid.__deserialize: missing required field "currency"');
    }
    if (!("paymentMethod" in obj)) {
      errors.push('Paid.__deserialize: missing required field "paymentMethod"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_amount = obj["amount"];
      instance.amount = __raw_amount;
    }
    {
      const __raw_currency = obj["currency"];
      instance.currency = __raw_currency;
    }
    {
      const __raw_paymentMethod = obj["paymentMethod"];
      instance.paymentMethod = __raw_paymentMethod;
    }
    return instance as Paid;
  }
}

/**  */
export interface TaxRate {
  id: string;

  name: string;

  taxAgency: string;
  zip: number;

  city: string;

  county: string;

  state: string;
  isActive: boolean;

  description: string;
  taxComponents: { [key: string]: number };
}

export namespace TaxRate {
  export function toJson(self: TaxRate): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: TaxRate,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "TaxRate", __id };
    result["id"] = self.id;
    result["name"] = self.name;
    result["taxAgency"] = self.taxAgency;
    result["zip"] = self.zip;
    result["city"] = self.city;
    result["county"] = self.county;
    result["state"] = self.state;
    result["isActive"] = self.isActive;
    result["description"] = self.description;
    result["taxComponents"] = self.taxComponents;
    return result;
  }
}

export namespace TaxRate {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<TaxRate, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "TaxRate.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): TaxRate | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("TaxRate.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('TaxRate.__deserialize: missing required field "id"');
    }
    if (!("name" in obj)) {
      errors.push('TaxRate.__deserialize: missing required field "name"');
    }
    if (!("taxAgency" in obj)) {
      errors.push('TaxRate.__deserialize: missing required field "taxAgency"');
    }
    if (!("zip" in obj)) {
      errors.push('TaxRate.__deserialize: missing required field "zip"');
    }
    if (!("city" in obj)) {
      errors.push('TaxRate.__deserialize: missing required field "city"');
    }
    if (!("county" in obj)) {
      errors.push('TaxRate.__deserialize: missing required field "county"');
    }
    if (!("state" in obj)) {
      errors.push('TaxRate.__deserialize: missing required field "state"');
    }
    if (!("isActive" in obj)) {
      errors.push('TaxRate.__deserialize: missing required field "isActive"');
    }
    if (!("description" in obj)) {
      errors.push(
        'TaxRate.__deserialize: missing required field "description"',
      );
    }
    if (!("taxComponents" in obj)) {
      errors.push(
        'TaxRate.__deserialize: missing required field "taxComponents"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_name = obj["name"];
      instance.name = __raw_name;
    }
    {
      const __raw_taxAgency = obj["taxAgency"];
      instance.taxAgency = __raw_taxAgency;
    }
    {
      const __raw_zip = obj["zip"];
      instance.zip = __raw_zip;
    }
    {
      const __raw_city = obj["city"];
      instance.city = __raw_city;
    }
    {
      const __raw_county = obj["county"];
      instance.county = __raw_county;
    }
    {
      const __raw_state = obj["state"];
      instance.state = __raw_state;
    }
    {
      const __raw_isActive = obj["isActive"];
      instance.isActive = __raw_isActive;
    }
    {
      const __raw_description = obj["description"];
      instance.description = __raw_description;
    }
    {
      const __raw_taxComponents = obj["taxComponents"];
      instance.taxComponents = __raw_taxComponents;
    }
    return instance as TaxRate;
  }
}

/**  */
export interface Address {
  street: string;

  city: string;

  state: string;

  zipcode: string;
}

export namespace Address {
  export function defaultValue(): Address {
    return { street: "", city: "", state: "", zipcode: "" } as Address;
  }
}

export namespace Address {
  export function toJson(self: Address): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Address,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Address", __id };
    result["street"] = self.street;
    result["city"] = self.city;
    result["state"] = self.state;
    result["zipcode"] = self.zipcode;
    return result;
  }
}

export namespace Address {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Address, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Address.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Address | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Address.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("street" in obj)) {
      errors.push('Address.__deserialize: missing required field "street"');
    }
    if (!("city" in obj)) {
      errors.push('Address.__deserialize: missing required field "city"');
    }
    if (!("state" in obj)) {
      errors.push('Address.__deserialize: missing required field "state"');
    }
    if (!("zipcode" in obj)) {
      errors.push('Address.__deserialize: missing required field "zipcode"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_street = obj["street"];
      instance.street = __raw_street;
    }
    {
      const __raw_city = obj["city"];
      instance.city = __raw_city;
    }
    {
      const __raw_state = obj["state"];
      instance.state = __raw_state;
    }
    {
      const __raw_zipcode = obj["zipcode"];
      instance.zipcode = __raw_zipcode;
    }
    return instance as Address;
  }
}

/**  */
export interface Lead {
  id: string;
  number: number | null;
  accepted: boolean;
  probability: number;
  priority: Priority;
  dueDate: string | null;
  closeDate: string | null;
  value: number;
  stage: LeadStage;

  status: string;
  description: string | null;
  nextStep: NextStep;
  favorite: boolean;
  dateAdded: string | null;
  taxRate: (string | TaxRate) | null;
  sector: Sector;
  leadName: AccountName;
  phones: PhoneNumber[];
  email: Email;
  leadSource: string | null;
  site: string | Site;

  memo: string;
  needsReview: boolean;
  hasAlert: boolean;
  salesRep: Represents[] | null;
  color: string | null;

  accountType: string;

  subtype: string;
  isTaxExempt: boolean;

  paymentTerms: string;
  tags: string[];
  customFields: [string, string][];
}

export namespace Lead {
  export function toJson(self: Lead): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Lead,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Lead", __id };
    result["id"] = self.id;
    if (self.number !== null) {
      result["number"] =
        typeof (self.number as any)?.__serialize === "function"
          ? (self.number as any).__serialize(ctx)
          : self.number;
    } else {
      result["number"] = null;
    }
    result["accepted"] = self.accepted;
    result["probability"] = self.probability;
    result["priority"] =
      typeof (self.priority as any)?.__serialize === "function"
        ? (self.priority as any).__serialize(ctx)
        : self.priority;
    if (self.dueDate !== null) {
      result["dueDate"] =
        typeof (self.dueDate as any)?.__serialize === "function"
          ? (self.dueDate as any).__serialize(ctx)
          : self.dueDate;
    } else {
      result["dueDate"] = null;
    }
    if (self.closeDate !== null) {
      result["closeDate"] =
        typeof (self.closeDate as any)?.__serialize === "function"
          ? (self.closeDate as any).__serialize(ctx)
          : self.closeDate;
    } else {
      result["closeDate"] = null;
    }
    result["value"] = self.value;
    result["stage"] =
      typeof (self.stage as any)?.__serialize === "function"
        ? (self.stage as any).__serialize(ctx)
        : self.stage;
    result["status"] = self.status;
    if (self.description !== null) {
      result["description"] =
        typeof (self.description as any)?.__serialize === "function"
          ? (self.description as any).__serialize(ctx)
          : self.description;
    } else {
      result["description"] = null;
    }
    result["nextStep"] =
      typeof (self.nextStep as any)?.__serialize === "function"
        ? (self.nextStep as any).__serialize(ctx)
        : self.nextStep;
    result["favorite"] = self.favorite;
    if (self.dateAdded !== null) {
      result["dateAdded"] =
        typeof (self.dateAdded as any)?.__serialize === "function"
          ? (self.dateAdded as any).__serialize(ctx)
          : self.dateAdded;
    } else {
      result["dateAdded"] = null;
    }
    if (self.taxRate !== null) {
      result["taxRate"] =
        typeof (self.taxRate as any)?.__serialize === "function"
          ? (self.taxRate as any).__serialize(ctx)
          : self.taxRate;
    } else {
      result["taxRate"] = null;
    }
    result["sector"] =
      typeof (self.sector as any)?.__serialize === "function"
        ? (self.sector as any).__serialize(ctx)
        : self.sector;
    result["leadName"] =
      typeof (self.leadName as any)?.__serialize === "function"
        ? (self.leadName as any).__serialize(ctx)
        : self.leadName;
    result["phones"] = self.phones.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["email"] =
      typeof (self.email as any)?.__serialize === "function"
        ? (self.email as any).__serialize(ctx)
        : self.email;
    if (self.leadSource !== null) {
      result["leadSource"] =
        typeof (self.leadSource as any)?.__serialize === "function"
          ? (self.leadSource as any).__serialize(ctx)
          : self.leadSource;
    } else {
      result["leadSource"] = null;
    }
    result["site"] = self.site;
    result["memo"] = self.memo;
    result["needsReview"] = self.needsReview;
    result["hasAlert"] = self.hasAlert;
    if (self.salesRep !== null) {
      result["salesRep"] =
        typeof (self.salesRep as any)?.__serialize === "function"
          ? (self.salesRep as any).__serialize(ctx)
          : self.salesRep;
    } else {
      result["salesRep"] = null;
    }
    if (self.color !== null) {
      result["color"] =
        typeof (self.color as any)?.__serialize === "function"
          ? (self.color as any).__serialize(ctx)
          : self.color;
    } else {
      result["color"] = null;
    }
    result["accountType"] = self.accountType;
    result["subtype"] = self.subtype;
    result["isTaxExempt"] = self.isTaxExempt;
    result["paymentTerms"] = self.paymentTerms;
    result["tags"] = self.tags.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["customFields"] = self.customFields.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    return result;
  }
}

export namespace Lead {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Lead, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err(["Lead.fromJson: root cannot be a forward reference"]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Lead | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Lead.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('Lead.__deserialize: missing required field "id"');
    }
    if (!("number" in obj)) {
      errors.push('Lead.__deserialize: missing required field "number"');
    }
    if (!("accepted" in obj)) {
      errors.push('Lead.__deserialize: missing required field "accepted"');
    }
    if (!("probability" in obj)) {
      errors.push('Lead.__deserialize: missing required field "probability"');
    }
    if (!("priority" in obj)) {
      errors.push('Lead.__deserialize: missing required field "priority"');
    }
    if (!("dueDate" in obj)) {
      errors.push('Lead.__deserialize: missing required field "dueDate"');
    }
    if (!("closeDate" in obj)) {
      errors.push('Lead.__deserialize: missing required field "closeDate"');
    }
    if (!("value" in obj)) {
      errors.push('Lead.__deserialize: missing required field "value"');
    }
    if (!("stage" in obj)) {
      errors.push('Lead.__deserialize: missing required field "stage"');
    }
    if (!("status" in obj)) {
      errors.push('Lead.__deserialize: missing required field "status"');
    }
    if (!("description" in obj)) {
      errors.push('Lead.__deserialize: missing required field "description"');
    }
    if (!("nextStep" in obj)) {
      errors.push('Lead.__deserialize: missing required field "nextStep"');
    }
    if (!("favorite" in obj)) {
      errors.push('Lead.__deserialize: missing required field "favorite"');
    }
    if (!("dateAdded" in obj)) {
      errors.push('Lead.__deserialize: missing required field "dateAdded"');
    }
    if (!("taxRate" in obj)) {
      errors.push('Lead.__deserialize: missing required field "taxRate"');
    }
    if (!("sector" in obj)) {
      errors.push('Lead.__deserialize: missing required field "sector"');
    }
    if (!("leadName" in obj)) {
      errors.push('Lead.__deserialize: missing required field "leadName"');
    }
    if (!("phones" in obj)) {
      errors.push('Lead.__deserialize: missing required field "phones"');
    }
    if (!("email" in obj)) {
      errors.push('Lead.__deserialize: missing required field "email"');
    }
    if (!("leadSource" in obj)) {
      errors.push('Lead.__deserialize: missing required field "leadSource"');
    }
    if (!("site" in obj)) {
      errors.push('Lead.__deserialize: missing required field "site"');
    }
    if (!("memo" in obj)) {
      errors.push('Lead.__deserialize: missing required field "memo"');
    }
    if (!("needsReview" in obj)) {
      errors.push('Lead.__deserialize: missing required field "needsReview"');
    }
    if (!("hasAlert" in obj)) {
      errors.push('Lead.__deserialize: missing required field "hasAlert"');
    }
    if (!("salesRep" in obj)) {
      errors.push('Lead.__deserialize: missing required field "salesRep"');
    }
    if (!("color" in obj)) {
      errors.push('Lead.__deserialize: missing required field "color"');
    }
    if (!("accountType" in obj)) {
      errors.push('Lead.__deserialize: missing required field "accountType"');
    }
    if (!("subtype" in obj)) {
      errors.push('Lead.__deserialize: missing required field "subtype"');
    }
    if (!("isTaxExempt" in obj)) {
      errors.push('Lead.__deserialize: missing required field "isTaxExempt"');
    }
    if (!("paymentTerms" in obj)) {
      errors.push('Lead.__deserialize: missing required field "paymentTerms"');
    }
    if (!("tags" in obj)) {
      errors.push('Lead.__deserialize: missing required field "tags"');
    }
    if (!("customFields" in obj)) {
      errors.push('Lead.__deserialize: missing required field "customFields"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_number = obj["number"];
      instance.number = __raw_number;
    }
    {
      const __raw_accepted = obj["accepted"];
      instance.accepted = __raw_accepted;
    }
    {
      const __raw_probability = obj["probability"];
      instance.probability = __raw_probability;
    }
    {
      const __raw_priority = obj["priority"];
      if (typeof (Priority as any)?.__deserialize === "function") {
        const __result = (Priority as any).__deserialize(__raw_priority, ctx);
        ctx.assignOrDefer(instance, "priority", __result);
      } else {
        instance.priority = __raw_priority;
      }
    }
    {
      const __raw_dueDate = obj["dueDate"];
      instance.dueDate = __raw_dueDate;
    }
    {
      const __raw_closeDate = obj["closeDate"];
      instance.closeDate = __raw_closeDate;
    }
    {
      const __raw_value = obj["value"];
      instance.value = __raw_value;
    }
    {
      const __raw_stage = obj["stage"];
      if (typeof (LeadStage as any)?.__deserialize === "function") {
        const __result = (LeadStage as any).__deserialize(__raw_stage, ctx);
        ctx.assignOrDefer(instance, "stage", __result);
      } else {
        instance.stage = __raw_stage;
      }
    }
    {
      const __raw_status = obj["status"];
      instance.status = __raw_status;
    }
    {
      const __raw_description = obj["description"];
      instance.description = __raw_description;
    }
    {
      const __raw_nextStep = obj["nextStep"];
      if (typeof (NextStep as any)?.__deserialize === "function") {
        const __result = (NextStep as any).__deserialize(__raw_nextStep, ctx);
        ctx.assignOrDefer(instance, "nextStep", __result);
      } else {
        instance.nextStep = __raw_nextStep;
      }
    }
    {
      const __raw_favorite = obj["favorite"];
      instance.favorite = __raw_favorite;
    }
    {
      const __raw_dateAdded = obj["dateAdded"];
      instance.dateAdded = __raw_dateAdded;
    }
    {
      const __raw_taxRate = obj["taxRate"];
      instance.taxRate = __raw_taxRate;
    }
    {
      const __raw_sector = obj["sector"];
      if (typeof (Sector as any)?.__deserialize === "function") {
        const __result = (Sector as any).__deserialize(__raw_sector, ctx);
        ctx.assignOrDefer(instance, "sector", __result);
      } else {
        instance.sector = __raw_sector;
      }
    }
    {
      const __raw_leadName = obj["leadName"];
      if (typeof (AccountName as any)?.__deserialize === "function") {
        const __result = (AccountName as any).__deserialize(
          __raw_leadName,
          ctx,
        );
        ctx.assignOrDefer(instance, "leadName", __result);
      } else {
        instance.leadName = __raw_leadName;
      }
    }
    {
      const __raw_phones = obj["phones"];
      instance.phones = __raw_phones;
    }
    {
      const __raw_email = obj["email"];
      if (typeof (Email as any)?.__deserialize === "function") {
        const __result = (Email as any).__deserialize(__raw_email, ctx);
        ctx.assignOrDefer(instance, "email", __result);
      } else {
        instance.email = __raw_email;
      }
    }
    {
      const __raw_leadSource = obj["leadSource"];
      instance.leadSource = __raw_leadSource;
    }
    {
      const __raw_site = obj["site"];
      instance.site = __raw_site;
    }
    {
      const __raw_memo = obj["memo"];
      instance.memo = __raw_memo;
    }
    {
      const __raw_needsReview = obj["needsReview"];
      instance.needsReview = __raw_needsReview;
    }
    {
      const __raw_hasAlert = obj["hasAlert"];
      instance.hasAlert = __raw_hasAlert;
    }
    {
      const __raw_salesRep = obj["salesRep"];
      instance.salesRep = __raw_salesRep;
    }
    {
      const __raw_color = obj["color"];
      instance.color = __raw_color;
    }
    {
      const __raw_accountType = obj["accountType"];
      instance.accountType = __raw_accountType;
    }
    {
      const __raw_subtype = obj["subtype"];
      instance.subtype = __raw_subtype;
    }
    {
      const __raw_isTaxExempt = obj["isTaxExempt"];
      instance.isTaxExempt = __raw_isTaxExempt;
    }
    {
      const __raw_paymentTerms = obj["paymentTerms"];
      instance.paymentTerms = __raw_paymentTerms;
    }
    {
      const __raw_tags = obj["tags"];
      instance.tags = __raw_tags;
    }
    {
      const __raw_customFields = obj["customFields"];
      instance.customFields = __raw_customFields;
    }
    return instance as Lead;
  }
}

/**  */
export interface AppPermissions {
  applications: Applications[];
  pages: Page[];
  data: Table[];
}

export namespace AppPermissions {
  export function defaultValue(): AppPermissions {
    return { applications: [], pages: [], data: [] } as AppPermissions;
  }
}

export namespace AppPermissions {
  export function toJson(self: AppPermissions): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: AppPermissions,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "AppPermissions", __id };
    result["applications"] = self.applications.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["pages"] = self.pages.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["data"] = self.data.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    return result;
  }
}

export namespace AppPermissions {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<AppPermissions, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "AppPermissions.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): AppPermissions | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("AppPermissions.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("applications" in obj)) {
      errors.push(
        'AppPermissions.__deserialize: missing required field "applications"',
      );
    }
    if (!("pages" in obj)) {
      errors.push(
        'AppPermissions.__deserialize: missing required field "pages"',
      );
    }
    if (!("data" in obj)) {
      errors.push(
        'AppPermissions.__deserialize: missing required field "data"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_applications = obj["applications"];
      instance.applications = __raw_applications;
    }
    {
      const __raw_pages = obj["pages"];
      instance.pages = __raw_pages;
    }
    {
      const __raw_data = obj["data"];
      instance.data = __raw_data;
    }
    return instance as AppPermissions;
  }
}

/**  */
export interface Company {
  id: string;

  legalName: string;
  headquarters: string | Site;
  phones: PhoneNumber[];

  fax: string;

  email: string;

  website: string;

  taxId: string;
  referenceNumber: number;

  postalCodeLookup: string;
  timeZone: string;
  defaultTax: string | TaxRate;

  defaultTaxLocation: string;
  defaultAreaCode: number;

  defaultAccountType: string;

  lookupFormatting: string;

  accountNameFormat: string;
  merchantServiceProvider: string | null;

  dateDisplayStyle: string;
  hasAutoCommission: boolean;
  hasAutoDaylightSavings: boolean;
  hasAutoFmsTracking: boolean;
  hasNotifications: boolean;
  hasRequiredLeadSource: boolean;
  hasRequiredEmail: boolean;
  hasSortServiceItemsAlphabetically: boolean;
  hasAttachOrderToAppointmentEmails: boolean;
  scheduleInterval: number;
  colorsConfig: ColorsConfig;
}

export namespace Company {
  export function toJson(self: Company): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Company,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Company", __id };
    result["id"] = self.id;
    result["legalName"] = self.legalName;
    result["headquarters"] = self.headquarters;
    result["phones"] = self.phones.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["fax"] = self.fax;
    result["email"] = self.email;
    result["website"] = self.website;
    result["taxId"] = self.taxId;
    result["referenceNumber"] = self.referenceNumber;
    result["postalCodeLookup"] = self.postalCodeLookup;
    result["timeZone"] = self.timeZone;
    result["defaultTax"] = self.defaultTax;
    result["defaultTaxLocation"] = self.defaultTaxLocation;
    result["defaultAreaCode"] = self.defaultAreaCode;
    result["defaultAccountType"] = self.defaultAccountType;
    result["lookupFormatting"] = self.lookupFormatting;
    result["accountNameFormat"] = self.accountNameFormat;
    if (self.merchantServiceProvider !== null) {
      result["merchantServiceProvider"] =
        typeof (self.merchantServiceProvider as any)?.__serialize === "function"
          ? (self.merchantServiceProvider as any).__serialize(ctx)
          : self.merchantServiceProvider;
    } else {
      result["merchantServiceProvider"] = null;
    }
    result["dateDisplayStyle"] = self.dateDisplayStyle;
    result["hasAutoCommission"] = self.hasAutoCommission;
    result["hasAutoDaylightSavings"] = self.hasAutoDaylightSavings;
    result["hasAutoFmsTracking"] = self.hasAutoFmsTracking;
    result["hasNotifications"] = self.hasNotifications;
    result["hasRequiredLeadSource"] = self.hasRequiredLeadSource;
    result["hasRequiredEmail"] = self.hasRequiredEmail;
    result["hasSortServiceItemsAlphabetically"] =
      self.hasSortServiceItemsAlphabetically;
    result["hasAttachOrderToAppointmentEmails"] =
      self.hasAttachOrderToAppointmentEmails;
    result["scheduleInterval"] = self.scheduleInterval;
    result["colorsConfig"] =
      typeof (self.colorsConfig as any)?.__serialize === "function"
        ? (self.colorsConfig as any).__serialize(ctx)
        : self.colorsConfig;
    return result;
  }
}

export namespace Company {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Company, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Company.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Company | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Company.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('Company.__deserialize: missing required field "id"');
    }
    if (!("legalName" in obj)) {
      errors.push('Company.__deserialize: missing required field "legalName"');
    }
    if (!("headquarters" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "headquarters"',
      );
    }
    if (!("phones" in obj)) {
      errors.push('Company.__deserialize: missing required field "phones"');
    }
    if (!("fax" in obj)) {
      errors.push('Company.__deserialize: missing required field "fax"');
    }
    if (!("email" in obj)) {
      errors.push('Company.__deserialize: missing required field "email"');
    }
    if (!("website" in obj)) {
      errors.push('Company.__deserialize: missing required field "website"');
    }
    if (!("taxId" in obj)) {
      errors.push('Company.__deserialize: missing required field "taxId"');
    }
    if (!("referenceNumber" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "referenceNumber"',
      );
    }
    if (!("postalCodeLookup" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "postalCodeLookup"',
      );
    }
    if (!("timeZone" in obj)) {
      errors.push('Company.__deserialize: missing required field "timeZone"');
    }
    if (!("defaultTax" in obj)) {
      errors.push('Company.__deserialize: missing required field "defaultTax"');
    }
    if (!("defaultTaxLocation" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "defaultTaxLocation"',
      );
    }
    if (!("defaultAreaCode" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "defaultAreaCode"',
      );
    }
    if (!("defaultAccountType" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "defaultAccountType"',
      );
    }
    if (!("lookupFormatting" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "lookupFormatting"',
      );
    }
    if (!("accountNameFormat" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "accountNameFormat"',
      );
    }
    if (!("merchantServiceProvider" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "merchantServiceProvider"',
      );
    }
    if (!("dateDisplayStyle" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "dateDisplayStyle"',
      );
    }
    if (!("hasAutoCommission" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "hasAutoCommission"',
      );
    }
    if (!("hasAutoDaylightSavings" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "hasAutoDaylightSavings"',
      );
    }
    if (!("hasAutoFmsTracking" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "hasAutoFmsTracking"',
      );
    }
    if (!("hasNotifications" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "hasNotifications"',
      );
    }
    if (!("hasRequiredLeadSource" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "hasRequiredLeadSource"',
      );
    }
    if (!("hasRequiredEmail" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "hasRequiredEmail"',
      );
    }
    if (!("hasSortServiceItemsAlphabetically" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "hasSortServiceItemsAlphabetically"',
      );
    }
    if (!("hasAttachOrderToAppointmentEmails" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "hasAttachOrderToAppointmentEmails"',
      );
    }
    if (!("scheduleInterval" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "scheduleInterval"',
      );
    }
    if (!("colorsConfig" in obj)) {
      errors.push(
        'Company.__deserialize: missing required field "colorsConfig"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_legalName = obj["legalName"];
      instance.legalName = __raw_legalName;
    }
    {
      const __raw_headquarters = obj["headquarters"];
      instance.headquarters = __raw_headquarters;
    }
    {
      const __raw_phones = obj["phones"];
      instance.phones = __raw_phones;
    }
    {
      const __raw_fax = obj["fax"];
      instance.fax = __raw_fax;
    }
    {
      const __raw_email = obj["email"];
      instance.email = __raw_email;
    }
    {
      const __raw_website = obj["website"];
      instance.website = __raw_website;
    }
    {
      const __raw_taxId = obj["taxId"];
      instance.taxId = __raw_taxId;
    }
    {
      const __raw_referenceNumber = obj["referenceNumber"];
      instance.referenceNumber = __raw_referenceNumber;
    }
    {
      const __raw_postalCodeLookup = obj["postalCodeLookup"];
      instance.postalCodeLookup = __raw_postalCodeLookup;
    }
    {
      const __raw_timeZone = obj["timeZone"];
      instance.timeZone = __raw_timeZone;
    }
    {
      const __raw_defaultTax = obj["defaultTax"];
      instance.defaultTax = __raw_defaultTax;
    }
    {
      const __raw_defaultTaxLocation = obj["defaultTaxLocation"];
      instance.defaultTaxLocation = __raw_defaultTaxLocation;
    }
    {
      const __raw_defaultAreaCode = obj["defaultAreaCode"];
      instance.defaultAreaCode = __raw_defaultAreaCode;
    }
    {
      const __raw_defaultAccountType = obj["defaultAccountType"];
      instance.defaultAccountType = __raw_defaultAccountType;
    }
    {
      const __raw_lookupFormatting = obj["lookupFormatting"];
      instance.lookupFormatting = __raw_lookupFormatting;
    }
    {
      const __raw_accountNameFormat = obj["accountNameFormat"];
      instance.accountNameFormat = __raw_accountNameFormat;
    }
    {
      const __raw_merchantServiceProvider = obj["merchantServiceProvider"];
      instance.merchantServiceProvider = __raw_merchantServiceProvider;
    }
    {
      const __raw_dateDisplayStyle = obj["dateDisplayStyle"];
      instance.dateDisplayStyle = __raw_dateDisplayStyle;
    }
    {
      const __raw_hasAutoCommission = obj["hasAutoCommission"];
      instance.hasAutoCommission = __raw_hasAutoCommission;
    }
    {
      const __raw_hasAutoDaylightSavings = obj["hasAutoDaylightSavings"];
      instance.hasAutoDaylightSavings = __raw_hasAutoDaylightSavings;
    }
    {
      const __raw_hasAutoFmsTracking = obj["hasAutoFmsTracking"];
      instance.hasAutoFmsTracking = __raw_hasAutoFmsTracking;
    }
    {
      const __raw_hasNotifications = obj["hasNotifications"];
      instance.hasNotifications = __raw_hasNotifications;
    }
    {
      const __raw_hasRequiredLeadSource = obj["hasRequiredLeadSource"];
      instance.hasRequiredLeadSource = __raw_hasRequiredLeadSource;
    }
    {
      const __raw_hasRequiredEmail = obj["hasRequiredEmail"];
      instance.hasRequiredEmail = __raw_hasRequiredEmail;
    }
    {
      const __raw_hasSortServiceItemsAlphabetically =
        obj["hasSortServiceItemsAlphabetically"];
      instance.hasSortServiceItemsAlphabetically =
        __raw_hasSortServiceItemsAlphabetically;
    }
    {
      const __raw_hasAttachOrderToAppointmentEmails =
        obj["hasAttachOrderToAppointmentEmails"];
      instance.hasAttachOrderToAppointmentEmails =
        __raw_hasAttachOrderToAppointmentEmails;
    }
    {
      const __raw_scheduleInterval = obj["scheduleInterval"];
      instance.scheduleInterval = __raw_scheduleInterval;
    }
    {
      const __raw_colorsConfig = obj["colorsConfig"];
      if (typeof (ColorsConfig as any)?.__deserialize === "function") {
        const __result = (ColorsConfig as any).__deserialize(
          __raw_colorsConfig,
          ctx,
        );
        ctx.assignOrDefer(instance, "colorsConfig", __result);
      } else {
        instance.colorsConfig = __raw_colorsConfig;
      }
    }
    return instance as Company;
  }
}

/**  */
export interface Ordinal {
  north: number;
  northeast: number;
  east: number;
  southeast: number;
  south: number;
  southwest: number;
  west: number;
  northwest: number;
}

export namespace Ordinal {
  export function defaultValue(): Ordinal {
    return {
      north: 0,
      northeast: 0,
      east: 0,
      southeast: 0,
      south: 0,
      southwest: 0,
      west: 0,
      northwest: 0,
    } as Ordinal;
  }
}

export namespace Ordinal {
  export function toJson(self: Ordinal): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Ordinal,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Ordinal", __id };
    result["north"] = self.north;
    result["northeast"] = self.northeast;
    result["east"] = self.east;
    result["southeast"] = self.southeast;
    result["south"] = self.south;
    result["southwest"] = self.southwest;
    result["west"] = self.west;
    result["northwest"] = self.northwest;
    return result;
  }
}

export namespace Ordinal {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Ordinal, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Ordinal.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Ordinal | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Ordinal.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("north" in obj)) {
      errors.push('Ordinal.__deserialize: missing required field "north"');
    }
    if (!("northeast" in obj)) {
      errors.push('Ordinal.__deserialize: missing required field "northeast"');
    }
    if (!("east" in obj)) {
      errors.push('Ordinal.__deserialize: missing required field "east"');
    }
    if (!("southeast" in obj)) {
      errors.push('Ordinal.__deserialize: missing required field "southeast"');
    }
    if (!("south" in obj)) {
      errors.push('Ordinal.__deserialize: missing required field "south"');
    }
    if (!("southwest" in obj)) {
      errors.push('Ordinal.__deserialize: missing required field "southwest"');
    }
    if (!("west" in obj)) {
      errors.push('Ordinal.__deserialize: missing required field "west"');
    }
    if (!("northwest" in obj)) {
      errors.push('Ordinal.__deserialize: missing required field "northwest"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_north = obj["north"];
      instance.north = __raw_north;
    }
    {
      const __raw_northeast = obj["northeast"];
      instance.northeast = __raw_northeast;
    }
    {
      const __raw_east = obj["east"];
      instance.east = __raw_east;
    }
    {
      const __raw_southeast = obj["southeast"];
      instance.southeast = __raw_southeast;
    }
    {
      const __raw_south = obj["south"];
      instance.south = __raw_south;
    }
    {
      const __raw_southwest = obj["southwest"];
      instance.southwest = __raw_southwest;
    }
    {
      const __raw_west = obj["west"];
      instance.west = __raw_west;
    }
    {
      const __raw_northwest = obj["northwest"];
      instance.northwest = __raw_northwest;
    }
    return instance as Ordinal;
  }
}

/**  */
export interface Password {
  password: string;
}

export namespace Password {
  export function defaultValue(): Password {
    return { password: "" } as Password;
  }
}

export namespace Password {
  export function toJson(self: Password): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Password,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Password", __id };
    result["password"] = self.password;
    return result;
  }
}

export namespace Password {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Password, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Password.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Password | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Password.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("password" in obj)) {
      errors.push('Password.__deserialize: missing required field "password"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_password = obj["password"];
      instance.password = __raw_password;
    }
    return instance as Password;
  }
}

/**  */
export interface Created {
  initialData: string | null;
}

export namespace Created {
  export function defaultValue(): Created {
    return { initialData: null } as Created;
  }
}

export namespace Created {
  export function toJson(self: Created): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Created,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Created", __id };
    if (self.initialData !== null) {
      result["initialData"] =
        typeof (self.initialData as any)?.__serialize === "function"
          ? (self.initialData as any).__serialize(ctx)
          : self.initialData;
    } else {
      result["initialData"] = null;
    }
    return result;
  }
}

export namespace Created {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Created, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Created.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Created | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Created.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("initialData" in obj)) {
      errors.push(
        'Created.__deserialize: missing required field "initialData"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_initialData = obj["initialData"];
      instance.initialData = __raw_initialData;
    }
    return instance as Created;
  }
}

/**  */
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

export namespace Employee {
  export function toJson(self: Employee): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Employee,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Employee", __id };
    result["id"] = self.id;
    if (self.imageUrl !== null) {
      result["imageUrl"] =
        typeof (self.imageUrl as any)?.__serialize === "function"
          ? (self.imageUrl as any).__serialize(ctx)
          : self.imageUrl;
    } else {
      result["imageUrl"] = null;
    }
    result["name"] = self.name;
    result["phones"] = self.phones.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["role"] = self.role;
    result["title"] =
      typeof (self.title as any)?.__serialize === "function"
        ? (self.title as any).__serialize(ctx)
        : self.title;
    result["email"] =
      typeof (self.email as any)?.__serialize === "function"
        ? (self.email as any).__serialize(ctx)
        : self.email;
    result["address"] = self.address;
    result["username"] = self.username;
    result["route"] = self.route;
    result["ratePerHour"] = self.ratePerHour;
    result["active"] = self.active;
    result["isTechnician"] = self.isTechnician;
    result["isSalesRep"] = self.isSalesRep;
    if (self.description !== null) {
      result["description"] =
        typeof (self.description as any)?.__serialize === "function"
          ? (self.description as any).__serialize(ctx)
          : self.description;
    } else {
      result["description"] = null;
    }
    if (self.linkedinUrl !== null) {
      result["linkedinUrl"] =
        typeof (self.linkedinUrl as any)?.__serialize === "function"
          ? (self.linkedinUrl as any).__serialize(ctx)
          : self.linkedinUrl;
    } else {
      result["linkedinUrl"] = null;
    }
    result["attendance"] = self.attendance.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    result["settings"] =
      typeof (self.settings as any)?.__serialize === "function"
        ? (self.settings as any).__serialize(ctx)
        : self.settings;
    return result;
  }
}

export namespace Employee {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Employee, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Employee.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Employee | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Employee.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('Employee.__deserialize: missing required field "id"');
    }
    if (!("imageUrl" in obj)) {
      errors.push('Employee.__deserialize: missing required field "imageUrl"');
    }
    if (!("name" in obj)) {
      errors.push('Employee.__deserialize: missing required field "name"');
    }
    if (!("phones" in obj)) {
      errors.push('Employee.__deserialize: missing required field "phones"');
    }
    if (!("role" in obj)) {
      errors.push('Employee.__deserialize: missing required field "role"');
    }
    if (!("title" in obj)) {
      errors.push('Employee.__deserialize: missing required field "title"');
    }
    if (!("email" in obj)) {
      errors.push('Employee.__deserialize: missing required field "email"');
    }
    if (!("address" in obj)) {
      errors.push('Employee.__deserialize: missing required field "address"');
    }
    if (!("username" in obj)) {
      errors.push('Employee.__deserialize: missing required field "username"');
    }
    if (!("route" in obj)) {
      errors.push('Employee.__deserialize: missing required field "route"');
    }
    if (!("ratePerHour" in obj)) {
      errors.push(
        'Employee.__deserialize: missing required field "ratePerHour"',
      );
    }
    if (!("active" in obj)) {
      errors.push('Employee.__deserialize: missing required field "active"');
    }
    if (!("isTechnician" in obj)) {
      errors.push(
        'Employee.__deserialize: missing required field "isTechnician"',
      );
    }
    if (!("isSalesRep" in obj)) {
      errors.push(
        'Employee.__deserialize: missing required field "isSalesRep"',
      );
    }
    if (!("description" in obj)) {
      errors.push(
        'Employee.__deserialize: missing required field "description"',
      );
    }
    if (!("linkedinUrl" in obj)) {
      errors.push(
        'Employee.__deserialize: missing required field "linkedinUrl"',
      );
    }
    if (!("attendance" in obj)) {
      errors.push(
        'Employee.__deserialize: missing required field "attendance"',
      );
    }
    if (!("settings" in obj)) {
      errors.push('Employee.__deserialize: missing required field "settings"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_imageUrl = obj["imageUrl"];
      instance.imageUrl = __raw_imageUrl;
    }
    {
      const __raw_name = obj["name"];
      instance.name = __raw_name;
    }
    {
      const __raw_phones = obj["phones"];
      instance.phones = __raw_phones;
    }
    {
      const __raw_role = obj["role"];
      instance.role = __raw_role;
    }
    {
      const __raw_title = obj["title"];
      if (typeof (JobTitle as any)?.__deserialize === "function") {
        const __result = (JobTitle as any).__deserialize(__raw_title, ctx);
        ctx.assignOrDefer(instance, "title", __result);
      } else {
        instance.title = __raw_title;
      }
    }
    {
      const __raw_email = obj["email"];
      if (typeof (Email as any)?.__deserialize === "function") {
        const __result = (Email as any).__deserialize(__raw_email, ctx);
        ctx.assignOrDefer(instance, "email", __result);
      } else {
        instance.email = __raw_email;
      }
    }
    {
      const __raw_address = obj["address"];
      instance.address = __raw_address;
    }
    {
      const __raw_username = obj["username"];
      instance.username = __raw_username;
    }
    {
      const __raw_route = obj["route"];
      instance.route = __raw_route;
    }
    {
      const __raw_ratePerHour = obj["ratePerHour"];
      instance.ratePerHour = __raw_ratePerHour;
    }
    {
      const __raw_active = obj["active"];
      instance.active = __raw_active;
    }
    {
      const __raw_isTechnician = obj["isTechnician"];
      instance.isTechnician = __raw_isTechnician;
    }
    {
      const __raw_isSalesRep = obj["isSalesRep"];
      instance.isSalesRep = __raw_isSalesRep;
    }
    {
      const __raw_description = obj["description"];
      instance.description = __raw_description;
    }
    {
      const __raw_linkedinUrl = obj["linkedinUrl"];
      instance.linkedinUrl = __raw_linkedinUrl;
    }
    {
      const __raw_attendance = obj["attendance"];
      instance.attendance = __raw_attendance;
    }
    {
      const __raw_settings = obj["settings"];
      if (typeof (Settings as any)?.__deserialize === "function") {
        const __result = (Settings as any).__deserialize(__raw_settings, ctx);
        ctx.assignOrDefer(instance, "settings", __result);
      } else {
        instance.settings = __raw_settings;
      }
    }
    return instance as Employee;
  }
}

/**  */
export interface Commissions {
  technician: string;

  salesRep: string;
}

export namespace Commissions {
  export function defaultValue(): Commissions {
    return { technician: "", salesRep: "" } as Commissions;
  }
}

export namespace Commissions {
  export function toJson(self: Commissions): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Commissions,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Commissions", __id };
    result["technician"] = self.technician;
    result["salesRep"] = self.salesRep;
    return result;
  }
}

export namespace Commissions {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Commissions, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Commissions.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Commissions | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Commissions.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("technician" in obj)) {
      errors.push(
        'Commissions.__deserialize: missing required field "technician"',
      );
    }
    if (!("salesRep" in obj)) {
      errors.push(
        'Commissions.__deserialize: missing required field "salesRep"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_technician = obj["technician"];
      instance.technician = __raw_technician;
    }
    {
      const __raw_salesRep = obj["salesRep"];
      instance.salesRep = __raw_salesRep;
    }
    return instance as Commissions;
  }
}

/**  */
export interface Number {
  countryCode: string;

  areaCode: string;

  localNumber: string;
}

export namespace Number {
  export function defaultValue(): Number {
    return { countryCode: "", areaCode: "", localNumber: "" } as Number;
  }
}

export namespace Number {
  export function toJson(self: Number): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Number,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Number", __id };
    result["countryCode"] = self.countryCode;
    result["areaCode"] = self.areaCode;
    result["localNumber"] = self.localNumber;
    return result;
  }
}

export namespace Number {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Number, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Number.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Number | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Number.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("countryCode" in obj)) {
      errors.push('Number.__deserialize: missing required field "countryCode"');
    }
    if (!("areaCode" in obj)) {
      errors.push('Number.__deserialize: missing required field "areaCode"');
    }
    if (!("localNumber" in obj)) {
      errors.push('Number.__deserialize: missing required field "localNumber"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_countryCode = obj["countryCode"];
      instance.countryCode = __raw_countryCode;
    }
    {
      const __raw_areaCode = obj["areaCode"];
      instance.areaCode = __raw_areaCode;
    }
    {
      const __raw_localNumber = obj["localNumber"];
      instance.localNumber = __raw_localNumber;
    }
    return instance as Number;
  }
}

/**  */
export interface DataPath {
  path: string[];
  formatter: string | null;
}

export namespace DataPath {
  export function defaultValue(): DataPath {
    return { path: [], formatter: null } as DataPath;
  }
}

export namespace DataPath {
  export function toJson(self: DataPath): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: DataPath,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "DataPath", __id };
    result["path"] = self.path.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    if (self.formatter !== null) {
      result["formatter"] =
        typeof (self.formatter as any)?.__serialize === "function"
          ? (self.formatter as any).__serialize(ctx)
          : self.formatter;
    } else {
      result["formatter"] = null;
    }
    return result;
  }
}

export namespace DataPath {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<DataPath, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "DataPath.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): DataPath | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("DataPath.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("path" in obj)) {
      errors.push('DataPath.__deserialize: missing required field "path"');
    }
    if (!("formatter" in obj)) {
      errors.push('DataPath.__deserialize: missing required field "formatter"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_path = obj["path"];
      instance.path = __raw_path;
    }
    {
      const __raw_formatter = obj["formatter"];
      instance.formatter = __raw_formatter;
    }
    return instance as DataPath;
  }
}

/**  */
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

export namespace Route {
  export function defaultValue(): Route {
    return {
      id: "",
      techs: null,
      active: false,
      name: "",
      phone: "",
      position: "",
      serviceRoute: false,
      defaultDurationHours: 0,
      tags: [],
      icon: null,
      color: null,
    } as Route;
  }
}

export namespace Route {
  export function toJson(self: Route): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Route,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Route", __id };
    result["id"] = self.id;
    if (self.techs !== null) {
      result["techs"] =
        typeof (self.techs as any)?.__serialize === "function"
          ? (self.techs as any).__serialize(ctx)
          : self.techs;
    } else {
      result["techs"] = null;
    }
    result["active"] = self.active;
    result["name"] = self.name;
    result["phone"] = self.phone;
    result["position"] = self.position;
    result["serviceRoute"] = self.serviceRoute;
    result["defaultDurationHours"] = self.defaultDurationHours;
    result["tags"] = self.tags.map((item: any) =>
      typeof item?.__serialize === "function" ? item.__serialize(ctx) : item,
    );
    if (self.icon !== null) {
      result["icon"] =
        typeof (self.icon as any)?.__serialize === "function"
          ? (self.icon as any).__serialize(ctx)
          : self.icon;
    } else {
      result["icon"] = null;
    }
    if (self.color !== null) {
      result["color"] =
        typeof (self.color as any)?.__serialize === "function"
          ? (self.color as any).__serialize(ctx)
          : self.color;
    } else {
      result["color"] = null;
    }
    return result;
  }
}

export namespace Route {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Route, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err(["Route.fromJson: root cannot be a forward reference"]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Route | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Route.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('Route.__deserialize: missing required field "id"');
    }
    if (!("techs" in obj)) {
      errors.push('Route.__deserialize: missing required field "techs"');
    }
    if (!("active" in obj)) {
      errors.push('Route.__deserialize: missing required field "active"');
    }
    if (!("name" in obj)) {
      errors.push('Route.__deserialize: missing required field "name"');
    }
    if (!("phone" in obj)) {
      errors.push('Route.__deserialize: missing required field "phone"');
    }
    if (!("position" in obj)) {
      errors.push('Route.__deserialize: missing required field "position"');
    }
    if (!("serviceRoute" in obj)) {
      errors.push('Route.__deserialize: missing required field "serviceRoute"');
    }
    if (!("defaultDurationHours" in obj)) {
      errors.push(
        'Route.__deserialize: missing required field "defaultDurationHours"',
      );
    }
    if (!("tags" in obj)) {
      errors.push('Route.__deserialize: missing required field "tags"');
    }
    if (!("icon" in obj)) {
      errors.push('Route.__deserialize: missing required field "icon"');
    }
    if (!("color" in obj)) {
      errors.push('Route.__deserialize: missing required field "color"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_techs = obj["techs"];
      instance.techs = __raw_techs;
    }
    {
      const __raw_active = obj["active"];
      instance.active = __raw_active;
    }
    {
      const __raw_name = obj["name"];
      instance.name = __raw_name;
    }
    {
      const __raw_phone = obj["phone"];
      instance.phone = __raw_phone;
    }
    {
      const __raw_position = obj["position"];
      instance.position = __raw_position;
    }
    {
      const __raw_serviceRoute = obj["serviceRoute"];
      instance.serviceRoute = __raw_serviceRoute;
    }
    {
      const __raw_defaultDurationHours = obj["defaultDurationHours"];
      instance.defaultDurationHours = __raw_defaultDurationHours;
    }
    {
      const __raw_tags = obj["tags"];
      instance.tags = __raw_tags;
    }
    {
      const __raw_icon = obj["icon"];
      instance.icon = __raw_icon;
    }
    {
      const __raw_color = obj["color"];
      instance.color = __raw_color;
    }
    return instance as Route;
  }
}

/**  */
export interface EmailParts {
  local: string;

  domainName: string;

  topLevelDomain: string;
}

export namespace EmailParts {
  export function defaultValue(): EmailParts {
    return { local: "", domainName: "", topLevelDomain: "" } as EmailParts;
  }
}

export namespace EmailParts {
  export function toJson(self: EmailParts): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: EmailParts,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "EmailParts", __id };
    result["local"] = self.local;
    result["domainName"] = self.domainName;
    result["topLevelDomain"] = self.topLevelDomain;
    return result;
  }
}

export namespace EmailParts {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<EmailParts, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "EmailParts.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): EmailParts | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("EmailParts.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("local" in obj)) {
      errors.push('EmailParts.__deserialize: missing required field "local"');
    }
    if (!("domainName" in obj)) {
      errors.push(
        'EmailParts.__deserialize: missing required field "domainName"',
      );
    }
    if (!("topLevelDomain" in obj)) {
      errors.push(
        'EmailParts.__deserialize: missing required field "topLevelDomain"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_local = obj["local"];
      instance.local = __raw_local;
    }
    {
      const __raw_domainName = obj["domainName"];
      instance.domainName = __raw_domainName;
    }
    {
      const __raw_topLevelDomain = obj["topLevelDomain"];
      instance.topLevelDomain = __raw_topLevelDomain;
    }
    return instance as EmailParts;
  }
}

/**  */
export interface Sent {
  recipient: string | null;
  method: string | null;
}

export namespace Sent {
  export function defaultValue(): Sent {
    return { recipient: null, method: null } as Sent;
  }
}

export namespace Sent {
  export function toJson(self: Sent): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Sent,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Sent", __id };
    if (self.recipient !== null) {
      result["recipient"] =
        typeof (self.recipient as any)?.__serialize === "function"
          ? (self.recipient as any).__serialize(ctx)
          : self.recipient;
    } else {
      result["recipient"] = null;
    }
    if (self.method !== null) {
      result["method"] =
        typeof (self.method as any)?.__serialize === "function"
          ? (self.method as any).__serialize(ctx)
          : self.method;
    } else {
      result["method"] = null;
    }
    return result;
  }
}

export namespace Sent {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Sent, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err(["Sent.fromJson: root cannot be a forward reference"]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Sent | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Sent.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("recipient" in obj)) {
      errors.push('Sent.__deserialize: missing required field "recipient"');
    }
    if (!("method" in obj)) {
      errors.push('Sent.__deserialize: missing required field "method"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_recipient = obj["recipient"];
      instance.recipient = __raw_recipient;
    }
    {
      const __raw_method = obj["method"];
      instance.method = __raw_method;
    }
    return instance as Sent;
  }
}

/**  */
export interface BilledItem {
  item: Item;
  quantity: number;
  taxed: boolean;
  upsale: boolean;
}

export namespace BilledItem {
  export function toJson(self: BilledItem): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: BilledItem,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "BilledItem", __id };
    result["item"] =
      typeof (self.item as any)?.__serialize === "function"
        ? (self.item as any).__serialize(ctx)
        : self.item;
    result["quantity"] = self.quantity;
    result["taxed"] = self.taxed;
    result["upsale"] = self.upsale;
    return result;
  }
}

export namespace BilledItem {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<BilledItem, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "BilledItem.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): BilledItem | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("BilledItem.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("item" in obj)) {
      errors.push('BilledItem.__deserialize: missing required field "item"');
    }
    if (!("quantity" in obj)) {
      errors.push(
        'BilledItem.__deserialize: missing required field "quantity"',
      );
    }
    if (!("taxed" in obj)) {
      errors.push('BilledItem.__deserialize: missing required field "taxed"');
    }
    if (!("upsale" in obj)) {
      errors.push('BilledItem.__deserialize: missing required field "upsale"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_item = obj["item"];
      if (typeof (Item as any)?.__deserialize === "function") {
        const __result = (Item as any).__deserialize(__raw_item, ctx);
        ctx.assignOrDefer(instance, "item", __result);
      } else {
        instance.item = __raw_item;
      }
    }
    {
      const __raw_quantity = obj["quantity"];
      instance.quantity = __raw_quantity;
    }
    {
      const __raw_taxed = obj["taxed"];
      instance.taxed = __raw_taxed;
    }
    {
      const __raw_upsale = obj["upsale"];
      instance.upsale = __raw_upsale;
    }
    return instance as BilledItem;
  }
}

/**  */
export interface Coordinates {
  lat: number;
  lng: number;
}

export namespace Coordinates {
  export function defaultValue(): Coordinates {
    return { lat: 0, lng: 0 } as Coordinates;
  }
}

export namespace Coordinates {
  export function toJson(self: Coordinates): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Coordinates,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Coordinates", __id };
    result["lat"] = self.lat;
    result["lng"] = self.lng;
    return result;
  }
}

export namespace Coordinates {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Coordinates, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Coordinates.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Coordinates | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Coordinates.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("lat" in obj)) {
      errors.push('Coordinates.__deserialize: missing required field "lat"');
    }
    if (!("lng" in obj)) {
      errors.push('Coordinates.__deserialize: missing required field "lng"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_lat = obj["lat"];
      instance.lat = __raw_lat;
    }
    {
      const __raw_lng = obj["lng"];
      instance.lng = __raw_lng;
    }
    return instance as Coordinates;
  }
}

/**  */
export interface Ordered {
  id: string;
  in: string | Account;
  out: string | Order;
  date: string;
}

export namespace Ordered {
  export function toJson(self: Ordered): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Ordered,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Ordered", __id };
    result["id"] = self.id;
    result["in"] = self.in;
    result["out"] = self.out;
    result["date"] = self.date;
    return result;
  }
}

export namespace Ordered {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Ordered, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Ordered.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Ordered | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Ordered.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
      errors.push('Ordered.__deserialize: missing required field "id"');
    }
    if (!("in" in obj)) {
      errors.push('Ordered.__deserialize: missing required field "in"');
    }
    if (!("out" in obj)) {
      errors.push('Ordered.__deserialize: missing required field "out"');
    }
    if (!("date" in obj)) {
      errors.push('Ordered.__deserialize: missing required field "date"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_in = obj["in"];
      instance.in = __raw_in;
    }
    {
      const __raw_out = obj["out"];
      instance.out = __raw_out;
    }
    {
      const __raw_date = obj["date"];
      instance.date = __raw_date;
    }
    return instance as Ordered;
  }
}

/**  */
export interface Email {
  canEmail: boolean;

  emailString: string;
}

export namespace Email {
  export function defaultValue(): Email {
    return { canEmail: false, emailString: "" } as Email;
  }
}

export namespace Email {
  export function toJson(self: Email): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Email,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Email", __id };
    result["canEmail"] = self.canEmail;
    result["emailString"] = self.emailString;
    return result;
  }
}

export namespace Email {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Email, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err(["Email.fromJson: root cannot be a forward reference"]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Email | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Email.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("canEmail" in obj)) {
      errors.push('Email.__deserialize: missing required field "canEmail"');
    }
    if (!("emailString" in obj)) {
      errors.push('Email.__deserialize: missing required field "emailString"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_canEmail = obj["canEmail"];
      instance.canEmail = __raw_canEmail;
    }
    {
      const __raw_emailString = obj["emailString"];
      instance.emailString = __raw_emailString;
    }
    return instance as Email;
  }
}

/**  */
export interface RecurrenceRule {
  interval: Interval;
  recurrenceBegins: string;
  recurrenceEnds: RecurrenceEnd | null;
  cancelledInstances: string[] | null;
  additionalInstances: string[] | null;
}

export namespace RecurrenceRule {
  export function toJson(self: RecurrenceRule): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: RecurrenceRule,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "RecurrenceRule", __id };
    result["interval"] =
      typeof (self.interval as any)?.__serialize === "function"
        ? (self.interval as any).__serialize(ctx)
        : self.interval;
    result["recurrenceBegins"] = self.recurrenceBegins;
    if (self.recurrenceEnds !== null) {
      result["recurrenceEnds"] =
        typeof (self.recurrenceEnds as any)?.__serialize === "function"
          ? (self.recurrenceEnds as any).__serialize(ctx)
          : self.recurrenceEnds;
    } else {
      result["recurrenceEnds"] = null;
    }
    if (self.cancelledInstances !== null) {
      result["cancelledInstances"] =
        typeof (self.cancelledInstances as any)?.__serialize === "function"
          ? (self.cancelledInstances as any).__serialize(ctx)
          : self.cancelledInstances;
    } else {
      result["cancelledInstances"] = null;
    }
    if (self.additionalInstances !== null) {
      result["additionalInstances"] =
        typeof (self.additionalInstances as any)?.__serialize === "function"
          ? (self.additionalInstances as any).__serialize(ctx)
          : self.additionalInstances;
    } else {
      result["additionalInstances"] = null;
    }
    return result;
  }
}

export namespace RecurrenceRule {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<RecurrenceRule, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "RecurrenceRule.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): RecurrenceRule | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("RecurrenceRule.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("interval" in obj)) {
      errors.push(
        'RecurrenceRule.__deserialize: missing required field "interval"',
      );
    }
    if (!("recurrenceBegins" in obj)) {
      errors.push(
        'RecurrenceRule.__deserialize: missing required field "recurrenceBegins"',
      );
    }
    if (!("recurrenceEnds" in obj)) {
      errors.push(
        'RecurrenceRule.__deserialize: missing required field "recurrenceEnds"',
      );
    }
    if (!("cancelledInstances" in obj)) {
      errors.push(
        'RecurrenceRule.__deserialize: missing required field "cancelledInstances"',
      );
    }
    if (!("additionalInstances" in obj)) {
      errors.push(
        'RecurrenceRule.__deserialize: missing required field "additionalInstances"',
      );
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_interval = obj["interval"];
      if (typeof (Interval as any)?.__deserialize === "function") {
        const __result = (Interval as any).__deserialize(__raw_interval, ctx);
        ctx.assignOrDefer(instance, "interval", __result);
      } else {
        instance.interval = __raw_interval;
      }
    }
    {
      const __raw_recurrenceBegins = obj["recurrenceBegins"];
      instance.recurrenceBegins = __raw_recurrenceBegins;
    }
    {
      const __raw_recurrenceEnds = obj["recurrenceEnds"];
      instance.recurrenceEnds = __raw_recurrenceEnds;
    }
    {
      const __raw_cancelledInstances = obj["cancelledInstances"];
      instance.cancelledInstances = __raw_cancelledInstances;
    }
    {
      const __raw_additionalInstances = obj["additionalInstances"];
      instance.additionalInstances = __raw_additionalInstances;
    }
    return instance as RecurrenceRule;
  }
}

/**  */
export interface LastName {
  name: string;
}

export namespace LastName {
  export function defaultValue(): LastName {
    return { name: "" } as LastName;
  }
}

export namespace LastName {
  export function toJson(self: LastName): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: LastName,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "LastName", __id };
    result["name"] = self.name;
    return result;
  }
}

export namespace LastName {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<LastName, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "LastName.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): LastName | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("LastName.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("name" in obj)) {
      errors.push('LastName.__deserialize: missing required field "name"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_name = obj["name"];
      instance.name = __raw_name;
    }
    return instance as LastName;
  }
}

/**  */
export interface Cardinal {
  north: number;
  east: number;
  south: number;
  west: number;
}

export namespace Cardinal {
  export function defaultValue(): Cardinal {
    return { north: 0, east: 0, south: 0, west: 0 } as Cardinal;
  }
}

export namespace Cardinal {
  export function toJson(self: Cardinal): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serialize(self, ctx));
  }
  export function __serialize(
    self: Cardinal,
    ctx: SerializeContext,
  ): Record<string, unknown> {
    const existingId = ctx.getId(self);
    if (existingId !== undefined) {
      return { __ref: existingId };
    }
    const __id = ctx.register(self);
    const result: Record<string, unknown> = { __type: "Cardinal", __id };
    result["north"] = self.north;
    result["east"] = self.east;
    result["south"] = self.south;
    result["west"] = self.west;
    return result;
  }
}

export namespace Cardinal {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Cardinal, string[]> {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const resultOrRef = __deserialize(raw, ctx);
    if (PendingRef.is(resultOrRef)) {
      return Result.err([
        "Cardinal.fromJson: root cannot be a forward reference",
      ]);
    }
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return Result.ok(resultOrRef);
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Cardinal | PendingRef {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      throw new Error("Cardinal.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("north" in obj)) {
      errors.push('Cardinal.__deserialize: missing required field "north"');
    }
    if (!("east" in obj)) {
      errors.push('Cardinal.__deserialize: missing required field "east"');
    }
    if (!("south" in obj)) {
      errors.push('Cardinal.__deserialize: missing required field "south"');
    }
    if (!("west" in obj)) {
      errors.push('Cardinal.__deserialize: missing required field "west"');
    }
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
      ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
      const __raw_north = obj["north"];
      instance.north = __raw_north;
    }
    {
      const __raw_east = obj["east"];
      instance.east = __raw_east;
    }
    {
      const __raw_south = obj["south"];
      instance.south = __raw_south;
    }
    {
      const __raw_west = obj["west"];
      instance.west = __raw_west;
    }
    return instance as Cardinal;
  }
}

/**  */
export type Interval =
  | DailyRecurrenceRule
  | WeeklyRecurrenceRule
  | MonthlyRecurrenceRule
  | YearlyRecurrenceRule;

export namespace Interval {
  export function fromJson(json: string, opts?: DeserializeOptions): Interval {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(value: any, ctx: DeserializeContext): Interval {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as Interval;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "Interval.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as Interval;
  }
}

/**  */
export type Page =
  | "SalesHomeDashboard"
  | "SalesHomeProducts"
  | "SalesHomeServices"
  | "SalesHomePackages"
  | "SalesHomeTaxRates"
  | "SalesLeadsOverview"
  | "SalesLeadsActivities"
  | "SalesLeadsCampaigns"
  | "SalesLeadsDripCampaigns"
  | "SalesLeadsOpportunities"
  | "SalesLeadsPromotions"
  | "SalesAccountsOverview"
  | "SalesAccountsActivities"
  | "SalesAccountsBilling"
  | "SalesAccountsContracts"
  | "SalesOrdersOverview"
  | "SalesOrdersActivities"
  | "SalesOrdersPayments"
  | "SalesOrdersCommissions"
  | "SalesSchedulingSchedule"
  | "SalesSchedulingAppointments"
  | "SalesSchedulingRecurring"
  | "SalesSchedulingRoutes"
  | "SalesSchedulingReminders"
  | "UserHome";

export namespace Page {
  export function fromJson(json: string, opts?: DeserializeOptions): Page {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(value: any, ctx: DeserializeContext): Page {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as Page;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "Page.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as Page;
  }
}

/**  */
export type UserRole =
  | "Administrator"
  | "SalesRepresentative"
  | "Technician"
  | "HumanResources"
  | "InformationTechnology";

export namespace UserRole {
  export function fromJson(json: string, opts?: DeserializeOptions): UserRole {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(value: any, ctx: DeserializeContext): UserRole {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as UserRole;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "UserRole.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as UserRole;
  }
}

/**  */
export type Target =
  | Account
  | User
  | Employee
  | Appointment
  | Lead
  | TaxRate
  | Site
  | Route
  | Company
  | Product
  | Service
  | Order
  | Payment
  | Package
  | Promotion
  | Represents
  | Ordered;

export namespace Target {
  export function fromJson(json: string, opts?: DeserializeOptions): Target {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(value: any, ctx: DeserializeContext): Target {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as Target;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "Target.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as Target;
  }
}

/**  */
export type RecurrenceEnd = number | string;

export namespace RecurrenceEnd {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): RecurrenceEnd {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): RecurrenceEnd {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as RecurrenceEnd;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "RecurrenceEnd.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as RecurrenceEnd;
  }
}

/**  */
export type OverviewDisplay = "Card" | "Table";

export namespace OverviewDisplay {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): OverviewDisplay {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): OverviewDisplay {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as OverviewDisplay;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "OverviewDisplay.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as OverviewDisplay;
  }
}

/**  */
export type IntervalUnit = "Day" | "Week" | "Month" | "Year";

export namespace IntervalUnit {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): IntervalUnit {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): IntervalUnit {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as IntervalUnit;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "IntervalUnit.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as IntervalUnit;
  }
}

/**  */
export type Sector = "Residential" | "Commercial";

export namespace Sector {
  export function fromJson(json: string, opts?: DeserializeOptions): Sector {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(value: any, ctx: DeserializeContext): Sector {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as Sector;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "Sector.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as Sector;
  }
}

/**  */
export type Weekday =
  | "Monday"
  | "Tuesday"
  | "Wednesday"
  | "Thursday"
  | "Friday"
  | "Saturday"
  | "Sunday";

export namespace Weekday {
  export function fromJson(json: string, opts?: DeserializeOptions): Weekday {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(value: any, ctx: DeserializeContext): Weekday {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as Weekday;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "Weekday.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as Weekday;
  }
}

/**  */
export type Status = "Scheduled" | "OnDeck" | "Waiting";

export namespace Status {
  export function fromJson(json: string, opts?: DeserializeOptions): Status {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(value: any, ctx: DeserializeContext): Status {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as Status;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "Status.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as Status;
  }
}

/**  */
export type NextStep =
  | "InitialContact"
  | "Qualified"
  | "Estimate"
  | "Negotiation";

export namespace NextStep {
  export function fromJson(json: string, opts?: DeserializeOptions): NextStep {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(value: any, ctx: DeserializeContext): NextStep {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as NextStep;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "NextStep.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as NextStep;
  }
}

/**  */
export type LeadStage =
  | "Open"
  | "InitialContact"
  | "Qualified"
  | "Estimate"
  | "Negotiation";

export namespace LeadStage {
  export function fromJson(json: string, opts?: DeserializeOptions): LeadStage {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): LeadStage {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as LeadStage;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "LeadStage.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as LeadStage;
  }
}

/**  */
export type AccountName = CompanyName | PersonName;

export namespace AccountName {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): AccountName {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): AccountName {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as AccountName;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "AccountName.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as AccountName;
  }
}

/**  */
export type Priority = "High" | "Medium" | "Low";

export namespace Priority {
  export function fromJson(json: string, opts?: DeserializeOptions): Priority {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(value: any, ctx: DeserializeContext): Priority {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as Priority;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "Priority.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as Priority;
  }
}

/**  */
export type Applications =
  | "Sales"
  | "Accounting"
  | "Errand"
  | "HumanResources"
  | "Logistics"
  | "Marketing"
  | "Website";

export namespace Applications {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): Applications {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): Applications {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as Applications;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "Applications.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as Applications;
  }
}

/**  */
export type JobTitle =
  | "Technician"
  | "SalesRepresentative"
  | "HumanResources"
  | "InformationTechnology";

export namespace JobTitle {
  export function fromJson(json: string, opts?: DeserializeOptions): JobTitle {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(value: any, ctx: DeserializeContext): JobTitle {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as JobTitle;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "JobTitle.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as JobTitle;
  }
}

/**  */
export type ColorsConfig = Cardinal | Ordinal | Custom | Gradient;

export namespace ColorsConfig {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): ColorsConfig {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): ColorsConfig {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as ColorsConfig;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "ColorsConfig.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as ColorsConfig;
  }
}

/**  */
export type WeekOfMonth = "First" | "Second" | "Third" | "Fourth" | "Last";

export namespace WeekOfMonth {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): WeekOfMonth {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): WeekOfMonth {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as WeekOfMonth;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "WeekOfMonth.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as WeekOfMonth;
  }
}

/**  */
export type ActivityType = Created | Edited | Sent | Viewed | Commented | Paid;

export namespace ActivityType {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): ActivityType {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): ActivityType {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as ActivityType;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "ActivityType.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as ActivityType;
  }
}

/**  */
export type RowHeight = "ExtraSmall" | "Small" | "Medium" | "Large";

export namespace RowHeight {
  export function fromJson(json: string, opts?: DeserializeOptions): RowHeight {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): RowHeight {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as RowHeight;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "RowHeight.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as RowHeight;
  }
}

/**  */
export type OrderStage = "Estimate" | "Active" | "Invoice";

export namespace OrderStage {
  export function fromJson(
    json: string,
    opts?: DeserializeOptions,
  ): OrderStage {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(
    value: any,
    ctx: DeserializeContext,
  ): OrderStage {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as OrderStage;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "OrderStage.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as OrderStage;
  }
}

/**  */
export type Table =
  | "Account"
  | "Did"
  | "Appointment"
  | "Lead"
  | "TaxRate"
  | "Site"
  | "Employee"
  | "Route"
  | "Company"
  | "Product"
  | "Service"
  | "User"
  | "Order"
  | "Payment"
  | "Package"
  | "Promotion"
  | "Represents"
  | "Ordered";

export namespace Table {
  export function fromJson(json: string, opts?: DeserializeOptions): Table {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(value: any, ctx: DeserializeContext): Table {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as Table;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "Table.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as Table;
  }
}

/**  */
export type Item = (string | Product) | (string | Service);

export namespace Item {
  export function fromJson(json: string, opts?: DeserializeOptions): Item {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(value: any, ctx: DeserializeContext): Item {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as Item;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "Item.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as Item;
  }
}

/**  */
export type Actor = User | Employee | Account;

export namespace Actor {
  export function fromJson(json: string, opts?: DeserializeOptions): Actor {
    const ctx = DeserializeContext.create();
    const raw = JSON.parse(json);
    const result = __deserialize(raw, ctx);
    ctx.applyPatches();
    if (opts?.freeze) {
      ctx.freezeAll();
    }
    return result;
  }
  export function __deserialize(value: any, ctx: DeserializeContext): Actor {
    if (value?.__ref !== undefined) {
      return ctx.getOrDefer(value.__ref) as Actor;
    }
    if (typeof (value as any)?.__type === "string") {
      throw new Error(
        "Actor.__deserialize: polymorphic deserialization requires type registry (TODO)",
      );
    }
    return value as Actor;
  }
}
