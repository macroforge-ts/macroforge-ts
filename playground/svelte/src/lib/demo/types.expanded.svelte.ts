import { SerializeContext } from "macroforge/serde";
import { Result } from "macroforge/result";
import { DeserializeContext } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */

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
  export function toStringifiedJSON(self: User): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<User, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "User.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as User;
  }
}

export namespace User {
  export type Data = User;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    email?: Array<string>;
    firstName?: Array<string>;
    lastName?: Array<string>;
    password?: Array<string>;
    metadata?: Array<string>;
    settings?: Settings.Errors;
    role?: UserRole.Errors;
    emailVerified?: Array<string>;
    verificationToken?: Array<string>;
    verificationExpires?: Array<string>;
    passwordResetToken?: Array<string>;
    passwordResetExpires?: Array<string>;
    permissions?: AppPermissions.Errors;
  };
  export type Tainted = {
    id?: boolean;
    email?: boolean;
    firstName?: boolean;
    lastName?: boolean;
    password?: boolean;
    metadata?: boolean;
    settings?: Settings.Tainted;
    role?: UserRole.Tainted;
    emailVerified?: boolean;
    verificationToken?: boolean;
    verificationExpires?: boolean;
    passwordResetToken?: boolean;
    passwordResetExpires?: boolean;
    permissions?: AppPermissions.Tainted;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.email = formData.get("email") ?? "";
    obj.firstName = formData.get("firstName") ?? "";
    obj.lastName = formData.get("lastName") ?? "";
    obj.password = formData.get("password") ?? "";
    obj.metadata = formData.get("metadata") ?? "";
    {
      // Collect nested object fields with prefix "settings."
      const settingsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("settings.")) {
          const fieldName = key.slice("settings.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = settingsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.settings = settingsObj;
    }
    {
      // Collect nested object fields with prefix "role."
      const roleObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("role.")) {
          const fieldName = key.slice("role.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = roleObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.role = roleObj;
    }
    {
      const emailVerifiedVal = formData.get("emailVerified");
      obj.emailVerified =
        emailVerifiedVal === "true" ||
        emailVerifiedVal === "on" ||
        emailVerifiedVal === "1";
    }
    obj.verificationToken = formData.get("verificationToken") ?? "";
    obj.verificationExpires = formData.get("verificationExpires") ?? "";
    obj.passwordResetToken = formData.get("passwordResetToken") ?? "";
    obj.passwordResetExpires = formData.get("passwordResetExpires") ?? "";
    {
      // Collect nested object fields with prefix "permissions."
      const permissionsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("permissions.")) {
          const fieldName = key.slice("permissions.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = permissionsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.permissions = permissionsObj;
    }
    return User.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      email: {
        path: ["email"] as const,
        name: "email",
        constraints: { required: true },

        get: (data: Data) => data.email,
        set: (data: Data, value: string | null) => {
          data.email = value;
        },
        getError: (errors: Errors) => errors?.email,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.email = value;
        },
        getTainted: (tainted: Tainted) => tainted?.email ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.email = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      firstName: {
        path: ["firstName"] as const,
        name: "firstName",
        constraints: { required: true },

        get: (data: Data) => data.firstName,
        set: (data: Data, value: string) => {
          data.firstName = value;
        },
        getError: (errors: Errors) => errors?.firstName,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.firstName = value;
        },
        getTainted: (tainted: Tainted) => tainted?.firstName ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.firstName = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      lastName: {
        path: ["lastName"] as const,
        name: "lastName",
        constraints: { required: true },

        get: (data: Data) => data.lastName,
        set: (data: Data, value: string) => {
          data.lastName = value;
        },
        getError: (errors: Errors) => errors?.lastName,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.lastName = value;
        },
        getTainted: (tainted: Tainted) => tainted?.lastName ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.lastName = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      password: {
        path: ["password"] as const,
        name: "password",
        constraints: { required: true },

        get: (data: Data) => data.password,
        set: (data: Data, value: string | null) => {
          data.password = value;
        },
        getError: (errors: Errors) => errors?.password,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.password = value;
        },
        getTainted: (tainted: Tainted) => tainted?.password ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.password = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      metadata: {
        path: ["metadata"] as const,
        name: "metadata",
        constraints: { required: true },

        get: (data: Data) => data.metadata,
        set: (data: Data, value: Metadata | null) => {
          data.metadata = value;
        },
        getError: (errors: Errors) => errors?.metadata,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.metadata = value;
        },
        getTainted: (tainted: Tainted) => tainted?.metadata ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.metadata = value;
        },
        validate: (_value: Metadata | null): Array<string> => [],
      },
      settings: {
        path: ["settings"] as const,
        name: "settings",
        constraints: { required: true },

        get: (data: Data) => data.settings,
        set: (data: Data, value: Settings) => {
          data.settings = value;
        },
        getError: (errors: Errors) => errors?.settings,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.settings = value;
        },
        getTainted: (tainted: Tainted) => tainted?.settings ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.settings = value;
        },
        validate: (_value: Settings): Array<string> => [],
      },
      role: {
        path: ["role"] as const,
        name: "role",
        constraints: { required: true },

        get: (data: Data) => data.role,
        set: (data: Data, value: UserRole) => {
          data.role = value;
        },
        getError: (errors: Errors) => errors?.role,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.role = value;
        },
        getTainted: (tainted: Tainted) => tainted?.role ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.role = value;
        },
        validate: (_value: UserRole): Array<string> => [],
      },
      emailVerified: {
        path: ["emailVerified"] as const,
        name: "emailVerified",
        constraints: { required: true },

        get: (data: Data) => data.emailVerified,
        set: (data: Data, value: boolean) => {
          data.emailVerified = value;
        },
        getError: (errors: Errors) => errors?.emailVerified,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.emailVerified = value;
        },
        getTainted: (tainted: Tainted) => tainted?.emailVerified ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.emailVerified = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      verificationToken: {
        path: ["verificationToken"] as const,
        name: "verificationToken",
        constraints: { required: true },

        get: (data: Data) => data.verificationToken,
        set: (data: Data, value: string | null) => {
          data.verificationToken = value;
        },
        getError: (errors: Errors) => errors?.verificationToken,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.verificationToken = value;
        },
        getTainted: (tainted: Tainted) => tainted?.verificationToken ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.verificationToken = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      verificationExpires: {
        path: ["verificationExpires"] as const,
        name: "verificationExpires",
        constraints: { required: true },

        get: (data: Data) => data.verificationExpires,
        set: (data: Data, value: string | null) => {
          data.verificationExpires = value;
        },
        getError: (errors: Errors) => errors?.verificationExpires,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.verificationExpires = value;
        },
        getTainted: (tainted: Tainted) => tainted?.verificationExpires ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.verificationExpires = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      passwordResetToken: {
        path: ["passwordResetToken"] as const,
        name: "passwordResetToken",
        constraints: { required: true },

        get: (data: Data) => data.passwordResetToken,
        set: (data: Data, value: string | null) => {
          data.passwordResetToken = value;
        },
        getError: (errors: Errors) => errors?.passwordResetToken,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.passwordResetToken = value;
        },
        getTainted: (tainted: Tainted) => tainted?.passwordResetToken ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.passwordResetToken = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      passwordResetExpires: {
        path: ["passwordResetExpires"] as const,
        name: "passwordResetExpires",
        constraints: { required: true },

        get: (data: Data) => data.passwordResetExpires,
        set: (data: Data, value: string | null) => {
          data.passwordResetExpires = value;
        },
        getError: (errors: Errors) => errors?.passwordResetExpires,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.passwordResetExpires = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.passwordResetExpires ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.passwordResetExpires = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      permissions: {
        path: ["permissions"] as const,
        name: "permissions",
        constraints: { required: true },

        get: (data: Data) => data.permissions,
        set: (data: Data, value: AppPermissions) => {
          data.permissions = value;
        },
        getError: (errors: Errors) => errors?.permissions,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.permissions = value;
        },
        getTainted: (tainted: Tainted) => tainted?.permissions ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.permissions = value;
        },
        validate: (_value: AppPermissions): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Service): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Service, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Service.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Service;
  }
}

export namespace Service {
  export type Data = Service;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    name?: Array<string>;
    quickCode?: Array<string>;
    group?: Array<string>;
    subgroup?: Array<string>;
    unit?: Array<string>;
    active?: Array<string>;
    commission?: Array<string>;
    favorite?: Array<string>;
    averageTime?: Array<string>;
    defaults?: ServiceDefaults.Errors;
  };
  export type Tainted = {
    id?: boolean;
    name?: boolean;
    quickCode?: boolean;
    group?: boolean;
    subgroup?: boolean;
    unit?: boolean;
    active?: boolean;
    commission?: boolean;
    favorite?: boolean;
    averageTime?: boolean;
    defaults?: ServiceDefaults.Tainted;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.name = formData.get("name") ?? "";
    obj.quickCode = formData.get("quickCode") ?? "";
    obj.group = formData.get("group") ?? "";
    obj.subgroup = formData.get("subgroup") ?? "";
    obj.unit = formData.get("unit") ?? "";
    {
      const activeVal = formData.get("active");
      obj.active =
        activeVal === "true" || activeVal === "on" || activeVal === "1";
    }
    {
      const commissionVal = formData.get("commission");
      obj.commission =
        commissionVal === "true" ||
        commissionVal === "on" ||
        commissionVal === "1";
    }
    {
      const favoriteVal = formData.get("favorite");
      obj.favorite =
        favoriteVal === "true" || favoriteVal === "on" || favoriteVal === "1";
    }
    obj.averageTime = formData.get("averageTime") ?? "";
    {
      // Collect nested object fields with prefix "defaults."
      const defaultsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("defaults.")) {
          const fieldName = key.slice("defaults.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = defaultsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.defaults = defaultsObj;
    }
    return Service.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: (data: Data) => data.name,
        set: (data: Data, value: string) => {
          data.name = value;
        },
        getError: (errors: Errors) => errors?.name,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: (tainted: Tainted) => tainted?.name ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.name = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      quickCode: {
        path: ["quickCode"] as const,
        name: "quickCode",
        constraints: { required: true },

        get: (data: Data) => data.quickCode,
        set: (data: Data, value: string) => {
          data.quickCode = value;
        },
        getError: (errors: Errors) => errors?.quickCode,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.quickCode = value;
        },
        getTainted: (tainted: Tainted) => tainted?.quickCode ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.quickCode = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      group: {
        path: ["group"] as const,
        name: "group",
        constraints: { required: true },

        get: (data: Data) => data.group,
        set: (data: Data, value: string | null) => {
          data.group = value;
        },
        getError: (errors: Errors) => errors?.group,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.group = value;
        },
        getTainted: (tainted: Tainted) => tainted?.group ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.group = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      subgroup: {
        path: ["subgroup"] as const,
        name: "subgroup",
        constraints: { required: true },

        get: (data: Data) => data.subgroup,
        set: (data: Data, value: string | null) => {
          data.subgroup = value;
        },
        getError: (errors: Errors) => errors?.subgroup,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.subgroup = value;
        },
        getTainted: (tainted: Tainted) => tainted?.subgroup ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.subgroup = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      unit: {
        path: ["unit"] as const,
        name: "unit",
        constraints: { required: true },

        get: (data: Data) => data.unit,
        set: (data: Data, value: string | null) => {
          data.unit = value;
        },
        getError: (errors: Errors) => errors?.unit,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.unit = value;
        },
        getTainted: (tainted: Tainted) => tainted?.unit ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.unit = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      active: {
        path: ["active"] as const,
        name: "active",
        constraints: { required: true },

        get: (data: Data) => data.active,
        set: (data: Data, value: boolean) => {
          data.active = value;
        },
        getError: (errors: Errors) => errors?.active,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.active = value;
        },
        getTainted: (tainted: Tainted) => tainted?.active ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.active = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      commission: {
        path: ["commission"] as const,
        name: "commission",
        constraints: { required: true },

        get: (data: Data) => data.commission,
        set: (data: Data, value: boolean) => {
          data.commission = value;
        },
        getError: (errors: Errors) => errors?.commission,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.commission = value;
        },
        getTainted: (tainted: Tainted) => tainted?.commission ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.commission = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      favorite: {
        path: ["favorite"] as const,
        name: "favorite",
        constraints: { required: true },

        get: (data: Data) => data.favorite,
        set: (data: Data, value: boolean) => {
          data.favorite = value;
        },
        getError: (errors: Errors) => errors?.favorite,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.favorite = value;
        },
        getTainted: (tainted: Tainted) => tainted?.favorite ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.favorite = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      averageTime: {
        path: ["averageTime"] as const,
        name: "averageTime",
        constraints: { required: true },

        get: (data: Data) => data.averageTime,
        set: (data: Data, value: string | null) => {
          data.averageTime = value;
        },
        getError: (errors: Errors) => errors?.averageTime,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.averageTime = value;
        },
        getTainted: (tainted: Tainted) => tainted?.averageTime ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.averageTime = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      defaults: {
        path: ["defaults"] as const,
        name: "defaults",
        constraints: { required: true },

        get: (data: Data) => data.defaults,
        set: (data: Data, value: ServiceDefaults) => {
          data.defaults = value;
        },
        getError: (errors: Errors) => errors?.defaults,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.defaults = value;
        },
        getTainted: (tainted: Tainted) => tainted?.defaults ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.defaults = value;
        },
        validate: (_value: ServiceDefaults): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: ServiceDefaults): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<ServiceDefaults, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "ServiceDefaults.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as ServiceDefaults;
  }
}

export namespace ServiceDefaults {
  export type Data = ServiceDefaults;
  export type Errors = {
    _errors?: Array<string>;
    price?: Array<string>;
    description?: Array<string>;
  };
  export type Tainted = { price?: boolean; description?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const priceStr = formData.get("price");
      obj.price = priceStr ? parseFloat(priceStr as string) : 0;
      if (obj.price !== undefined && Number.isNaN(obj.price)) obj.price = 0;
    }
    obj.description = formData.get("description") ?? "";
    return ServiceDefaults.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      price: {
        path: ["price"] as const,
        name: "price",
        constraints: { required: true },

        get: (data: Data) => data.price,
        set: (data: Data, value: number) => {
          data.price = value;
        },
        getError: (errors: Errors) => errors?.price,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.price = value;
        },
        getTainted: (tainted: Tainted) => tainted?.price ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.price = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      description: {
        path: ["description"] as const,
        name: "description",
        constraints: { required: true },

        get: (data: Data) => data.description,
        set: (data: Data, value: string) => {
          data.description = value;
        },
        getError: (errors: Errors) => errors?.description,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.description = value;
        },
        getTainted: (tainted: Tainted) => tainted?.description ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.description = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: Did): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Did, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Did.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Did;
  }
}

export namespace Did {
  export type Data = Did;
  export type Errors = {
    _errors?: Array<string>;
    in?: Array<string>;
    out?: Array<string>;
    id?: Array<string>;
    activityType?: ActivityType.Errors;
    createdAt?: Array<string>;
    metadata?: Array<string>;
  };
  export type Tainted = {
    in?: boolean;
    out?: boolean;
    id?: boolean;
    activityType?: ActivityType.Tainted;
    createdAt?: boolean;
    metadata?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.in = formData.get("in") ?? "";
    obj.out = formData.get("out") ?? "";
    obj.id = formData.get("id") ?? "";
    {
      // Collect nested object fields with prefix "activityType."
      const activityTypeObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("activityType.")) {
          const fieldName = key.slice("activityType.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = activityTypeObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.activityType = activityTypeObj;
    }
    obj.createdAt = formData.get("createdAt") ?? "";
    obj.metadata = formData.get("metadata") ?? "";
    return Did.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      in: {
        path: ["in"] as const,
        name: "in",
        constraints: { required: true },

        get: (data: Data) => data.in,
        set: (data: Data, value: string | Actor) => {
          data.in = value;
        },
        getError: (errors: Errors) => errors?.in,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.in = value;
        },
        getTainted: (tainted: Tainted) => tainted?.in ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.in = value;
        },
        validate: (_value: string | Actor): Array<string> => [],
      },
      out: {
        path: ["out"] as const,
        name: "out",
        constraints: { required: true },

        get: (data: Data) => data.out,
        set: (data: Data, value: string | Target) => {
          data.out = value;
        },
        getError: (errors: Errors) => errors?.out,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.out = value;
        },
        getTainted: (tainted: Tainted) => tainted?.out ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.out = value;
        },
        validate: (_value: string | Target): Array<string> => [],
      },
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      activityType: {
        path: ["activityType"] as const,
        name: "activityType",
        constraints: { required: true },

        get: (data: Data) => data.activityType,
        set: (data: Data, value: ActivityType) => {
          data.activityType = value;
        },
        getError: (errors: Errors) => errors?.activityType,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.activityType = value;
        },
        getTainted: (tainted: Tainted) => tainted?.activityType ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.activityType = value;
        },
        validate: (_value: ActivityType): Array<string> => [],
      },
      createdAt: {
        path: ["createdAt"] as const,
        name: "createdAt",
        constraints: { required: true },

        get: (data: Data) => data.createdAt,
        set: (data: Data, value: string) => {
          data.createdAt = value;
        },
        getError: (errors: Errors) => errors?.createdAt,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.createdAt = value;
        },
        getTainted: (tainted: Tainted) => tainted?.createdAt ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.createdAt = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      metadata: {
        path: ["metadata"] as const,
        name: "metadata",
        constraints: { required: true },

        get: (data: Data) => data.metadata,
        set: (data: Data, value: string | null) => {
          data.metadata = value;
        },
        getError: (errors: Errors) => errors?.metadata,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.metadata = value;
        },
        getTainted: (tainted: Tainted) => tainted?.metadata ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.metadata = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: PersonName): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<PersonName, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "PersonName.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as PersonName;
  }
}

export namespace PersonName {
  export type Data = PersonName;
  export type Errors = {
    _errors?: Array<string>;
    firstName?: Array<string>;
    lastName?: Array<string>;
  };
  export type Tainted = { firstName?: boolean; lastName?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.firstName = formData.get("firstName") ?? "";
    obj.lastName = formData.get("lastName") ?? "";
    return PersonName.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      firstName: {
        path: ["firstName"] as const,
        name: "firstName",
        constraints: { required: true },

        get: (data: Data) => data.firstName,
        set: (data: Data, value: string) => {
          data.firstName = value;
        },
        getError: (errors: Errors) => errors?.firstName,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.firstName = value;
        },
        getTainted: (tainted: Tainted) => tainted?.firstName ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.firstName = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      lastName: {
        path: ["lastName"] as const,
        name: "lastName",
        constraints: { required: true },

        get: (data: Data) => data.lastName,
        set: (data: Data, value: string) => {
          data.lastName = value;
        },
        getError: (errors: Errors) => errors?.lastName,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.lastName = value;
        },
        getTainted: (tainted: Tainted) => tainted?.lastName ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.lastName = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: Promotion): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Promotion, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Promotion.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Promotion;
  }
}

export namespace Promotion {
  export type Data = Promotion;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    date?: Array<string>;
  };
  export type Tainted = { id?: boolean; date?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.date = formData.get("date") ?? "";
    return Promotion.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      date: {
        path: ["date"] as const,
        name: "date",
        constraints: { required: true },

        get: (data: Data) => data.date,
        set: (data: Data, value: string) => {
          data.date = value;
        },
        getError: (errors: Errors) => errors?.date,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.date = value;
        },
        getTainted: (tainted: Tainted) => tainted?.date ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.date = value;
        },
        validate: (_value: string): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Site): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Site, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Site.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Site;
  }
}

export namespace Site {
  export type Data = Site;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    addressLine1?: Array<string>;
    addressLine2?: Array<string>;
    sublocalityLevel1?: Array<string>;
    locality?: Array<string>;
    administrativeAreaLevel3?: Array<string>;
    administrativeAreaLevel2?: Array<string>;
    administrativeAreaLevel1?: Array<string>;
    country?: Array<string>;
    postalCode?: Array<string>;
    postalCodeSuffix?: Array<string>;
    coordinates?: Coordinates.Errors;
  };
  export type Tainted = {
    id?: boolean;
    addressLine1?: boolean;
    addressLine2?: boolean;
    sublocalityLevel1?: boolean;
    locality?: boolean;
    administrativeAreaLevel3?: boolean;
    administrativeAreaLevel2?: boolean;
    administrativeAreaLevel1?: boolean;
    country?: boolean;
    postalCode?: boolean;
    postalCodeSuffix?: boolean;
    coordinates?: Coordinates.Tainted;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.addressLine1 = formData.get("addressLine1") ?? "";
    obj.addressLine2 = formData.get("addressLine2") ?? "";
    obj.sublocalityLevel1 = formData.get("sublocalityLevel1") ?? "";
    obj.locality = formData.get("locality") ?? "";
    obj.administrativeAreaLevel3 =
      formData.get("administrativeAreaLevel3") ?? "";
    obj.administrativeAreaLevel2 =
      formData.get("administrativeAreaLevel2") ?? "";
    obj.administrativeAreaLevel1 =
      formData.get("administrativeAreaLevel1") ?? "";
    obj.country = formData.get("country") ?? "";
    obj.postalCode = formData.get("postalCode") ?? "";
    obj.postalCodeSuffix = formData.get("postalCodeSuffix") ?? "";
    {
      // Collect nested object fields with prefix "coordinates."
      const coordinatesObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("coordinates.")) {
          const fieldName = key.slice("coordinates.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = coordinatesObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.coordinates = coordinatesObj;
    }
    return Site.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      addressLine1: {
        path: ["addressLine1"] as const,
        name: "addressLine1",
        constraints: { required: true },

        get: (data: Data) => data.addressLine1,
        set: (data: Data, value: string) => {
          data.addressLine1 = value;
        },
        getError: (errors: Errors) => errors?.addressLine1,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.addressLine1 = value;
        },
        getTainted: (tainted: Tainted) => tainted?.addressLine1 ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.addressLine1 = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      addressLine2: {
        path: ["addressLine2"] as const,
        name: "addressLine2",
        constraints: { required: true },

        get: (data: Data) => data.addressLine2,
        set: (data: Data, value: string | null) => {
          data.addressLine2 = value;
        },
        getError: (errors: Errors) => errors?.addressLine2,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.addressLine2 = value;
        },
        getTainted: (tainted: Tainted) => tainted?.addressLine2 ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.addressLine2 = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      sublocalityLevel1: {
        path: ["sublocalityLevel1"] as const,
        name: "sublocalityLevel1",
        constraints: { required: true },

        get: (data: Data) => data.sublocalityLevel1,
        set: (data: Data, value: string | null) => {
          data.sublocalityLevel1 = value;
        },
        getError: (errors: Errors) => errors?.sublocalityLevel1,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.sublocalityLevel1 = value;
        },
        getTainted: (tainted: Tainted) => tainted?.sublocalityLevel1 ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.sublocalityLevel1 = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      locality: {
        path: ["locality"] as const,
        name: "locality",
        constraints: { required: true },

        get: (data: Data) => data.locality,
        set: (data: Data, value: string) => {
          data.locality = value;
        },
        getError: (errors: Errors) => errors?.locality,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.locality = value;
        },
        getTainted: (tainted: Tainted) => tainted?.locality ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.locality = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      administrativeAreaLevel3: {
        path: ["administrativeAreaLevel3"] as const,
        name: "administrativeAreaLevel3",
        constraints: { required: true },

        get: (data: Data) => data.administrativeAreaLevel3,
        set: (data: Data, value: string | null) => {
          data.administrativeAreaLevel3 = value;
        },
        getError: (errors: Errors) => errors?.administrativeAreaLevel3,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.administrativeAreaLevel3 = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.administrativeAreaLevel3 ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.administrativeAreaLevel3 = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      administrativeAreaLevel2: {
        path: ["administrativeAreaLevel2"] as const,
        name: "administrativeAreaLevel2",
        constraints: { required: true },

        get: (data: Data) => data.administrativeAreaLevel2,
        set: (data: Data, value: string | null) => {
          data.administrativeAreaLevel2 = value;
        },
        getError: (errors: Errors) => errors?.administrativeAreaLevel2,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.administrativeAreaLevel2 = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.administrativeAreaLevel2 ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.administrativeAreaLevel2 = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      administrativeAreaLevel1: {
        path: ["administrativeAreaLevel1"] as const,
        name: "administrativeAreaLevel1",
        constraints: { required: true },

        get: (data: Data) => data.administrativeAreaLevel1,
        set: (data: Data, value: string) => {
          data.administrativeAreaLevel1 = value;
        },
        getError: (errors: Errors) => errors?.administrativeAreaLevel1,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.administrativeAreaLevel1 = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.administrativeAreaLevel1 ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.administrativeAreaLevel1 = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      country: {
        path: ["country"] as const,
        name: "country",
        constraints: { required: true },

        get: (data: Data) => data.country,
        set: (data: Data, value: string) => {
          data.country = value;
        },
        getError: (errors: Errors) => errors?.country,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.country = value;
        },
        getTainted: (tainted: Tainted) => tainted?.country ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.country = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      postalCode: {
        path: ["postalCode"] as const,
        name: "postalCode",
        constraints: { required: true },

        get: (data: Data) => data.postalCode,
        set: (data: Data, value: string) => {
          data.postalCode = value;
        },
        getError: (errors: Errors) => errors?.postalCode,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.postalCode = value;
        },
        getTainted: (tainted: Tainted) => tainted?.postalCode ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.postalCode = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      postalCodeSuffix: {
        path: ["postalCodeSuffix"] as const,
        name: "postalCodeSuffix",
        constraints: { required: true },

        get: (data: Data) => data.postalCodeSuffix,
        set: (data: Data, value: string | null) => {
          data.postalCodeSuffix = value;
        },
        getError: (errors: Errors) => errors?.postalCodeSuffix,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.postalCodeSuffix = value;
        },
        getTainted: (tainted: Tainted) => tainted?.postalCodeSuffix ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.postalCodeSuffix = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      coordinates: {
        path: ["coordinates"] as const,
        name: "coordinates",
        constraints: { required: true },

        get: (data: Data) => data.coordinates,
        set: (data: Data, value: Coordinates) => {
          data.coordinates = value;
        },
        getError: (errors: Errors) => errors?.coordinates,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.coordinates = value;
        },
        getTainted: (tainted: Tainted) => tainted?.coordinates ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.coordinates = value;
        },
        validate: (_value: Coordinates): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Metadata): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Metadata, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Metadata.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Metadata;
  }
}

export namespace Metadata {
  export type Data = Metadata;
  export type Errors = {
    _errors?: Array<string>;
    createdAt?: Array<string>;
    lastLogin?: Array<string>;
    isActive?: Array<string>;
    roles?: { _errors?: Array<string>; [index: number]: Array<string> };
  };
  export type Tainted = {
    createdAt?: boolean;
    lastLogin?: boolean;
    isActive?: boolean;
    roles?: { [index: number]: boolean };
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.createdAt = formData.get("createdAt") ?? "";
    obj.lastLogin = formData.get("lastLogin") ?? "";
    {
      const isActiveVal = formData.get("isActive");
      obj.isActive =
        isActiveVal === "true" || isActiveVal === "on" || isActiveVal === "1";
    }
    obj.roles = formData.getAll("roles") as Array<string>;
    return Metadata.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      createdAt: {
        path: ["createdAt"] as const,
        name: "createdAt",
        constraints: { required: true },

        get: (data: Data) => data.createdAt,
        set: (data: Data, value: string) => {
          data.createdAt = value;
        },
        getError: (errors: Errors) => errors?.createdAt,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.createdAt = value;
        },
        getTainted: (tainted: Tainted) => tainted?.createdAt ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.createdAt = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      lastLogin: {
        path: ["lastLogin"] as const,
        name: "lastLogin",
        constraints: { required: true },

        get: (data: Data) => data.lastLogin,
        set: (data: Data, value: string | null) => {
          data.lastLogin = value;
        },
        getError: (errors: Errors) => errors?.lastLogin,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.lastLogin = value;
        },
        getTainted: (tainted: Tainted) => tainted?.lastLogin ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.lastLogin = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      isActive: {
        path: ["isActive"] as const,
        name: "isActive",
        constraints: { required: true },

        get: (data: Data) => data.isActive,
        set: (data: Data, value: boolean) => {
          data.isActive = value;
        },
        getError: (errors: Errors) => errors?.isActive,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.isActive = value;
        },
        getTainted: (tainted: Tainted) => tainted?.isActive ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.isActive = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      roles: {
        path: ["roles"] as const,
        name: "roles",
        constraints: { required: true },

        get: (data: Data) => data.roles,
        set: (data: Data, value: string[]) => {
          data.roles = value;
        },
        getError: (errors: Errors) => errors?.roles,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.roles = value;
        },
        getTainted: (tainted: Tainted) => tainted?.roles ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.roles = value;
        },
        validate: (_value: string[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["roles"], index] as const,
          name: `roles.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.roles[index],
          set: (data: Data, value: string) => {
            data.roles[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.roles as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.roles ??= {};
            (errors.roles as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.roles?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.roles ??= {};
            tainted.roles[index] = value;
          },
          validate: (_value: string): Array<string> => [],
        }),
        push: (data: Data, item: string) => {
          data.roles.push(item);
        },
        remove: (data: Data, index: number) => {
          data.roles.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.roles[a], data.roles[b]] = [data.roles[b], data.roles[a]];
        },
      },
    } as const;
}

/**  */
export interface ColumnConfig {
  heading: string;
  dataPath: DataPath;
}

export namespace ColumnConfig {
  export function toStringifiedJSON(self: ColumnConfig): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<ColumnConfig, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "ColumnConfig.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as ColumnConfig;
  }
}

export namespace ColumnConfig {
  export type Data = ColumnConfig;
  export type Errors = {
    _errors?: Array<string>;
    heading?: Array<string>;
    dataPath?: DataPath.Errors;
  };
  export type Tainted = { heading?: boolean; dataPath?: DataPath.Tainted };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.heading = formData.get("heading") ?? "";
    {
      // Collect nested object fields with prefix "dataPath."
      const dataPathObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("dataPath.")) {
          const fieldName = key.slice("dataPath.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = dataPathObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.dataPath = dataPathObj;
    }
    return ColumnConfig.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      heading: {
        path: ["heading"] as const,
        name: "heading",
        constraints: { required: true },

        get: (data: Data) => data.heading,
        set: (data: Data, value: string) => {
          data.heading = value;
        },
        getError: (errors: Errors) => errors?.heading,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.heading = value;
        },
        getTainted: (tainted: Tainted) => tainted?.heading ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.heading = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      dataPath: {
        path: ["dataPath"] as const,
        name: "dataPath",
        constraints: { required: true },

        get: (data: Data) => data.dataPath,
        set: (data: Data, value: DataPath) => {
          data.dataPath = value;
        },
        getError: (errors: Errors) => errors?.dataPath,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.dataPath = value;
        },
        getTainted: (tainted: Tainted) => tainted?.dataPath ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.dataPath = value;
        },
        validate: (_value: DataPath): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: PhoneNumber): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<PhoneNumber, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "PhoneNumber.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as PhoneNumber;
  }
}

export namespace PhoneNumber {
  export type Data = PhoneNumber;
  export type Errors = {
    _errors?: Array<string>;
    main?: Array<string>;
    phoneType?: Array<string>;
    number?: Array<string>;
    canText?: Array<string>;
    canCall?: Array<string>;
  };
  export type Tainted = {
    main?: boolean;
    phoneType?: boolean;
    number?: boolean;
    canText?: boolean;
    canCall?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const mainVal = formData.get("main");
      obj.main = mainVal === "true" || mainVal === "on" || mainVal === "1";
    }
    obj.phoneType = formData.get("phoneType") ?? "";
    obj.number = formData.get("number") ?? "";
    {
      const canTextVal = formData.get("canText");
      obj.canText =
        canTextVal === "true" || canTextVal === "on" || canTextVal === "1";
    }
    {
      const canCallVal = formData.get("canCall");
      obj.canCall =
        canCallVal === "true" || canCallVal === "on" || canCallVal === "1";
    }
    return PhoneNumber.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      main: {
        path: ["main"] as const,
        name: "main",
        constraints: { required: true },

        get: (data: Data) => data.main,
        set: (data: Data, value: boolean) => {
          data.main = value;
        },
        getError: (errors: Errors) => errors?.main,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.main = value;
        },
        getTainted: (tainted: Tainted) => tainted?.main ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.main = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      phoneType: {
        path: ["phoneType"] as const,
        name: "phoneType",
        constraints: { required: true },

        get: (data: Data) => data.phoneType,
        set: (data: Data, value: string) => {
          data.phoneType = value;
        },
        getError: (errors: Errors) => errors?.phoneType,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.phoneType = value;
        },
        getTainted: (tainted: Tainted) => tainted?.phoneType ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.phoneType = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      number: {
        path: ["number"] as const,
        name: "number",
        constraints: { required: true },

        get: (data: Data) => data.number,
        set: (data: Data, value: string) => {
          data.number = value;
        },
        getError: (errors: Errors) => errors?.number,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.number = value;
        },
        getTainted: (tainted: Tainted) => tainted?.number ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.number = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      canText: {
        path: ["canText"] as const,
        name: "canText",
        constraints: { required: true },

        get: (data: Data) => data.canText,
        set: (data: Data, value: boolean) => {
          data.canText = value;
        },
        getError: (errors: Errors) => errors?.canText,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.canText = value;
        },
        getTainted: (tainted: Tainted) => tainted?.canText ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.canText = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      canCall: {
        path: ["canCall"] as const,
        name: "canCall",
        constraints: { required: true },

        get: (data: Data) => data.canCall,
        set: (data: Data, value: boolean) => {
          data.canCall = value;
        },
        getError: (errors: Errors) => errors?.canCall,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.canCall = value;
        },
        getTainted: (tainted: Tainted) => tainted?.canCall ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.canCall = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Gradient): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Gradient, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Gradient.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Gradient;
  }
}

export namespace Gradient {
  export type Data = Gradient;
  export type Errors = { _errors?: Array<string>; startHue?: Array<string> };
  export type Tainted = { startHue?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const startHueStr = formData.get("startHue");
      obj.startHue = startHueStr ? parseFloat(startHueStr as string) : 0;
      if (obj.startHue !== undefined && Number.isNaN(obj.startHue))
        obj.startHue = 0;
    }
    return Gradient.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      startHue: {
        path: ["startHue"] as const,
        name: "startHue",
        constraints: { required: true },

        get: (data: Data) => data.startHue,
        set: (data: Data, value: number) => {
          data.startHue = value;
        },
        getError: (errors: Errors) => errors?.startHue,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.startHue = value;
        },
        getTainted: (tainted: Tainted) => tainted?.startHue ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.startHue = value;
        },
        validate: (_value: number): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Product): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Product, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Product.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Product;
  }
}

export namespace Product {
  export type Data = Product;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    name?: Array<string>;
    quickCode?: Array<string>;
    group?: Array<string>;
    subgroup?: Array<string>;
    unit?: Array<string>;
    active?: Array<string>;
    commission?: Array<string>;
    favorite?: Array<string>;
    defaults?: ProductDefaults.Errors;
  };
  export type Tainted = {
    id?: boolean;
    name?: boolean;
    quickCode?: boolean;
    group?: boolean;
    subgroup?: boolean;
    unit?: boolean;
    active?: boolean;
    commission?: boolean;
    favorite?: boolean;
    defaults?: ProductDefaults.Tainted;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.name = formData.get("name") ?? "";
    obj.quickCode = formData.get("quickCode") ?? "";
    obj.group = formData.get("group") ?? "";
    obj.subgroup = formData.get("subgroup") ?? "";
    obj.unit = formData.get("unit") ?? "";
    {
      const activeVal = formData.get("active");
      obj.active =
        activeVal === "true" || activeVal === "on" || activeVal === "1";
    }
    {
      const commissionVal = formData.get("commission");
      obj.commission =
        commissionVal === "true" ||
        commissionVal === "on" ||
        commissionVal === "1";
    }
    {
      const favoriteVal = formData.get("favorite");
      obj.favorite =
        favoriteVal === "true" || favoriteVal === "on" || favoriteVal === "1";
    }
    {
      // Collect nested object fields with prefix "defaults."
      const defaultsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("defaults.")) {
          const fieldName = key.slice("defaults.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = defaultsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.defaults = defaultsObj;
    }
    return Product.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: (data: Data) => data.name,
        set: (data: Data, value: string) => {
          data.name = value;
        },
        getError: (errors: Errors) => errors?.name,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: (tainted: Tainted) => tainted?.name ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.name = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      quickCode: {
        path: ["quickCode"] as const,
        name: "quickCode",
        constraints: { required: true },

        get: (data: Data) => data.quickCode,
        set: (data: Data, value: string) => {
          data.quickCode = value;
        },
        getError: (errors: Errors) => errors?.quickCode,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.quickCode = value;
        },
        getTainted: (tainted: Tainted) => tainted?.quickCode ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.quickCode = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      group: {
        path: ["group"] as const,
        name: "group",
        constraints: { required: true },

        get: (data: Data) => data.group,
        set: (data: Data, value: string | null) => {
          data.group = value;
        },
        getError: (errors: Errors) => errors?.group,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.group = value;
        },
        getTainted: (tainted: Tainted) => tainted?.group ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.group = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      subgroup: {
        path: ["subgroup"] as const,
        name: "subgroup",
        constraints: { required: true },

        get: (data: Data) => data.subgroup,
        set: (data: Data, value: string | null) => {
          data.subgroup = value;
        },
        getError: (errors: Errors) => errors?.subgroup,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.subgroup = value;
        },
        getTainted: (tainted: Tainted) => tainted?.subgroup ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.subgroup = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      unit: {
        path: ["unit"] as const,
        name: "unit",
        constraints: { required: true },

        get: (data: Data) => data.unit,
        set: (data: Data, value: string | null) => {
          data.unit = value;
        },
        getError: (errors: Errors) => errors?.unit,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.unit = value;
        },
        getTainted: (tainted: Tainted) => tainted?.unit ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.unit = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      active: {
        path: ["active"] as const,
        name: "active",
        constraints: { required: true },

        get: (data: Data) => data.active,
        set: (data: Data, value: boolean) => {
          data.active = value;
        },
        getError: (errors: Errors) => errors?.active,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.active = value;
        },
        getTainted: (tainted: Tainted) => tainted?.active ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.active = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      commission: {
        path: ["commission"] as const,
        name: "commission",
        constraints: { required: true },

        get: (data: Data) => data.commission,
        set: (data: Data, value: boolean) => {
          data.commission = value;
        },
        getError: (errors: Errors) => errors?.commission,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.commission = value;
        },
        getTainted: (tainted: Tainted) => tainted?.commission ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.commission = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      favorite: {
        path: ["favorite"] as const,
        name: "favorite",
        constraints: { required: true },

        get: (data: Data) => data.favorite,
        set: (data: Data, value: boolean) => {
          data.favorite = value;
        },
        getError: (errors: Errors) => errors?.favorite,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.favorite = value;
        },
        getTainted: (tainted: Tainted) => tainted?.favorite ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.favorite = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      defaults: {
        path: ["defaults"] as const,
        name: "defaults",
        constraints: { required: true },

        get: (data: Data) => data.defaults,
        set: (data: Data, value: ProductDefaults) => {
          data.defaults = value;
        },
        getError: (errors: Errors) => errors?.defaults,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.defaults = value;
        },
        getTainted: (tainted: Tainted) => tainted?.defaults ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.defaults = value;
        },
        validate: (_value: ProductDefaults): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: YearlyRecurrenceRule): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<YearlyRecurrenceRule, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "YearlyRecurrenceRule.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as YearlyRecurrenceRule;
  }
}

export namespace YearlyRecurrenceRule {
  export type Data = YearlyRecurrenceRule;
  export type Errors = {
    _errors?: Array<string>;
    quantityOfYears?: Array<string>;
  };
  export type Tainted = { quantityOfYears?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const quantityOfYearsStr = formData.get("quantityOfYears");
      obj.quantityOfYears = quantityOfYearsStr
        ? parseFloat(quantityOfYearsStr as string)
        : 0;
      if (
        obj.quantityOfYears !== undefined &&
        Number.isNaN(obj.quantityOfYears)
      )
        obj.quantityOfYears = 0;
    }
    return YearlyRecurrenceRule.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      quantityOfYears: {
        path: ["quantityOfYears"] as const,
        name: "quantityOfYears",
        constraints: { required: true },

        get: (data: Data) => data.quantityOfYears,
        set: (data: Data, value: number) => {
          data.quantityOfYears = value;
        },
        getError: (errors: Errors) => errors?.quantityOfYears,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.quantityOfYears = value;
        },
        getTainted: (tainted: Tainted) => tainted?.quantityOfYears ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.quantityOfYears = value;
        },
        validate: (_value: number): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: AppointmentNotifications): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<AppointmentNotifications, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "AppointmentNotifications.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as AppointmentNotifications;
  }
}

export namespace AppointmentNotifications {
  export type Data = AppointmentNotifications;
  export type Errors = {
    _errors?: Array<string>;
    personalScheduleChangeNotifications?: Array<string>;
    allScheduleChangeNotifications?: Array<string>;
  };
  export type Tainted = {
    personalScheduleChangeNotifications?: boolean;
    allScheduleChangeNotifications?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.personalScheduleChangeNotifications =
      formData.get("personalScheduleChangeNotifications") ?? "";
    obj.allScheduleChangeNotifications =
      formData.get("allScheduleChangeNotifications") ?? "";
    return AppointmentNotifications.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      personalScheduleChangeNotifications: {
        path: ["personalScheduleChangeNotifications"] as const,
        name: "personalScheduleChangeNotifications",
        constraints: { required: true },

        get: (data: Data) => data.personalScheduleChangeNotifications,
        set: (data: Data, value: string) => {
          data.personalScheduleChangeNotifications = value;
        },
        getError: (errors: Errors) =>
          errors?.personalScheduleChangeNotifications,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.personalScheduleChangeNotifications = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.personalScheduleChangeNotifications ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.personalScheduleChangeNotifications = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      allScheduleChangeNotifications: {
        path: ["allScheduleChangeNotifications"] as const,
        name: "allScheduleChangeNotifications",
        constraints: { required: true },

        get: (data: Data) => data.allScheduleChangeNotifications,
        set: (data: Data, value: string) => {
          data.allScheduleChangeNotifications = value;
        },
        getError: (errors: Errors) => errors?.allScheduleChangeNotifications,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.allScheduleChangeNotifications = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.allScheduleChangeNotifications ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.allScheduleChangeNotifications = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: DirectionHue): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<DirectionHue, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "DirectionHue.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as DirectionHue;
  }
}

export namespace DirectionHue {
  export type Data = DirectionHue;
  export type Errors = {
    _errors?: Array<string>;
    bearing?: Array<string>;
    hue?: Array<string>;
  };
  export type Tainted = { bearing?: boolean; hue?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const bearingStr = formData.get("bearing");
      obj.bearing = bearingStr ? parseFloat(bearingStr as string) : 0;
      if (obj.bearing !== undefined && Number.isNaN(obj.bearing))
        obj.bearing = 0;
    }
    {
      const hueStr = formData.get("hue");
      obj.hue = hueStr ? parseFloat(hueStr as string) : 0;
      if (obj.hue !== undefined && Number.isNaN(obj.hue)) obj.hue = 0;
    }
    return DirectionHue.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      bearing: {
        path: ["bearing"] as const,
        name: "bearing",
        constraints: { required: true },

        get: (data: Data) => data.bearing,
        set: (data: Data, value: number) => {
          data.bearing = value;
        },
        getError: (errors: Errors) => errors?.bearing,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.bearing = value;
        },
        getTainted: (tainted: Tainted) => tainted?.bearing ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.bearing = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      hue: {
        path: ["hue"] as const,
        name: "hue",
        constraints: { required: true },

        get: (data: Data) => data.hue,
        set: (data: Data, value: number) => {
          data.hue = value;
        },
        getError: (errors: Errors) => errors?.hue,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.hue = value;
        },
        getTainted: (tainted: Tainted) => tainted?.hue ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.hue = value;
        },
        validate: (_value: number): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: MonthlyRecurrenceRule): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<MonthlyRecurrenceRule, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "MonthlyRecurrenceRule.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as MonthlyRecurrenceRule;
  }
}

export namespace MonthlyRecurrenceRule {
  export type Data = MonthlyRecurrenceRule;
  export type Errors = {
    _errors?: Array<string>;
    quantityOfMonths?: Array<string>;
    day?: Array<string>;
    name?: Array<string>;
  };
  export type Tainted = {
    quantityOfMonths?: boolean;
    day?: boolean;
    name?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const quantityOfMonthsStr = formData.get("quantityOfMonths");
      obj.quantityOfMonths = quantityOfMonthsStr
        ? parseFloat(quantityOfMonthsStr as string)
        : 0;
      if (
        obj.quantityOfMonths !== undefined &&
        Number.isNaN(obj.quantityOfMonths)
      )
        obj.quantityOfMonths = 0;
    }
    {
      const dayStr = formData.get("day");
      obj.day = dayStr ? parseFloat(dayStr as string) : 0;
      if (obj.day !== undefined && Number.isNaN(obj.day)) obj.day = 0;
    }
    obj.name = formData.get("name") ?? "";
    return MonthlyRecurrenceRule.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      quantityOfMonths: {
        path: ["quantityOfMonths"] as const,
        name: "quantityOfMonths",
        constraints: { required: true },

        get: (data: Data) => data.quantityOfMonths,
        set: (data: Data, value: number) => {
          data.quantityOfMonths = value;
        },
        getError: (errors: Errors) => errors?.quantityOfMonths,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.quantityOfMonths = value;
        },
        getTainted: (tainted: Tainted) => tainted?.quantityOfMonths ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.quantityOfMonths = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      day: {
        path: ["day"] as const,
        name: "day",
        constraints: { required: true },

        get: (data: Data) => data.day,
        set: (data: Data, value: number) => {
          data.day = value;
        },
        getError: (errors: Errors) => errors?.day,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.day = value;
        },
        getTainted: (tainted: Tainted) => tainted?.day ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.day = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: (data: Data) => data.name,
        set: (data: Data, value: string) => {
          data.name = value;
        },
        getError: (errors: Errors) => errors?.name,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: (tainted: Tainted) => tainted?.name ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.name = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
}

/**  */
export interface Represents {
  in: string | Employee;
  out: string | Account;
  id: string;
  dateStarted: string;
}

export namespace Represents {
  export function toStringifiedJSON(self: Represents): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Represents, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Represents.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Represents;
  }
}

export namespace Represents {
  export type Data = Represents;
  export type Errors = {
    _errors?: Array<string>;
    in?: Array<string>;
    out?: Array<string>;
    id?: Array<string>;
    dateStarted?: Array<string>;
  };
  export type Tainted = {
    in?: boolean;
    out?: boolean;
    id?: boolean;
    dateStarted?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.in = formData.get("in") ?? "";
    obj.out = formData.get("out") ?? "";
    obj.id = formData.get("id") ?? "";
    obj.dateStarted = formData.get("dateStarted") ?? "";
    return Represents.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      in: {
        path: ["in"] as const,
        name: "in",
        constraints: { required: true },

        get: (data: Data) => data.in,
        set: (data: Data, value: string | Employee) => {
          data.in = value;
        },
        getError: (errors: Errors) => errors?.in,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.in = value;
        },
        getTainted: (tainted: Tainted) => tainted?.in ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.in = value;
        },
        validate: (_value: string | Employee): Array<string> => [],
      },
      out: {
        path: ["out"] as const,
        name: "out",
        constraints: { required: true },

        get: (data: Data) => data.out,
        set: (data: Data, value: string | Account) => {
          data.out = value;
        },
        getError: (errors: Errors) => errors?.out,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.out = value;
        },
        getTainted: (tainted: Tainted) => tainted?.out ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.out = value;
        },
        validate: (_value: string | Account): Array<string> => [],
      },
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      dateStarted: {
        path: ["dateStarted"] as const,
        name: "dateStarted",
        constraints: { required: true },

        get: (data: Data) => data.dateStarted,
        set: (data: Data, value: string) => {
          data.dateStarted = value;
        },
        getError: (errors: Errors) => errors?.dateStarted,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.dateStarted = value;
        },
        getTainted: (tainted: Tainted) => tainted?.dateStarted ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.dateStarted = value;
        },
        validate: (_value: string): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Payment): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Payment, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Payment.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Payment;
  }
}

export namespace Payment {
  export type Data = Payment;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    date?: Array<string>;
  };
  export type Tainted = { id?: boolean; date?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.date = formData.get("date") ?? "";
    return Payment.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      date: {
        path: ["date"] as const,
        name: "date",
        constraints: { required: true },

        get: (data: Data) => data.date,
        set: (data: Data, value: string) => {
          data.date = value;
        },
        getError: (errors: Errors) => errors?.date,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.date = value;
        },
        getTainted: (tainted: Tainted) => tainted?.date ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.date = value;
        },
        validate: (_value: string): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Settings): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Settings, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Settings.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Settings;
  }
}

export namespace Settings {
  export type Data = Settings;
  export type Errors = {
    _errors?: Array<string>;
    appointmentNotifications?: Array<string>;
    commissions?: Array<string>;
    scheduleSettings?: ScheduleSettings.Errors;
    accountOverviewSettings?: OverviewSettings.Errors;
    serviceOverviewSettings?: OverviewSettings.Errors;
    appointmentOverviewSettings?: OverviewSettings.Errors;
    leadOverviewSettings?: OverviewSettings.Errors;
    packageOverviewSettings?: OverviewSettings.Errors;
    productOverviewSettings?: OverviewSettings.Errors;
    orderOverviewSettings?: OverviewSettings.Errors;
    taxRateOverviewSettings?: OverviewSettings.Errors;
    homePage?: Page.Errors;
  };
  export type Tainted = {
    appointmentNotifications?: boolean;
    commissions?: boolean;
    scheduleSettings?: ScheduleSettings.Tainted;
    accountOverviewSettings?: OverviewSettings.Tainted;
    serviceOverviewSettings?: OverviewSettings.Tainted;
    appointmentOverviewSettings?: OverviewSettings.Tainted;
    leadOverviewSettings?: OverviewSettings.Tainted;
    packageOverviewSettings?: OverviewSettings.Tainted;
    productOverviewSettings?: OverviewSettings.Tainted;
    orderOverviewSettings?: OverviewSettings.Tainted;
    taxRateOverviewSettings?: OverviewSettings.Tainted;
    homePage?: Page.Tainted;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.appointmentNotifications =
      formData.get("appointmentNotifications") ?? "";
    obj.commissions = formData.get("commissions") ?? "";
    {
      // Collect nested object fields with prefix "scheduleSettings."
      const scheduleSettingsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("scheduleSettings.")) {
          const fieldName = key.slice("scheduleSettings.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = scheduleSettingsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.scheduleSettings = scheduleSettingsObj;
    }
    {
      // Collect nested object fields with prefix "accountOverviewSettings."
      const accountOverviewSettingsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("accountOverviewSettings.")) {
          const fieldName = key.slice("accountOverviewSettings.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = accountOverviewSettingsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.accountOverviewSettings = accountOverviewSettingsObj;
    }
    {
      // Collect nested object fields with prefix "serviceOverviewSettings."
      const serviceOverviewSettingsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("serviceOverviewSettings.")) {
          const fieldName = key.slice("serviceOverviewSettings.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = serviceOverviewSettingsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.serviceOverviewSettings = serviceOverviewSettingsObj;
    }
    {
      // Collect nested object fields with prefix "appointmentOverviewSettings."
      const appointmentOverviewSettingsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("appointmentOverviewSettings.")) {
          const fieldName = key.slice("appointmentOverviewSettings.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = appointmentOverviewSettingsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.appointmentOverviewSettings = appointmentOverviewSettingsObj;
    }
    {
      // Collect nested object fields with prefix "leadOverviewSettings."
      const leadOverviewSettingsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("leadOverviewSettings.")) {
          const fieldName = key.slice("leadOverviewSettings.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = leadOverviewSettingsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.leadOverviewSettings = leadOverviewSettingsObj;
    }
    {
      // Collect nested object fields with prefix "packageOverviewSettings."
      const packageOverviewSettingsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("packageOverviewSettings.")) {
          const fieldName = key.slice("packageOverviewSettings.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = packageOverviewSettingsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.packageOverviewSettings = packageOverviewSettingsObj;
    }
    {
      // Collect nested object fields with prefix "productOverviewSettings."
      const productOverviewSettingsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("productOverviewSettings.")) {
          const fieldName = key.slice("productOverviewSettings.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = productOverviewSettingsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.productOverviewSettings = productOverviewSettingsObj;
    }
    {
      // Collect nested object fields with prefix "orderOverviewSettings."
      const orderOverviewSettingsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("orderOverviewSettings.")) {
          const fieldName = key.slice("orderOverviewSettings.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = orderOverviewSettingsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.orderOverviewSettings = orderOverviewSettingsObj;
    }
    {
      // Collect nested object fields with prefix "taxRateOverviewSettings."
      const taxRateOverviewSettingsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("taxRateOverviewSettings.")) {
          const fieldName = key.slice("taxRateOverviewSettings.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = taxRateOverviewSettingsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.taxRateOverviewSettings = taxRateOverviewSettingsObj;
    }
    {
      // Collect nested object fields with prefix "homePage."
      const homePageObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("homePage.")) {
          const fieldName = key.slice("homePage.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = homePageObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.homePage = homePageObj;
    }
    return Settings.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      appointmentNotifications: {
        path: ["appointmentNotifications"] as const,
        name: "appointmentNotifications",
        constraints: { required: true },

        get: (data: Data) => data.appointmentNotifications,
        set: (data: Data, value: AppointmentNotifications | null) => {
          data.appointmentNotifications = value;
        },
        getError: (errors: Errors) => errors?.appointmentNotifications,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.appointmentNotifications = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.appointmentNotifications ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.appointmentNotifications = value;
        },
        validate: (
          _value: AppointmentNotifications | null,
        ): Array<string> => [],
      },
      commissions: {
        path: ["commissions"] as const,
        name: "commissions",
        constraints: { required: true },

        get: (data: Data) => data.commissions,
        set: (data: Data, value: Commissions | null) => {
          data.commissions = value;
        },
        getError: (errors: Errors) => errors?.commissions,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.commissions = value;
        },
        getTainted: (tainted: Tainted) => tainted?.commissions ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.commissions = value;
        },
        validate: (_value: Commissions | null): Array<string> => [],
      },
      scheduleSettings: {
        path: ["scheduleSettings"] as const,
        name: "scheduleSettings",
        constraints: { required: true },

        get: (data: Data) => data.scheduleSettings,
        set: (data: Data, value: ScheduleSettings) => {
          data.scheduleSettings = value;
        },
        getError: (errors: Errors) => errors?.scheduleSettings,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.scheduleSettings = value;
        },
        getTainted: (tainted: Tainted) => tainted?.scheduleSettings ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.scheduleSettings = value;
        },
        validate: (_value: ScheduleSettings): Array<string> => [],
      },
      accountOverviewSettings: {
        path: ["accountOverviewSettings"] as const,
        name: "accountOverviewSettings",
        constraints: { required: true },

        get: (data: Data) => data.accountOverviewSettings,
        set: (data: Data, value: OverviewSettings) => {
          data.accountOverviewSettings = value;
        },
        getError: (errors: Errors) => errors?.accountOverviewSettings,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.accountOverviewSettings = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.accountOverviewSettings ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.accountOverviewSettings = value;
        },
        validate: (_value: OverviewSettings): Array<string> => [],
      },
      serviceOverviewSettings: {
        path: ["serviceOverviewSettings"] as const,
        name: "serviceOverviewSettings",
        constraints: { required: true },

        get: (data: Data) => data.serviceOverviewSettings,
        set: (data: Data, value: OverviewSettings) => {
          data.serviceOverviewSettings = value;
        },
        getError: (errors: Errors) => errors?.serviceOverviewSettings,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.serviceOverviewSettings = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.serviceOverviewSettings ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.serviceOverviewSettings = value;
        },
        validate: (_value: OverviewSettings): Array<string> => [],
      },
      appointmentOverviewSettings: {
        path: ["appointmentOverviewSettings"] as const,
        name: "appointmentOverviewSettings",
        constraints: { required: true },

        get: (data: Data) => data.appointmentOverviewSettings,
        set: (data: Data, value: OverviewSettings) => {
          data.appointmentOverviewSettings = value;
        },
        getError: (errors: Errors) => errors?.appointmentOverviewSettings,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.appointmentOverviewSettings = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.appointmentOverviewSettings ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.appointmentOverviewSettings = value;
        },
        validate: (_value: OverviewSettings): Array<string> => [],
      },
      leadOverviewSettings: {
        path: ["leadOverviewSettings"] as const,
        name: "leadOverviewSettings",
        constraints: { required: true },

        get: (data: Data) => data.leadOverviewSettings,
        set: (data: Data, value: OverviewSettings) => {
          data.leadOverviewSettings = value;
        },
        getError: (errors: Errors) => errors?.leadOverviewSettings,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.leadOverviewSettings = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.leadOverviewSettings ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.leadOverviewSettings = value;
        },
        validate: (_value: OverviewSettings): Array<string> => [],
      },
      packageOverviewSettings: {
        path: ["packageOverviewSettings"] as const,
        name: "packageOverviewSettings",
        constraints: { required: true },

        get: (data: Data) => data.packageOverviewSettings,
        set: (data: Data, value: OverviewSettings) => {
          data.packageOverviewSettings = value;
        },
        getError: (errors: Errors) => errors?.packageOverviewSettings,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.packageOverviewSettings = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.packageOverviewSettings ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.packageOverviewSettings = value;
        },
        validate: (_value: OverviewSettings): Array<string> => [],
      },
      productOverviewSettings: {
        path: ["productOverviewSettings"] as const,
        name: "productOverviewSettings",
        constraints: { required: true },

        get: (data: Data) => data.productOverviewSettings,
        set: (data: Data, value: OverviewSettings) => {
          data.productOverviewSettings = value;
        },
        getError: (errors: Errors) => errors?.productOverviewSettings,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.productOverviewSettings = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.productOverviewSettings ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.productOverviewSettings = value;
        },
        validate: (_value: OverviewSettings): Array<string> => [],
      },
      orderOverviewSettings: {
        path: ["orderOverviewSettings"] as const,
        name: "orderOverviewSettings",
        constraints: { required: true },

        get: (data: Data) => data.orderOverviewSettings,
        set: (data: Data, value: OverviewSettings) => {
          data.orderOverviewSettings = value;
        },
        getError: (errors: Errors) => errors?.orderOverviewSettings,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.orderOverviewSettings = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.orderOverviewSettings ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.orderOverviewSettings = value;
        },
        validate: (_value: OverviewSettings): Array<string> => [],
      },
      taxRateOverviewSettings: {
        path: ["taxRateOverviewSettings"] as const,
        name: "taxRateOverviewSettings",
        constraints: { required: true },

        get: (data: Data) => data.taxRateOverviewSettings,
        set: (data: Data, value: OverviewSettings) => {
          data.taxRateOverviewSettings = value;
        },
        getError: (errors: Errors) => errors?.taxRateOverviewSettings,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.taxRateOverviewSettings = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.taxRateOverviewSettings ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.taxRateOverviewSettings = value;
        },
        validate: (_value: OverviewSettings): Array<string> => [],
      },
      homePage: {
        path: ["homePage"] as const,
        name: "homePage",
        constraints: { required: true },

        get: (data: Data) => data.homePage,
        set: (data: Data, value: Page) => {
          data.homePage = value;
        },
        getError: (errors: Errors) => errors?.homePage,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.homePage = value;
        },
        getTainted: (tainted: Tainted) => tainted?.homePage ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.homePage = value;
        },
        validate: (_value: Page): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Color): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Color, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Color.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Color;
  }
}

export namespace Color {
  export type Data = Color;
  export type Errors = {
    _errors?: Array<string>;
    red?: Array<string>;
    green?: Array<string>;
    blue?: Array<string>;
  };
  export type Tainted = { red?: boolean; green?: boolean; blue?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const redStr = formData.get("red");
      obj.red = redStr ? parseFloat(redStr as string) : 0;
      if (obj.red !== undefined && Number.isNaN(obj.red)) obj.red = 0;
    }
    {
      const greenStr = formData.get("green");
      obj.green = greenStr ? parseFloat(greenStr as string) : 0;
      if (obj.green !== undefined && Number.isNaN(obj.green)) obj.green = 0;
    }
    {
      const blueStr = formData.get("blue");
      obj.blue = blueStr ? parseFloat(blueStr as string) : 0;
      if (obj.blue !== undefined && Number.isNaN(obj.blue)) obj.blue = 0;
    }
    return Color.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      red: {
        path: ["red"] as const,
        name: "red",
        constraints: { required: true },

        get: (data: Data) => data.red,
        set: (data: Data, value: number) => {
          data.red = value;
        },
        getError: (errors: Errors) => errors?.red,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.red = value;
        },
        getTainted: (tainted: Tainted) => tainted?.red ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.red = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      green: {
        path: ["green"] as const,
        name: "green",
        constraints: { required: true },

        get: (data: Data) => data.green,
        set: (data: Data, value: number) => {
          data.green = value;
        },
        getError: (errors: Errors) => errors?.green,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.green = value;
        },
        getTainted: (tainted: Tainted) => tainted?.green ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.green = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      blue: {
        path: ["blue"] as const,
        name: "blue",
        constraints: { required: true },

        get: (data: Data) => data.blue,
        set: (data: Data, value: number) => {
          data.blue = value;
        },
        getError: (errors: Errors) => errors?.blue,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.blue = value;
        },
        getTainted: (tainted: Tainted) => tainted?.blue ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.blue = value;
        },
        validate: (_value: number): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: CompanyName): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<CompanyName, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "CompanyName.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as CompanyName;
  }
}

export namespace CompanyName {
  export type Data = CompanyName;
  export type Errors = { _errors?: Array<string>; companyName?: Array<string> };
  export type Tainted = { companyName?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.companyName = formData.get("companyName") ?? "";
    return CompanyName.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      companyName: {
        path: ["companyName"] as const,
        name: "companyName",
        constraints: { required: true },

        get: (data: Data) => data.companyName,
        set: (data: Data, value: string) => {
          data.companyName = value;
        },
        getError: (errors: Errors) => errors?.companyName,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.companyName = value;
        },
        getTainted: (tainted: Tainted) => tainted?.companyName ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.companyName = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: Appointment): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Appointment, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Appointment.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Appointment;
  }
}

export namespace Appointment {
  export type Data = Appointment;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    title?: Array<string>;
    status?: Status.Errors;
    begins?: Array<string>;
    duration?: Array<string>;
    timeZone?: Array<string>;
    offsetMs?: Array<string>;
    allDay?: Array<string>;
    multiDay?: Array<string>;
    employees?: { _errors?: Array<string>; [index: number]: Array<string> };
    location?: Array<string>;
    description?: Array<string>;
    colors?: Colors.Errors;
    recurrenceRule?: Array<string>;
  };
  export type Tainted = {
    id?: boolean;
    title?: boolean;
    status?: Status.Tainted;
    begins?: boolean;
    duration?: boolean;
    timeZone?: boolean;
    offsetMs?: boolean;
    allDay?: boolean;
    multiDay?: boolean;
    employees?: { [index: number]: boolean };
    location?: boolean;
    description?: boolean;
    colors?: Colors.Tainted;
    recurrenceRule?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.title = formData.get("title") ?? "";
    {
      // Collect nested object fields with prefix "status."
      const statusObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("status.")) {
          const fieldName = key.slice("status.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = statusObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.status = statusObj;
    }
    obj.begins = formData.get("begins") ?? "";
    {
      const durationStr = formData.get("duration");
      obj.duration = durationStr ? parseFloat(durationStr as string) : 0;
      if (obj.duration !== undefined && Number.isNaN(obj.duration))
        obj.duration = 0;
    }
    obj.timeZone = formData.get("timeZone") ?? "";
    {
      const offsetMsStr = formData.get("offsetMs");
      obj.offsetMs = offsetMsStr ? parseFloat(offsetMsStr as string) : 0;
      if (obj.offsetMs !== undefined && Number.isNaN(obj.offsetMs))
        obj.offsetMs = 0;
    }
    {
      const allDayVal = formData.get("allDay");
      obj.allDay =
        allDayVal === "true" || allDayVal === "on" || allDayVal === "1";
    }
    {
      const multiDayVal = formData.get("multiDay");
      obj.multiDay =
        multiDayVal === "true" || multiDayVal === "on" || multiDayVal === "1";
    }
    {
      // Collect array items from indexed form fields
      const employeesItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("employees." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("employees." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("employees." + idx + ".")) {
              const fieldName = key.slice(
                "employees.".length + String(idx).length + 1,
              );
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
    obj.location = formData.get("location") ?? "";
    obj.description = formData.get("description") ?? "";
    {
      // Collect nested object fields with prefix "colors."
      const colorsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("colors.")) {
          const fieldName = key.slice("colors.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = colorsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.colors = colorsObj;
    }
    obj.recurrenceRule = formData.get("recurrenceRule") ?? "";
    return Appointment.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      title: {
        path: ["title"] as const,
        name: "title",
        constraints: { required: true },

        get: (data: Data) => data.title,
        set: (data: Data, value: string) => {
          data.title = value;
        },
        getError: (errors: Errors) => errors?.title,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.title = value;
        },
        getTainted: (tainted: Tainted) => tainted?.title ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.title = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      status: {
        path: ["status"] as const,
        name: "status",
        constraints: { required: true },

        get: (data: Data) => data.status,
        set: (data: Data, value: Status) => {
          data.status = value;
        },
        getError: (errors: Errors) => errors?.status,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.status = value;
        },
        getTainted: (tainted: Tainted) => tainted?.status ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.status = value;
        },
        validate: (_value: Status): Array<string> => [],
      },
      begins: {
        path: ["begins"] as const,
        name: "begins",
        constraints: { required: true },

        get: (data: Data) => data.begins,
        set: (data: Data, value: string) => {
          data.begins = value;
        },
        getError: (errors: Errors) => errors?.begins,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.begins = value;
        },
        getTainted: (tainted: Tainted) => tainted?.begins ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.begins = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      duration: {
        path: ["duration"] as const,
        name: "duration",
        constraints: { required: true },

        get: (data: Data) => data.duration,
        set: (data: Data, value: number) => {
          data.duration = value;
        },
        getError: (errors: Errors) => errors?.duration,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.duration = value;
        },
        getTainted: (tainted: Tainted) => tainted?.duration ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.duration = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      timeZone: {
        path: ["timeZone"] as const,
        name: "timeZone",
        constraints: { required: true },

        get: (data: Data) => data.timeZone,
        set: (data: Data, value: string) => {
          data.timeZone = value;
        },
        getError: (errors: Errors) => errors?.timeZone,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.timeZone = value;
        },
        getTainted: (tainted: Tainted) => tainted?.timeZone ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.timeZone = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      offsetMs: {
        path: ["offsetMs"] as const,
        name: "offsetMs",
        constraints: { required: true },

        get: (data: Data) => data.offsetMs,
        set: (data: Data, value: number) => {
          data.offsetMs = value;
        },
        getError: (errors: Errors) => errors?.offsetMs,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.offsetMs = value;
        },
        getTainted: (tainted: Tainted) => tainted?.offsetMs ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.offsetMs = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      allDay: {
        path: ["allDay"] as const,
        name: "allDay",
        constraints: { required: true },

        get: (data: Data) => data.allDay,
        set: (data: Data, value: boolean) => {
          data.allDay = value;
        },
        getError: (errors: Errors) => errors?.allDay,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.allDay = value;
        },
        getTainted: (tainted: Tainted) => tainted?.allDay ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.allDay = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      multiDay: {
        path: ["multiDay"] as const,
        name: "multiDay",
        constraints: { required: true },

        get: (data: Data) => data.multiDay,
        set: (data: Data, value: boolean) => {
          data.multiDay = value;
        },
        getError: (errors: Errors) => errors?.multiDay,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.multiDay = value;
        },
        getTainted: (tainted: Tainted) => tainted?.multiDay ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.multiDay = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      employees: {
        path: ["employees"] as const,
        name: "employees",
        constraints: { required: true },

        get: (data: Data) => data.employees,
        set: (data: Data, value: (string | Employee)[]) => {
          data.employees = value;
        },
        getError: (errors: Errors) => errors?.employees,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.employees = value;
        },
        getTainted: (tainted: Tainted) => tainted?.employees ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.employees = value;
        },
        validate: (_value: (string | Employee)[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["employees"], index] as const,
          name: `employees.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.employees[index],
          set: (data: Data, value: string | Employee) => {
            data.employees[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.employees as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.employees ??= {};
            (errors.employees as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.employees?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.employees ??= {};
            tainted.employees[index] = value;
          },
          validate: (_value: string | Employee): Array<string> => [],
        }),
        push: (data: Data, item: string | Employee) => {
          data.employees.push(item);
        },
        remove: (data: Data, index: number) => {
          data.employees.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.employees[a], data.employees[b]] = [
            data.employees[b],
            data.employees[a],
          ];
        },
      },
      location: {
        path: ["location"] as const,
        name: "location",
        constraints: { required: true },

        get: (data: Data) => data.location,
        set: (data: Data, value: string | Site) => {
          data.location = value;
        },
        getError: (errors: Errors) => errors?.location,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.location = value;
        },
        getTainted: (tainted: Tainted) => tainted?.location ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.location = value;
        },
        validate: (_value: string | Site): Array<string> => [],
      },
      description: {
        path: ["description"] as const,
        name: "description",
        constraints: { required: true },

        get: (data: Data) => data.description,
        set: (data: Data, value: string | null) => {
          data.description = value;
        },
        getError: (errors: Errors) => errors?.description,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.description = value;
        },
        getTainted: (tainted: Tainted) => tainted?.description ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.description = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      colors: {
        path: ["colors"] as const,
        name: "colors",
        constraints: { required: true },

        get: (data: Data) => data.colors,
        set: (data: Data, value: Colors) => {
          data.colors = value;
        },
        getError: (errors: Errors) => errors?.colors,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.colors = value;
        },
        getTainted: (tainted: Tainted) => tainted?.colors ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.colors = value;
        },
        validate: (_value: Colors): Array<string> => [],
      },
      recurrenceRule: {
        path: ["recurrenceRule"] as const,
        name: "recurrenceRule",
        constraints: { required: true },

        get: (data: Data) => data.recurrenceRule,
        set: (data: Data, value: RecurrenceRule | null) => {
          data.recurrenceRule = value;
        },
        getError: (errors: Errors) => errors?.recurrenceRule,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.recurrenceRule = value;
        },
        getTainted: (tainted: Tainted) => tainted?.recurrenceRule ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.recurrenceRule = value;
        },
        validate: (_value: RecurrenceRule | null): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Package): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Package, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Package.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Package;
  }
}

export namespace Package {
  export type Data = Package;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    date?: Array<string>;
  };
  export type Tainted = { id?: boolean; date?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.date = formData.get("date") ?? "";
    return Package.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      date: {
        path: ["date"] as const,
        name: "date",
        constraints: { required: true },

        get: (data: Data) => data.date,
        set: (data: Data, value: string) => {
          data.date = value;
        },
        getError: (errors: Errors) => errors?.date,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.date = value;
        },
        getTainted: (tainted: Tainted) => tainted?.date ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.date = value;
        },
        validate: (_value: string): Array<string> => [],
      },
    } as const;
}

/**  */
export interface ScheduleSettings {
  daysPerWeek: number;
  rowHeight: RowHeight;
  visibleRoutes: string[];
  detailedCards: boolean;
}

export namespace ScheduleSettings {
  export function toStringifiedJSON(self: ScheduleSettings): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<ScheduleSettings, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "ScheduleSettings.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as ScheduleSettings;
  }
}

export namespace ScheduleSettings {
  export type Data = ScheduleSettings;
  export type Errors = {
    _errors?: Array<string>;
    daysPerWeek?: Array<string>;
    rowHeight?: RowHeight.Errors;
    visibleRoutes?: { _errors?: Array<string>; [index: number]: Array<string> };
    detailedCards?: Array<string>;
  };
  export type Tainted = {
    daysPerWeek?: boolean;
    rowHeight?: RowHeight.Tainted;
    visibleRoutes?: { [index: number]: boolean };
    detailedCards?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const daysPerWeekStr = formData.get("daysPerWeek");
      obj.daysPerWeek = daysPerWeekStr
        ? parseFloat(daysPerWeekStr as string)
        : 0;
      if (obj.daysPerWeek !== undefined && Number.isNaN(obj.daysPerWeek))
        obj.daysPerWeek = 0;
    }
    {
      // Collect nested object fields with prefix "rowHeight."
      const rowHeightObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("rowHeight.")) {
          const fieldName = key.slice("rowHeight.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = rowHeightObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.rowHeight = rowHeightObj;
    }
    obj.visibleRoutes = formData.getAll("visibleRoutes") as Array<string>;
    {
      const detailedCardsVal = formData.get("detailedCards");
      obj.detailedCards =
        detailedCardsVal === "true" ||
        detailedCardsVal === "on" ||
        detailedCardsVal === "1";
    }
    return ScheduleSettings.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      daysPerWeek: {
        path: ["daysPerWeek"] as const,
        name: "daysPerWeek",
        constraints: { required: true },

        get: (data: Data) => data.daysPerWeek,
        set: (data: Data, value: number) => {
          data.daysPerWeek = value;
        },
        getError: (errors: Errors) => errors?.daysPerWeek,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.daysPerWeek = value;
        },
        getTainted: (tainted: Tainted) => tainted?.daysPerWeek ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.daysPerWeek = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      rowHeight: {
        path: ["rowHeight"] as const,
        name: "rowHeight",
        constraints: { required: true },

        get: (data: Data) => data.rowHeight,
        set: (data: Data, value: RowHeight) => {
          data.rowHeight = value;
        },
        getError: (errors: Errors) => errors?.rowHeight,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.rowHeight = value;
        },
        getTainted: (tainted: Tainted) => tainted?.rowHeight ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.rowHeight = value;
        },
        validate: (_value: RowHeight): Array<string> => [],
      },
      visibleRoutes: {
        path: ["visibleRoutes"] as const,
        name: "visibleRoutes",
        constraints: { required: true },

        get: (data: Data) => data.visibleRoutes,
        set: (data: Data, value: string[]) => {
          data.visibleRoutes = value;
        },
        getError: (errors: Errors) => errors?.visibleRoutes,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.visibleRoutes = value;
        },
        getTainted: (tainted: Tainted) => tainted?.visibleRoutes ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.visibleRoutes = value;
        },
        validate: (_value: string[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["visibleRoutes"], index] as const,
          name: `visibleRoutes.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.visibleRoutes[index],
          set: (data: Data, value: string) => {
            data.visibleRoutes[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.visibleRoutes as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.visibleRoutes ??= {};
            (errors.visibleRoutes as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: (tainted: Tainted) =>
            tainted.visibleRoutes?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.visibleRoutes ??= {};
            tainted.visibleRoutes[index] = value;
          },
          validate: (_value: string): Array<string> => [],
        }),
        push: (data: Data, item: string) => {
          data.visibleRoutes.push(item);
        },
        remove: (data: Data, index: number) => {
          data.visibleRoutes.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.visibleRoutes[a], data.visibleRoutes[b]] = [
            data.visibleRoutes[b],
            data.visibleRoutes[a],
          ];
        },
      },
      detailedCards: {
        path: ["detailedCards"] as const,
        name: "detailedCards",
        constraints: { required: true },

        get: (data: Data) => data.detailedCards,
        set: (data: Data, value: boolean) => {
          data.detailedCards = value;
        },
        getError: (errors: Errors) => errors?.detailedCards,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.detailedCards = value;
        },
        getTainted: (tainted: Tainted) => tainted?.detailedCards ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.detailedCards = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: DailyRecurrenceRule): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<DailyRecurrenceRule, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "DailyRecurrenceRule.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as DailyRecurrenceRule;
  }
}

export namespace DailyRecurrenceRule {
  export type Data = DailyRecurrenceRule;
  export type Errors = {
    _errors?: Array<string>;
    quantityOfDays?: Array<string>;
  };
  export type Tainted = { quantityOfDays?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const quantityOfDaysStr = formData.get("quantityOfDays");
      obj.quantityOfDays = quantityOfDaysStr
        ? parseFloat(quantityOfDaysStr as string)
        : 0;
      if (obj.quantityOfDays !== undefined && Number.isNaN(obj.quantityOfDays))
        obj.quantityOfDays = 0;
    }
    return DailyRecurrenceRule.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      quantityOfDays: {
        path: ["quantityOfDays"] as const,
        name: "quantityOfDays",
        constraints: { required: true },

        get: (data: Data) => data.quantityOfDays,
        set: (data: Data, value: number) => {
          data.quantityOfDays = value;
        },
        getError: (errors: Errors) => errors?.quantityOfDays,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.quantityOfDays = value;
        },
        getTainted: (tainted: Tainted) => tainted?.quantityOfDays ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.quantityOfDays = value;
        },
        validate: (_value: number): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: SignUpCredentials): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<SignUpCredentials, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "SignUpCredentials.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as SignUpCredentials;
  }
}

export namespace SignUpCredentials {
  export type Data = SignUpCredentials;
  export type Errors = {
    _errors?: Array<string>;
    firstName?: FirstName.Errors;
    lastName?: LastName.Errors;
    email?: EmailParts.Errors;
    password?: Password.Errors;
    rememberMe?: Array<string>;
  };
  export type Tainted = {
    firstName?: FirstName.Tainted;
    lastName?: LastName.Tainted;
    email?: EmailParts.Tainted;
    password?: Password.Tainted;
    rememberMe?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      // Collect nested object fields with prefix "firstName."
      const firstNameObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("firstName.")) {
          const fieldName = key.slice("firstName.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = firstNameObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.firstName = firstNameObj;
    }
    {
      // Collect nested object fields with prefix "lastName."
      const lastNameObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("lastName.")) {
          const fieldName = key.slice("lastName.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = lastNameObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.lastName = lastNameObj;
    }
    {
      // Collect nested object fields with prefix "email."
      const emailObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("email.")) {
          const fieldName = key.slice("email.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = emailObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.email = emailObj;
    }
    {
      // Collect nested object fields with prefix "password."
      const passwordObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("password.")) {
          const fieldName = key.slice("password.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = passwordObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.password = passwordObj;
    }
    {
      const rememberMeVal = formData.get("rememberMe");
      obj.rememberMe =
        rememberMeVal === "true" ||
        rememberMeVal === "on" ||
        rememberMeVal === "1";
    }
    return SignUpCredentials.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      firstName: {
        path: ["firstName"] as const,
        name: "firstName",
        constraints: { required: true },

        get: (data: Data) => data.firstName,
        set: (data: Data, value: FirstName) => {
          data.firstName = value;
        },
        getError: (errors: Errors) => errors?.firstName,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.firstName = value;
        },
        getTainted: (tainted: Tainted) => tainted?.firstName ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.firstName = value;
        },
        validate: (_value: FirstName): Array<string> => [],
      },
      lastName: {
        path: ["lastName"] as const,
        name: "lastName",
        constraints: { required: true },

        get: (data: Data) => data.lastName,
        set: (data: Data, value: LastName) => {
          data.lastName = value;
        },
        getError: (errors: Errors) => errors?.lastName,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.lastName = value;
        },
        getTainted: (tainted: Tainted) => tainted?.lastName ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.lastName = value;
        },
        validate: (_value: LastName): Array<string> => [],
      },
      email: {
        path: ["email"] as const,
        name: "email",
        constraints: { required: true },

        get: (data: Data) => data.email,
        set: (data: Data, value: EmailParts) => {
          data.email = value;
        },
        getError: (errors: Errors) => errors?.email,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.email = value;
        },
        getTainted: (tainted: Tainted) => tainted?.email ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.email = value;
        },
        validate: (_value: EmailParts): Array<string> => [],
      },
      password: {
        path: ["password"] as const,
        name: "password",
        constraints: { required: true },

        get: (data: Data) => data.password,
        set: (data: Data, value: Password) => {
          data.password = value;
        },
        getError: (errors: Errors) => errors?.password,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.password = value;
        },
        getTainted: (tainted: Tainted) => tainted?.password ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.password = value;
        },
        validate: (_value: Password): Array<string> => [],
      },
      rememberMe: {
        path: ["rememberMe"] as const,
        name: "rememberMe",
        constraints: { required: true },

        get: (data: Data) => data.rememberMe,
        set: (data: Data, value: boolean) => {
          data.rememberMe = value;
        },
        getError: (errors: Errors) => errors?.rememberMe,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.rememberMe = value;
        },
        getTainted: (tainted: Tainted) => tainted?.rememberMe ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.rememberMe = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
    } as const;
}

/**  */
export interface OverviewSettings {
  rowHeight: RowHeight;
  cardOrRow: OverviewDisplay;
  perPage: number;
  columnConfigs: ColumnConfig[];
}

export namespace OverviewSettings {
  export function toStringifiedJSON(self: OverviewSettings): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<OverviewSettings, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "OverviewSettings.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as OverviewSettings;
  }
}

export namespace OverviewSettings {
  export type Data = OverviewSettings;
  export type Errors = {
    _errors?: Array<string>;
    rowHeight?: RowHeight.Errors;
    cardOrRow?: OverviewDisplay.Errors;
    perPage?: Array<string>;
    columnConfigs?: {
      _errors?: Array<string>;
      [index: number]: ColumnConfig.Errors;
    };
  };
  export type Tainted = {
    rowHeight?: RowHeight.Tainted;
    cardOrRow?: OverviewDisplay.Tainted;
    perPage?: boolean;
    columnConfigs?: { [index: number]: ColumnConfig.Tainted };
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      // Collect nested object fields with prefix "rowHeight."
      const rowHeightObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("rowHeight.")) {
          const fieldName = key.slice("rowHeight.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = rowHeightObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.rowHeight = rowHeightObj;
    }
    {
      // Collect nested object fields with prefix "cardOrRow."
      const cardOrRowObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("cardOrRow.")) {
          const fieldName = key.slice("cardOrRow.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = cardOrRowObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.cardOrRow = cardOrRowObj;
    }
    {
      const perPageStr = formData.get("perPage");
      obj.perPage = perPageStr ? parseFloat(perPageStr as string) : 0;
      if (obj.perPage !== undefined && Number.isNaN(obj.perPage))
        obj.perPage = 0;
    }
    {
      // Collect array items from indexed form fields
      const columnConfigsItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("columnConfigs." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("columnConfigs." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("columnConfigs." + idx + ".")) {
              const fieldName = key.slice(
                "columnConfigs.".length + String(idx).length + 1,
              );
              item[fieldName] = value;
            }
          }
          columnConfigsItems.push(item);
        }
        idx++;
        if (idx > 1000) break; // Safety limit
      }
      obj.columnConfigs = columnConfigsItems;
    }
    return OverviewSettings.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      rowHeight: {
        path: ["rowHeight"] as const,
        name: "rowHeight",
        constraints: { required: true },

        get: (data: Data) => data.rowHeight,
        set: (data: Data, value: RowHeight) => {
          data.rowHeight = value;
        },
        getError: (errors: Errors) => errors?.rowHeight,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.rowHeight = value;
        },
        getTainted: (tainted: Tainted) => tainted?.rowHeight ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.rowHeight = value;
        },
        validate: (_value: RowHeight): Array<string> => [],
      },
      cardOrRow: {
        path: ["cardOrRow"] as const,
        name: "cardOrRow",
        constraints: { required: true },

        get: (data: Data) => data.cardOrRow,
        set: (data: Data, value: OverviewDisplay) => {
          data.cardOrRow = value;
        },
        getError: (errors: Errors) => errors?.cardOrRow,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.cardOrRow = value;
        },
        getTainted: (tainted: Tainted) => tainted?.cardOrRow ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.cardOrRow = value;
        },
        validate: (_value: OverviewDisplay): Array<string> => [],
      },
      perPage: {
        path: ["perPage"] as const,
        name: "perPage",
        constraints: { required: true },

        get: (data: Data) => data.perPage,
        set: (data: Data, value: number) => {
          data.perPage = value;
        },
        getError: (errors: Errors) => errors?.perPage,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.perPage = value;
        },
        getTainted: (tainted: Tainted) => tainted?.perPage ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.perPage = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      columnConfigs: {
        path: ["columnConfigs"] as const,
        name: "columnConfigs",
        constraints: { required: true },

        get: (data: Data) => data.columnConfigs,
        set: (data: Data, value: ColumnConfig[]) => {
          data.columnConfigs = value;
        },
        getError: (errors: Errors) => errors?.columnConfigs,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.columnConfigs = value;
        },
        getTainted: (tainted: Tainted) => tainted?.columnConfigs ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.columnConfigs = value;
        },
        validate: (_value: ColumnConfig[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["columnConfigs"], index] as const,
          name: `columnConfigs.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.columnConfigs[index],
          set: (data: Data, value: ColumnConfig) => {
            data.columnConfigs[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.columnConfigs as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.columnConfigs ??= {};
            (errors.columnConfigs as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: (tainted: Tainted) =>
            tainted.columnConfigs?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.columnConfigs ??= {};
            tainted.columnConfigs[index] = value;
          },
          validate: (_value: ColumnConfig): Array<string> => [],
        }),
        push: (data: Data, item: ColumnConfig) => {
          data.columnConfigs.push(item);
        },
        remove: (data: Data, index: number) => {
          data.columnConfigs.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.columnConfigs[a], data.columnConfigs[b]] = [
            data.columnConfigs[b],
            data.columnConfigs[a],
          ];
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: FirstName): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<FirstName, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "FirstName.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as FirstName;
  }
}

export namespace FirstName {
  export type Data = FirstName;
  export type Errors = { _errors?: Array<string>; name?: Array<string> };
  export type Tainted = { name?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.name = formData.get("name") ?? "";
    return FirstName.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: (data: Data) => data.name,
        set: (data: Data, value: string) => {
          data.name = value;
        },
        getError: (errors: Errors) => errors?.name,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: (tainted: Tainted) => tainted?.name ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.name = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: Account): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Account, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Account.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Account;
  }
}

export namespace Account {
  export type Data = Account;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    taxRate?: Array<string>;
    site?: Array<string>;
    salesRep?: Array<string>;
    orders?: { _errors?: Array<string>; [index: number]: Ordered.Errors };
    activity?: { _errors?: Array<string>; [index: number]: Did.Errors };
    customFields?: { _errors?: Array<string>; [index: number]: Array<string> };
    accountName?: AccountName.Errors;
    sector?: Sector.Errors;
    memo?: Array<string>;
    phones?: { _errors?: Array<string>; [index: number]: PhoneNumber.Errors };
    email?: Email.Errors;
    leadSource?: Array<string>;
    colors?: Colors.Errors;
    needsReview?: Array<string>;
    hasAlert?: Array<string>;
    accountType?: Array<string>;
    subtype?: Array<string>;
    isTaxExempt?: Array<string>;
    paymentTerms?: Array<string>;
    tags?: { _errors?: Array<string>; [index: number]: Array<string> };
    dateAdded?: Array<string>;
  };
  export type Tainted = {
    id?: boolean;
    taxRate?: boolean;
    site?: boolean;
    salesRep?: boolean;
    orders?: { [index: number]: Ordered.Tainted };
    activity?: { [index: number]: Did.Tainted };
    customFields?: { [index: number]: boolean };
    accountName?: AccountName.Tainted;
    sector?: Sector.Tainted;
    memo?: boolean;
    phones?: { [index: number]: PhoneNumber.Tainted };
    email?: Email.Tainted;
    leadSource?: boolean;
    colors?: Colors.Tainted;
    needsReview?: boolean;
    hasAlert?: boolean;
    accountType?: boolean;
    subtype?: boolean;
    isTaxExempt?: boolean;
    paymentTerms?: boolean;
    tags?: { [index: number]: boolean };
    dateAdded?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.taxRate = formData.get("taxRate") ?? "";
    obj.site = formData.get("site") ?? "";
    obj.salesRep = formData.get("salesRep") ?? "";
    {
      // Collect array items from indexed form fields
      const ordersItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("orders." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("orders." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("orders." + idx + ".")) {
              const fieldName = key.slice(
                "orders.".length + String(idx).length + 1,
              );
              item[fieldName] = value;
            }
          }
          ordersItems.push(item);
        }
        idx++;
        if (idx > 1000) break; // Safety limit
      }
      obj.orders = ordersItems;
    }
    {
      // Collect array items from indexed form fields
      const activityItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("activity." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("activity." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("activity." + idx + ".")) {
              const fieldName = key.slice(
                "activity.".length + String(idx).length + 1,
              );
              item[fieldName] = value;
            }
          }
          activityItems.push(item);
        }
        idx++;
        if (idx > 1000) break; // Safety limit
      }
      obj.activity = activityItems;
    }
    {
      // Collect array items from indexed form fields
      const customFieldsItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("customFields." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("customFields." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("customFields." + idx + ".")) {
              const fieldName = key.slice(
                "customFields.".length + String(idx).length + 1,
              );
              item[fieldName] = value;
            }
          }
          customFieldsItems.push(item);
        }
        idx++;
        if (idx > 1000) break; // Safety limit
      }
      obj.customFields = customFieldsItems;
    }
    {
      // Collect nested object fields with prefix "accountName."
      const accountNameObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("accountName.")) {
          const fieldName = key.slice("accountName.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = accountNameObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.accountName = accountNameObj;
    }
    {
      // Collect nested object fields with prefix "sector."
      const sectorObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("sector.")) {
          const fieldName = key.slice("sector.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = sectorObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.sector = sectorObj;
    }
    obj.memo = formData.get("memo") ?? "";
    {
      // Collect array items from indexed form fields
      const phonesItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("phones." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("phones." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("phones." + idx + ".")) {
              const fieldName = key.slice(
                "phones.".length + String(idx).length + 1,
              );
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
    {
      // Collect nested object fields with prefix "email."
      const emailObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("email.")) {
          const fieldName = key.slice("email.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = emailObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.email = emailObj;
    }
    obj.leadSource = formData.get("leadSource") ?? "";
    {
      // Collect nested object fields with prefix "colors."
      const colorsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("colors.")) {
          const fieldName = key.slice("colors.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = colorsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.colors = colorsObj;
    }
    {
      const needsReviewVal = formData.get("needsReview");
      obj.needsReview =
        needsReviewVal === "true" ||
        needsReviewVal === "on" ||
        needsReviewVal === "1";
    }
    {
      const hasAlertVal = formData.get("hasAlert");
      obj.hasAlert =
        hasAlertVal === "true" || hasAlertVal === "on" || hasAlertVal === "1";
    }
    obj.accountType = formData.get("accountType") ?? "";
    obj.subtype = formData.get("subtype") ?? "";
    {
      const isTaxExemptVal = formData.get("isTaxExempt");
      obj.isTaxExempt =
        isTaxExemptVal === "true" ||
        isTaxExemptVal === "on" ||
        isTaxExemptVal === "1";
    }
    obj.paymentTerms = formData.get("paymentTerms") ?? "";
    obj.tags = formData.getAll("tags") as Array<string>;
    obj.dateAdded = formData.get("dateAdded") ?? "";
    return Account.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      taxRate: {
        path: ["taxRate"] as const,
        name: "taxRate",
        constraints: { required: true },

        get: (data: Data) => data.taxRate,
        set: (data: Data, value: string | TaxRate) => {
          data.taxRate = value;
        },
        getError: (errors: Errors) => errors?.taxRate,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.taxRate = value;
        },
        getTainted: (tainted: Tainted) => tainted?.taxRate ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.taxRate = value;
        },
        validate: (_value: string | TaxRate): Array<string> => [],
      },
      site: {
        path: ["site"] as const,
        name: "site",
        constraints: { required: true },

        get: (data: Data) => data.site,
        set: (data: Data, value: string | Site) => {
          data.site = value;
        },
        getError: (errors: Errors) => errors?.site,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.site = value;
        },
        getTainted: (tainted: Tainted) => tainted?.site ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.site = value;
        },
        validate: (_value: string | Site): Array<string> => [],
      },
      salesRep: {
        path: ["salesRep"] as const,
        name: "salesRep",
        constraints: { required: true },

        get: (data: Data) => data.salesRep,
        set: (data: Data, value: Represents[] | null) => {
          data.salesRep = value;
        },
        getError: (errors: Errors) => errors?.salesRep,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.salesRep = value;
        },
        getTainted: (tainted: Tainted) => tainted?.salesRep ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.salesRep = value;
        },
        validate: (_value: Represents[] | null): Array<string> => [],
      },
      orders: {
        path: ["orders"] as const,
        name: "orders",
        constraints: { required: true },

        get: (data: Data) => data.orders,
        set: (data: Data, value: Ordered[]) => {
          data.orders = value;
        },
        getError: (errors: Errors) => errors?.orders,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.orders = value;
        },
        getTainted: (tainted: Tainted) => tainted?.orders ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.orders = value;
        },
        validate: (_value: Ordered[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["orders"], index] as const,
          name: `orders.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.orders[index],
          set: (data: Data, value: Ordered) => {
            data.orders[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.orders as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.orders ??= {};
            (errors.orders as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.orders?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.orders ??= {};
            tainted.orders[index] = value;
          },
          validate: (_value: Ordered): Array<string> => [],
        }),
        push: (data: Data, item: Ordered) => {
          data.orders.push(item);
        },
        remove: (data: Data, index: number) => {
          data.orders.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.orders[a], data.orders[b]] = [data.orders[b], data.orders[a]];
        },
      },
      activity: {
        path: ["activity"] as const,
        name: "activity",
        constraints: { required: true },

        get: (data: Data) => data.activity,
        set: (data: Data, value: Did[]) => {
          data.activity = value;
        },
        getError: (errors: Errors) => errors?.activity,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.activity = value;
        },
        getTainted: (tainted: Tainted) => tainted?.activity ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.activity = value;
        },
        validate: (_value: Did[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["activity"], index] as const,
          name: `activity.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.activity[index],
          set: (data: Data, value: Did) => {
            data.activity[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.activity as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.activity ??= {};
            (errors.activity as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.activity?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.activity ??= {};
            tainted.activity[index] = value;
          },
          validate: (_value: Did): Array<string> => [],
        }),
        push: (data: Data, item: Did) => {
          data.activity.push(item);
        },
        remove: (data: Data, index: number) => {
          data.activity.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.activity[a], data.activity[b]] = [
            data.activity[b],
            data.activity[a],
          ];
        },
      },
      customFields: {
        path: ["customFields"] as const,
        name: "customFields",
        constraints: { required: true },

        get: (data: Data) => data.customFields,
        set: (data: Data, value: [string, string][]) => {
          data.customFields = value;
        },
        getError: (errors: Errors) => errors?.customFields,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.customFields = value;
        },
        getTainted: (tainted: Tainted) => tainted?.customFields ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.customFields = value;
        },
        validate: (_value: [string, string][]): Array<string> => [],

        at: (index: number) => ({
          path: [...["customFields"], index] as const,
          name: `customFields.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.customFields[index],
          set: (data: Data, value: [string, string]) => {
            data.customFields[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.customFields as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.customFields ??= {};
            (errors.customFields as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: (tainted: Tainted) =>
            tainted.customFields?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.customFields ??= {};
            tainted.customFields[index] = value;
          },
          validate: (_value: [string, string]): Array<string> => [],
        }),
        push: (data: Data, item: [string, string]) => {
          data.customFields.push(item);
        },
        remove: (data: Data, index: number) => {
          data.customFields.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.customFields[a], data.customFields[b]] = [
            data.customFields[b],
            data.customFields[a],
          ];
        },
      },
      accountName: {
        path: ["accountName"] as const,
        name: "accountName",
        constraints: { required: true },

        get: (data: Data) => data.accountName,
        set: (data: Data, value: AccountName) => {
          data.accountName = value;
        },
        getError: (errors: Errors) => errors?.accountName,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.accountName = value;
        },
        getTainted: (tainted: Tainted) => tainted?.accountName ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.accountName = value;
        },
        validate: (_value: AccountName): Array<string> => [],
      },
      sector: {
        path: ["sector"] as const,
        name: "sector",
        constraints: { required: true },

        get: (data: Data) => data.sector,
        set: (data: Data, value: Sector) => {
          data.sector = value;
        },
        getError: (errors: Errors) => errors?.sector,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.sector = value;
        },
        getTainted: (tainted: Tainted) => tainted?.sector ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.sector = value;
        },
        validate: (_value: Sector): Array<string> => [],
      },
      memo: {
        path: ["memo"] as const,
        name: "memo",
        constraints: { required: true },

        get: (data: Data) => data.memo,
        set: (data: Data, value: string | null) => {
          data.memo = value;
        },
        getError: (errors: Errors) => errors?.memo,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.memo = value;
        },
        getTainted: (tainted: Tainted) => tainted?.memo ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.memo = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      phones: {
        path: ["phones"] as const,
        name: "phones",
        constraints: { required: true },

        get: (data: Data) => data.phones,
        set: (data: Data, value: PhoneNumber[]) => {
          data.phones = value;
        },
        getError: (errors: Errors) => errors?.phones,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.phones = value;
        },
        getTainted: (tainted: Tainted) => tainted?.phones ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.phones = value;
        },
        validate: (_value: PhoneNumber[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["phones"], index] as const,
          name: `phones.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.phones[index],
          set: (data: Data, value: PhoneNumber) => {
            data.phones[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.phones as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.phones ??= {};
            (errors.phones as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.phones?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.phones ??= {};
            tainted.phones[index] = value;
          },
          validate: (_value: PhoneNumber): Array<string> => [],
        }),
        push: (data: Data, item: PhoneNumber) => {
          data.phones.push(item);
        },
        remove: (data: Data, index: number) => {
          data.phones.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.phones[a], data.phones[b]] = [data.phones[b], data.phones[a]];
        },
      },
      email: {
        path: ["email"] as const,
        name: "email",
        constraints: { required: true },

        get: (data: Data) => data.email,
        set: (data: Data, value: Email) => {
          data.email = value;
        },
        getError: (errors: Errors) => errors?.email,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.email = value;
        },
        getTainted: (tainted: Tainted) => tainted?.email ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.email = value;
        },
        validate: (_value: Email): Array<string> => [],
      },
      leadSource: {
        path: ["leadSource"] as const,
        name: "leadSource",
        constraints: { required: true },

        get: (data: Data) => data.leadSource,
        set: (data: Data, value: string) => {
          data.leadSource = value;
        },
        getError: (errors: Errors) => errors?.leadSource,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.leadSource = value;
        },
        getTainted: (tainted: Tainted) => tainted?.leadSource ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.leadSource = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      colors: {
        path: ["colors"] as const,
        name: "colors",
        constraints: { required: true },

        get: (data: Data) => data.colors,
        set: (data: Data, value: Colors) => {
          data.colors = value;
        },
        getError: (errors: Errors) => errors?.colors,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.colors = value;
        },
        getTainted: (tainted: Tainted) => tainted?.colors ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.colors = value;
        },
        validate: (_value: Colors): Array<string> => [],
      },
      needsReview: {
        path: ["needsReview"] as const,
        name: "needsReview",
        constraints: { required: true },

        get: (data: Data) => data.needsReview,
        set: (data: Data, value: boolean) => {
          data.needsReview = value;
        },
        getError: (errors: Errors) => errors?.needsReview,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.needsReview = value;
        },
        getTainted: (tainted: Tainted) => tainted?.needsReview ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.needsReview = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      hasAlert: {
        path: ["hasAlert"] as const,
        name: "hasAlert",
        constraints: { required: true },

        get: (data: Data) => data.hasAlert,
        set: (data: Data, value: boolean) => {
          data.hasAlert = value;
        },
        getError: (errors: Errors) => errors?.hasAlert,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.hasAlert = value;
        },
        getTainted: (tainted: Tainted) => tainted?.hasAlert ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.hasAlert = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      accountType: {
        path: ["accountType"] as const,
        name: "accountType",
        constraints: { required: true },

        get: (data: Data) => data.accountType,
        set: (data: Data, value: string) => {
          data.accountType = value;
        },
        getError: (errors: Errors) => errors?.accountType,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.accountType = value;
        },
        getTainted: (tainted: Tainted) => tainted?.accountType ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.accountType = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      subtype: {
        path: ["subtype"] as const,
        name: "subtype",
        constraints: { required: true },

        get: (data: Data) => data.subtype,
        set: (data: Data, value: string) => {
          data.subtype = value;
        },
        getError: (errors: Errors) => errors?.subtype,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.subtype = value;
        },
        getTainted: (tainted: Tainted) => tainted?.subtype ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.subtype = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      isTaxExempt: {
        path: ["isTaxExempt"] as const,
        name: "isTaxExempt",
        constraints: { required: true },

        get: (data: Data) => data.isTaxExempt,
        set: (data: Data, value: boolean) => {
          data.isTaxExempt = value;
        },
        getError: (errors: Errors) => errors?.isTaxExempt,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.isTaxExempt = value;
        },
        getTainted: (tainted: Tainted) => tainted?.isTaxExempt ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.isTaxExempt = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      paymentTerms: {
        path: ["paymentTerms"] as const,
        name: "paymentTerms",
        constraints: { required: true },

        get: (data: Data) => data.paymentTerms,
        set: (data: Data, value: string) => {
          data.paymentTerms = value;
        },
        getError: (errors: Errors) => errors?.paymentTerms,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.paymentTerms = value;
        },
        getTainted: (tainted: Tainted) => tainted?.paymentTerms ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.paymentTerms = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      tags: {
        path: ["tags"] as const,
        name: "tags",
        constraints: { required: true },

        get: (data: Data) => data.tags,
        set: (data: Data, value: string[]) => {
          data.tags = value;
        },
        getError: (errors: Errors) => errors?.tags,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.tags = value;
        },
        getTainted: (tainted: Tainted) => tainted?.tags ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.tags = value;
        },
        validate: (_value: string[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["tags"], index] as const,
          name: `tags.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.tags[index],
          set: (data: Data, value: string) => {
            data.tags[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.tags as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.tags ??= {};
            (errors.tags as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.tags?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.tags ??= {};
            tainted.tags[index] = value;
          },
          validate: (_value: string): Array<string> => [],
        }),
        push: (data: Data, item: string) => {
          data.tags.push(item);
        },
        remove: (data: Data, index: number) => {
          data.tags.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.tags[a], data.tags[b]] = [data.tags[b], data.tags[a]];
        },
      },
      dateAdded: {
        path: ["dateAdded"] as const,
        name: "dateAdded",
        constraints: { required: true },

        get: (data: Data) => data.dateAdded,
        set: (data: Data, value: string) => {
          data.dateAdded = value;
        },
        getError: (errors: Errors) => errors?.dateAdded,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.dateAdded = value;
        },
        getTainted: (tainted: Tainted) => tainted?.dateAdded ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.dateAdded = value;
        },
        validate: (_value: string): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Edited): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Edited, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Edited.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Edited;
  }
}

export namespace Edited {
  export type Data = Edited;
  export type Errors = {
    _errors?: Array<string>;
    fieldName?: Array<string>;
    oldValue?: Array<string>;
    newValue?: Array<string>;
  };
  export type Tainted = {
    fieldName?: boolean;
    oldValue?: boolean;
    newValue?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.fieldName = formData.get("fieldName") ?? "";
    obj.oldValue = formData.get("oldValue") ?? "";
    obj.newValue = formData.get("newValue") ?? "";
    return Edited.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      fieldName: {
        path: ["fieldName"] as const,
        name: "fieldName",
        constraints: { required: true },

        get: (data: Data) => data.fieldName,
        set: (data: Data, value: string) => {
          data.fieldName = value;
        },
        getError: (errors: Errors) => errors?.fieldName,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.fieldName = value;
        },
        getTainted: (tainted: Tainted) => tainted?.fieldName ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.fieldName = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      oldValue: {
        path: ["oldValue"] as const,
        name: "oldValue",
        constraints: { required: true },

        get: (data: Data) => data.oldValue,
        set: (data: Data, value: string | null) => {
          data.oldValue = value;
        },
        getError: (errors: Errors) => errors?.oldValue,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.oldValue = value;
        },
        getTainted: (tainted: Tainted) => tainted?.oldValue ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.oldValue = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      newValue: {
        path: ["newValue"] as const,
        name: "newValue",
        constraints: { required: true },

        get: (data: Data) => data.newValue,
        set: (data: Data, value: string | null) => {
          data.newValue = value;
        },
        getError: (errors: Errors) => errors?.newValue,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.newValue = value;
        },
        getTainted: (tainted: Tainted) => tainted?.newValue ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.newValue = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Order): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Order, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Order.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Order;
  }
}

export namespace Order {
  export type Data = Order;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    account?: Array<string>;
    stage?: OrderStage.Errors;
    number?: Array<string>;
    payments?: { _errors?: Array<string>; [index: number]: Array<string> };
    opportunity?: Array<string>;
    reference?: Array<string>;
    leadSource?: Array<string>;
    salesRep?: Array<string>;
    group?: Array<string>;
    subgroup?: Array<string>;
    isPosted?: Array<string>;
    needsReview?: Array<string>;
    actionItem?: Array<string>;
    upsale?: Array<string>;
    dateCreated?: Array<string>;
    appointment?: Array<string>;
    lastTechs?: { _errors?: Array<string>; [index: number]: Array<string> };
    package?: Array<string>;
    promotion?: Array<string>;
    balance?: Array<string>;
    due?: Array<string>;
    total?: Array<string>;
    site?: Array<string>;
    billedItems?: {
      _errors?: Array<string>;
      [index: number]: BilledItem.Errors;
    };
    memo?: Array<string>;
    discount?: Array<string>;
    tip?: Array<string>;
    commissions?: { _errors?: Array<string>; [index: number]: Array<string> };
  };
  export type Tainted = {
    id?: boolean;
    account?: boolean;
    stage?: OrderStage.Tainted;
    number?: boolean;
    payments?: { [index: number]: boolean };
    opportunity?: boolean;
    reference?: boolean;
    leadSource?: boolean;
    salesRep?: boolean;
    group?: boolean;
    subgroup?: boolean;
    isPosted?: boolean;
    needsReview?: boolean;
    actionItem?: boolean;
    upsale?: boolean;
    dateCreated?: boolean;
    appointment?: boolean;
    lastTechs?: { [index: number]: boolean };
    package?: boolean;
    promotion?: boolean;
    balance?: boolean;
    due?: boolean;
    total?: boolean;
    site?: boolean;
    billedItems?: { [index: number]: BilledItem.Tainted };
    memo?: boolean;
    discount?: boolean;
    tip?: boolean;
    commissions?: { [index: number]: boolean };
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.account = formData.get("account") ?? "";
    {
      // Collect nested object fields with prefix "stage."
      const stageObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("stage.")) {
          const fieldName = key.slice("stage.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = stageObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.stage = stageObj;
    }
    {
      const numberStr = formData.get("number");
      obj.number = numberStr ? parseFloat(numberStr as string) : 0;
      if (obj.number !== undefined && Number.isNaN(obj.number)) obj.number = 0;
    }
    {
      // Collect array items from indexed form fields
      const paymentsItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("payments." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("payments." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("payments." + idx + ".")) {
              const fieldName = key.slice(
                "payments.".length + String(idx).length + 1,
              );
              item[fieldName] = value;
            }
          }
          paymentsItems.push(item);
        }
        idx++;
        if (idx > 1000) break; // Safety limit
      }
      obj.payments = paymentsItems;
    }
    obj.opportunity = formData.get("opportunity") ?? "";
    obj.reference = formData.get("reference") ?? "";
    obj.leadSource = formData.get("leadSource") ?? "";
    obj.salesRep = formData.get("salesRep") ?? "";
    obj.group = formData.get("group") ?? "";
    obj.subgroup = formData.get("subgroup") ?? "";
    {
      const isPostedVal = formData.get("isPosted");
      obj.isPosted =
        isPostedVal === "true" || isPostedVal === "on" || isPostedVal === "1";
    }
    {
      const needsReviewVal = formData.get("needsReview");
      obj.needsReview =
        needsReviewVal === "true" ||
        needsReviewVal === "on" ||
        needsReviewVal === "1";
    }
    obj.actionItem = formData.get("actionItem") ?? "";
    {
      const upsaleStr = formData.get("upsale");
      obj.upsale = upsaleStr ? parseFloat(upsaleStr as string) : 0;
      if (obj.upsale !== undefined && Number.isNaN(obj.upsale)) obj.upsale = 0;
    }
    obj.dateCreated = formData.get("dateCreated") ?? "";
    obj.appointment = formData.get("appointment") ?? "";
    {
      // Collect array items from indexed form fields
      const lastTechsItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("lastTechs." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("lastTechs." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("lastTechs." + idx + ".")) {
              const fieldName = key.slice(
                "lastTechs.".length + String(idx).length + 1,
              );
              item[fieldName] = value;
            }
          }
          lastTechsItems.push(item);
        }
        idx++;
        if (idx > 1000) break; // Safety limit
      }
      obj.lastTechs = lastTechsItems;
    }
    obj.package = formData.get("package") ?? "";
    obj.promotion = formData.get("promotion") ?? "";
    {
      const balanceStr = formData.get("balance");
      obj.balance = balanceStr ? parseFloat(balanceStr as string) : 0;
      if (obj.balance !== undefined && Number.isNaN(obj.balance))
        obj.balance = 0;
    }
    obj.due = formData.get("due") ?? "";
    {
      const totalStr = formData.get("total");
      obj.total = totalStr ? parseFloat(totalStr as string) : 0;
      if (obj.total !== undefined && Number.isNaN(obj.total)) obj.total = 0;
    }
    obj.site = formData.get("site") ?? "";
    {
      // Collect array items from indexed form fields
      const billedItemsItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("billedItems." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("billedItems." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("billedItems." + idx + ".")) {
              const fieldName = key.slice(
                "billedItems.".length + String(idx).length + 1,
              );
              item[fieldName] = value;
            }
          }
          billedItemsItems.push(item);
        }
        idx++;
        if (idx > 1000) break; // Safety limit
      }
      obj.billedItems = billedItemsItems;
    }
    obj.memo = formData.get("memo") ?? "";
    {
      const discountStr = formData.get("discount");
      obj.discount = discountStr ? parseFloat(discountStr as string) : 0;
      if (obj.discount !== undefined && Number.isNaN(obj.discount))
        obj.discount = 0;
    }
    {
      const tipStr = formData.get("tip");
      obj.tip = tipStr ? parseFloat(tipStr as string) : 0;
      if (obj.tip !== undefined && Number.isNaN(obj.tip)) obj.tip = 0;
    }
    obj.commissions = formData
      .getAll("commissions")
      .map((v) => parseFloat(v as string))
      .filter((n) => !Number.isNaN(n));
    return Order.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      account: {
        path: ["account"] as const,
        name: "account",
        constraints: { required: true },

        get: (data: Data) => data.account,
        set: (data: Data, value: string | Account) => {
          data.account = value;
        },
        getError: (errors: Errors) => errors?.account,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.account = value;
        },
        getTainted: (tainted: Tainted) => tainted?.account ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.account = value;
        },
        validate: (_value: string | Account): Array<string> => [],
      },
      stage: {
        path: ["stage"] as const,
        name: "stage",
        constraints: { required: true },

        get: (data: Data) => data.stage,
        set: (data: Data, value: OrderStage) => {
          data.stage = value;
        },
        getError: (errors: Errors) => errors?.stage,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.stage = value;
        },
        getTainted: (tainted: Tainted) => tainted?.stage ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.stage = value;
        },
        validate: (_value: OrderStage): Array<string> => [],
      },
      number: {
        path: ["number"] as const,
        name: "number",
        constraints: { required: true },

        get: (data: Data) => data.number,
        set: (data: Data, value: number) => {
          data.number = value;
        },
        getError: (errors: Errors) => errors?.number,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.number = value;
        },
        getTainted: (tainted: Tainted) => tainted?.number ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.number = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      payments: {
        path: ["payments"] as const,
        name: "payments",
        constraints: { required: true },

        get: (data: Data) => data.payments,
        set: (data: Data, value: (string | Payment)[]) => {
          data.payments = value;
        },
        getError: (errors: Errors) => errors?.payments,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.payments = value;
        },
        getTainted: (tainted: Tainted) => tainted?.payments ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.payments = value;
        },
        validate: (_value: (string | Payment)[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["payments"], index] as const,
          name: `payments.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.payments[index],
          set: (data: Data, value: string | Payment) => {
            data.payments[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.payments as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.payments ??= {};
            (errors.payments as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.payments?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.payments ??= {};
            tainted.payments[index] = value;
          },
          validate: (_value: string | Payment): Array<string> => [],
        }),
        push: (data: Data, item: string | Payment) => {
          data.payments.push(item);
        },
        remove: (data: Data, index: number) => {
          data.payments.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.payments[a], data.payments[b]] = [
            data.payments[b],
            data.payments[a],
          ];
        },
      },
      opportunity: {
        path: ["opportunity"] as const,
        name: "opportunity",
        constraints: { required: true },

        get: (data: Data) => data.opportunity,
        set: (data: Data, value: string) => {
          data.opportunity = value;
        },
        getError: (errors: Errors) => errors?.opportunity,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.opportunity = value;
        },
        getTainted: (tainted: Tainted) => tainted?.opportunity ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.opportunity = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      reference: {
        path: ["reference"] as const,
        name: "reference",
        constraints: { required: true },

        get: (data: Data) => data.reference,
        set: (data: Data, value: string) => {
          data.reference = value;
        },
        getError: (errors: Errors) => errors?.reference,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.reference = value;
        },
        getTainted: (tainted: Tainted) => tainted?.reference ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.reference = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      leadSource: {
        path: ["leadSource"] as const,
        name: "leadSource",
        constraints: { required: true },

        get: (data: Data) => data.leadSource,
        set: (data: Data, value: string) => {
          data.leadSource = value;
        },
        getError: (errors: Errors) => errors?.leadSource,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.leadSource = value;
        },
        getTainted: (tainted: Tainted) => tainted?.leadSource ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.leadSource = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      salesRep: {
        path: ["salesRep"] as const,
        name: "salesRep",
        constraints: { required: true },

        get: (data: Data) => data.salesRep,
        set: (data: Data, value: string | Employee) => {
          data.salesRep = value;
        },
        getError: (errors: Errors) => errors?.salesRep,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.salesRep = value;
        },
        getTainted: (tainted: Tainted) => tainted?.salesRep ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.salesRep = value;
        },
        validate: (_value: string | Employee): Array<string> => [],
      },
      group: {
        path: ["group"] as const,
        name: "group",
        constraints: { required: true },

        get: (data: Data) => data.group,
        set: (data: Data, value: string) => {
          data.group = value;
        },
        getError: (errors: Errors) => errors?.group,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.group = value;
        },
        getTainted: (tainted: Tainted) => tainted?.group ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.group = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      subgroup: {
        path: ["subgroup"] as const,
        name: "subgroup",
        constraints: { required: true },

        get: (data: Data) => data.subgroup,
        set: (data: Data, value: string) => {
          data.subgroup = value;
        },
        getError: (errors: Errors) => errors?.subgroup,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.subgroup = value;
        },
        getTainted: (tainted: Tainted) => tainted?.subgroup ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.subgroup = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      isPosted: {
        path: ["isPosted"] as const,
        name: "isPosted",
        constraints: { required: true },

        get: (data: Data) => data.isPosted,
        set: (data: Data, value: boolean) => {
          data.isPosted = value;
        },
        getError: (errors: Errors) => errors?.isPosted,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.isPosted = value;
        },
        getTainted: (tainted: Tainted) => tainted?.isPosted ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.isPosted = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      needsReview: {
        path: ["needsReview"] as const,
        name: "needsReview",
        constraints: { required: true },

        get: (data: Data) => data.needsReview,
        set: (data: Data, value: boolean) => {
          data.needsReview = value;
        },
        getError: (errors: Errors) => errors?.needsReview,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.needsReview = value;
        },
        getTainted: (tainted: Tainted) => tainted?.needsReview ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.needsReview = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      actionItem: {
        path: ["actionItem"] as const,
        name: "actionItem",
        constraints: { required: true },

        get: (data: Data) => data.actionItem,
        set: (data: Data, value: string) => {
          data.actionItem = value;
        },
        getError: (errors: Errors) => errors?.actionItem,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.actionItem = value;
        },
        getTainted: (tainted: Tainted) => tainted?.actionItem ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.actionItem = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      upsale: {
        path: ["upsale"] as const,
        name: "upsale",
        constraints: { required: true },

        get: (data: Data) => data.upsale,
        set: (data: Data, value: number) => {
          data.upsale = value;
        },
        getError: (errors: Errors) => errors?.upsale,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.upsale = value;
        },
        getTainted: (tainted: Tainted) => tainted?.upsale ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.upsale = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      dateCreated: {
        path: ["dateCreated"] as const,
        name: "dateCreated",
        constraints: { required: true },

        get: (data: Data) => data.dateCreated,
        set: (data: Data, value: string) => {
          data.dateCreated = value;
        },
        getError: (errors: Errors) => errors?.dateCreated,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.dateCreated = value;
        },
        getTainted: (tainted: Tainted) => tainted?.dateCreated ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.dateCreated = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      appointment: {
        path: ["appointment"] as const,
        name: "appointment",
        constraints: { required: true },

        get: (data: Data) => data.appointment,
        set: (data: Data, value: string | Appointment) => {
          data.appointment = value;
        },
        getError: (errors: Errors) => errors?.appointment,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.appointment = value;
        },
        getTainted: (tainted: Tainted) => tainted?.appointment ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.appointment = value;
        },
        validate: (_value: string | Appointment): Array<string> => [],
      },
      lastTechs: {
        path: ["lastTechs"] as const,
        name: "lastTechs",
        constraints: { required: true },

        get: (data: Data) => data.lastTechs,
        set: (data: Data, value: (string | Employee)[]) => {
          data.lastTechs = value;
        },
        getError: (errors: Errors) => errors?.lastTechs,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.lastTechs = value;
        },
        getTainted: (tainted: Tainted) => tainted?.lastTechs ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.lastTechs = value;
        },
        validate: (_value: (string | Employee)[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["lastTechs"], index] as const,
          name: `lastTechs.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.lastTechs[index],
          set: (data: Data, value: string | Employee) => {
            data.lastTechs[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.lastTechs as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.lastTechs ??= {};
            (errors.lastTechs as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.lastTechs?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.lastTechs ??= {};
            tainted.lastTechs[index] = value;
          },
          validate: (_value: string | Employee): Array<string> => [],
        }),
        push: (data: Data, item: string | Employee) => {
          data.lastTechs.push(item);
        },
        remove: (data: Data, index: number) => {
          data.lastTechs.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.lastTechs[a], data.lastTechs[b]] = [
            data.lastTechs[b],
            data.lastTechs[a],
          ];
        },
      },
      package: {
        path: ["package"] as const,
        name: "package",
        constraints: { required: true },

        get: (data: Data) => data.package,
        set: (data: Data, value: (string | Package)[] | null) => {
          data.package = value;
        },
        getError: (errors: Errors) => errors?.package,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.package = value;
        },
        getTainted: (tainted: Tainted) => tainted?.package ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.package = value;
        },
        validate: (_value: (string | Package)[] | null): Array<string> => [],
      },
      promotion: {
        path: ["promotion"] as const,
        name: "promotion",
        constraints: { required: true },

        get: (data: Data) => data.promotion,
        set: (data: Data, value: (string | Promotion)[] | null) => {
          data.promotion = value;
        },
        getError: (errors: Errors) => errors?.promotion,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.promotion = value;
        },
        getTainted: (tainted: Tainted) => tainted?.promotion ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.promotion = value;
        },
        validate: (_value: (string | Promotion)[] | null): Array<string> => [],
      },
      balance: {
        path: ["balance"] as const,
        name: "balance",
        constraints: { required: true },

        get: (data: Data) => data.balance,
        set: (data: Data, value: number) => {
          data.balance = value;
        },
        getError: (errors: Errors) => errors?.balance,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.balance = value;
        },
        getTainted: (tainted: Tainted) => tainted?.balance ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.balance = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      due: {
        path: ["due"] as const,
        name: "due",
        constraints: { required: true },

        get: (data: Data) => data.due,
        set: (data: Data, value: string) => {
          data.due = value;
        },
        getError: (errors: Errors) => errors?.due,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.due = value;
        },
        getTainted: (tainted: Tainted) => tainted?.due ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.due = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      total: {
        path: ["total"] as const,
        name: "total",
        constraints: { required: true },

        get: (data: Data) => data.total,
        set: (data: Data, value: number) => {
          data.total = value;
        },
        getError: (errors: Errors) => errors?.total,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.total = value;
        },
        getTainted: (tainted: Tainted) => tainted?.total ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.total = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      site: {
        path: ["site"] as const,
        name: "site",
        constraints: { required: true },

        get: (data: Data) => data.site,
        set: (data: Data, value: string | Site) => {
          data.site = value;
        },
        getError: (errors: Errors) => errors?.site,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.site = value;
        },
        getTainted: (tainted: Tainted) => tainted?.site ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.site = value;
        },
        validate: (_value: string | Site): Array<string> => [],
      },
      billedItems: {
        path: ["billedItems"] as const,
        name: "billedItems",
        constraints: { required: true },

        get: (data: Data) => data.billedItems,
        set: (data: Data, value: BilledItem[]) => {
          data.billedItems = value;
        },
        getError: (errors: Errors) => errors?.billedItems,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.billedItems = value;
        },
        getTainted: (tainted: Tainted) => tainted?.billedItems ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.billedItems = value;
        },
        validate: (_value: BilledItem[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["billedItems"], index] as const,
          name: `billedItems.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.billedItems[index],
          set: (data: Data, value: BilledItem) => {
            data.billedItems[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.billedItems as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.billedItems ??= {};
            (errors.billedItems as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: (tainted: Tainted) =>
            tainted.billedItems?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.billedItems ??= {};
            tainted.billedItems[index] = value;
          },
          validate: (_value: BilledItem): Array<string> => [],
        }),
        push: (data: Data, item: BilledItem) => {
          data.billedItems.push(item);
        },
        remove: (data: Data, index: number) => {
          data.billedItems.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.billedItems[a], data.billedItems[b]] = [
            data.billedItems[b],
            data.billedItems[a],
          ];
        },
      },
      memo: {
        path: ["memo"] as const,
        name: "memo",
        constraints: { required: true },

        get: (data: Data) => data.memo,
        set: (data: Data, value: string) => {
          data.memo = value;
        },
        getError: (errors: Errors) => errors?.memo,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.memo = value;
        },
        getTainted: (tainted: Tainted) => tainted?.memo ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.memo = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      discount: {
        path: ["discount"] as const,
        name: "discount",
        constraints: { required: true },

        get: (data: Data) => data.discount,
        set: (data: Data, value: number) => {
          data.discount = value;
        },
        getError: (errors: Errors) => errors?.discount,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.discount = value;
        },
        getTainted: (tainted: Tainted) => tainted?.discount ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.discount = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      tip: {
        path: ["tip"] as const,
        name: "tip",
        constraints: { required: true },

        get: (data: Data) => data.tip,
        set: (data: Data, value: number) => {
          data.tip = value;
        },
        getError: (errors: Errors) => errors?.tip,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.tip = value;
        },
        getTainted: (tainted: Tainted) => tainted?.tip ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.tip = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      commissions: {
        path: ["commissions"] as const,
        name: "commissions",
        constraints: { required: true },

        get: (data: Data) => data.commissions,
        set: (data: Data, value: number[]) => {
          data.commissions = value;
        },
        getError: (errors: Errors) => errors?.commissions,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.commissions = value;
        },
        getTainted: (tainted: Tainted) => tainted?.commissions ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.commissions = value;
        },
        validate: (_value: number[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["commissions"], index] as const,
          name: `commissions.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.commissions[index],
          set: (data: Data, value: number) => {
            data.commissions[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.commissions as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.commissions ??= {};
            (errors.commissions as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: (tainted: Tainted) =>
            tainted.commissions?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.commissions ??= {};
            tainted.commissions[index] = value;
          },
          validate: (_value: number): Array<string> => [],
        }),
        push: (data: Data, item: number) => {
          data.commissions.push(item);
        },
        remove: (data: Data, index: number) => {
          data.commissions.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.commissions[a], data.commissions[b]] = [
            data.commissions[b],
            data.commissions[a],
          ];
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: Commented): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Commented, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Commented.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Commented;
  }
}

export namespace Commented {
  export type Data = Commented;
  export type Errors = {
    _errors?: Array<string>;
    comment?: Array<string>;
    replyTo?: Array<string>;
  };
  export type Tainted = { comment?: boolean; replyTo?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.comment = formData.get("comment") ?? "";
    obj.replyTo = formData.get("replyTo") ?? "";
    return Commented.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      comment: {
        path: ["comment"] as const,
        name: "comment",
        constraints: { required: true },

        get: (data: Data) => data.comment,
        set: (data: Data, value: string) => {
          data.comment = value;
        },
        getError: (errors: Errors) => errors?.comment,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.comment = value;
        },
        getTainted: (tainted: Tainted) => tainted?.comment ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.comment = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      replyTo: {
        path: ["replyTo"] as const,
        name: "replyTo",
        constraints: { required: true },

        get: (data: Data) => data.replyTo,
        set: (data: Data, value: string | null) => {
          data.replyTo = value;
        },
        getError: (errors: Errors) => errors?.replyTo,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.replyTo = value;
        },
        getTainted: (tainted: Tainted) => tainted?.replyTo ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.replyTo = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Custom): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Custom, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Custom.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Custom;
  }
}

export namespace Custom {
  export type Data = Custom;
  export type Errors = {
    _errors?: Array<string>;
    mappings?: {
      _errors?: Array<string>;
      [index: number]: DirectionHue.Errors;
    };
  };
  export type Tainted = {
    mappings?: { [index: number]: DirectionHue.Tainted };
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      // Collect array items from indexed form fields
      const mappingsItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("mappings." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("mappings." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("mappings." + idx + ".")) {
              const fieldName = key.slice(
                "mappings.".length + String(idx).length + 1,
              );
              item[fieldName] = value;
            }
          }
          mappingsItems.push(item);
        }
        idx++;
        if (idx > 1000) break; // Safety limit
      }
      obj.mappings = mappingsItems;
    }
    return Custom.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      mappings: {
        path: ["mappings"] as const,
        name: "mappings",
        constraints: { required: true },

        get: (data: Data) => data.mappings,
        set: (data: Data, value: DirectionHue[]) => {
          data.mappings = value;
        },
        getError: (errors: Errors) => errors?.mappings,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.mappings = value;
        },
        getTainted: (tainted: Tainted) => tainted?.mappings ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.mappings = value;
        },
        validate: (_value: DirectionHue[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["mappings"], index] as const,
          name: `mappings.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.mappings[index],
          set: (data: Data, value: DirectionHue) => {
            data.mappings[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.mappings as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.mappings ??= {};
            (errors.mappings as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.mappings?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.mappings ??= {};
            tainted.mappings[index] = value;
          },
          validate: (_value: DirectionHue): Array<string> => [],
        }),
        push: (data: Data, item: DirectionHue) => {
          data.mappings.push(item);
        },
        remove: (data: Data, index: number) => {
          data.mappings.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.mappings[a], data.mappings[b]] = [
            data.mappings[b],
            data.mappings[a],
          ];
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: Colors): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Colors, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Colors.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Colors;
  }
}

export namespace Colors {
  export type Data = Colors;
  export type Errors = {
    _errors?: Array<string>;
    main?: Array<string>;
    hover?: Array<string>;
    active?: Array<string>;
  };
  export type Tainted = { main?: boolean; hover?: boolean; active?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.main = formData.get("main") ?? "";
    obj.hover = formData.get("hover") ?? "";
    obj.active = formData.get("active") ?? "";
    return Colors.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      main: {
        path: ["main"] as const,
        name: "main",
        constraints: { required: true },

        get: (data: Data) => data.main,
        set: (data: Data, value: string) => {
          data.main = value;
        },
        getError: (errors: Errors) => errors?.main,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.main = value;
        },
        getTainted: (tainted: Tainted) => tainted?.main ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.main = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      hover: {
        path: ["hover"] as const,
        name: "hover",
        constraints: { required: true },

        get: (data: Data) => data.hover,
        set: (data: Data, value: string) => {
          data.hover = value;
        },
        getError: (errors: Errors) => errors?.hover,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.hover = value;
        },
        getTainted: (tainted: Tainted) => tainted?.hover ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.hover = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      active: {
        path: ["active"] as const,
        name: "active",
        constraints: { required: true },

        get: (data: Data) => data.active,
        set: (data: Data, value: string) => {
          data.active = value;
        },
        getError: (errors: Errors) => errors?.active,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.active = value;
        },
        getTainted: (tainted: Tainted) => tainted?.active ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.active = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: ProductDefaults): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<ProductDefaults, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "ProductDefaults.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as ProductDefaults;
  }
}

export namespace ProductDefaults {
  export type Data = ProductDefaults;
  export type Errors = {
    _errors?: Array<string>;
    price?: Array<string>;
    description?: Array<string>;
  };
  export type Tainted = { price?: boolean; description?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const priceStr = formData.get("price");
      obj.price = priceStr ? parseFloat(priceStr as string) : 0;
      if (obj.price !== undefined && Number.isNaN(obj.price)) obj.price = 0;
    }
    obj.description = formData.get("description") ?? "";
    return ProductDefaults.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      price: {
        path: ["price"] as const,
        name: "price",
        constraints: { required: true },

        get: (data: Data) => data.price,
        set: (data: Data, value: number) => {
          data.price = value;
        },
        getError: (errors: Errors) => errors?.price,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.price = value;
        },
        getTainted: (tainted: Tainted) => tainted?.price ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.price = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      description: {
        path: ["description"] as const,
        name: "description",
        constraints: { required: true },

        get: (data: Data) => data.description,
        set: (data: Data, value: string) => {
          data.description = value;
        },
        getError: (errors: Errors) => errors?.description,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.description = value;
        },
        getTainted: (tainted: Tainted) => tainted?.description ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.description = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: Viewed): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Viewed, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Viewed.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Viewed;
  }
}

export namespace Viewed {
  export type Data = Viewed;
  export type Errors = {
    _errors?: Array<string>;
    durationSeconds?: Array<string>;
    source?: Array<string>;
  };
  export type Tainted = { durationSeconds?: boolean; source?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const durationSecondsStr = formData.get("durationSeconds");
      obj.durationSeconds = durationSecondsStr
        ? parseFloat(durationSecondsStr as string)
        : 0;
      if (
        obj.durationSeconds !== undefined &&
        Number.isNaN(obj.durationSeconds)
      )
        obj.durationSeconds = 0;
    }
    obj.source = formData.get("source") ?? "";
    return Viewed.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      durationSeconds: {
        path: ["durationSeconds"] as const,
        name: "durationSeconds",
        constraints: { required: true },

        get: (data: Data) => data.durationSeconds,
        set: (data: Data, value: number | null) => {
          data.durationSeconds = value;
        },
        getError: (errors: Errors) => errors?.durationSeconds,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.durationSeconds = value;
        },
        getTainted: (tainted: Tainted) => tainted?.durationSeconds ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.durationSeconds = value;
        },
        validate: (_value: number | null): Array<string> => [],
      },
      source: {
        path: ["source"] as const,
        name: "source",
        constraints: { required: true },

        get: (data: Data) => data.source,
        set: (data: Data, value: string | null) => {
          data.source = value;
        },
        getError: (errors: Errors) => errors?.source,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.source = value;
        },
        getTainted: (tainted: Tainted) => tainted?.source ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.source = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: WeeklyRecurrenceRule): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<WeeklyRecurrenceRule, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "WeeklyRecurrenceRule.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as WeeklyRecurrenceRule;
  }
}

export namespace WeeklyRecurrenceRule {
  export type Data = WeeklyRecurrenceRule;
  export type Errors = {
    _errors?: Array<string>;
    quantityOfWeeks?: Array<string>;
    weekdays?: { _errors?: Array<string>; [index: number]: Weekday.Errors };
  };
  export type Tainted = {
    quantityOfWeeks?: boolean;
    weekdays?: { [index: number]: Weekday.Tainted };
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const quantityOfWeeksStr = formData.get("quantityOfWeeks");
      obj.quantityOfWeeks = quantityOfWeeksStr
        ? parseFloat(quantityOfWeeksStr as string)
        : 0;
      if (
        obj.quantityOfWeeks !== undefined &&
        Number.isNaN(obj.quantityOfWeeks)
      )
        obj.quantityOfWeeks = 0;
    }
    {
      // Collect array items from indexed form fields
      const weekdaysItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("weekdays." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("weekdays." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("weekdays." + idx + ".")) {
              const fieldName = key.slice(
                "weekdays.".length + String(idx).length + 1,
              );
              item[fieldName] = value;
            }
          }
          weekdaysItems.push(item);
        }
        idx++;
        if (idx > 1000) break; // Safety limit
      }
      obj.weekdays = weekdaysItems;
    }
    return WeeklyRecurrenceRule.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      quantityOfWeeks: {
        path: ["quantityOfWeeks"] as const,
        name: "quantityOfWeeks",
        constraints: { required: true },

        get: (data: Data) => data.quantityOfWeeks,
        set: (data: Data, value: number) => {
          data.quantityOfWeeks = value;
        },
        getError: (errors: Errors) => errors?.quantityOfWeeks,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.quantityOfWeeks = value;
        },
        getTainted: (tainted: Tainted) => tainted?.quantityOfWeeks ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.quantityOfWeeks = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      weekdays: {
        path: ["weekdays"] as const,
        name: "weekdays",
        constraints: { required: true },

        get: (data: Data) => data.weekdays,
        set: (data: Data, value: Weekday[]) => {
          data.weekdays = value;
        },
        getError: (errors: Errors) => errors?.weekdays,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.weekdays = value;
        },
        getTainted: (tainted: Tainted) => tainted?.weekdays ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.weekdays = value;
        },
        validate: (_value: Weekday[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["weekdays"], index] as const,
          name: `weekdays.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.weekdays[index],
          set: (data: Data, value: Weekday) => {
            data.weekdays[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.weekdays as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.weekdays ??= {};
            (errors.weekdays as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.weekdays?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.weekdays ??= {};
            tainted.weekdays[index] = value;
          },
          validate: (_value: Weekday): Array<string> => [],
        }),
        push: (data: Data, item: Weekday) => {
          data.weekdays.push(item);
        },
        remove: (data: Data, index: number) => {
          data.weekdays.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.weekdays[a], data.weekdays[b]] = [
            data.weekdays[b],
            data.weekdays[a],
          ];
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: Paid): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Paid, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Paid.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Paid;
  }
}

export namespace Paid {
  export type Data = Paid;
  export type Errors = {
    _errors?: Array<string>;
    amount?: Array<string>;
    currency?: Array<string>;
    paymentMethod?: Array<string>;
  };
  export type Tainted = {
    amount?: boolean;
    currency?: boolean;
    paymentMethod?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const amountStr = formData.get("amount");
      obj.amount = amountStr ? parseFloat(amountStr as string) : 0;
      if (obj.amount !== undefined && Number.isNaN(obj.amount)) obj.amount = 0;
    }
    obj.currency = formData.get("currency") ?? "";
    obj.paymentMethod = formData.get("paymentMethod") ?? "";
    return Paid.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      amount: {
        path: ["amount"] as const,
        name: "amount",
        constraints: { required: true },

        get: (data: Data) => data.amount,
        set: (data: Data, value: number | null) => {
          data.amount = value;
        },
        getError: (errors: Errors) => errors?.amount,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.amount = value;
        },
        getTainted: (tainted: Tainted) => tainted?.amount ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.amount = value;
        },
        validate: (_value: number | null): Array<string> => [],
      },
      currency: {
        path: ["currency"] as const,
        name: "currency",
        constraints: { required: true },

        get: (data: Data) => data.currency,
        set: (data: Data, value: string | null) => {
          data.currency = value;
        },
        getError: (errors: Errors) => errors?.currency,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.currency = value;
        },
        getTainted: (tainted: Tainted) => tainted?.currency ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.currency = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      paymentMethod: {
        path: ["paymentMethod"] as const,
        name: "paymentMethod",
        constraints: { required: true },

        get: (data: Data) => data.paymentMethod,
        set: (data: Data, value: string | null) => {
          data.paymentMethod = value;
        },
        getError: (errors: Errors) => errors?.paymentMethod,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.paymentMethod = value;
        },
        getTainted: (tainted: Tainted) => tainted?.paymentMethod ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.paymentMethod = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: TaxRate): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<TaxRate, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "TaxRate.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as TaxRate;
  }
}

export namespace TaxRate {
  export type Data = TaxRate;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    name?: Array<string>;
    taxAgency?: Array<string>;
    zip?: Array<string>;
    city?: Array<string>;
    county?: Array<string>;
    state?: Array<string>;
    isActive?: Array<string>;
    description?: Array<string>;
    taxComponents?: Array<string>;
  };
  export type Tainted = {
    id?: boolean;
    name?: boolean;
    taxAgency?: boolean;
    zip?: boolean;
    city?: boolean;
    county?: boolean;
    state?: boolean;
    isActive?: boolean;
    description?: boolean;
    taxComponents?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.name = formData.get("name") ?? "";
    obj.taxAgency = formData.get("taxAgency") ?? "";
    {
      const zipStr = formData.get("zip");
      obj.zip = zipStr ? parseFloat(zipStr as string) : 0;
      if (obj.zip !== undefined && Number.isNaN(obj.zip)) obj.zip = 0;
    }
    obj.city = formData.get("city") ?? "";
    obj.county = formData.get("county") ?? "";
    obj.state = formData.get("state") ?? "";
    {
      const isActiveVal = formData.get("isActive");
      obj.isActive =
        isActiveVal === "true" || isActiveVal === "on" || isActiveVal === "1";
    }
    obj.description = formData.get("description") ?? "";
    obj.taxComponents = formData.get("taxComponents") ?? "";
    return TaxRate.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: (data: Data) => data.name,
        set: (data: Data, value: string) => {
          data.name = value;
        },
        getError: (errors: Errors) => errors?.name,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: (tainted: Tainted) => tainted?.name ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.name = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      taxAgency: {
        path: ["taxAgency"] as const,
        name: "taxAgency",
        constraints: { required: true },

        get: (data: Data) => data.taxAgency,
        set: (data: Data, value: string) => {
          data.taxAgency = value;
        },
        getError: (errors: Errors) => errors?.taxAgency,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.taxAgency = value;
        },
        getTainted: (tainted: Tainted) => tainted?.taxAgency ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.taxAgency = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      zip: {
        path: ["zip"] as const,
        name: "zip",
        constraints: { required: true },

        get: (data: Data) => data.zip,
        set: (data: Data, value: number) => {
          data.zip = value;
        },
        getError: (errors: Errors) => errors?.zip,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.zip = value;
        },
        getTainted: (tainted: Tainted) => tainted?.zip ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.zip = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      city: {
        path: ["city"] as const,
        name: "city",
        constraints: { required: true },

        get: (data: Data) => data.city,
        set: (data: Data, value: string) => {
          data.city = value;
        },
        getError: (errors: Errors) => errors?.city,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.city = value;
        },
        getTainted: (tainted: Tainted) => tainted?.city ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.city = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      county: {
        path: ["county"] as const,
        name: "county",
        constraints: { required: true },

        get: (data: Data) => data.county,
        set: (data: Data, value: string) => {
          data.county = value;
        },
        getError: (errors: Errors) => errors?.county,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.county = value;
        },
        getTainted: (tainted: Tainted) => tainted?.county ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.county = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      state: {
        path: ["state"] as const,
        name: "state",
        constraints: { required: true },

        get: (data: Data) => data.state,
        set: (data: Data, value: string) => {
          data.state = value;
        },
        getError: (errors: Errors) => errors?.state,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.state = value;
        },
        getTainted: (tainted: Tainted) => tainted?.state ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.state = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      isActive: {
        path: ["isActive"] as const,
        name: "isActive",
        constraints: { required: true },

        get: (data: Data) => data.isActive,
        set: (data: Data, value: boolean) => {
          data.isActive = value;
        },
        getError: (errors: Errors) => errors?.isActive,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.isActive = value;
        },
        getTainted: (tainted: Tainted) => tainted?.isActive ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.isActive = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      description: {
        path: ["description"] as const,
        name: "description",
        constraints: { required: true },

        get: (data: Data) => data.description,
        set: (data: Data, value: string) => {
          data.description = value;
        },
        getError: (errors: Errors) => errors?.description,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.description = value;
        },
        getTainted: (tainted: Tainted) => tainted?.description ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.description = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      taxComponents: {
        path: ["taxComponents"] as const,
        name: "taxComponents",
        constraints: { required: true },

        get: (data: Data) => data.taxComponents,
        set: (data: Data, value: { [key: string]: number }) => {
          data.taxComponents = value;
        },
        getError: (errors: Errors) => errors?.taxComponents,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.taxComponents = value;
        },
        getTainted: (tainted: Tainted) => tainted?.taxComponents ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.taxComponents = value;
        },
        validate: (_value: { [key: string]: number }): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Address): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Address, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Address.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Address;
  }
}

export namespace Address {
  export type Data = Address;
  export type Errors = {
    _errors?: Array<string>;
    street?: Array<string>;
    city?: Array<string>;
    state?: Array<string>;
    zipcode?: Array<string>;
  };
  export type Tainted = {
    street?: boolean;
    city?: boolean;
    state?: boolean;
    zipcode?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.street = formData.get("street") ?? "";
    obj.city = formData.get("city") ?? "";
    obj.state = formData.get("state") ?? "";
    obj.zipcode = formData.get("zipcode") ?? "";
    return Address.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      street: {
        path: ["street"] as const,
        name: "street",
        constraints: { required: true },

        get: (data: Data) => data.street,
        set: (data: Data, value: string) => {
          data.street = value;
        },
        getError: (errors: Errors) => errors?.street,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.street = value;
        },
        getTainted: (tainted: Tainted) => tainted?.street ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.street = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      city: {
        path: ["city"] as const,
        name: "city",
        constraints: { required: true },

        get: (data: Data) => data.city,
        set: (data: Data, value: string) => {
          data.city = value;
        },
        getError: (errors: Errors) => errors?.city,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.city = value;
        },
        getTainted: (tainted: Tainted) => tainted?.city ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.city = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      state: {
        path: ["state"] as const,
        name: "state",
        constraints: { required: true },

        get: (data: Data) => data.state,
        set: (data: Data, value: string) => {
          data.state = value;
        },
        getError: (errors: Errors) => errors?.state,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.state = value;
        },
        getTainted: (tainted: Tainted) => tainted?.state ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.state = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      zipcode: {
        path: ["zipcode"] as const,
        name: "zipcode",
        constraints: { required: true },

        get: (data: Data) => data.zipcode,
        set: (data: Data, value: string) => {
          data.zipcode = value;
        },
        getError: (errors: Errors) => errors?.zipcode,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.zipcode = value;
        },
        getTainted: (tainted: Tainted) => tainted?.zipcode ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.zipcode = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: Lead): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Lead, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Lead.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Lead;
  }
}

export namespace Lead {
  export type Data = Lead;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    number?: Array<string>;
    accepted?: Array<string>;
    probability?: Array<string>;
    priority?: Priority.Errors;
    dueDate?: Array<string>;
    closeDate?: Array<string>;
    value?: Array<string>;
    stage?: LeadStage.Errors;
    status?: Array<string>;
    description?: Array<string>;
    nextStep?: NextStep.Errors;
    favorite?: Array<string>;
    dateAdded?: Array<string>;
    taxRate?: Array<string>;
    sector?: Sector.Errors;
    leadName?: AccountName.Errors;
    phones?: { _errors?: Array<string>; [index: number]: PhoneNumber.Errors };
    email?: Email.Errors;
    leadSource?: Array<string>;
    site?: Array<string>;
    memo?: Array<string>;
    needsReview?: Array<string>;
    hasAlert?: Array<string>;
    salesRep?: Array<string>;
    color?: Array<string>;
    accountType?: Array<string>;
    subtype?: Array<string>;
    isTaxExempt?: Array<string>;
    paymentTerms?: Array<string>;
    tags?: { _errors?: Array<string>; [index: number]: Array<string> };
    customFields?: { _errors?: Array<string>; [index: number]: Array<string> };
  };
  export type Tainted = {
    id?: boolean;
    number?: boolean;
    accepted?: boolean;
    probability?: boolean;
    priority?: Priority.Tainted;
    dueDate?: boolean;
    closeDate?: boolean;
    value?: boolean;
    stage?: LeadStage.Tainted;
    status?: boolean;
    description?: boolean;
    nextStep?: NextStep.Tainted;
    favorite?: boolean;
    dateAdded?: boolean;
    taxRate?: boolean;
    sector?: Sector.Tainted;
    leadName?: AccountName.Tainted;
    phones?: { [index: number]: PhoneNumber.Tainted };
    email?: Email.Tainted;
    leadSource?: boolean;
    site?: boolean;
    memo?: boolean;
    needsReview?: boolean;
    hasAlert?: boolean;
    salesRep?: boolean;
    color?: boolean;
    accountType?: boolean;
    subtype?: boolean;
    isTaxExempt?: boolean;
    paymentTerms?: boolean;
    tags?: { [index: number]: boolean };
    customFields?: { [index: number]: boolean };
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    {
      const numberStr = formData.get("number");
      obj.number = numberStr ? parseFloat(numberStr as string) : 0;
      if (obj.number !== undefined && Number.isNaN(obj.number)) obj.number = 0;
    }
    {
      const acceptedVal = formData.get("accepted");
      obj.accepted =
        acceptedVal === "true" || acceptedVal === "on" || acceptedVal === "1";
    }
    {
      const probabilityStr = formData.get("probability");
      obj.probability = probabilityStr
        ? parseFloat(probabilityStr as string)
        : 0;
      if (obj.probability !== undefined && Number.isNaN(obj.probability))
        obj.probability = 0;
    }
    {
      // Collect nested object fields with prefix "priority."
      const priorityObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("priority.")) {
          const fieldName = key.slice("priority.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = priorityObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.priority = priorityObj;
    }
    obj.dueDate = formData.get("dueDate") ?? "";
    obj.closeDate = formData.get("closeDate") ?? "";
    {
      const valueStr = formData.get("value");
      obj.value = valueStr ? parseFloat(valueStr as string) : 0;
      if (obj.value !== undefined && Number.isNaN(obj.value)) obj.value = 0;
    }
    {
      // Collect nested object fields with prefix "stage."
      const stageObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("stage.")) {
          const fieldName = key.slice("stage.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = stageObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.stage = stageObj;
    }
    obj.status = formData.get("status") ?? "";
    obj.description = formData.get("description") ?? "";
    {
      // Collect nested object fields with prefix "nextStep."
      const nextStepObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("nextStep.")) {
          const fieldName = key.slice("nextStep.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = nextStepObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.nextStep = nextStepObj;
    }
    {
      const favoriteVal = formData.get("favorite");
      obj.favorite =
        favoriteVal === "true" || favoriteVal === "on" || favoriteVal === "1";
    }
    obj.dateAdded = formData.get("dateAdded") ?? "";
    obj.taxRate = formData.get("taxRate") ?? "";
    {
      // Collect nested object fields with prefix "sector."
      const sectorObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("sector.")) {
          const fieldName = key.slice("sector.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = sectorObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.sector = sectorObj;
    }
    {
      // Collect nested object fields with prefix "leadName."
      const leadNameObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("leadName.")) {
          const fieldName = key.slice("leadName.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = leadNameObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.leadName = leadNameObj;
    }
    {
      // Collect array items from indexed form fields
      const phonesItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("phones." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("phones." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("phones." + idx + ".")) {
              const fieldName = key.slice(
                "phones.".length + String(idx).length + 1,
              );
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
    {
      // Collect nested object fields with prefix "email."
      const emailObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("email.")) {
          const fieldName = key.slice("email.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = emailObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.email = emailObj;
    }
    obj.leadSource = formData.get("leadSource") ?? "";
    obj.site = formData.get("site") ?? "";
    obj.memo = formData.get("memo") ?? "";
    {
      const needsReviewVal = formData.get("needsReview");
      obj.needsReview =
        needsReviewVal === "true" ||
        needsReviewVal === "on" ||
        needsReviewVal === "1";
    }
    {
      const hasAlertVal = formData.get("hasAlert");
      obj.hasAlert =
        hasAlertVal === "true" || hasAlertVal === "on" || hasAlertVal === "1";
    }
    obj.salesRep = formData.get("salesRep") ?? "";
    obj.color = formData.get("color") ?? "";
    obj.accountType = formData.get("accountType") ?? "";
    obj.subtype = formData.get("subtype") ?? "";
    {
      const isTaxExemptVal = formData.get("isTaxExempt");
      obj.isTaxExempt =
        isTaxExemptVal === "true" ||
        isTaxExemptVal === "on" ||
        isTaxExemptVal === "1";
    }
    obj.paymentTerms = formData.get("paymentTerms") ?? "";
    obj.tags = formData.getAll("tags") as Array<string>;
    {
      // Collect array items from indexed form fields
      const customFieldsItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("customFields." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("customFields." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("customFields." + idx + ".")) {
              const fieldName = key.slice(
                "customFields.".length + String(idx).length + 1,
              );
              item[fieldName] = value;
            }
          }
          customFieldsItems.push(item);
        }
        idx++;
        if (idx > 1000) break; // Safety limit
      }
      obj.customFields = customFieldsItems;
    }
    return Lead.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      number: {
        path: ["number"] as const,
        name: "number",
        constraints: { required: true },

        get: (data: Data) => data.number,
        set: (data: Data, value: number | null) => {
          data.number = value;
        },
        getError: (errors: Errors) => errors?.number,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.number = value;
        },
        getTainted: (tainted: Tainted) => tainted?.number ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.number = value;
        },
        validate: (_value: number | null): Array<string> => [],
      },
      accepted: {
        path: ["accepted"] as const,
        name: "accepted",
        constraints: { required: true },

        get: (data: Data) => data.accepted,
        set: (data: Data, value: boolean) => {
          data.accepted = value;
        },
        getError: (errors: Errors) => errors?.accepted,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.accepted = value;
        },
        getTainted: (tainted: Tainted) => tainted?.accepted ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.accepted = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      probability: {
        path: ["probability"] as const,
        name: "probability",
        constraints: { required: true },

        get: (data: Data) => data.probability,
        set: (data: Data, value: number) => {
          data.probability = value;
        },
        getError: (errors: Errors) => errors?.probability,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.probability = value;
        },
        getTainted: (tainted: Tainted) => tainted?.probability ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.probability = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      priority: {
        path: ["priority"] as const,
        name: "priority",
        constraints: { required: true },

        get: (data: Data) => data.priority,
        set: (data: Data, value: Priority) => {
          data.priority = value;
        },
        getError: (errors: Errors) => errors?.priority,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.priority = value;
        },
        getTainted: (tainted: Tainted) => tainted?.priority ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.priority = value;
        },
        validate: (_value: Priority): Array<string> => [],
      },
      dueDate: {
        path: ["dueDate"] as const,
        name: "dueDate",
        constraints: { required: true },

        get: (data: Data) => data.dueDate,
        set: (data: Data, value: string | null) => {
          data.dueDate = value;
        },
        getError: (errors: Errors) => errors?.dueDate,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.dueDate = value;
        },
        getTainted: (tainted: Tainted) => tainted?.dueDate ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.dueDate = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      closeDate: {
        path: ["closeDate"] as const,
        name: "closeDate",
        constraints: { required: true },

        get: (data: Data) => data.closeDate,
        set: (data: Data, value: string | null) => {
          data.closeDate = value;
        },
        getError: (errors: Errors) => errors?.closeDate,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.closeDate = value;
        },
        getTainted: (tainted: Tainted) => tainted?.closeDate ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.closeDate = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      value: {
        path: ["value"] as const,
        name: "value",
        constraints: { required: true },

        get: (data: Data) => data.value,
        set: (data: Data, value: number) => {
          data.value = value;
        },
        getError: (errors: Errors) => errors?.value,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.value = value;
        },
        getTainted: (tainted: Tainted) => tainted?.value ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.value = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      stage: {
        path: ["stage"] as const,
        name: "stage",
        constraints: { required: true },

        get: (data: Data) => data.stage,
        set: (data: Data, value: LeadStage) => {
          data.stage = value;
        },
        getError: (errors: Errors) => errors?.stage,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.stage = value;
        },
        getTainted: (tainted: Tainted) => tainted?.stage ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.stage = value;
        },
        validate: (_value: LeadStage): Array<string> => [],
      },
      status: {
        path: ["status"] as const,
        name: "status",
        constraints: { required: true },

        get: (data: Data) => data.status,
        set: (data: Data, value: string) => {
          data.status = value;
        },
        getError: (errors: Errors) => errors?.status,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.status = value;
        },
        getTainted: (tainted: Tainted) => tainted?.status ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.status = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      description: {
        path: ["description"] as const,
        name: "description",
        constraints: { required: true },

        get: (data: Data) => data.description,
        set: (data: Data, value: string | null) => {
          data.description = value;
        },
        getError: (errors: Errors) => errors?.description,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.description = value;
        },
        getTainted: (tainted: Tainted) => tainted?.description ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.description = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      nextStep: {
        path: ["nextStep"] as const,
        name: "nextStep",
        constraints: { required: true },

        get: (data: Data) => data.nextStep,
        set: (data: Data, value: NextStep) => {
          data.nextStep = value;
        },
        getError: (errors: Errors) => errors?.nextStep,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.nextStep = value;
        },
        getTainted: (tainted: Tainted) => tainted?.nextStep ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.nextStep = value;
        },
        validate: (_value: NextStep): Array<string> => [],
      },
      favorite: {
        path: ["favorite"] as const,
        name: "favorite",
        constraints: { required: true },

        get: (data: Data) => data.favorite,
        set: (data: Data, value: boolean) => {
          data.favorite = value;
        },
        getError: (errors: Errors) => errors?.favorite,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.favorite = value;
        },
        getTainted: (tainted: Tainted) => tainted?.favorite ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.favorite = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      dateAdded: {
        path: ["dateAdded"] as const,
        name: "dateAdded",
        constraints: { required: true },

        get: (data: Data) => data.dateAdded,
        set: (data: Data, value: string | null) => {
          data.dateAdded = value;
        },
        getError: (errors: Errors) => errors?.dateAdded,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.dateAdded = value;
        },
        getTainted: (tainted: Tainted) => tainted?.dateAdded ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.dateAdded = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      taxRate: {
        path: ["taxRate"] as const,
        name: "taxRate",
        constraints: { required: true },

        get: (data: Data) => data.taxRate,
        set: (data: Data, value: (string | TaxRate) | null) => {
          data.taxRate = value;
        },
        getError: (errors: Errors) => errors?.taxRate,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.taxRate = value;
        },
        getTainted: (tainted: Tainted) => tainted?.taxRate ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.taxRate = value;
        },
        validate: (_value: (string | TaxRate) | null): Array<string> => [],
      },
      sector: {
        path: ["sector"] as const,
        name: "sector",
        constraints: { required: true },

        get: (data: Data) => data.sector,
        set: (data: Data, value: Sector) => {
          data.sector = value;
        },
        getError: (errors: Errors) => errors?.sector,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.sector = value;
        },
        getTainted: (tainted: Tainted) => tainted?.sector ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.sector = value;
        },
        validate: (_value: Sector): Array<string> => [],
      },
      leadName: {
        path: ["leadName"] as const,
        name: "leadName",
        constraints: { required: true },

        get: (data: Data) => data.leadName,
        set: (data: Data, value: AccountName) => {
          data.leadName = value;
        },
        getError: (errors: Errors) => errors?.leadName,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.leadName = value;
        },
        getTainted: (tainted: Tainted) => tainted?.leadName ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.leadName = value;
        },
        validate: (_value: AccountName): Array<string> => [],
      },
      phones: {
        path: ["phones"] as const,
        name: "phones",
        constraints: { required: true },

        get: (data: Data) => data.phones,
        set: (data: Data, value: PhoneNumber[]) => {
          data.phones = value;
        },
        getError: (errors: Errors) => errors?.phones,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.phones = value;
        },
        getTainted: (tainted: Tainted) => tainted?.phones ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.phones = value;
        },
        validate: (_value: PhoneNumber[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["phones"], index] as const,
          name: `phones.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.phones[index],
          set: (data: Data, value: PhoneNumber) => {
            data.phones[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.phones as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.phones ??= {};
            (errors.phones as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.phones?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.phones ??= {};
            tainted.phones[index] = value;
          },
          validate: (_value: PhoneNumber): Array<string> => [],
        }),
        push: (data: Data, item: PhoneNumber) => {
          data.phones.push(item);
        },
        remove: (data: Data, index: number) => {
          data.phones.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.phones[a], data.phones[b]] = [data.phones[b], data.phones[a]];
        },
      },
      email: {
        path: ["email"] as const,
        name: "email",
        constraints: { required: true },

        get: (data: Data) => data.email,
        set: (data: Data, value: Email) => {
          data.email = value;
        },
        getError: (errors: Errors) => errors?.email,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.email = value;
        },
        getTainted: (tainted: Tainted) => tainted?.email ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.email = value;
        },
        validate: (_value: Email): Array<string> => [],
      },
      leadSource: {
        path: ["leadSource"] as const,
        name: "leadSource",
        constraints: { required: true },

        get: (data: Data) => data.leadSource,
        set: (data: Data, value: string | null) => {
          data.leadSource = value;
        },
        getError: (errors: Errors) => errors?.leadSource,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.leadSource = value;
        },
        getTainted: (tainted: Tainted) => tainted?.leadSource ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.leadSource = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      site: {
        path: ["site"] as const,
        name: "site",
        constraints: { required: true },

        get: (data: Data) => data.site,
        set: (data: Data, value: string | Site) => {
          data.site = value;
        },
        getError: (errors: Errors) => errors?.site,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.site = value;
        },
        getTainted: (tainted: Tainted) => tainted?.site ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.site = value;
        },
        validate: (_value: string | Site): Array<string> => [],
      },
      memo: {
        path: ["memo"] as const,
        name: "memo",
        constraints: { required: true },

        get: (data: Data) => data.memo,
        set: (data: Data, value: string) => {
          data.memo = value;
        },
        getError: (errors: Errors) => errors?.memo,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.memo = value;
        },
        getTainted: (tainted: Tainted) => tainted?.memo ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.memo = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      needsReview: {
        path: ["needsReview"] as const,
        name: "needsReview",
        constraints: { required: true },

        get: (data: Data) => data.needsReview,
        set: (data: Data, value: boolean) => {
          data.needsReview = value;
        },
        getError: (errors: Errors) => errors?.needsReview,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.needsReview = value;
        },
        getTainted: (tainted: Tainted) => tainted?.needsReview ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.needsReview = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      hasAlert: {
        path: ["hasAlert"] as const,
        name: "hasAlert",
        constraints: { required: true },

        get: (data: Data) => data.hasAlert,
        set: (data: Data, value: boolean) => {
          data.hasAlert = value;
        },
        getError: (errors: Errors) => errors?.hasAlert,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.hasAlert = value;
        },
        getTainted: (tainted: Tainted) => tainted?.hasAlert ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.hasAlert = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      salesRep: {
        path: ["salesRep"] as const,
        name: "salesRep",
        constraints: { required: true },

        get: (data: Data) => data.salesRep,
        set: (data: Data, value: Represents[] | null) => {
          data.salesRep = value;
        },
        getError: (errors: Errors) => errors?.salesRep,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.salesRep = value;
        },
        getTainted: (tainted: Tainted) => tainted?.salesRep ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.salesRep = value;
        },
        validate: (_value: Represents[] | null): Array<string> => [],
      },
      color: {
        path: ["color"] as const,
        name: "color",
        constraints: { required: true },

        get: (data: Data) => data.color,
        set: (data: Data, value: string | null) => {
          data.color = value;
        },
        getError: (errors: Errors) => errors?.color,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.color = value;
        },
        getTainted: (tainted: Tainted) => tainted?.color ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.color = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      accountType: {
        path: ["accountType"] as const,
        name: "accountType",
        constraints: { required: true },

        get: (data: Data) => data.accountType,
        set: (data: Data, value: string) => {
          data.accountType = value;
        },
        getError: (errors: Errors) => errors?.accountType,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.accountType = value;
        },
        getTainted: (tainted: Tainted) => tainted?.accountType ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.accountType = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      subtype: {
        path: ["subtype"] as const,
        name: "subtype",
        constraints: { required: true },

        get: (data: Data) => data.subtype,
        set: (data: Data, value: string) => {
          data.subtype = value;
        },
        getError: (errors: Errors) => errors?.subtype,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.subtype = value;
        },
        getTainted: (tainted: Tainted) => tainted?.subtype ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.subtype = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      isTaxExempt: {
        path: ["isTaxExempt"] as const,
        name: "isTaxExempt",
        constraints: { required: true },

        get: (data: Data) => data.isTaxExempt,
        set: (data: Data, value: boolean) => {
          data.isTaxExempt = value;
        },
        getError: (errors: Errors) => errors?.isTaxExempt,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.isTaxExempt = value;
        },
        getTainted: (tainted: Tainted) => tainted?.isTaxExempt ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.isTaxExempt = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      paymentTerms: {
        path: ["paymentTerms"] as const,
        name: "paymentTerms",
        constraints: { required: true },

        get: (data: Data) => data.paymentTerms,
        set: (data: Data, value: string) => {
          data.paymentTerms = value;
        },
        getError: (errors: Errors) => errors?.paymentTerms,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.paymentTerms = value;
        },
        getTainted: (tainted: Tainted) => tainted?.paymentTerms ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.paymentTerms = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      tags: {
        path: ["tags"] as const,
        name: "tags",
        constraints: { required: true },

        get: (data: Data) => data.tags,
        set: (data: Data, value: string[]) => {
          data.tags = value;
        },
        getError: (errors: Errors) => errors?.tags,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.tags = value;
        },
        getTainted: (tainted: Tainted) => tainted?.tags ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.tags = value;
        },
        validate: (_value: string[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["tags"], index] as const,
          name: `tags.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.tags[index],
          set: (data: Data, value: string) => {
            data.tags[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.tags as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.tags ??= {};
            (errors.tags as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.tags?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.tags ??= {};
            tainted.tags[index] = value;
          },
          validate: (_value: string): Array<string> => [],
        }),
        push: (data: Data, item: string) => {
          data.tags.push(item);
        },
        remove: (data: Data, index: number) => {
          data.tags.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.tags[a], data.tags[b]] = [data.tags[b], data.tags[a]];
        },
      },
      customFields: {
        path: ["customFields"] as const,
        name: "customFields",
        constraints: { required: true },

        get: (data: Data) => data.customFields,
        set: (data: Data, value: [string, string][]) => {
          data.customFields = value;
        },
        getError: (errors: Errors) => errors?.customFields,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.customFields = value;
        },
        getTainted: (tainted: Tainted) => tainted?.customFields ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.customFields = value;
        },
        validate: (_value: [string, string][]): Array<string> => [],

        at: (index: number) => ({
          path: [...["customFields"], index] as const,
          name: `customFields.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.customFields[index],
          set: (data: Data, value: [string, string]) => {
            data.customFields[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.customFields as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.customFields ??= {};
            (errors.customFields as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: (tainted: Tainted) =>
            tainted.customFields?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.customFields ??= {};
            tainted.customFields[index] = value;
          },
          validate: (_value: [string, string]): Array<string> => [],
        }),
        push: (data: Data, item: [string, string]) => {
          data.customFields.push(item);
        },
        remove: (data: Data, index: number) => {
          data.customFields.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.customFields[a], data.customFields[b]] = [
            data.customFields[b],
            data.customFields[a],
          ];
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: AppPermissions): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<AppPermissions, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "AppPermissions.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as AppPermissions;
  }
}

export namespace AppPermissions {
  export type Data = AppPermissions;
  export type Errors = {
    _errors?: Array<string>;
    applications?: {
      _errors?: Array<string>;
      [index: number]: Applications.Errors;
    };
    pages?: { _errors?: Array<string>; [index: number]: Page.Errors };
    data?: { _errors?: Array<string>; [index: number]: Table.Errors };
  };
  export type Tainted = {
    applications?: { [index: number]: Applications.Tainted };
    pages?: { [index: number]: Page.Tainted };
    data?: { [index: number]: Table.Tainted };
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      // Collect array items from indexed form fields
      const applicationsItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("applications." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("applications." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("applications." + idx + ".")) {
              const fieldName = key.slice(
                "applications.".length + String(idx).length + 1,
              );
              item[fieldName] = value;
            }
          }
          applicationsItems.push(item);
        }
        idx++;
        if (idx > 1000) break; // Safety limit
      }
      obj.applications = applicationsItems;
    }
    {
      // Collect array items from indexed form fields
      const pagesItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("pages." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("pages." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("pages." + idx + ".")) {
              const fieldName = key.slice(
                "pages.".length + String(idx).length + 1,
              );
              item[fieldName] = value;
            }
          }
          pagesItems.push(item);
        }
        idx++;
        if (idx > 1000) break; // Safety limit
      }
      obj.pages = pagesItems;
    }
    {
      // Collect array items from indexed form fields
      const dataItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("data." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("data." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("data." + idx + ".")) {
              const fieldName = key.slice(
                "data.".length + String(idx).length + 1,
              );
              item[fieldName] = value;
            }
          }
          dataItems.push(item);
        }
        idx++;
        if (idx > 1000) break; // Safety limit
      }
      obj.data = dataItems;
    }
    return AppPermissions.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      applications: {
        path: ["applications"] as const,
        name: "applications",
        constraints: { required: true },

        get: (data: Data) => data.applications,
        set: (data: Data, value: Applications[]) => {
          data.applications = value;
        },
        getError: (errors: Errors) => errors?.applications,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.applications = value;
        },
        getTainted: (tainted: Tainted) => tainted?.applications ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.applications = value;
        },
        validate: (_value: Applications[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["applications"], index] as const,
          name: `applications.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.applications[index],
          set: (data: Data, value: Applications) => {
            data.applications[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.applications as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.applications ??= {};
            (errors.applications as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: (tainted: Tainted) =>
            tainted.applications?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.applications ??= {};
            tainted.applications[index] = value;
          },
          validate: (_value: Applications): Array<string> => [],
        }),
        push: (data: Data, item: Applications) => {
          data.applications.push(item);
        },
        remove: (data: Data, index: number) => {
          data.applications.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.applications[a], data.applications[b]] = [
            data.applications[b],
            data.applications[a],
          ];
        },
      },
      pages: {
        path: ["pages"] as const,
        name: "pages",
        constraints: { required: true },

        get: (data: Data) => data.pages,
        set: (data: Data, value: Page[]) => {
          data.pages = value;
        },
        getError: (errors: Errors) => errors?.pages,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.pages = value;
        },
        getTainted: (tainted: Tainted) => tainted?.pages ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.pages = value;
        },
        validate: (_value: Page[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["pages"], index] as const,
          name: `pages.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.pages[index],
          set: (data: Data, value: Page) => {
            data.pages[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.pages as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.pages ??= {};
            (errors.pages as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.pages?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.pages ??= {};
            tainted.pages[index] = value;
          },
          validate: (_value: Page): Array<string> => [],
        }),
        push: (data: Data, item: Page) => {
          data.pages.push(item);
        },
        remove: (data: Data, index: number) => {
          data.pages.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.pages[a], data.pages[b]] = [data.pages[b], data.pages[a]];
        },
      },
      data: {
        path: ["data"] as const,
        name: "data",
        constraints: { required: true },

        get: (data: Data) => data.data,
        set: (data: Data, value: Table[]) => {
          data.data = value;
        },
        getError: (errors: Errors) => errors?.data,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.data = value;
        },
        getTainted: (tainted: Tainted) => tainted?.data ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.data = value;
        },
        validate: (_value: Table[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["data"], index] as const,
          name: `data.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.data[index],
          set: (data: Data, value: Table) => {
            data.data[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.data as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.data ??= {};
            (errors.data as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.data?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.data ??= {};
            tainted.data[index] = value;
          },
          validate: (_value: Table): Array<string> => [],
        }),
        push: (data: Data, item: Table) => {
          data.data.push(item);
        },
        remove: (data: Data, index: number) => {
          data.data.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.data[a], data.data[b]] = [data.data[b], data.data[a]];
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: Company): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Company, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Company.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Company;
  }
}

export namespace Company {
  export type Data = Company;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    legalName?: Array<string>;
    headquarters?: Array<string>;
    phones?: { _errors?: Array<string>; [index: number]: PhoneNumber.Errors };
    fax?: Array<string>;
    email?: Array<string>;
    website?: Array<string>;
    taxId?: Array<string>;
    referenceNumber?: Array<string>;
    postalCodeLookup?: Array<string>;
    timeZone?: Array<string>;
    defaultTax?: Array<string>;
    defaultTaxLocation?: Array<string>;
    defaultAreaCode?: Array<string>;
    defaultAccountType?: Array<string>;
    lookupFormatting?: Array<string>;
    accountNameFormat?: Array<string>;
    merchantServiceProvider?: Array<string>;
    dateDisplayStyle?: Array<string>;
    hasAutoCommission?: Array<string>;
    hasAutoDaylightSavings?: Array<string>;
    hasAutoFmsTracking?: Array<string>;
    hasNotifications?: Array<string>;
    hasRequiredLeadSource?: Array<string>;
    hasRequiredEmail?: Array<string>;
    hasSortServiceItemsAlphabetically?: Array<string>;
    hasAttachOrderToAppointmentEmails?: Array<string>;
    scheduleInterval?: Array<string>;
    colorsConfig?: ColorsConfig.Errors;
  };
  export type Tainted = {
    id?: boolean;
    legalName?: boolean;
    headquarters?: boolean;
    phones?: { [index: number]: PhoneNumber.Tainted };
    fax?: boolean;
    email?: boolean;
    website?: boolean;
    taxId?: boolean;
    referenceNumber?: boolean;
    postalCodeLookup?: boolean;
    timeZone?: boolean;
    defaultTax?: boolean;
    defaultTaxLocation?: boolean;
    defaultAreaCode?: boolean;
    defaultAccountType?: boolean;
    lookupFormatting?: boolean;
    accountNameFormat?: boolean;
    merchantServiceProvider?: boolean;
    dateDisplayStyle?: boolean;
    hasAutoCommission?: boolean;
    hasAutoDaylightSavings?: boolean;
    hasAutoFmsTracking?: boolean;
    hasNotifications?: boolean;
    hasRequiredLeadSource?: boolean;
    hasRequiredEmail?: boolean;
    hasSortServiceItemsAlphabetically?: boolean;
    hasAttachOrderToAppointmentEmails?: boolean;
    scheduleInterval?: boolean;
    colorsConfig?: ColorsConfig.Tainted;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.legalName = formData.get("legalName") ?? "";
    obj.headquarters = formData.get("headquarters") ?? "";
    {
      // Collect array items from indexed form fields
      const phonesItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("phones." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("phones." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("phones." + idx + ".")) {
              const fieldName = key.slice(
                "phones.".length + String(idx).length + 1,
              );
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
    obj.fax = formData.get("fax") ?? "";
    obj.email = formData.get("email") ?? "";
    obj.website = formData.get("website") ?? "";
    obj.taxId = formData.get("taxId") ?? "";
    {
      const referenceNumberStr = formData.get("referenceNumber");
      obj.referenceNumber = referenceNumberStr
        ? parseFloat(referenceNumberStr as string)
        : 0;
      if (
        obj.referenceNumber !== undefined &&
        Number.isNaN(obj.referenceNumber)
      )
        obj.referenceNumber = 0;
    }
    obj.postalCodeLookup = formData.get("postalCodeLookup") ?? "";
    obj.timeZone = formData.get("timeZone") ?? "";
    obj.defaultTax = formData.get("defaultTax") ?? "";
    obj.defaultTaxLocation = formData.get("defaultTaxLocation") ?? "";
    {
      const defaultAreaCodeStr = formData.get("defaultAreaCode");
      obj.defaultAreaCode = defaultAreaCodeStr
        ? parseFloat(defaultAreaCodeStr as string)
        : 0;
      if (
        obj.defaultAreaCode !== undefined &&
        Number.isNaN(obj.defaultAreaCode)
      )
        obj.defaultAreaCode = 0;
    }
    obj.defaultAccountType = formData.get("defaultAccountType") ?? "";
    obj.lookupFormatting = formData.get("lookupFormatting") ?? "";
    obj.accountNameFormat = formData.get("accountNameFormat") ?? "";
    obj.merchantServiceProvider = formData.get("merchantServiceProvider") ?? "";
    obj.dateDisplayStyle = formData.get("dateDisplayStyle") ?? "";
    {
      const hasAutoCommissionVal = formData.get("hasAutoCommission");
      obj.hasAutoCommission =
        hasAutoCommissionVal === "true" ||
        hasAutoCommissionVal === "on" ||
        hasAutoCommissionVal === "1";
    }
    {
      const hasAutoDaylightSavingsVal = formData.get("hasAutoDaylightSavings");
      obj.hasAutoDaylightSavings =
        hasAutoDaylightSavingsVal === "true" ||
        hasAutoDaylightSavingsVal === "on" ||
        hasAutoDaylightSavingsVal === "1";
    }
    {
      const hasAutoFmsTrackingVal = formData.get("hasAutoFmsTracking");
      obj.hasAutoFmsTracking =
        hasAutoFmsTrackingVal === "true" ||
        hasAutoFmsTrackingVal === "on" ||
        hasAutoFmsTrackingVal === "1";
    }
    {
      const hasNotificationsVal = formData.get("hasNotifications");
      obj.hasNotifications =
        hasNotificationsVal === "true" ||
        hasNotificationsVal === "on" ||
        hasNotificationsVal === "1";
    }
    {
      const hasRequiredLeadSourceVal = formData.get("hasRequiredLeadSource");
      obj.hasRequiredLeadSource =
        hasRequiredLeadSourceVal === "true" ||
        hasRequiredLeadSourceVal === "on" ||
        hasRequiredLeadSourceVal === "1";
    }
    {
      const hasRequiredEmailVal = formData.get("hasRequiredEmail");
      obj.hasRequiredEmail =
        hasRequiredEmailVal === "true" ||
        hasRequiredEmailVal === "on" ||
        hasRequiredEmailVal === "1";
    }
    {
      const hasSortServiceItemsAlphabeticallyVal = formData.get(
        "hasSortServiceItemsAlphabetically",
      );
      obj.hasSortServiceItemsAlphabetically =
        hasSortServiceItemsAlphabeticallyVal === "true" ||
        hasSortServiceItemsAlphabeticallyVal === "on" ||
        hasSortServiceItemsAlphabeticallyVal === "1";
    }
    {
      const hasAttachOrderToAppointmentEmailsVal = formData.get(
        "hasAttachOrderToAppointmentEmails",
      );
      obj.hasAttachOrderToAppointmentEmails =
        hasAttachOrderToAppointmentEmailsVal === "true" ||
        hasAttachOrderToAppointmentEmailsVal === "on" ||
        hasAttachOrderToAppointmentEmailsVal === "1";
    }
    {
      const scheduleIntervalStr = formData.get("scheduleInterval");
      obj.scheduleInterval = scheduleIntervalStr
        ? parseFloat(scheduleIntervalStr as string)
        : 0;
      if (
        obj.scheduleInterval !== undefined &&
        Number.isNaN(obj.scheduleInterval)
      )
        obj.scheduleInterval = 0;
    }
    {
      // Collect nested object fields with prefix "colorsConfig."
      const colorsConfigObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("colorsConfig.")) {
          const fieldName = key.slice("colorsConfig.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = colorsConfigObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.colorsConfig = colorsConfigObj;
    }
    return Company.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      legalName: {
        path: ["legalName"] as const,
        name: "legalName",
        constraints: { required: true },

        get: (data: Data) => data.legalName,
        set: (data: Data, value: string) => {
          data.legalName = value;
        },
        getError: (errors: Errors) => errors?.legalName,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.legalName = value;
        },
        getTainted: (tainted: Tainted) => tainted?.legalName ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.legalName = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      headquarters: {
        path: ["headquarters"] as const,
        name: "headquarters",
        constraints: { required: true },

        get: (data: Data) => data.headquarters,
        set: (data: Data, value: string | Site) => {
          data.headquarters = value;
        },
        getError: (errors: Errors) => errors?.headquarters,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.headquarters = value;
        },
        getTainted: (tainted: Tainted) => tainted?.headquarters ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.headquarters = value;
        },
        validate: (_value: string | Site): Array<string> => [],
      },
      phones: {
        path: ["phones"] as const,
        name: "phones",
        constraints: { required: true },

        get: (data: Data) => data.phones,
        set: (data: Data, value: PhoneNumber[]) => {
          data.phones = value;
        },
        getError: (errors: Errors) => errors?.phones,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.phones = value;
        },
        getTainted: (tainted: Tainted) => tainted?.phones ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.phones = value;
        },
        validate: (_value: PhoneNumber[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["phones"], index] as const,
          name: `phones.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.phones[index],
          set: (data: Data, value: PhoneNumber) => {
            data.phones[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.phones as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.phones ??= {};
            (errors.phones as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.phones?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.phones ??= {};
            tainted.phones[index] = value;
          },
          validate: (_value: PhoneNumber): Array<string> => [],
        }),
        push: (data: Data, item: PhoneNumber) => {
          data.phones.push(item);
        },
        remove: (data: Data, index: number) => {
          data.phones.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.phones[a], data.phones[b]] = [data.phones[b], data.phones[a]];
        },
      },
      fax: {
        path: ["fax"] as const,
        name: "fax",
        constraints: { required: true },

        get: (data: Data) => data.fax,
        set: (data: Data, value: string) => {
          data.fax = value;
        },
        getError: (errors: Errors) => errors?.fax,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.fax = value;
        },
        getTainted: (tainted: Tainted) => tainted?.fax ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.fax = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      email: {
        path: ["email"] as const,
        name: "email",
        constraints: { required: true },

        get: (data: Data) => data.email,
        set: (data: Data, value: string) => {
          data.email = value;
        },
        getError: (errors: Errors) => errors?.email,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.email = value;
        },
        getTainted: (tainted: Tainted) => tainted?.email ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.email = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      website: {
        path: ["website"] as const,
        name: "website",
        constraints: { required: true },

        get: (data: Data) => data.website,
        set: (data: Data, value: string) => {
          data.website = value;
        },
        getError: (errors: Errors) => errors?.website,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.website = value;
        },
        getTainted: (tainted: Tainted) => tainted?.website ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.website = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      taxId: {
        path: ["taxId"] as const,
        name: "taxId",
        constraints: { required: true },

        get: (data: Data) => data.taxId,
        set: (data: Data, value: string) => {
          data.taxId = value;
        },
        getError: (errors: Errors) => errors?.taxId,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.taxId = value;
        },
        getTainted: (tainted: Tainted) => tainted?.taxId ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.taxId = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      referenceNumber: {
        path: ["referenceNumber"] as const,
        name: "referenceNumber",
        constraints: { required: true },

        get: (data: Data) => data.referenceNumber,
        set: (data: Data, value: number) => {
          data.referenceNumber = value;
        },
        getError: (errors: Errors) => errors?.referenceNumber,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.referenceNumber = value;
        },
        getTainted: (tainted: Tainted) => tainted?.referenceNumber ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.referenceNumber = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      postalCodeLookup: {
        path: ["postalCodeLookup"] as const,
        name: "postalCodeLookup",
        constraints: { required: true },

        get: (data: Data) => data.postalCodeLookup,
        set: (data: Data, value: string) => {
          data.postalCodeLookup = value;
        },
        getError: (errors: Errors) => errors?.postalCodeLookup,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.postalCodeLookup = value;
        },
        getTainted: (tainted: Tainted) => tainted?.postalCodeLookup ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.postalCodeLookup = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      timeZone: {
        path: ["timeZone"] as const,
        name: "timeZone",
        constraints: { required: true },

        get: (data: Data) => data.timeZone,
        set: (data: Data, value: string) => {
          data.timeZone = value;
        },
        getError: (errors: Errors) => errors?.timeZone,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.timeZone = value;
        },
        getTainted: (tainted: Tainted) => tainted?.timeZone ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.timeZone = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      defaultTax: {
        path: ["defaultTax"] as const,
        name: "defaultTax",
        constraints: { required: true },

        get: (data: Data) => data.defaultTax,
        set: (data: Data, value: string | TaxRate) => {
          data.defaultTax = value;
        },
        getError: (errors: Errors) => errors?.defaultTax,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.defaultTax = value;
        },
        getTainted: (tainted: Tainted) => tainted?.defaultTax ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.defaultTax = value;
        },
        validate: (_value: string | TaxRate): Array<string> => [],
      },
      defaultTaxLocation: {
        path: ["defaultTaxLocation"] as const,
        name: "defaultTaxLocation",
        constraints: { required: true },

        get: (data: Data) => data.defaultTaxLocation,
        set: (data: Data, value: string) => {
          data.defaultTaxLocation = value;
        },
        getError: (errors: Errors) => errors?.defaultTaxLocation,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.defaultTaxLocation = value;
        },
        getTainted: (tainted: Tainted) => tainted?.defaultTaxLocation ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.defaultTaxLocation = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      defaultAreaCode: {
        path: ["defaultAreaCode"] as const,
        name: "defaultAreaCode",
        constraints: { required: true },

        get: (data: Data) => data.defaultAreaCode,
        set: (data: Data, value: number) => {
          data.defaultAreaCode = value;
        },
        getError: (errors: Errors) => errors?.defaultAreaCode,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.defaultAreaCode = value;
        },
        getTainted: (tainted: Tainted) => tainted?.defaultAreaCode ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.defaultAreaCode = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      defaultAccountType: {
        path: ["defaultAccountType"] as const,
        name: "defaultAccountType",
        constraints: { required: true },

        get: (data: Data) => data.defaultAccountType,
        set: (data: Data, value: string) => {
          data.defaultAccountType = value;
        },
        getError: (errors: Errors) => errors?.defaultAccountType,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.defaultAccountType = value;
        },
        getTainted: (tainted: Tainted) => tainted?.defaultAccountType ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.defaultAccountType = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      lookupFormatting: {
        path: ["lookupFormatting"] as const,
        name: "lookupFormatting",
        constraints: { required: true },

        get: (data: Data) => data.lookupFormatting,
        set: (data: Data, value: string) => {
          data.lookupFormatting = value;
        },
        getError: (errors: Errors) => errors?.lookupFormatting,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.lookupFormatting = value;
        },
        getTainted: (tainted: Tainted) => tainted?.lookupFormatting ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.lookupFormatting = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      accountNameFormat: {
        path: ["accountNameFormat"] as const,
        name: "accountNameFormat",
        constraints: { required: true },

        get: (data: Data) => data.accountNameFormat,
        set: (data: Data, value: string) => {
          data.accountNameFormat = value;
        },
        getError: (errors: Errors) => errors?.accountNameFormat,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.accountNameFormat = value;
        },
        getTainted: (tainted: Tainted) => tainted?.accountNameFormat ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.accountNameFormat = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      merchantServiceProvider: {
        path: ["merchantServiceProvider"] as const,
        name: "merchantServiceProvider",
        constraints: { required: true },

        get: (data: Data) => data.merchantServiceProvider,
        set: (data: Data, value: string | null) => {
          data.merchantServiceProvider = value;
        },
        getError: (errors: Errors) => errors?.merchantServiceProvider,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.merchantServiceProvider = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.merchantServiceProvider ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.merchantServiceProvider = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      dateDisplayStyle: {
        path: ["dateDisplayStyle"] as const,
        name: "dateDisplayStyle",
        constraints: { required: true },

        get: (data: Data) => data.dateDisplayStyle,
        set: (data: Data, value: string) => {
          data.dateDisplayStyle = value;
        },
        getError: (errors: Errors) => errors?.dateDisplayStyle,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.dateDisplayStyle = value;
        },
        getTainted: (tainted: Tainted) => tainted?.dateDisplayStyle ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.dateDisplayStyle = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      hasAutoCommission: {
        path: ["hasAutoCommission"] as const,
        name: "hasAutoCommission",
        constraints: { required: true },

        get: (data: Data) => data.hasAutoCommission,
        set: (data: Data, value: boolean) => {
          data.hasAutoCommission = value;
        },
        getError: (errors: Errors) => errors?.hasAutoCommission,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.hasAutoCommission = value;
        },
        getTainted: (tainted: Tainted) => tainted?.hasAutoCommission ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.hasAutoCommission = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      hasAutoDaylightSavings: {
        path: ["hasAutoDaylightSavings"] as const,
        name: "hasAutoDaylightSavings",
        constraints: { required: true },

        get: (data: Data) => data.hasAutoDaylightSavings,
        set: (data: Data, value: boolean) => {
          data.hasAutoDaylightSavings = value;
        },
        getError: (errors: Errors) => errors?.hasAutoDaylightSavings,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.hasAutoDaylightSavings = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.hasAutoDaylightSavings ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.hasAutoDaylightSavings = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      hasAutoFmsTracking: {
        path: ["hasAutoFmsTracking"] as const,
        name: "hasAutoFmsTracking",
        constraints: { required: true },

        get: (data: Data) => data.hasAutoFmsTracking,
        set: (data: Data, value: boolean) => {
          data.hasAutoFmsTracking = value;
        },
        getError: (errors: Errors) => errors?.hasAutoFmsTracking,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.hasAutoFmsTracking = value;
        },
        getTainted: (tainted: Tainted) => tainted?.hasAutoFmsTracking ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.hasAutoFmsTracking = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      hasNotifications: {
        path: ["hasNotifications"] as const,
        name: "hasNotifications",
        constraints: { required: true },

        get: (data: Data) => data.hasNotifications,
        set: (data: Data, value: boolean) => {
          data.hasNotifications = value;
        },
        getError: (errors: Errors) => errors?.hasNotifications,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.hasNotifications = value;
        },
        getTainted: (tainted: Tainted) => tainted?.hasNotifications ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.hasNotifications = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      hasRequiredLeadSource: {
        path: ["hasRequiredLeadSource"] as const,
        name: "hasRequiredLeadSource",
        constraints: { required: true },

        get: (data: Data) => data.hasRequiredLeadSource,
        set: (data: Data, value: boolean) => {
          data.hasRequiredLeadSource = value;
        },
        getError: (errors: Errors) => errors?.hasRequiredLeadSource,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.hasRequiredLeadSource = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.hasRequiredLeadSource ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.hasRequiredLeadSource = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      hasRequiredEmail: {
        path: ["hasRequiredEmail"] as const,
        name: "hasRequiredEmail",
        constraints: { required: true },

        get: (data: Data) => data.hasRequiredEmail,
        set: (data: Data, value: boolean) => {
          data.hasRequiredEmail = value;
        },
        getError: (errors: Errors) => errors?.hasRequiredEmail,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.hasRequiredEmail = value;
        },
        getTainted: (tainted: Tainted) => tainted?.hasRequiredEmail ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.hasRequiredEmail = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      hasSortServiceItemsAlphabetically: {
        path: ["hasSortServiceItemsAlphabetically"] as const,
        name: "hasSortServiceItemsAlphabetically",
        constraints: { required: true },

        get: (data: Data) => data.hasSortServiceItemsAlphabetically,
        set: (data: Data, value: boolean) => {
          data.hasSortServiceItemsAlphabetically = value;
        },
        getError: (errors: Errors) => errors?.hasSortServiceItemsAlphabetically,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.hasSortServiceItemsAlphabetically = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.hasSortServiceItemsAlphabetically ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.hasSortServiceItemsAlphabetically = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      hasAttachOrderToAppointmentEmails: {
        path: ["hasAttachOrderToAppointmentEmails"] as const,
        name: "hasAttachOrderToAppointmentEmails",
        constraints: { required: true },

        get: (data: Data) => data.hasAttachOrderToAppointmentEmails,
        set: (data: Data, value: boolean) => {
          data.hasAttachOrderToAppointmentEmails = value;
        },
        getError: (errors: Errors) => errors?.hasAttachOrderToAppointmentEmails,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.hasAttachOrderToAppointmentEmails = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.hasAttachOrderToAppointmentEmails ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.hasAttachOrderToAppointmentEmails = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      scheduleInterval: {
        path: ["scheduleInterval"] as const,
        name: "scheduleInterval",
        constraints: { required: true },

        get: (data: Data) => data.scheduleInterval,
        set: (data: Data, value: number) => {
          data.scheduleInterval = value;
        },
        getError: (errors: Errors) => errors?.scheduleInterval,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.scheduleInterval = value;
        },
        getTainted: (tainted: Tainted) => tainted?.scheduleInterval ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.scheduleInterval = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      colorsConfig: {
        path: ["colorsConfig"] as const,
        name: "colorsConfig",
        constraints: { required: true },

        get: (data: Data) => data.colorsConfig,
        set: (data: Data, value: ColorsConfig) => {
          data.colorsConfig = value;
        },
        getError: (errors: Errors) => errors?.colorsConfig,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.colorsConfig = value;
        },
        getTainted: (tainted: Tainted) => tainted?.colorsConfig ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.colorsConfig = value;
        },
        validate: (_value: ColorsConfig): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Ordinal): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Ordinal, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Ordinal.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Ordinal;
  }
}

export namespace Ordinal {
  export type Data = Ordinal;
  export type Errors = {
    _errors?: Array<string>;
    north?: Array<string>;
    northeast?: Array<string>;
    east?: Array<string>;
    southeast?: Array<string>;
    south?: Array<string>;
    southwest?: Array<string>;
    west?: Array<string>;
    northwest?: Array<string>;
  };
  export type Tainted = {
    north?: boolean;
    northeast?: boolean;
    east?: boolean;
    southeast?: boolean;
    south?: boolean;
    southwest?: boolean;
    west?: boolean;
    northwest?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const northStr = formData.get("north");
      obj.north = northStr ? parseFloat(northStr as string) : 0;
      if (obj.north !== undefined && Number.isNaN(obj.north)) obj.north = 0;
    }
    {
      const northeastStr = formData.get("northeast");
      obj.northeast = northeastStr ? parseFloat(northeastStr as string) : 0;
      if (obj.northeast !== undefined && Number.isNaN(obj.northeast))
        obj.northeast = 0;
    }
    {
      const eastStr = formData.get("east");
      obj.east = eastStr ? parseFloat(eastStr as string) : 0;
      if (obj.east !== undefined && Number.isNaN(obj.east)) obj.east = 0;
    }
    {
      const southeastStr = formData.get("southeast");
      obj.southeast = southeastStr ? parseFloat(southeastStr as string) : 0;
      if (obj.southeast !== undefined && Number.isNaN(obj.southeast))
        obj.southeast = 0;
    }
    {
      const southStr = formData.get("south");
      obj.south = southStr ? parseFloat(southStr as string) : 0;
      if (obj.south !== undefined && Number.isNaN(obj.south)) obj.south = 0;
    }
    {
      const southwestStr = formData.get("southwest");
      obj.southwest = southwestStr ? parseFloat(southwestStr as string) : 0;
      if (obj.southwest !== undefined && Number.isNaN(obj.southwest))
        obj.southwest = 0;
    }
    {
      const westStr = formData.get("west");
      obj.west = westStr ? parseFloat(westStr as string) : 0;
      if (obj.west !== undefined && Number.isNaN(obj.west)) obj.west = 0;
    }
    {
      const northwestStr = formData.get("northwest");
      obj.northwest = northwestStr ? parseFloat(northwestStr as string) : 0;
      if (obj.northwest !== undefined && Number.isNaN(obj.northwest))
        obj.northwest = 0;
    }
    return Ordinal.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      north: {
        path: ["north"] as const,
        name: "north",
        constraints: { required: true },

        get: (data: Data) => data.north,
        set: (data: Data, value: number) => {
          data.north = value;
        },
        getError: (errors: Errors) => errors?.north,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.north = value;
        },
        getTainted: (tainted: Tainted) => tainted?.north ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.north = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      northeast: {
        path: ["northeast"] as const,
        name: "northeast",
        constraints: { required: true },

        get: (data: Data) => data.northeast,
        set: (data: Data, value: number) => {
          data.northeast = value;
        },
        getError: (errors: Errors) => errors?.northeast,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.northeast = value;
        },
        getTainted: (tainted: Tainted) => tainted?.northeast ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.northeast = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      east: {
        path: ["east"] as const,
        name: "east",
        constraints: { required: true },

        get: (data: Data) => data.east,
        set: (data: Data, value: number) => {
          data.east = value;
        },
        getError: (errors: Errors) => errors?.east,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.east = value;
        },
        getTainted: (tainted: Tainted) => tainted?.east ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.east = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      southeast: {
        path: ["southeast"] as const,
        name: "southeast",
        constraints: { required: true },

        get: (data: Data) => data.southeast,
        set: (data: Data, value: number) => {
          data.southeast = value;
        },
        getError: (errors: Errors) => errors?.southeast,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.southeast = value;
        },
        getTainted: (tainted: Tainted) => tainted?.southeast ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.southeast = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      south: {
        path: ["south"] as const,
        name: "south",
        constraints: { required: true },

        get: (data: Data) => data.south,
        set: (data: Data, value: number) => {
          data.south = value;
        },
        getError: (errors: Errors) => errors?.south,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.south = value;
        },
        getTainted: (tainted: Tainted) => tainted?.south ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.south = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      southwest: {
        path: ["southwest"] as const,
        name: "southwest",
        constraints: { required: true },

        get: (data: Data) => data.southwest,
        set: (data: Data, value: number) => {
          data.southwest = value;
        },
        getError: (errors: Errors) => errors?.southwest,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.southwest = value;
        },
        getTainted: (tainted: Tainted) => tainted?.southwest ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.southwest = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      west: {
        path: ["west"] as const,
        name: "west",
        constraints: { required: true },

        get: (data: Data) => data.west,
        set: (data: Data, value: number) => {
          data.west = value;
        },
        getError: (errors: Errors) => errors?.west,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.west = value;
        },
        getTainted: (tainted: Tainted) => tainted?.west ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.west = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      northwest: {
        path: ["northwest"] as const,
        name: "northwest",
        constraints: { required: true },

        get: (data: Data) => data.northwest,
        set: (data: Data, value: number) => {
          data.northwest = value;
        },
        getError: (errors: Errors) => errors?.northwest,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.northwest = value;
        },
        getTainted: (tainted: Tainted) => tainted?.northwest ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.northwest = value;
        },
        validate: (_value: number): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Password): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Password, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Password.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Password;
  }
}

export namespace Password {
  export type Data = Password;
  export type Errors = { _errors?: Array<string>; password?: Array<string> };
  export type Tainted = { password?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.password = formData.get("password") ?? "";
    return Password.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      password: {
        path: ["password"] as const,
        name: "password",
        constraints: { required: true },

        get: (data: Data) => data.password,
        set: (data: Data, value: string) => {
          data.password = value;
        },
        getError: (errors: Errors) => errors?.password,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.password = value;
        },
        getTainted: (tainted: Tainted) => tainted?.password ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.password = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: Created): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Created, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Created.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Created;
  }
}

export namespace Created {
  export type Data = Created;
  export type Errors = { _errors?: Array<string>; initialData?: Array<string> };
  export type Tainted = { initialData?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.initialData = formData.get("initialData") ?? "";
    return Created.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      initialData: {
        path: ["initialData"] as const,
        name: "initialData",
        constraints: { required: true },

        get: (data: Data) => data.initialData,
        set: (data: Data, value: string | null) => {
          data.initialData = value;
        },
        getError: (errors: Errors) => errors?.initialData,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.initialData = value;
        },
        getTainted: (tainted: Tainted) => tainted?.initialData ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.initialData = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Employee): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Employee, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Employee.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Employee;
  }
}

export namespace Employee {
  export type Data = Employee;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    imageUrl?: Array<string>;
    name?: Array<string>;
    phones?: { _errors?: Array<string>; [index: number]: PhoneNumber.Errors };
    role?: Array<string>;
    title?: JobTitle.Errors;
    email?: Email.Errors;
    address?: Array<string>;
    username?: Array<string>;
    route?: Array<string>;
    ratePerHour?: Array<string>;
    active?: Array<string>;
    isTechnician?: Array<string>;
    isSalesRep?: Array<string>;
    description?: Array<string>;
    linkedinUrl?: Array<string>;
    attendance?: { _errors?: Array<string>; [index: number]: Array<string> };
    settings?: Settings.Errors;
  };
  export type Tainted = {
    id?: boolean;
    imageUrl?: boolean;
    name?: boolean;
    phones?: { [index: number]: PhoneNumber.Tainted };
    role?: boolean;
    title?: JobTitle.Tainted;
    email?: Email.Tainted;
    address?: boolean;
    username?: boolean;
    route?: boolean;
    ratePerHour?: boolean;
    active?: boolean;
    isTechnician?: boolean;
    isSalesRep?: boolean;
    description?: boolean;
    linkedinUrl?: boolean;
    attendance?: { [index: number]: boolean };
    settings?: Settings.Tainted;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.imageUrl = formData.get("imageUrl") ?? "";
    obj.name = formData.get("name") ?? "";
    {
      // Collect array items from indexed form fields
      const phonesItems: Array<Record<string, unknown>> = [];
      let idx = 0;
      while (formData.has("phones." + idx + ".") || idx === 0) {
        // Check if any field with this index exists
        const hasAny = Array.from(formData.keys()).some((k) =>
          k.startsWith("phones." + idx + "."),
        );
        if (!hasAny && idx > 0) break;
        if (hasAny) {
          const item: Record<string, unknown> = {};
          for (const [key, value] of formData.entries()) {
            if (key.startsWith("phones." + idx + ".")) {
              const fieldName = key.slice(
                "phones.".length + String(idx).length + 1,
              );
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
    obj.role = formData.get("role") ?? "";
    {
      // Collect nested object fields with prefix "title."
      const titleObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("title.")) {
          const fieldName = key.slice("title.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = titleObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.title = titleObj;
    }
    {
      // Collect nested object fields with prefix "email."
      const emailObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("email.")) {
          const fieldName = key.slice("email.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = emailObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.email = emailObj;
    }
    obj.address = formData.get("address") ?? "";
    obj.username = formData.get("username") ?? "";
    obj.route = formData.get("route") ?? "";
    {
      const ratePerHourStr = formData.get("ratePerHour");
      obj.ratePerHour = ratePerHourStr
        ? parseFloat(ratePerHourStr as string)
        : 0;
      if (obj.ratePerHour !== undefined && Number.isNaN(obj.ratePerHour))
        obj.ratePerHour = 0;
    }
    {
      const activeVal = formData.get("active");
      obj.active =
        activeVal === "true" || activeVal === "on" || activeVal === "1";
    }
    {
      const isTechnicianVal = formData.get("isTechnician");
      obj.isTechnician =
        isTechnicianVal === "true" ||
        isTechnicianVal === "on" ||
        isTechnicianVal === "1";
    }
    {
      const isSalesRepVal = formData.get("isSalesRep");
      obj.isSalesRep =
        isSalesRepVal === "true" ||
        isSalesRepVal === "on" ||
        isSalesRepVal === "1";
    }
    obj.description = formData.get("description") ?? "";
    obj.linkedinUrl = formData.get("linkedinUrl") ?? "";
    obj.attendance = formData.getAll("attendance") as Array<string>;
    {
      // Collect nested object fields with prefix "settings."
      const settingsObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("settings.")) {
          const fieldName = key.slice("settings.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = settingsObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.settings = settingsObj;
    }
    return Employee.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      imageUrl: {
        path: ["imageUrl"] as const,
        name: "imageUrl",
        constraints: { required: true },

        get: (data: Data) => data.imageUrl,
        set: (data: Data, value: string | null) => {
          data.imageUrl = value;
        },
        getError: (errors: Errors) => errors?.imageUrl,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.imageUrl = value;
        },
        getTainted: (tainted: Tainted) => tainted?.imageUrl ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.imageUrl = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: (data: Data) => data.name,
        set: (data: Data, value: string) => {
          data.name = value;
        },
        getError: (errors: Errors) => errors?.name,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: (tainted: Tainted) => tainted?.name ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.name = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      phones: {
        path: ["phones"] as const,
        name: "phones",
        constraints: { required: true },

        get: (data: Data) => data.phones,
        set: (data: Data, value: PhoneNumber[]) => {
          data.phones = value;
        },
        getError: (errors: Errors) => errors?.phones,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.phones = value;
        },
        getTainted: (tainted: Tainted) => tainted?.phones ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.phones = value;
        },
        validate: (_value: PhoneNumber[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["phones"], index] as const,
          name: `phones.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.phones[index],
          set: (data: Data, value: PhoneNumber) => {
            data.phones[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.phones as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.phones ??= {};
            (errors.phones as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.phones?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.phones ??= {};
            tainted.phones[index] = value;
          },
          validate: (_value: PhoneNumber): Array<string> => [],
        }),
        push: (data: Data, item: PhoneNumber) => {
          data.phones.push(item);
        },
        remove: (data: Data, index: number) => {
          data.phones.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.phones[a], data.phones[b]] = [data.phones[b], data.phones[a]];
        },
      },
      role: {
        path: ["role"] as const,
        name: "role",
        constraints: { required: true },

        get: (data: Data) => data.role,
        set: (data: Data, value: string) => {
          data.role = value;
        },
        getError: (errors: Errors) => errors?.role,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.role = value;
        },
        getTainted: (tainted: Tainted) => tainted?.role ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.role = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      title: {
        path: ["title"] as const,
        name: "title",
        constraints: { required: true },

        get: (data: Data) => data.title,
        set: (data: Data, value: JobTitle) => {
          data.title = value;
        },
        getError: (errors: Errors) => errors?.title,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.title = value;
        },
        getTainted: (tainted: Tainted) => tainted?.title ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.title = value;
        },
        validate: (_value: JobTitle): Array<string> => [],
      },
      email: {
        path: ["email"] as const,
        name: "email",
        constraints: { required: true },

        get: (data: Data) => data.email,
        set: (data: Data, value: Email) => {
          data.email = value;
        },
        getError: (errors: Errors) => errors?.email,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.email = value;
        },
        getTainted: (tainted: Tainted) => tainted?.email ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.email = value;
        },
        validate: (_value: Email): Array<string> => [],
      },
      address: {
        path: ["address"] as const,
        name: "address",
        constraints: { required: true },

        get: (data: Data) => data.address,
        set: (data: Data, value: string) => {
          data.address = value;
        },
        getError: (errors: Errors) => errors?.address,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.address = value;
        },
        getTainted: (tainted: Tainted) => tainted?.address ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.address = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      username: {
        path: ["username"] as const,
        name: "username",
        constraints: { required: true },

        get: (data: Data) => data.username,
        set: (data: Data, value: string) => {
          data.username = value;
        },
        getError: (errors: Errors) => errors?.username,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.username = value;
        },
        getTainted: (tainted: Tainted) => tainted?.username ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.username = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      route: {
        path: ["route"] as const,
        name: "route",
        constraints: { required: true },

        get: (data: Data) => data.route,
        set: (data: Data, value: string | Route) => {
          data.route = value;
        },
        getError: (errors: Errors) => errors?.route,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.route = value;
        },
        getTainted: (tainted: Tainted) => tainted?.route ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.route = value;
        },
        validate: (_value: string | Route): Array<string> => [],
      },
      ratePerHour: {
        path: ["ratePerHour"] as const,
        name: "ratePerHour",
        constraints: { required: true },

        get: (data: Data) => data.ratePerHour,
        set: (data: Data, value: number) => {
          data.ratePerHour = value;
        },
        getError: (errors: Errors) => errors?.ratePerHour,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.ratePerHour = value;
        },
        getTainted: (tainted: Tainted) => tainted?.ratePerHour ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.ratePerHour = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      active: {
        path: ["active"] as const,
        name: "active",
        constraints: { required: true },

        get: (data: Data) => data.active,
        set: (data: Data, value: boolean) => {
          data.active = value;
        },
        getError: (errors: Errors) => errors?.active,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.active = value;
        },
        getTainted: (tainted: Tainted) => tainted?.active ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.active = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      isTechnician: {
        path: ["isTechnician"] as const,
        name: "isTechnician",
        constraints: { required: true },

        get: (data: Data) => data.isTechnician,
        set: (data: Data, value: boolean) => {
          data.isTechnician = value;
        },
        getError: (errors: Errors) => errors?.isTechnician,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.isTechnician = value;
        },
        getTainted: (tainted: Tainted) => tainted?.isTechnician ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.isTechnician = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      isSalesRep: {
        path: ["isSalesRep"] as const,
        name: "isSalesRep",
        constraints: { required: true },

        get: (data: Data) => data.isSalesRep,
        set: (data: Data, value: boolean) => {
          data.isSalesRep = value;
        },
        getError: (errors: Errors) => errors?.isSalesRep,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.isSalesRep = value;
        },
        getTainted: (tainted: Tainted) => tainted?.isSalesRep ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.isSalesRep = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      description: {
        path: ["description"] as const,
        name: "description",
        constraints: { required: true },

        get: (data: Data) => data.description,
        set: (data: Data, value: string | null) => {
          data.description = value;
        },
        getError: (errors: Errors) => errors?.description,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.description = value;
        },
        getTainted: (tainted: Tainted) => tainted?.description ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.description = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      linkedinUrl: {
        path: ["linkedinUrl"] as const,
        name: "linkedinUrl",
        constraints: { required: true },

        get: (data: Data) => data.linkedinUrl,
        set: (data: Data, value: string | null) => {
          data.linkedinUrl = value;
        },
        getError: (errors: Errors) => errors?.linkedinUrl,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.linkedinUrl = value;
        },
        getTainted: (tainted: Tainted) => tainted?.linkedinUrl ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.linkedinUrl = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      attendance: {
        path: ["attendance"] as const,
        name: "attendance",
        constraints: { required: true },

        get: (data: Data) => data.attendance,
        set: (data: Data, value: string[]) => {
          data.attendance = value;
        },
        getError: (errors: Errors) => errors?.attendance,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.attendance = value;
        },
        getTainted: (tainted: Tainted) => tainted?.attendance ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.attendance = value;
        },
        validate: (_value: string[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["attendance"], index] as const,
          name: `attendance.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.attendance[index],
          set: (data: Data, value: string) => {
            data.attendance[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.attendance as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.attendance ??= {};
            (errors.attendance as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: (tainted: Tainted) =>
            tainted.attendance?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.attendance ??= {};
            tainted.attendance[index] = value;
          },
          validate: (_value: string): Array<string> => [],
        }),
        push: (data: Data, item: string) => {
          data.attendance.push(item);
        },
        remove: (data: Data, index: number) => {
          data.attendance.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.attendance[a], data.attendance[b]] = [
            data.attendance[b],
            data.attendance[a],
          ];
        },
      },
      settings: {
        path: ["settings"] as const,
        name: "settings",
        constraints: { required: true },

        get: (data: Data) => data.settings,
        set: (data: Data, value: Settings) => {
          data.settings = value;
        },
        getError: (errors: Errors) => errors?.settings,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.settings = value;
        },
        getTainted: (tainted: Tainted) => tainted?.settings ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.settings = value;
        },
        validate: (_value: Settings): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Commissions): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Commissions, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Commissions.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Commissions;
  }
}

export namespace Commissions {
  export type Data = Commissions;
  export type Errors = {
    _errors?: Array<string>;
    technician?: Array<string>;
    salesRep?: Array<string>;
  };
  export type Tainted = { technician?: boolean; salesRep?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.technician = formData.get("technician") ?? "";
    obj.salesRep = formData.get("salesRep") ?? "";
    return Commissions.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      technician: {
        path: ["technician"] as const,
        name: "technician",
        constraints: { required: true },

        get: (data: Data) => data.technician,
        set: (data: Data, value: string) => {
          data.technician = value;
        },
        getError: (errors: Errors) => errors?.technician,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.technician = value;
        },
        getTainted: (tainted: Tainted) => tainted?.technician ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.technician = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      salesRep: {
        path: ["salesRep"] as const,
        name: "salesRep",
        constraints: { required: true },

        get: (data: Data) => data.salesRep,
        set: (data: Data, value: string) => {
          data.salesRep = value;
        },
        getError: (errors: Errors) => errors?.salesRep,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.salesRep = value;
        },
        getTainted: (tainted: Tainted) => tainted?.salesRep ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.salesRep = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: Number): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Number, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Number.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Number;
  }
}

export namespace Number {
  export type Data = Number;
  export type Errors = {
    _errors?: Array<string>;
    countryCode?: Array<string>;
    areaCode?: Array<string>;
    localNumber?: Array<string>;
  };
  export type Tainted = {
    countryCode?: boolean;
    areaCode?: boolean;
    localNumber?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.countryCode = formData.get("countryCode") ?? "";
    obj.areaCode = formData.get("areaCode") ?? "";
    obj.localNumber = formData.get("localNumber") ?? "";
    return Number.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      countryCode: {
        path: ["countryCode"] as const,
        name: "countryCode",
        constraints: { required: true },

        get: (data: Data) => data.countryCode,
        set: (data: Data, value: string) => {
          data.countryCode = value;
        },
        getError: (errors: Errors) => errors?.countryCode,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.countryCode = value;
        },
        getTainted: (tainted: Tainted) => tainted?.countryCode ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.countryCode = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      areaCode: {
        path: ["areaCode"] as const,
        name: "areaCode",
        constraints: { required: true },

        get: (data: Data) => data.areaCode,
        set: (data: Data, value: string) => {
          data.areaCode = value;
        },
        getError: (errors: Errors) => errors?.areaCode,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.areaCode = value;
        },
        getTainted: (tainted: Tainted) => tainted?.areaCode ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.areaCode = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      localNumber: {
        path: ["localNumber"] as const,
        name: "localNumber",
        constraints: { required: true },

        get: (data: Data) => data.localNumber,
        set: (data: Data, value: string) => {
          data.localNumber = value;
        },
        getError: (errors: Errors) => errors?.localNumber,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.localNumber = value;
        },
        getTainted: (tainted: Tainted) => tainted?.localNumber ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.localNumber = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: DataPath): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<DataPath, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "DataPath.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as DataPath;
  }
}

export namespace DataPath {
  export type Data = DataPath;
  export type Errors = {
    _errors?: Array<string>;
    path?: { _errors?: Array<string>; [index: number]: Array<string> };
    formatter?: Array<string>;
  };
  export type Tainted = {
    path?: { [index: number]: boolean };
    formatter?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.path = formData.getAll("path") as Array<string>;
    obj.formatter = formData.get("formatter") ?? "";
    return DataPath.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      path: {
        path: ["path"] as const,
        name: "path",
        constraints: { required: true },

        get: (data: Data) => data.path,
        set: (data: Data, value: string[]) => {
          data.path = value;
        },
        getError: (errors: Errors) => errors?.path,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.path = value;
        },
        getTainted: (tainted: Tainted) => tainted?.path ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.path = value;
        },
        validate: (_value: string[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["path"], index] as const,
          name: `path.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.path[index],
          set: (data: Data, value: string) => {
            data.path[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.path as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.path ??= {};
            (errors.path as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.path?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.path ??= {};
            tainted.path[index] = value;
          },
          validate: (_value: string): Array<string> => [],
        }),
        push: (data: Data, item: string) => {
          data.path.push(item);
        },
        remove: (data: Data, index: number) => {
          data.path.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.path[a], data.path[b]] = [data.path[b], data.path[a]];
        },
      },
      formatter: {
        path: ["formatter"] as const,
        name: "formatter",
        constraints: { required: true },

        get: (data: Data) => data.formatter,
        set: (data: Data, value: string | null) => {
          data.formatter = value;
        },
        getError: (errors: Errors) => errors?.formatter,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.formatter = value;
        },
        getTainted: (tainted: Tainted) => tainted?.formatter ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.formatter = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Route): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Route, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Route.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Route;
  }
}

export namespace Route {
  export type Data = Route;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    techs?: Array<string>;
    active?: Array<string>;
    name?: Array<string>;
    phone?: Array<string>;
    position?: Array<string>;
    serviceRoute?: Array<string>;
    defaultDurationHours?: Array<string>;
    tags?: { _errors?: Array<string>; [index: number]: Array<string> };
    icon?: Array<string>;
    color?: Array<string>;
  };
  export type Tainted = {
    id?: boolean;
    techs?: boolean;
    active?: boolean;
    name?: boolean;
    phone?: boolean;
    position?: boolean;
    serviceRoute?: boolean;
    defaultDurationHours?: boolean;
    tags?: { [index: number]: boolean };
    icon?: boolean;
    color?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.techs = formData.get("techs") ?? "";
    {
      const activeVal = formData.get("active");
      obj.active =
        activeVal === "true" || activeVal === "on" || activeVal === "1";
    }
    obj.name = formData.get("name") ?? "";
    obj.phone = formData.get("phone") ?? "";
    obj.position = formData.get("position") ?? "";
    {
      const serviceRouteVal = formData.get("serviceRoute");
      obj.serviceRoute =
        serviceRouteVal === "true" ||
        serviceRouteVal === "on" ||
        serviceRouteVal === "1";
    }
    {
      const defaultDurationHoursStr = formData.get("defaultDurationHours");
      obj.defaultDurationHours = defaultDurationHoursStr
        ? parseFloat(defaultDurationHoursStr as string)
        : 0;
      if (
        obj.defaultDurationHours !== undefined &&
        Number.isNaN(obj.defaultDurationHours)
      )
        obj.defaultDurationHours = 0;
    }
    obj.tags = formData.getAll("tags") as Array<string>;
    obj.icon = formData.get("icon") ?? "";
    obj.color = formData.get("color") ?? "";
    return Route.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      techs: {
        path: ["techs"] as const,
        name: "techs",
        constraints: { required: true },

        get: (data: Data) => data.techs,
        set: (data: Data, value: (string | Employee)[] | null) => {
          data.techs = value;
        },
        getError: (errors: Errors) => errors?.techs,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.techs = value;
        },
        getTainted: (tainted: Tainted) => tainted?.techs ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.techs = value;
        },
        validate: (_value: (string | Employee)[] | null): Array<string> => [],
      },
      active: {
        path: ["active"] as const,
        name: "active",
        constraints: { required: true },

        get: (data: Data) => data.active,
        set: (data: Data, value: boolean) => {
          data.active = value;
        },
        getError: (errors: Errors) => errors?.active,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.active = value;
        },
        getTainted: (tainted: Tainted) => tainted?.active ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.active = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: (data: Data) => data.name,
        set: (data: Data, value: string) => {
          data.name = value;
        },
        getError: (errors: Errors) => errors?.name,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: (tainted: Tainted) => tainted?.name ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.name = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      phone: {
        path: ["phone"] as const,
        name: "phone",
        constraints: { required: true },

        get: (data: Data) => data.phone,
        set: (data: Data, value: string) => {
          data.phone = value;
        },
        getError: (errors: Errors) => errors?.phone,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.phone = value;
        },
        getTainted: (tainted: Tainted) => tainted?.phone ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.phone = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      position: {
        path: ["position"] as const,
        name: "position",
        constraints: { required: true },

        get: (data: Data) => data.position,
        set: (data: Data, value: string) => {
          data.position = value;
        },
        getError: (errors: Errors) => errors?.position,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.position = value;
        },
        getTainted: (tainted: Tainted) => tainted?.position ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.position = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      serviceRoute: {
        path: ["serviceRoute"] as const,
        name: "serviceRoute",
        constraints: { required: true },

        get: (data: Data) => data.serviceRoute,
        set: (data: Data, value: boolean) => {
          data.serviceRoute = value;
        },
        getError: (errors: Errors) => errors?.serviceRoute,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.serviceRoute = value;
        },
        getTainted: (tainted: Tainted) => tainted?.serviceRoute ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.serviceRoute = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      defaultDurationHours: {
        path: ["defaultDurationHours"] as const,
        name: "defaultDurationHours",
        constraints: { required: true },

        get: (data: Data) => data.defaultDurationHours,
        set: (data: Data, value: number) => {
          data.defaultDurationHours = value;
        },
        getError: (errors: Errors) => errors?.defaultDurationHours,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.defaultDurationHours = value;
        },
        getTainted: (tainted: Tainted) =>
          tainted?.defaultDurationHours ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.defaultDurationHours = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      tags: {
        path: ["tags"] as const,
        name: "tags",
        constraints: { required: true },

        get: (data: Data) => data.tags,
        set: (data: Data, value: string[]) => {
          data.tags = value;
        },
        getError: (errors: Errors) => errors?.tags,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.tags = value;
        },
        getTainted: (tainted: Tainted) => tainted?.tags ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.tags = value;
        },
        validate: (_value: string[]): Array<string> => [],

        at: (index: number) => ({
          path: [...["tags"], index] as const,
          name: `tags.${index}`,
          constraints: { required: true },
          get: (data: Data) => data.tags[index],
          set: (data: Data, value: string) => {
            data.tags[index] = value;
          },
          getError: (errors: Errors) =>
            (errors.tags as Record<number, Array<string>>)?.[index],
          setError: (errors: Errors, value: Array<string> | undefined) => {
            errors.tags ??= {};
            (errors.tags as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: (tainted: Tainted) => tainted.tags?.[index] ?? false,
          setTainted: (tainted: Tainted, value: boolean) => {
            tainted.tags ??= {};
            tainted.tags[index] = value;
          },
          validate: (_value: string): Array<string> => [],
        }),
        push: (data: Data, item: string) => {
          data.tags.push(item);
        },
        remove: (data: Data, index: number) => {
          data.tags.splice(index, 1);
        },
        swap: (data: Data, a: number, b: number) => {
          [data.tags[a], data.tags[b]] = [data.tags[b], data.tags[a]];
        },
      },
      icon: {
        path: ["icon"] as const,
        name: "icon",
        constraints: { required: true },

        get: (data: Data) => data.icon,
        set: (data: Data, value: string | null) => {
          data.icon = value;
        },
        getError: (errors: Errors) => errors?.icon,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.icon = value;
        },
        getTainted: (tainted: Tainted) => tainted?.icon ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.icon = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      color: {
        path: ["color"] as const,
        name: "color",
        constraints: { required: true },

        get: (data: Data) => data.color,
        set: (data: Data, value: string | null) => {
          data.color = value;
        },
        getError: (errors: Errors) => errors?.color,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.color = value;
        },
        getTainted: (tainted: Tainted) => tainted?.color ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.color = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: EmailParts): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<EmailParts, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "EmailParts.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as EmailParts;
  }
}

export namespace EmailParts {
  export type Data = EmailParts;
  export type Errors = {
    _errors?: Array<string>;
    local?: Array<string>;
    domainName?: Array<string>;
    topLevelDomain?: Array<string>;
  };
  export type Tainted = {
    local?: boolean;
    domainName?: boolean;
    topLevelDomain?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.local = formData.get("local") ?? "";
    obj.domainName = formData.get("domainName") ?? "";
    obj.topLevelDomain = formData.get("topLevelDomain") ?? "";
    return EmailParts.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      local: {
        path: ["local"] as const,
        name: "local",
        constraints: { required: true },

        get: (data: Data) => data.local,
        set: (data: Data, value: string) => {
          data.local = value;
        },
        getError: (errors: Errors) => errors?.local,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.local = value;
        },
        getTainted: (tainted: Tainted) => tainted?.local ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.local = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      domainName: {
        path: ["domainName"] as const,
        name: "domainName",
        constraints: { required: true },

        get: (data: Data) => data.domainName,
        set: (data: Data, value: string) => {
          data.domainName = value;
        },
        getError: (errors: Errors) => errors?.domainName,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.domainName = value;
        },
        getTainted: (tainted: Tainted) => tainted?.domainName ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.domainName = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
      topLevelDomain: {
        path: ["topLevelDomain"] as const,
        name: "topLevelDomain",
        constraints: { required: true },

        get: (data: Data) => data.topLevelDomain,
        set: (data: Data, value: string) => {
          data.topLevelDomain = value;
        },
        getError: (errors: Errors) => errors?.topLevelDomain,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.topLevelDomain = value;
        },
        getTainted: (tainted: Tainted) => tainted?.topLevelDomain ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.topLevelDomain = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: Sent): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Sent, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Sent.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Sent;
  }
}

export namespace Sent {
  export type Data = Sent;
  export type Errors = {
    _errors?: Array<string>;
    recipient?: Array<string>;
    method?: Array<string>;
  };
  export type Tainted = { recipient?: boolean; method?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.recipient = formData.get("recipient") ?? "";
    obj.method = formData.get("method") ?? "";
    return Sent.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      recipient: {
        path: ["recipient"] as const,
        name: "recipient",
        constraints: { required: true },

        get: (data: Data) => data.recipient,
        set: (data: Data, value: string | null) => {
          data.recipient = value;
        },
        getError: (errors: Errors) => errors?.recipient,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.recipient = value;
        },
        getTainted: (tainted: Tainted) => tainted?.recipient ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.recipient = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
      method: {
        path: ["method"] as const,
        name: "method",
        constraints: { required: true },

        get: (data: Data) => data.method,
        set: (data: Data, value: string | null) => {
          data.method = value;
        },
        getError: (errors: Errors) => errors?.method,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.method = value;
        },
        getTainted: (tainted: Tainted) => tainted?.method ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.method = value;
        },
        validate: (_value: string | null): Array<string> => [],
      },
    } as const;
}

/**  */
export interface BilledItem {
  item: Item;
  quantity: number;
  taxed: boolean;
  upsale: boolean;
}

export namespace BilledItem {
  export function toStringifiedJSON(self: BilledItem): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<BilledItem, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "BilledItem.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as BilledItem;
  }
}

export namespace BilledItem {
  export type Data = BilledItem;
  export type Errors = {
    _errors?: Array<string>;
    item?: Item.Errors;
    quantity?: Array<string>;
    taxed?: Array<string>;
    upsale?: Array<string>;
  };
  export type Tainted = {
    item?: Item.Tainted;
    quantity?: boolean;
    taxed?: boolean;
    upsale?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      // Collect nested object fields with prefix "item."
      const itemObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("item.")) {
          const fieldName = key.slice("item.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = itemObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.item = itemObj;
    }
    {
      const quantityStr = formData.get("quantity");
      obj.quantity = quantityStr ? parseFloat(quantityStr as string) : 0;
      if (obj.quantity !== undefined && Number.isNaN(obj.quantity))
        obj.quantity = 0;
    }
    {
      const taxedVal = formData.get("taxed");
      obj.taxed = taxedVal === "true" || taxedVal === "on" || taxedVal === "1";
    }
    {
      const upsaleVal = formData.get("upsale");
      obj.upsale =
        upsaleVal === "true" || upsaleVal === "on" || upsaleVal === "1";
    }
    return BilledItem.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      item: {
        path: ["item"] as const,
        name: "item",
        constraints: { required: true },

        get: (data: Data) => data.item,
        set: (data: Data, value: Item) => {
          data.item = value;
        },
        getError: (errors: Errors) => errors?.item,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.item = value;
        },
        getTainted: (tainted: Tainted) => tainted?.item ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.item = value;
        },
        validate: (_value: Item): Array<string> => [],
      },
      quantity: {
        path: ["quantity"] as const,
        name: "quantity",
        constraints: { required: true },

        get: (data: Data) => data.quantity,
        set: (data: Data, value: number) => {
          data.quantity = value;
        },
        getError: (errors: Errors) => errors?.quantity,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.quantity = value;
        },
        getTainted: (tainted: Tainted) => tainted?.quantity ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.quantity = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      taxed: {
        path: ["taxed"] as const,
        name: "taxed",
        constraints: { required: true },

        get: (data: Data) => data.taxed,
        set: (data: Data, value: boolean) => {
          data.taxed = value;
        },
        getError: (errors: Errors) => errors?.taxed,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.taxed = value;
        },
        getTainted: (tainted: Tainted) => tainted?.taxed ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.taxed = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      upsale: {
        path: ["upsale"] as const,
        name: "upsale",
        constraints: { required: true },

        get: (data: Data) => data.upsale,
        set: (data: Data, value: boolean) => {
          data.upsale = value;
        },
        getError: (errors: Errors) => errors?.upsale,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.upsale = value;
        },
        getTainted: (tainted: Tainted) => tainted?.upsale ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.upsale = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Coordinates): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Coordinates, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Coordinates.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Coordinates;
  }
}

export namespace Coordinates {
  export type Data = Coordinates;
  export type Errors = {
    _errors?: Array<string>;
    lat?: Array<string>;
    lng?: Array<string>;
  };
  export type Tainted = { lat?: boolean; lng?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const latStr = formData.get("lat");
      obj.lat = latStr ? parseFloat(latStr as string) : 0;
      if (obj.lat !== undefined && Number.isNaN(obj.lat)) obj.lat = 0;
    }
    {
      const lngStr = formData.get("lng");
      obj.lng = lngStr ? parseFloat(lngStr as string) : 0;
      if (obj.lng !== undefined && Number.isNaN(obj.lng)) obj.lng = 0;
    }
    return Coordinates.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      lat: {
        path: ["lat"] as const,
        name: "lat",
        constraints: { required: true },

        get: (data: Data) => data.lat,
        set: (data: Data, value: number) => {
          data.lat = value;
        },
        getError: (errors: Errors) => errors?.lat,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.lat = value;
        },
        getTainted: (tainted: Tainted) => tainted?.lat ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.lat = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      lng: {
        path: ["lng"] as const,
        name: "lng",
        constraints: { required: true },

        get: (data: Data) => data.lng,
        set: (data: Data, value: number) => {
          data.lng = value;
        },
        getError: (errors: Errors) => errors?.lng,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.lng = value;
        },
        getTainted: (tainted: Tainted) => tainted?.lng ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.lng = value;
        },
        validate: (_value: number): Array<string> => [],
      },
    } as const;
}

/**  */
export interface Ordered {
  id: string;
  in: string | Account;
  out: string | Order;
  date: string;
}

export namespace Ordered {
  export function toStringifiedJSON(self: Ordered): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Ordered, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Ordered.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Ordered;
  }
}

export namespace Ordered {
  export type Data = Ordered;
  export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    in?: Array<string>;
    out?: Array<string>;
    date?: Array<string>;
  };
  export type Tainted = {
    id?: boolean;
    in?: boolean;
    out?: boolean;
    date?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.in = formData.get("in") ?? "";
    obj.out = formData.get("out") ?? "";
    obj.date = formData.get("date") ?? "";
    return Ordered.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: (data: Data) => data.id,
        set: (data: Data, value: string) => {
          data.id = value;
        },
        getError: (errors: Errors) => errors?.id,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: (tainted: Tainted) => tainted?.id ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.id = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      in: {
        path: ["in"] as const,
        name: "in",
        constraints: { required: true },

        get: (data: Data) => data.in,
        set: (data: Data, value: string | Account) => {
          data.in = value;
        },
        getError: (errors: Errors) => errors?.in,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.in = value;
        },
        getTainted: (tainted: Tainted) => tainted?.in ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.in = value;
        },
        validate: (_value: string | Account): Array<string> => [],
      },
      out: {
        path: ["out"] as const,
        name: "out",
        constraints: { required: true },

        get: (data: Data) => data.out,
        set: (data: Data, value: string | Order) => {
          data.out = value;
        },
        getError: (errors: Errors) => errors?.out,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.out = value;
        },
        getTainted: (tainted: Tainted) => tainted?.out ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.out = value;
        },
        validate: (_value: string | Order): Array<string> => [],
      },
      date: {
        path: ["date"] as const,
        name: "date",
        constraints: { required: true },

        get: (data: Data) => data.date,
        set: (data: Data, value: string) => {
          data.date = value;
        },
        getError: (errors: Errors) => errors?.date,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.date = value;
        },
        getTainted: (tainted: Tainted) => tainted?.date ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.date = value;
        },
        validate: (_value: string): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: Email): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Email, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Email.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Email;
  }
}

export namespace Email {
  export type Data = Email;
  export type Errors = {
    _errors?: Array<string>;
    canEmail?: Array<string>;
    emailString?: Array<string>;
  };
  export type Tainted = { canEmail?: boolean; emailString?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const canEmailVal = formData.get("canEmail");
      obj.canEmail =
        canEmailVal === "true" || canEmailVal === "on" || canEmailVal === "1";
    }
    obj.emailString = formData.get("emailString") ?? "";
    return Email.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      canEmail: {
        path: ["canEmail"] as const,
        name: "canEmail",
        constraints: { required: true },

        get: (data: Data) => data.canEmail,
        set: (data: Data, value: boolean) => {
          data.canEmail = value;
        },
        getError: (errors: Errors) => errors?.canEmail,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.canEmail = value;
        },
        getTainted: (tainted: Tainted) => tainted?.canEmail ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.canEmail = value;
        },
        validate: (_value: boolean): Array<string> => [],
      },
      emailString: {
        path: ["emailString"] as const,
        name: "emailString",
        constraints: { required: true },

        get: (data: Data) => data.emailString,
        set: (data: Data, value: string) => {
          data.emailString = value;
        },
        getError: (errors: Errors) => errors?.emailString,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.emailString = value;
        },
        getTainted: (tainted: Tainted) => tainted?.emailString ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.emailString = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: RecurrenceRule): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<RecurrenceRule, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "RecurrenceRule.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as RecurrenceRule;
  }
}

export namespace RecurrenceRule {
  export type Data = RecurrenceRule;
  export type Errors = {
    _errors?: Array<string>;
    interval?: Interval.Errors;
    recurrenceBegins?: Array<string>;
    recurrenceEnds?: Array<string>;
    cancelledInstances?: Array<string>;
    additionalInstances?: Array<string>;
  };
  export type Tainted = {
    interval?: Interval.Tainted;
    recurrenceBegins?: boolean;
    recurrenceEnds?: boolean;
    cancelledInstances?: boolean;
    additionalInstances?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      // Collect nested object fields with prefix "interval."
      const intervalObj: Record<string, unknown> = {};
      for (const [key, value] of formData.entries()) {
        if (key.startsWith("interval.")) {
          const fieldName = key.slice("interval.".length);
          // Handle deeper nesting by splitting on dots
          const parts = fieldName.split(".");
          let current = intervalObj;
          for (let i = 0; i < parts.length - 1; i++) {
            const part = parts[i];
            if (!(part in current)) {
              current[part] = {};
            }
            current = current[part] as Record<string, unknown>;
          }
          current[parts[parts.length - 1]] = value;
        }
      }
      obj.interval = intervalObj;
    }
    obj.recurrenceBegins = formData.get("recurrenceBegins") ?? "";
    obj.recurrenceEnds = formData.get("recurrenceEnds") ?? "";
    obj.cancelledInstances = formData.get("cancelledInstances") ?? "";
    obj.additionalInstances = formData.get("additionalInstances") ?? "";
    return RecurrenceRule.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      interval: {
        path: ["interval"] as const,
        name: "interval",
        constraints: { required: true },

        get: (data: Data) => data.interval,
        set: (data: Data, value: Interval) => {
          data.interval = value;
        },
        getError: (errors: Errors) => errors?.interval,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.interval = value;
        },
        getTainted: (tainted: Tainted) => tainted?.interval ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.interval = value;
        },
        validate: (_value: Interval): Array<string> => [],
      },
      recurrenceBegins: {
        path: ["recurrenceBegins"] as const,
        name: "recurrenceBegins",
        constraints: { required: true },

        get: (data: Data) => data.recurrenceBegins,
        set: (data: Data, value: string) => {
          data.recurrenceBegins = value;
        },
        getError: (errors: Errors) => errors?.recurrenceBegins,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.recurrenceBegins = value;
        },
        getTainted: (tainted: Tainted) => tainted?.recurrenceBegins ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.recurrenceBegins = value;
        },
        validate: (_value: string): Array<string> => [],
      },
      recurrenceEnds: {
        path: ["recurrenceEnds"] as const,
        name: "recurrenceEnds",
        constraints: { required: true },

        get: (data: Data) => data.recurrenceEnds,
        set: (data: Data, value: RecurrenceEnd | null) => {
          data.recurrenceEnds = value;
        },
        getError: (errors: Errors) => errors?.recurrenceEnds,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.recurrenceEnds = value;
        },
        getTainted: (tainted: Tainted) => tainted?.recurrenceEnds ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.recurrenceEnds = value;
        },
        validate: (_value: RecurrenceEnd | null): Array<string> => [],
      },
      cancelledInstances: {
        path: ["cancelledInstances"] as const,
        name: "cancelledInstances",
        constraints: { required: true },

        get: (data: Data) => data.cancelledInstances,
        set: (data: Data, value: string[] | null) => {
          data.cancelledInstances = value;
        },
        getError: (errors: Errors) => errors?.cancelledInstances,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.cancelledInstances = value;
        },
        getTainted: (tainted: Tainted) => tainted?.cancelledInstances ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.cancelledInstances = value;
        },
        validate: (_value: string[] | null): Array<string> => [],
      },
      additionalInstances: {
        path: ["additionalInstances"] as const,
        name: "additionalInstances",
        constraints: { required: true },

        get: (data: Data) => data.additionalInstances,
        set: (data: Data, value: string[] | null) => {
          data.additionalInstances = value;
        },
        getError: (errors: Errors) => errors?.additionalInstances,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.additionalInstances = value;
        },
        getTainted: (tainted: Tainted) => tainted?.additionalInstances ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.additionalInstances = value;
        },
        validate: (_value: string[] | null): Array<string> => [],
      },
    } as const;
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
  export function toStringifiedJSON(self: LastName): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<LastName, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "LastName.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as LastName;
  }
}

export namespace LastName {
  export type Data = LastName;
  export type Errors = { _errors?: Array<string>; name?: Array<string> };
  export type Tainted = { name?: boolean };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    obj.name = formData.get("name") ?? "";
    return LastName.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: (data: Data) => data.name,
        set: (data: Data, value: string) => {
          data.name = value;
        },
        getError: (errors: Errors) => errors?.name,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: (tainted: Tainted) => tainted?.name ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.name = value;
        },
        validate: (value: string): Array<string> => {
          const errors: Array<string> = [];
          if (typeof value === "string" && value.length === 0)
            errors.push("Must not be empty");
          return errors;
        },
      },
    } as const;
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
  export function toStringifiedJSON(self: Cardinal): string {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Result<Cardinal, string[]> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          "Cardinal.fromStringifiedJSON: root cannot be a forward reference",
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
    if (errors.length > 0) {
      throw new Error(errors.join("; "));
    }
    return instance as Cardinal;
  }
}

export namespace Cardinal {
  export type Data = Cardinal;
  export type Errors = {
    _errors?: Array<string>;
    north?: Array<string>;
    east?: Array<string>;
    south?: Array<string>;
    west?: Array<string>;
  };
  export type Tainted = {
    north?: boolean;
    east?: boolean;
    south?: boolean;
    west?: boolean;
  };
  /** ParsesFormDataandvalidatesit,returningaResultwiththeparseddataorerrors.DelegatesvalidationtofromJSON()from@derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Data, Errors> {
    const obj: Record<string, unknown> = {};
    {
      const northStr = formData.get("north");
      obj.north = northStr ? parseFloat(northStr as string) : 0;
      if (obj.north !== undefined && Number.isNaN(obj.north)) obj.north = 0;
    }
    {
      const eastStr = formData.get("east");
      obj.east = eastStr ? parseFloat(eastStr as string) : 0;
      if (obj.east !== undefined && Number.isNaN(obj.east)) obj.east = 0;
    }
    {
      const southStr = formData.get("south");
      obj.south = southStr ? parseFloat(southStr as string) : 0;
      if (obj.south !== undefined && Number.isNaN(obj.south)) obj.south = 0;
    }
    {
      const westStr = formData.get("west");
      obj.west = westStr ? parseFloat(westStr as string) : 0;
      if (obj.west !== undefined && Number.isNaN(obj.west)) obj.west = 0;
    }
    return Cardinal.fromJSON(obj);
  }
  /** Fielddescriptorswithtype-safeaccessorsandvalidation. */ export const fields =
    {
      north: {
        path: ["north"] as const,
        name: "north",
        constraints: { required: true },

        get: (data: Data) => data.north,
        set: (data: Data, value: number) => {
          data.north = value;
        },
        getError: (errors: Errors) => errors?.north,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.north = value;
        },
        getTainted: (tainted: Tainted) => tainted?.north ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.north = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      east: {
        path: ["east"] as const,
        name: "east",
        constraints: { required: true },

        get: (data: Data) => data.east,
        set: (data: Data, value: number) => {
          data.east = value;
        },
        getError: (errors: Errors) => errors?.east,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.east = value;
        },
        getTainted: (tainted: Tainted) => tainted?.east ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.east = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      south: {
        path: ["south"] as const,
        name: "south",
        constraints: { required: true },

        get: (data: Data) => data.south,
        set: (data: Data, value: number) => {
          data.south = value;
        },
        getError: (errors: Errors) => errors?.south,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.south = value;
        },
        getTainted: (tainted: Tainted) => tainted?.south ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.south = value;
        },
        validate: (_value: number): Array<string> => [],
      },
      west: {
        path: ["west"] as const,
        name: "west",
        constraints: { required: true },

        get: (data: Data) => data.west,
        set: (data: Data, value: number) => {
          data.west = value;
        },
        getError: (errors: Errors) => errors?.west,
        setError: (errors: Errors, value: Array<string> | undefined) => {
          errors.west = value;
        },
        getTainted: (tainted: Tainted) => tainted?.west ?? false,
        setTainted: (tainted: Tainted, value: boolean) => {
          tainted.west = value;
        },
        validate: (_value: number): Array<string> => [],
      },
    } as const;
}

/**  */
export type Interval =
  | DailyRecurrenceRule
  | WeeklyRecurrenceRule
  | MonthlyRecurrenceRule
  | YearlyRecurrenceRule;

export namespace Interval {
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Interval {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Page {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): UserRole {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Target {
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
  export function fromStringifiedJSON(
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
  export function fromStringifiedJSON(
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
  export function fromStringifiedJSON(
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Sector {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Weekday {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Status {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): NextStep {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): LeadStage {
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
  export function fromStringifiedJSON(
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Priority {
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
  export function fromStringifiedJSON(
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): JobTitle {
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
  export function fromStringifiedJSON(
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
  export function fromStringifiedJSON(
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
  export function fromStringifiedJSON(
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): RowHeight {
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
  export function fromStringifiedJSON(
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Table {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Item {
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
  export function fromStringifiedJSON(
    json: string,
    opts?: DeserializeOptions,
  ): Actor {
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
