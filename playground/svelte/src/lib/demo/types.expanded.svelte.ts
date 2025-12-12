import { SerializeContext } from "macroforge/serde";
import { Result } from "macroforge/result";
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
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
  export function defaultValue(): User {
    return {
      id: "",
      email: null,
      firstName: "",
      lastName: "",
      password: null,
      metadata: null,
      settings: Settings.defaultValue(),
      role: "Administrator",
      emailVerified: false,
      verificationToken: null,
      verificationExpires: null,
      passwordResetToken: null,
      passwordResetExpires: null,
      permissions: AppPermissions.defaultValue(),
    } as User;
  }
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
  ): Result<User, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "User.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "User.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("email" in obj)) {
      errors.push({ field: "email", message: "missing required field" });
    }
    if (!("firstName" in obj)) {
      errors.push({ field: "firstName", message: "missing required field" });
    }
    if (!("lastName" in obj)) {
      errors.push({ field: "lastName", message: "missing required field" });
    }
    if (!("password" in obj)) {
      errors.push({ field: "password", message: "missing required field" });
    }
    if (!("metadata" in obj)) {
      errors.push({ field: "metadata", message: "missing required field" });
    }
    if (!("settings" in obj)) {
      errors.push({ field: "settings", message: "missing required field" });
    }
    if (!("role" in obj)) {
      errors.push({ field: "role", message: "missing required field" });
    }
    if (!("emailVerified" in obj)) {
      errors.push({
        field: "emailVerified",
        message: "missing required field",
      });
    }
    if (!("verificationToken" in obj)) {
      errors.push({
        field: "verificationToken",
        message: "missing required field",
      });
    }
    if (!("verificationExpires" in obj)) {
      errors.push({
        field: "verificationExpires",
        message: "missing required field",
      });
    }
    if (!("passwordResetToken" in obj)) {
      errors.push({
        field: "passwordResetToken",
        message: "missing required field",
      });
    }
    if (!("passwordResetExpires" in obj)) {
      errors.push({
        field: "passwordResetExpires",
        message: "missing required field",
      });
    }
    if (!("permissions" in obj)) {
      errors.push({ field: "permissions", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as User;
  }
}

export namespace User {
  /** Nested error structure matching the data shape */ export type Errors = {
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
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
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
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly email: FieldController<string | null>;
    readonly firstName: FieldController<string>;
    readonly lastName: FieldController<string>;
    readonly password: FieldController<string | null>;
    readonly metadata: FieldController<Metadata | null>;
    readonly settings: FieldController<Settings>;
    readonly role: FieldController<UserRole>;
    readonly emailVerified: FieldController<boolean>;
    readonly verificationToken: FieldController<string | null>;
    readonly verificationExpires: FieldController<string | null>;
    readonly passwordResetToken: FieldController<string | null>;
    readonly passwordResetExpires: FieldController<string | null>;
    readonly permissions: FieldController<AppPermissions>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: User;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<User, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<User>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<User>,
  ): Gigaform {
    let data = $state({ ...User.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = User.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      email: {
        path: ["email"] as const,
        name: "email",
        constraints: { required: true },

        get: () => data.email,
        set: (value: string | null) => {
          data.email = value;
        },
        getError: () => errors?.email,
        setError: (value: Array<string> | undefined) => {
          errors.email = value;
        },
        getTainted: () => tainted?.email ?? false,
        setTainted: (value: boolean) => {
          tainted.email = value;
        },
        validate: (): Array<string> => {
          const result = User.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "email")
              .map((e) => e.message);
          }
          return [];
        },
      },
      firstName: {
        path: ["firstName"] as const,
        name: "firstName",
        constraints: { required: true },

        get: () => data.firstName,
        set: (value: string) => {
          data.firstName = value;
        },
        getError: () => errors?.firstName,
        setError: (value: Array<string> | undefined) => {
          errors.firstName = value;
        },
        getTainted: () => tainted?.firstName ?? false,
        setTainted: (value: boolean) => {
          tainted.firstName = value;
        },
        validate: (): Array<string> => {
          const result = User.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "firstName")
              .map((e) => e.message);
          }
          return [];
        },
      },
      lastName: {
        path: ["lastName"] as const,
        name: "lastName",
        constraints: { required: true },

        get: () => data.lastName,
        set: (value: string) => {
          data.lastName = value;
        },
        getError: () => errors?.lastName,
        setError: (value: Array<string> | undefined) => {
          errors.lastName = value;
        },
        getTainted: () => tainted?.lastName ?? false,
        setTainted: (value: boolean) => {
          tainted.lastName = value;
        },
        validate: (): Array<string> => {
          const result = User.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "lastName")
              .map((e) => e.message);
          }
          return [];
        },
      },
      password: {
        path: ["password"] as const,
        name: "password",
        constraints: { required: true },

        get: () => data.password,
        set: (value: string | null) => {
          data.password = value;
        },
        getError: () => errors?.password,
        setError: (value: Array<string> | undefined) => {
          errors.password = value;
        },
        getTainted: () => tainted?.password ?? false,
        setTainted: (value: boolean) => {
          tainted.password = value;
        },
        validate: (): Array<string> => {
          const result = User.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "password")
              .map((e) => e.message);
          }
          return [];
        },
      },
      metadata: {
        path: ["metadata"] as const,
        name: "metadata",
        constraints: { required: true },

        get: () => data.metadata,
        set: (value: Metadata | null) => {
          data.metadata = value;
        },
        getError: () => errors?.metadata,
        setError: (value: Array<string> | undefined) => {
          errors.metadata = value;
        },
        getTainted: () => tainted?.metadata ?? false,
        setTainted: (value: boolean) => {
          tainted.metadata = value;
        },
        validate: (): Array<string> => {
          const result = User.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "metadata")
              .map((e) => e.message);
          }
          return [];
        },
      },
      settings: {
        path: ["settings"] as const,
        name: "settings",
        constraints: { required: true },

        get: () => data.settings,
        set: (value: Settings) => {
          data.settings = value;
        },
        getError: () => errors?.settings,
        setError: (value: Array<string> | undefined) => {
          errors.settings = value;
        },
        getTainted: () => tainted?.settings ?? false,
        setTainted: (value: boolean) => {
          tainted.settings = value;
        },
        validate: (): Array<string> => {
          const result = User.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "settings")
              .map((e) => e.message);
          }
          return [];
        },
      },
      role: {
        path: ["role"] as const,
        name: "role",
        constraints: { required: true },

        get: () => data.role,
        set: (value: UserRole) => {
          data.role = value;
        },
        getError: () => errors?.role,
        setError: (value: Array<string> | undefined) => {
          errors.role = value;
        },
        getTainted: () => tainted?.role ?? false,
        setTainted: (value: boolean) => {
          tainted.role = value;
        },
        validate: (): Array<string> => {
          const result = User.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "role")
              .map((e) => e.message);
          }
          return [];
        },
      },
      emailVerified: {
        path: ["emailVerified"] as const,
        name: "emailVerified",
        constraints: { required: true },

        get: () => data.emailVerified,
        set: (value: boolean) => {
          data.emailVerified = value;
        },
        getError: () => errors?.emailVerified,
        setError: (value: Array<string> | undefined) => {
          errors.emailVerified = value;
        },
        getTainted: () => tainted?.emailVerified ?? false,
        setTainted: (value: boolean) => {
          tainted.emailVerified = value;
        },
        validate: (): Array<string> => {
          const result = User.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "emailVerified")
              .map((e) => e.message);
          }
          return [];
        },
      },
      verificationToken: {
        path: ["verificationToken"] as const,
        name: "verificationToken",
        constraints: { required: true },

        get: () => data.verificationToken,
        set: (value: string | null) => {
          data.verificationToken = value;
        },
        getError: () => errors?.verificationToken,
        setError: (value: Array<string> | undefined) => {
          errors.verificationToken = value;
        },
        getTainted: () => tainted?.verificationToken ?? false,
        setTainted: (value: boolean) => {
          tainted.verificationToken = value;
        },
        validate: (): Array<string> => {
          const result = User.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "verificationToken")
              .map((e) => e.message);
          }
          return [];
        },
      },
      verificationExpires: {
        path: ["verificationExpires"] as const,
        name: "verificationExpires",
        constraints: { required: true },

        get: () => data.verificationExpires,
        set: (value: string | null) => {
          data.verificationExpires = value;
        },
        getError: () => errors?.verificationExpires,
        setError: (value: Array<string> | undefined) => {
          errors.verificationExpires = value;
        },
        getTainted: () => tainted?.verificationExpires ?? false,
        setTainted: (value: boolean) => {
          tainted.verificationExpires = value;
        },
        validate: (): Array<string> => {
          const result = User.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "verificationExpires")
              .map((e) => e.message);
          }
          return [];
        },
      },
      passwordResetToken: {
        path: ["passwordResetToken"] as const,
        name: "passwordResetToken",
        constraints: { required: true },

        get: () => data.passwordResetToken,
        set: (value: string | null) => {
          data.passwordResetToken = value;
        },
        getError: () => errors?.passwordResetToken,
        setError: (value: Array<string> | undefined) => {
          errors.passwordResetToken = value;
        },
        getTainted: () => tainted?.passwordResetToken ?? false,
        setTainted: (value: boolean) => {
          tainted.passwordResetToken = value;
        },
        validate: (): Array<string> => {
          const result = User.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "passwordResetToken")
              .map((e) => e.message);
          }
          return [];
        },
      },
      passwordResetExpires: {
        path: ["passwordResetExpires"] as const,
        name: "passwordResetExpires",
        constraints: { required: true },

        get: () => data.passwordResetExpires,
        set: (value: string | null) => {
          data.passwordResetExpires = value;
        },
        getError: () => errors?.passwordResetExpires,
        setError: (value: Array<string> | undefined) => {
          errors.passwordResetExpires = value;
        },
        getTainted: () => tainted?.passwordResetExpires ?? false,
        setTainted: (value: boolean) => {
          tainted.passwordResetExpires = value;
        },
        validate: (): Array<string> => {
          const result = User.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "passwordResetExpires")
              .map((e) => e.message);
          }
          return [];
        },
      },
      permissions: {
        path: ["permissions"] as const,
        name: "permissions",
        constraints: { required: true },

        get: () => data.permissions,
        set: (value: AppPermissions) => {
          data.permissions = value;
        },
        getError: () => errors?.permissions,
        setError: (value: Array<string> | undefined) => {
          errors.permissions = value;
        },
        getTainted: () => tainted?.permissions ?? false,
        setTainted: (value: boolean) => {
          tainted.permissions = value;
        },
        validate: (): Array<string> => {
          const result = User.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "permissions")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      User,
      Array<{ field: string; message: string }>
    > {
      return User.fromJSON(data);
    }
    function reset(newOverrides?: Partial<User>): void {
      data = { ...User.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<User, Array<{ field: string; message: string }>> {
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
    return User.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): Service {
    return {
      id: "",
      name: "",
      quickCode: "",
      group: null,
      subgroup: null,
      unit: null,
      active: false,
      commission: false,
      favorite: false,
      averageTime: null,
      defaults: ServiceDefaults.defaultValue(),
    } as Service;
  }
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
  ): Result<Service, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Service.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Service.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("name" in obj)) {
      errors.push({ field: "name", message: "missing required field" });
    }
    if (!("quickCode" in obj)) {
      errors.push({ field: "quickCode", message: "missing required field" });
    }
    if (!("group" in obj)) {
      errors.push({ field: "group", message: "missing required field" });
    }
    if (!("subgroup" in obj)) {
      errors.push({ field: "subgroup", message: "missing required field" });
    }
    if (!("unit" in obj)) {
      errors.push({ field: "unit", message: "missing required field" });
    }
    if (!("active" in obj)) {
      errors.push({ field: "active", message: "missing required field" });
    }
    if (!("commission" in obj)) {
      errors.push({ field: "commission", message: "missing required field" });
    }
    if (!("favorite" in obj)) {
      errors.push({ field: "favorite", message: "missing required field" });
    }
    if (!("averageTime" in obj)) {
      errors.push({ field: "averageTime", message: "missing required field" });
    }
    if (!("defaults" in obj)) {
      errors.push({ field: "defaults", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Service;
  }
}

export namespace Service {
  /** Nested error structure matching the data shape */ export type Errors = {
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
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
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
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly name: FieldController<string>;
    readonly quickCode: FieldController<string>;
    readonly group: FieldController<string | null>;
    readonly subgroup: FieldController<string | null>;
    readonly unit: FieldController<string | null>;
    readonly active: FieldController<boolean>;
    readonly commission: FieldController<boolean>;
    readonly favorite: FieldController<boolean>;
    readonly averageTime: FieldController<string | null>;
    readonly defaults: FieldController<ServiceDefaults>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Service;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Service, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Service>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Service>,
  ): Gigaform {
    let data = $state({ ...Service.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Service.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: () => data.name,
        set: (value: string) => {
          data.name = value;
        },
        getError: () => errors?.name,
        setError: (value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: () => tainted?.name ?? false,
        setTainted: (value: boolean) => {
          tainted.name = value;
        },
        validate: (): Array<string> => {
          const result = Service.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "name")
              .map((e) => e.message);
          }
          return [];
        },
      },
      quickCode: {
        path: ["quickCode"] as const,
        name: "quickCode",
        constraints: { required: true },

        get: () => data.quickCode,
        set: (value: string) => {
          data.quickCode = value;
        },
        getError: () => errors?.quickCode,
        setError: (value: Array<string> | undefined) => {
          errors.quickCode = value;
        },
        getTainted: () => tainted?.quickCode ?? false,
        setTainted: (value: boolean) => {
          tainted.quickCode = value;
        },
        validate: (): Array<string> => {
          const result = Service.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "quickCode")
              .map((e) => e.message);
          }
          return [];
        },
      },
      group: {
        path: ["group"] as const,
        name: "group",
        constraints: { required: true },

        get: () => data.group,
        set: (value: string | null) => {
          data.group = value;
        },
        getError: () => errors?.group,
        setError: (value: Array<string> | undefined) => {
          errors.group = value;
        },
        getTainted: () => tainted?.group ?? false,
        setTainted: (value: boolean) => {
          tainted.group = value;
        },
        validate: (): Array<string> => {
          const result = Service.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "group")
              .map((e) => e.message);
          }
          return [];
        },
      },
      subgroup: {
        path: ["subgroup"] as const,
        name: "subgroup",
        constraints: { required: true },

        get: () => data.subgroup,
        set: (value: string | null) => {
          data.subgroup = value;
        },
        getError: () => errors?.subgroup,
        setError: (value: Array<string> | undefined) => {
          errors.subgroup = value;
        },
        getTainted: () => tainted?.subgroup ?? false,
        setTainted: (value: boolean) => {
          tainted.subgroup = value;
        },
        validate: (): Array<string> => {
          const result = Service.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "subgroup")
              .map((e) => e.message);
          }
          return [];
        },
      },
      unit: {
        path: ["unit"] as const,
        name: "unit",
        constraints: { required: true },

        get: () => data.unit,
        set: (value: string | null) => {
          data.unit = value;
        },
        getError: () => errors?.unit,
        setError: (value: Array<string> | undefined) => {
          errors.unit = value;
        },
        getTainted: () => tainted?.unit ?? false,
        setTainted: (value: boolean) => {
          tainted.unit = value;
        },
        validate: (): Array<string> => {
          const result = Service.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "unit")
              .map((e) => e.message);
          }
          return [];
        },
      },
      active: {
        path: ["active"] as const,
        name: "active",
        constraints: { required: true },

        get: () => data.active,
        set: (value: boolean) => {
          data.active = value;
        },
        getError: () => errors?.active,
        setError: (value: Array<string> | undefined) => {
          errors.active = value;
        },
        getTainted: () => tainted?.active ?? false,
        setTainted: (value: boolean) => {
          tainted.active = value;
        },
        validate: (): Array<string> => {
          const result = Service.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "active")
              .map((e) => e.message);
          }
          return [];
        },
      },
      commission: {
        path: ["commission"] as const,
        name: "commission",
        constraints: { required: true },

        get: () => data.commission,
        set: (value: boolean) => {
          data.commission = value;
        },
        getError: () => errors?.commission,
        setError: (value: Array<string> | undefined) => {
          errors.commission = value;
        },
        getTainted: () => tainted?.commission ?? false,
        setTainted: (value: boolean) => {
          tainted.commission = value;
        },
        validate: (): Array<string> => {
          const result = Service.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "commission")
              .map((e) => e.message);
          }
          return [];
        },
      },
      favorite: {
        path: ["favorite"] as const,
        name: "favorite",
        constraints: { required: true },

        get: () => data.favorite,
        set: (value: boolean) => {
          data.favorite = value;
        },
        getError: () => errors?.favorite,
        setError: (value: Array<string> | undefined) => {
          errors.favorite = value;
        },
        getTainted: () => tainted?.favorite ?? false,
        setTainted: (value: boolean) => {
          tainted.favorite = value;
        },
        validate: (): Array<string> => {
          const result = Service.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "favorite")
              .map((e) => e.message);
          }
          return [];
        },
      },
      averageTime: {
        path: ["averageTime"] as const,
        name: "averageTime",
        constraints: { required: true },

        get: () => data.averageTime,
        set: (value: string | null) => {
          data.averageTime = value;
        },
        getError: () => errors?.averageTime,
        setError: (value: Array<string> | undefined) => {
          errors.averageTime = value;
        },
        getTainted: () => tainted?.averageTime ?? false,
        setTainted: (value: boolean) => {
          tainted.averageTime = value;
        },
        validate: (): Array<string> => {
          const result = Service.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "averageTime")
              .map((e) => e.message);
          }
          return [];
        },
      },
      defaults: {
        path: ["defaults"] as const,
        name: "defaults",
        constraints: { required: true },

        get: () => data.defaults,
        set: (value: ServiceDefaults) => {
          data.defaults = value;
        },
        getError: () => errors?.defaults,
        setError: (value: Array<string> | undefined) => {
          errors.defaults = value;
        },
        getTainted: () => tainted?.defaults ?? false,
        setTainted: (value: boolean) => {
          tainted.defaults = value;
        },
        validate: (): Array<string> => {
          const result = Service.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "defaults")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Service,
      Array<{ field: string; message: string }>
    > {
      return Service.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Service>): void {
      data = { ...Service.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Service, Array<{ field: string; message: string }>> {
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
    return Service.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<ServiceDefaults, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "ServiceDefaults.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "ServiceDefaults.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("price" in obj)) {
      errors.push({ field: "price", message: "missing required field" });
    }
    if (!("description" in obj)) {
      errors.push({ field: "description", message: "missing required field" });
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
      const __raw_price = obj["price"];
      instance.price = __raw_price;
    }
    {
      const __raw_description = obj["description"];
      instance.description = __raw_description;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as ServiceDefaults;
  }
}

export namespace ServiceDefaults {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    price?: Array<string>;
    description?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { price?: boolean; description?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly price: FieldController<number>;
    readonly description: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: ServiceDefaults;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<
      ServiceDefaults,
      Array<{ field: string; message: string }>
    >;
    reset(overrides?: Partial<ServiceDefaults>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<ServiceDefaults>,
  ): Gigaform {
    let data = $state({ ...ServiceDefaults.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      price: {
        path: ["price"] as const,
        name: "price",
        constraints: { required: true },

        get: () => data.price,
        set: (value: number) => {
          data.price = value;
        },
        getError: () => errors?.price,
        setError: (value: Array<string> | undefined) => {
          errors.price = value;
        },
        getTainted: () => tainted?.price ?? false,
        setTainted: (value: boolean) => {
          tainted.price = value;
        },
        validate: (): Array<string> => {
          const result = ServiceDefaults.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "price")
              .map((e) => e.message);
          }
          return [];
        },
      },
      description: {
        path: ["description"] as const,
        name: "description",
        constraints: { required: true },

        get: () => data.description,
        set: (value: string) => {
          data.description = value;
        },
        getError: () => errors?.description,
        setError: (value: Array<string> | undefined) => {
          errors.description = value;
        },
        getTainted: () => tainted?.description ?? false,
        setTainted: (value: boolean) => {
          tainted.description = value;
        },
        validate: (): Array<string> => {
          const result = ServiceDefaults.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "description")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      ServiceDefaults,
      Array<{ field: string; message: string }>
    > {
      return ServiceDefaults.fromJSON(data);
    }
    function reset(newOverrides?: Partial<ServiceDefaults>): void {
      data = { ...ServiceDefaults.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<ServiceDefaults, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
      const priceStr = formData.get("price");
      obj.price = priceStr ? parseFloat(priceStr as string) : 0;
      if (obj.price !== undefined && Number.isNaN(obj.price)) obj.price = 0;
    }
    obj.description = formData.get("description") ?? "";
    return ServiceDefaults.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): Did {
    return {
      in: "",
      out: "",
      id: "",
      activityType: Created.defaultValue(),
      createdAt: "",
      metadata: null,
    } as Did;
  }
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
  ): Result<Did, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Did.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "Did.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("in" in obj)) {
      errors.push({ field: "in", message: "missing required field" });
    }
    if (!("out" in obj)) {
      errors.push({ field: "out", message: "missing required field" });
    }
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("activityType" in obj)) {
      errors.push({ field: "activityType", message: "missing required field" });
    }
    if (!("createdAt" in obj)) {
      errors.push({ field: "createdAt", message: "missing required field" });
    }
    if (!("metadata" in obj)) {
      errors.push({ field: "metadata", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Did;
  }
}

export namespace Did {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    in?: Array<string>;
    out?: Array<string>;
    id?: Array<string>;
    activityType?: ActivityType.Errors;
    createdAt?: Array<string>;
    metadata?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
      in?: boolean;
      out?: boolean;
      id?: boolean;
      activityType?: ActivityType.Tainted;
      createdAt?: boolean;
      metadata?: boolean;
    };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly in: FieldController<string | Actor>;
    readonly out: FieldController<string | Target>;
    readonly id: FieldController<string>;
    readonly activityType: FieldController<ActivityType>;
    readonly createdAt: FieldController<string>;
    readonly metadata: FieldController<string | null>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Did;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Did, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Did>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Did>,
  ): Gigaform {
    let data = $state({ ...Did.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      in: {
        path: ["in"] as const,
        name: "in",
        constraints: { required: true },

        get: () => data.in,
        set: (value: string | Actor) => {
          data.in = value;
        },
        getError: () => errors?.in,
        setError: (value: Array<string> | undefined) => {
          errors.in = value;
        },
        getTainted: () => tainted?.in ?? false,
        setTainted: (value: boolean) => {
          tainted.in = value;
        },
        validate: (): Array<string> => {
          const result = Did.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "in")
              .map((e) => e.message);
          }
          return [];
        },
      },
      out: {
        path: ["out"] as const,
        name: "out",
        constraints: { required: true },

        get: () => data.out,
        set: (value: string | Target) => {
          data.out = value;
        },
        getError: () => errors?.out,
        setError: (value: Array<string> | undefined) => {
          errors.out = value;
        },
        getTainted: () => tainted?.out ?? false,
        setTainted: (value: boolean) => {
          tainted.out = value;
        },
        validate: (): Array<string> => {
          const result = Did.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "out")
              .map((e) => e.message);
          }
          return [];
        },
      },
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Did.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      activityType: {
        path: ["activityType"] as const,
        name: "activityType",
        constraints: { required: true },

        get: () => data.activityType,
        set: (value: ActivityType) => {
          data.activityType = value;
        },
        getError: () => errors?.activityType,
        setError: (value: Array<string> | undefined) => {
          errors.activityType = value;
        },
        getTainted: () => tainted?.activityType ?? false,
        setTainted: (value: boolean) => {
          tainted.activityType = value;
        },
        validate: (): Array<string> => {
          const result = Did.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "activityType")
              .map((e) => e.message);
          }
          return [];
        },
      },
      createdAt: {
        path: ["createdAt"] as const,
        name: "createdAt",
        constraints: { required: true },

        get: () => data.createdAt,
        set: (value: string) => {
          data.createdAt = value;
        },
        getError: () => errors?.createdAt,
        setError: (value: Array<string> | undefined) => {
          errors.createdAt = value;
        },
        getTainted: () => tainted?.createdAt ?? false,
        setTainted: (value: boolean) => {
          tainted.createdAt = value;
        },
        validate: (): Array<string> => {
          const result = Did.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "createdAt")
              .map((e) => e.message);
          }
          return [];
        },
      },
      metadata: {
        path: ["metadata"] as const,
        name: "metadata",
        constraints: { required: true },

        get: () => data.metadata,
        set: (value: string | null) => {
          data.metadata = value;
        },
        getError: () => errors?.metadata,
        setError: (value: Array<string> | undefined) => {
          errors.metadata = value;
        },
        getTainted: () => tainted?.metadata ?? false,
        setTainted: (value: boolean) => {
          tainted.metadata = value;
        },
        validate: (): Array<string> => {
          const result = Did.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "metadata")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Did,
      Array<{ field: string; message: string }>
    > {
      return Did.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Did>): void {
      data = { ...Did.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Did, Array<{ field: string; message: string }>> {
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
    return Did.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<PersonName, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "PersonName.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "PersonName.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("firstName" in obj)) {
      errors.push({ field: "firstName", message: "missing required field" });
    }
    if (!("lastName" in obj)) {
      errors.push({ field: "lastName", message: "missing required field" });
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
      const __raw_firstName = obj["firstName"];
      instance.firstName = __raw_firstName;
    }
    {
      const __raw_lastName = obj["lastName"];
      instance.lastName = __raw_lastName;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as PersonName;
  }
}

export namespace PersonName {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    firstName?: Array<string>;
    lastName?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { firstName?: boolean; lastName?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly firstName: FieldController<string>;
    readonly lastName: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: PersonName;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<PersonName, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<PersonName>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<PersonName>,
  ): Gigaform {
    let data = $state({ ...PersonName.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      firstName: {
        path: ["firstName"] as const,
        name: "firstName",
        constraints: { required: true },

        get: () => data.firstName,
        set: (value: string) => {
          data.firstName = value;
        },
        getError: () => errors?.firstName,
        setError: (value: Array<string> | undefined) => {
          errors.firstName = value;
        },
        getTainted: () => tainted?.firstName ?? false,
        setTainted: (value: boolean) => {
          tainted.firstName = value;
        },
        validate: (): Array<string> => {
          const result = PersonName.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "firstName")
              .map((e) => e.message);
          }
          return [];
        },
      },
      lastName: {
        path: ["lastName"] as const,
        name: "lastName",
        constraints: { required: true },

        get: () => data.lastName,
        set: (value: string) => {
          data.lastName = value;
        },
        getError: () => errors?.lastName,
        setError: (value: Array<string> | undefined) => {
          errors.lastName = value;
        },
        getTainted: () => tainted?.lastName ?? false,
        setTainted: (value: boolean) => {
          tainted.lastName = value;
        },
        validate: (): Array<string> => {
          const result = PersonName.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "lastName")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      PersonName,
      Array<{ field: string; message: string }>
    > {
      return PersonName.fromJSON(data);
    }
    function reset(newOverrides?: Partial<PersonName>): void {
      data = { ...PersonName.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<PersonName, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.firstName = formData.get("firstName") ?? "";
    obj.lastName = formData.get("lastName") ?? "";
    return PersonName.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Promotion, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Promotion.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Promotion.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("date" in obj)) {
      errors.push({ field: "date", message: "missing required field" });
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
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_date = obj["date"];
      instance.date = __raw_date;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as Promotion;
  }
}

export namespace Promotion {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    date?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { id?: boolean; date?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly date: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Promotion;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Promotion, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Promotion>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Promotion>,
  ): Gigaform {
    let data = $state({ ...Promotion.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Promotion.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      date: {
        path: ["date"] as const,
        name: "date",
        constraints: { required: true },

        get: () => data.date,
        set: (value: string) => {
          data.date = value;
        },
        getError: () => errors?.date,
        setError: (value: Array<string> | undefined) => {
          errors.date = value;
        },
        getTainted: () => tainted?.date ?? false,
        setTainted: (value: boolean) => {
          tainted.date = value;
        },
        validate: (): Array<string> => {
          const result = Promotion.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "date")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Promotion,
      Array<{ field: string; message: string }>
    > {
      return Promotion.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Promotion>): void {
      data = { ...Promotion.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Promotion, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.date = formData.get("date") ?? "";
    return Promotion.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): Site {
    return {
      id: "",
      addressLine1: "",
      addressLine2: null,
      sublocalityLevel1: null,
      locality: "",
      administrativeAreaLevel3: null,
      administrativeAreaLevel2: null,
      administrativeAreaLevel1: "",
      country: "",
      postalCode: "",
      postalCodeSuffix: null,
      coordinates: Coordinates.defaultValue(),
    } as Site;
  }
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
  ): Result<Site, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Site.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "Site.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("addressLine1" in obj)) {
      errors.push({ field: "addressLine1", message: "missing required field" });
    }
    if (!("addressLine2" in obj)) {
      errors.push({ field: "addressLine2", message: "missing required field" });
    }
    if (!("sublocalityLevel1" in obj)) {
      errors.push({
        field: "sublocalityLevel1",
        message: "missing required field",
      });
    }
    if (!("locality" in obj)) {
      errors.push({ field: "locality", message: "missing required field" });
    }
    if (!("administrativeAreaLevel3" in obj)) {
      errors.push({
        field: "administrativeAreaLevel3",
        message: "missing required field",
      });
    }
    if (!("administrativeAreaLevel2" in obj)) {
      errors.push({
        field: "administrativeAreaLevel2",
        message: "missing required field",
      });
    }
    if (!("administrativeAreaLevel1" in obj)) {
      errors.push({
        field: "administrativeAreaLevel1",
        message: "missing required field",
      });
    }
    if (!("country" in obj)) {
      errors.push({ field: "country", message: "missing required field" });
    }
    if (!("postalCode" in obj)) {
      errors.push({ field: "postalCode", message: "missing required field" });
    }
    if (!("postalCodeSuffix" in obj)) {
      errors.push({
        field: "postalCodeSuffix",
        message: "missing required field",
      });
    }
    if (!("coordinates" in obj)) {
      errors.push({ field: "coordinates", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Site;
  }
}

export namespace Site {
  /** Nested error structure matching the data shape */ export type Errors = {
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
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
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
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly addressLine1: FieldController<string>;
    readonly addressLine2: FieldController<string | null>;
    readonly sublocalityLevel1: FieldController<string | null>;
    readonly locality: FieldController<string>;
    readonly administrativeAreaLevel3: FieldController<string | null>;
    readonly administrativeAreaLevel2: FieldController<string | null>;
    readonly administrativeAreaLevel1: FieldController<string>;
    readonly country: FieldController<string>;
    readonly postalCode: FieldController<string>;
    readonly postalCodeSuffix: FieldController<string | null>;
    readonly coordinates: FieldController<Coordinates>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Site;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Site, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Site>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Site>,
  ): Gigaform {
    let data = $state({ ...Site.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Site.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      addressLine1: {
        path: ["addressLine1"] as const,
        name: "addressLine1",
        constraints: { required: true },

        get: () => data.addressLine1,
        set: (value: string) => {
          data.addressLine1 = value;
        },
        getError: () => errors?.addressLine1,
        setError: (value: Array<string> | undefined) => {
          errors.addressLine1 = value;
        },
        getTainted: () => tainted?.addressLine1 ?? false,
        setTainted: (value: boolean) => {
          tainted.addressLine1 = value;
        },
        validate: (): Array<string> => {
          const result = Site.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "addressLine1")
              .map((e) => e.message);
          }
          return [];
        },
      },
      addressLine2: {
        path: ["addressLine2"] as const,
        name: "addressLine2",
        constraints: { required: true },

        get: () => data.addressLine2,
        set: (value: string | null) => {
          data.addressLine2 = value;
        },
        getError: () => errors?.addressLine2,
        setError: (value: Array<string> | undefined) => {
          errors.addressLine2 = value;
        },
        getTainted: () => tainted?.addressLine2 ?? false,
        setTainted: (value: boolean) => {
          tainted.addressLine2 = value;
        },
        validate: (): Array<string> => {
          const result = Site.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "addressLine2")
              .map((e) => e.message);
          }
          return [];
        },
      },
      sublocalityLevel1: {
        path: ["sublocalityLevel1"] as const,
        name: "sublocalityLevel1",
        constraints: { required: true },

        get: () => data.sublocalityLevel1,
        set: (value: string | null) => {
          data.sublocalityLevel1 = value;
        },
        getError: () => errors?.sublocalityLevel1,
        setError: (value: Array<string> | undefined) => {
          errors.sublocalityLevel1 = value;
        },
        getTainted: () => tainted?.sublocalityLevel1 ?? false,
        setTainted: (value: boolean) => {
          tainted.sublocalityLevel1 = value;
        },
        validate: (): Array<string> => {
          const result = Site.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "sublocalityLevel1")
              .map((e) => e.message);
          }
          return [];
        },
      },
      locality: {
        path: ["locality"] as const,
        name: "locality",
        constraints: { required: true },

        get: () => data.locality,
        set: (value: string) => {
          data.locality = value;
        },
        getError: () => errors?.locality,
        setError: (value: Array<string> | undefined) => {
          errors.locality = value;
        },
        getTainted: () => tainted?.locality ?? false,
        setTainted: (value: boolean) => {
          tainted.locality = value;
        },
        validate: (): Array<string> => {
          const result = Site.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "locality")
              .map((e) => e.message);
          }
          return [];
        },
      },
      administrativeAreaLevel3: {
        path: ["administrativeAreaLevel3"] as const,
        name: "administrativeAreaLevel3",
        constraints: { required: true },

        get: () => data.administrativeAreaLevel3,
        set: (value: string | null) => {
          data.administrativeAreaLevel3 = value;
        },
        getError: () => errors?.administrativeAreaLevel3,
        setError: (value: Array<string> | undefined) => {
          errors.administrativeAreaLevel3 = value;
        },
        getTainted: () => tainted?.administrativeAreaLevel3 ?? false,
        setTainted: (value: boolean) => {
          tainted.administrativeAreaLevel3 = value;
        },
        validate: (): Array<string> => {
          const result = Site.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "administrativeAreaLevel3")
              .map((e) => e.message);
          }
          return [];
        },
      },
      administrativeAreaLevel2: {
        path: ["administrativeAreaLevel2"] as const,
        name: "administrativeAreaLevel2",
        constraints: { required: true },

        get: () => data.administrativeAreaLevel2,
        set: (value: string | null) => {
          data.administrativeAreaLevel2 = value;
        },
        getError: () => errors?.administrativeAreaLevel2,
        setError: (value: Array<string> | undefined) => {
          errors.administrativeAreaLevel2 = value;
        },
        getTainted: () => tainted?.administrativeAreaLevel2 ?? false,
        setTainted: (value: boolean) => {
          tainted.administrativeAreaLevel2 = value;
        },
        validate: (): Array<string> => {
          const result = Site.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "administrativeAreaLevel2")
              .map((e) => e.message);
          }
          return [];
        },
      },
      administrativeAreaLevel1: {
        path: ["administrativeAreaLevel1"] as const,
        name: "administrativeAreaLevel1",
        constraints: { required: true },

        get: () => data.administrativeAreaLevel1,
        set: (value: string) => {
          data.administrativeAreaLevel1 = value;
        },
        getError: () => errors?.administrativeAreaLevel1,
        setError: (value: Array<string> | undefined) => {
          errors.administrativeAreaLevel1 = value;
        },
        getTainted: () => tainted?.administrativeAreaLevel1 ?? false,
        setTainted: (value: boolean) => {
          tainted.administrativeAreaLevel1 = value;
        },
        validate: (): Array<string> => {
          const result = Site.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "administrativeAreaLevel1")
              .map((e) => e.message);
          }
          return [];
        },
      },
      country: {
        path: ["country"] as const,
        name: "country",
        constraints: { required: true },

        get: () => data.country,
        set: (value: string) => {
          data.country = value;
        },
        getError: () => errors?.country,
        setError: (value: Array<string> | undefined) => {
          errors.country = value;
        },
        getTainted: () => tainted?.country ?? false,
        setTainted: (value: boolean) => {
          tainted.country = value;
        },
        validate: (): Array<string> => {
          const result = Site.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "country")
              .map((e) => e.message);
          }
          return [];
        },
      },
      postalCode: {
        path: ["postalCode"] as const,
        name: "postalCode",
        constraints: { required: true },

        get: () => data.postalCode,
        set: (value: string) => {
          data.postalCode = value;
        },
        getError: () => errors?.postalCode,
        setError: (value: Array<string> | undefined) => {
          errors.postalCode = value;
        },
        getTainted: () => tainted?.postalCode ?? false,
        setTainted: (value: boolean) => {
          tainted.postalCode = value;
        },
        validate: (): Array<string> => {
          const result = Site.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "postalCode")
              .map((e) => e.message);
          }
          return [];
        },
      },
      postalCodeSuffix: {
        path: ["postalCodeSuffix"] as const,
        name: "postalCodeSuffix",
        constraints: { required: true },

        get: () => data.postalCodeSuffix,
        set: (value: string | null) => {
          data.postalCodeSuffix = value;
        },
        getError: () => errors?.postalCodeSuffix,
        setError: (value: Array<string> | undefined) => {
          errors.postalCodeSuffix = value;
        },
        getTainted: () => tainted?.postalCodeSuffix ?? false,
        setTainted: (value: boolean) => {
          tainted.postalCodeSuffix = value;
        },
        validate: (): Array<string> => {
          const result = Site.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "postalCodeSuffix")
              .map((e) => e.message);
          }
          return [];
        },
      },
      coordinates: {
        path: ["coordinates"] as const,
        name: "coordinates",
        constraints: { required: true },

        get: () => data.coordinates,
        set: (value: Coordinates) => {
          data.coordinates = value;
        },
        getError: () => errors?.coordinates,
        setError: (value: Array<string> | undefined) => {
          errors.coordinates = value;
        },
        getTainted: () => tainted?.coordinates ?? false,
        setTainted: (value: boolean) => {
          tainted.coordinates = value;
        },
        validate: (): Array<string> => {
          const result = Site.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "coordinates")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Site,
      Array<{ field: string; message: string }>
    > {
      return Site.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Site>): void {
      data = { ...Site.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Site, Array<{ field: string; message: string }>> {
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
    return Site.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Metadata, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Metadata.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Metadata.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("createdAt" in obj)) {
      errors.push({ field: "createdAt", message: "missing required field" });
    }
    if (!("lastLogin" in obj)) {
      errors.push({ field: "lastLogin", message: "missing required field" });
    }
    if (!("isActive" in obj)) {
      errors.push({ field: "isActive", message: "missing required field" });
    }
    if (!("roles" in obj)) {
      errors.push({ field: "roles", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Metadata;
  }
}

export namespace Metadata {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    createdAt?: Array<string>;
    lastLogin?: Array<string>;
    isActive?: Array<string>;
    roles?: { _errors?: Array<string>; [index: number]: Array<string> };
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
      createdAt?: boolean;
      lastLogin?: boolean;
      isActive?: boolean;
      roles?: { [index: number]: boolean };
    };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly createdAt: FieldController<string>;
    readonly lastLogin: FieldController<string | null>;
    readonly isActive: FieldController<boolean>;
    readonly roles: FieldController<string[]>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Metadata;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Metadata, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Metadata>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Metadata>,
  ): Gigaform {
    let data = $state({ ...Metadata.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      createdAt: {
        path: ["createdAt"] as const,
        name: "createdAt",
        constraints: { required: true },

        get: () => data.createdAt,
        set: (value: string) => {
          data.createdAt = value;
        },
        getError: () => errors?.createdAt,
        setError: (value: Array<string> | undefined) => {
          errors.createdAt = value;
        },
        getTainted: () => tainted?.createdAt ?? false,
        setTainted: (value: boolean) => {
          tainted.createdAt = value;
        },
        validate: (): Array<string> => {
          const result = Metadata.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "createdAt")
              .map((e) => e.message);
          }
          return [];
        },
      },
      lastLogin: {
        path: ["lastLogin"] as const,
        name: "lastLogin",
        constraints: { required: true },

        get: () => data.lastLogin,
        set: (value: string | null) => {
          data.lastLogin = value;
        },
        getError: () => errors?.lastLogin,
        setError: (value: Array<string> | undefined) => {
          errors.lastLogin = value;
        },
        getTainted: () => tainted?.lastLogin ?? false,
        setTainted: (value: boolean) => {
          tainted.lastLogin = value;
        },
        validate: (): Array<string> => {
          const result = Metadata.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "lastLogin")
              .map((e) => e.message);
          }
          return [];
        },
      },
      isActive: {
        path: ["isActive"] as const,
        name: "isActive",
        constraints: { required: true },

        get: () => data.isActive,
        set: (value: boolean) => {
          data.isActive = value;
        },
        getError: () => errors?.isActive,
        setError: (value: Array<string> | undefined) => {
          errors.isActive = value;
        },
        getTainted: () => tainted?.isActive ?? false,
        setTainted: (value: boolean) => {
          tainted.isActive = value;
        },
        validate: (): Array<string> => {
          const result = Metadata.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "isActive")
              .map((e) => e.message);
          }
          return [];
        },
      },
      roles: {
        path: ["roles"] as const,
        name: "roles",
        constraints: { required: true },

        get: () => data.roles,
        set: (value: string[]) => {
          data.roles = value;
        },
        getError: () => errors?.roles,
        setError: (value: Array<string> | undefined) => {
          errors.roles = value;
        },
        getTainted: () => tainted?.roles ?? false,
        setTainted: (value: boolean) => {
          tainted.roles = value;
        },
        validate: (): Array<string> => {
          const result = Metadata.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "roles")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["roles", index] as const,
          name: `roles.${index}`,
          constraints: { required: true },
          get: () => data.roles[index],
          set: (value: string) => {
            data.roles[index] = value;
          },
          getError: () =>
            (errors.roles as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.roles ??= {};
            (errors.roles as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.roles?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.roles ??= {};
            tainted.roles[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: string) => {
          data.roles.push(item);
        },
        remove: (index: number) => {
          data.roles.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.roles[a], data.roles[b]] = [data.roles[b], data.roles[a]];
        },
      },
    };
    function validate(): Result<
      Metadata,
      Array<{ field: string; message: string }>
    > {
      return Metadata.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Metadata>): void {
      data = { ...Metadata.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Metadata, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.createdAt = formData.get("createdAt") ?? "";
    obj.lastLogin = formData.get("lastLogin") ?? "";
    {
      const isActiveVal = formData.get("isActive");
      obj.isActive =
        isActiveVal === "true" || isActiveVal === "on" || isActiveVal === "1";
    }
    obj.roles = formData.getAll("roles") as Array<string>;
    return Metadata.fromStringifiedJSON(JSON.stringify(obj));
  }
}

/**  */
export interface ColumnConfig {
  heading: string;

  dataPath: DataPath;
}

export namespace ColumnConfig {
  export function defaultValue(): ColumnConfig {
    return { heading: "", dataPath: DataPath.defaultValue() } as ColumnConfig;
  }
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
  ): Result<ColumnConfig, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "ColumnConfig.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "ColumnConfig.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("heading" in obj)) {
      errors.push({ field: "heading", message: "missing required field" });
    }
    if (!("dataPath" in obj)) {
      errors.push({ field: "dataPath", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as ColumnConfig;
  }
}

export namespace ColumnConfig {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    heading?: Array<string>;
    dataPath?: DataPath.Errors;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { heading?: boolean; dataPath?: DataPath.Tainted };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly heading: FieldController<string>;
    readonly dataPath: FieldController<DataPath>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: ColumnConfig;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<ColumnConfig, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<ColumnConfig>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<ColumnConfig>,
  ): Gigaform {
    let data = $state({ ...ColumnConfig.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      heading: {
        path: ["heading"] as const,
        name: "heading",
        constraints: { required: true },

        get: () => data.heading,
        set: (value: string) => {
          data.heading = value;
        },
        getError: () => errors?.heading,
        setError: (value: Array<string> | undefined) => {
          errors.heading = value;
        },
        getTainted: () => tainted?.heading ?? false,
        setTainted: (value: boolean) => {
          tainted.heading = value;
        },
        validate: (): Array<string> => {
          const result = ColumnConfig.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "heading")
              .map((e) => e.message);
          }
          return [];
        },
      },
      dataPath: {
        path: ["dataPath"] as const,
        name: "dataPath",
        constraints: { required: true },

        get: () => data.dataPath,
        set: (value: DataPath) => {
          data.dataPath = value;
        },
        getError: () => errors?.dataPath,
        setError: (value: Array<string> | undefined) => {
          errors.dataPath = value;
        },
        getTainted: () => tainted?.dataPath ?? false,
        setTainted: (value: boolean) => {
          tainted.dataPath = value;
        },
        validate: (): Array<string> => {
          const result = ColumnConfig.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "dataPath")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      ColumnConfig,
      Array<{ field: string; message: string }>
    > {
      return ColumnConfig.fromJSON(data);
    }
    function reset(newOverrides?: Partial<ColumnConfig>): void {
      data = { ...ColumnConfig.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<ColumnConfig, Array<{ field: string; message: string }>> {
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
    return ColumnConfig.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<PhoneNumber, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "PhoneNumber.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "PhoneNumber.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("main" in obj)) {
      errors.push({ field: "main", message: "missing required field" });
    }
    if (!("phoneType" in obj)) {
      errors.push({ field: "phoneType", message: "missing required field" });
    }
    if (!("number" in obj)) {
      errors.push({ field: "number", message: "missing required field" });
    }
    if (!("canText" in obj)) {
      errors.push({ field: "canText", message: "missing required field" });
    }
    if (!("canCall" in obj)) {
      errors.push({ field: "canCall", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as PhoneNumber;
  }
}

export namespace PhoneNumber {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    main?: Array<string>;
    phoneType?: Array<string>;
    number?: Array<string>;
    canText?: Array<string>;
    canCall?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
      main?: boolean;
      phoneType?: boolean;
      number?: boolean;
      canText?: boolean;
      canCall?: boolean;
    };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly main: FieldController<boolean>;
    readonly phoneType: FieldController<string>;
    readonly number: FieldController<string>;
    readonly canText: FieldController<boolean>;
    readonly canCall: FieldController<boolean>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: PhoneNumber;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<PhoneNumber, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<PhoneNumber>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<PhoneNumber>,
  ): Gigaform {
    let data = $state({ ...PhoneNumber.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      main: {
        path: ["main"] as const,
        name: "main",
        constraints: { required: true },

        get: () => data.main,
        set: (value: boolean) => {
          data.main = value;
        },
        getError: () => errors?.main,
        setError: (value: Array<string> | undefined) => {
          errors.main = value;
        },
        getTainted: () => tainted?.main ?? false,
        setTainted: (value: boolean) => {
          tainted.main = value;
        },
        validate: (): Array<string> => {
          const result = PhoneNumber.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "main")
              .map((e) => e.message);
          }
          return [];
        },
      },
      phoneType: {
        path: ["phoneType"] as const,
        name: "phoneType",
        constraints: { required: true },

        get: () => data.phoneType,
        set: (value: string) => {
          data.phoneType = value;
        },
        getError: () => errors?.phoneType,
        setError: (value: Array<string> | undefined) => {
          errors.phoneType = value;
        },
        getTainted: () => tainted?.phoneType ?? false,
        setTainted: (value: boolean) => {
          tainted.phoneType = value;
        },
        validate: (): Array<string> => {
          const result = PhoneNumber.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "phoneType")
              .map((e) => e.message);
          }
          return [];
        },
      },
      number: {
        path: ["number"] as const,
        name: "number",
        constraints: { required: true },

        get: () => data.number,
        set: (value: string) => {
          data.number = value;
        },
        getError: () => errors?.number,
        setError: (value: Array<string> | undefined) => {
          errors.number = value;
        },
        getTainted: () => tainted?.number ?? false,
        setTainted: (value: boolean) => {
          tainted.number = value;
        },
        validate: (): Array<string> => {
          const result = PhoneNumber.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "number")
              .map((e) => e.message);
          }
          return [];
        },
      },
      canText: {
        path: ["canText"] as const,
        name: "canText",
        constraints: { required: true },

        get: () => data.canText,
        set: (value: boolean) => {
          data.canText = value;
        },
        getError: () => errors?.canText,
        setError: (value: Array<string> | undefined) => {
          errors.canText = value;
        },
        getTainted: () => tainted?.canText ?? false,
        setTainted: (value: boolean) => {
          tainted.canText = value;
        },
        validate: (): Array<string> => {
          const result = PhoneNumber.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "canText")
              .map((e) => e.message);
          }
          return [];
        },
      },
      canCall: {
        path: ["canCall"] as const,
        name: "canCall",
        constraints: { required: true },

        get: () => data.canCall,
        set: (value: boolean) => {
          data.canCall = value;
        },
        getError: () => errors?.canCall,
        setError: (value: Array<string> | undefined) => {
          errors.canCall = value;
        },
        getTainted: () => tainted?.canCall ?? false,
        setTainted: (value: boolean) => {
          tainted.canCall = value;
        },
        validate: (): Array<string> => {
          const result = PhoneNumber.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "canCall")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      PhoneNumber,
      Array<{ field: string; message: string }>
    > {
      return PhoneNumber.fromJSON(data);
    }
    function reset(newOverrides?: Partial<PhoneNumber>): void {
      data = { ...PhoneNumber.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<PhoneNumber, Array<{ field: string; message: string }>> {
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
    return PhoneNumber.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Gradient, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Gradient.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Gradient.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("startHue" in obj)) {
      errors.push({ field: "startHue", message: "missing required field" });
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
      const __raw_startHue = obj["startHue"];
      instance.startHue = __raw_startHue;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as Gradient;
  }
}

export namespace Gradient {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    startHue?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { startHue?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly startHue: FieldController<number>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Gradient;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Gradient, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Gradient>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Gradient>,
  ): Gigaform {
    let data = $state({ ...Gradient.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      startHue: {
        path: ["startHue"] as const,
        name: "startHue",
        constraints: { required: true },

        get: () => data.startHue,
        set: (value: number) => {
          data.startHue = value;
        },
        getError: () => errors?.startHue,
        setError: (value: Array<string> | undefined) => {
          errors.startHue = value;
        },
        getTainted: () => tainted?.startHue ?? false,
        setTainted: (value: boolean) => {
          tainted.startHue = value;
        },
        validate: (): Array<string> => {
          const result = Gradient.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "startHue")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Gradient,
      Array<{ field: string; message: string }>
    > {
      return Gradient.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Gradient>): void {
      data = { ...Gradient.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Gradient, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
      const startHueStr = formData.get("startHue");
      obj.startHue = startHueStr ? parseFloat(startHueStr as string) : 0;
      if (obj.startHue !== undefined && Number.isNaN(obj.startHue))
        obj.startHue = 0;
    }
    return Gradient.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): Product {
    return {
      id: "",
      name: "",
      quickCode: "",
      group: null,
      subgroup: null,
      unit: null,
      active: false,
      commission: false,
      favorite: false,
      defaults: ProductDefaults.defaultValue(),
    } as Product;
  }
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
  ): Result<Product, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Product.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Product.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("name" in obj)) {
      errors.push({ field: "name", message: "missing required field" });
    }
    if (!("quickCode" in obj)) {
      errors.push({ field: "quickCode", message: "missing required field" });
    }
    if (!("group" in obj)) {
      errors.push({ field: "group", message: "missing required field" });
    }
    if (!("subgroup" in obj)) {
      errors.push({ field: "subgroup", message: "missing required field" });
    }
    if (!("unit" in obj)) {
      errors.push({ field: "unit", message: "missing required field" });
    }
    if (!("active" in obj)) {
      errors.push({ field: "active", message: "missing required field" });
    }
    if (!("commission" in obj)) {
      errors.push({ field: "commission", message: "missing required field" });
    }
    if (!("favorite" in obj)) {
      errors.push({ field: "favorite", message: "missing required field" });
    }
    if (!("defaults" in obj)) {
      errors.push({ field: "defaults", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Product;
  }
}

export namespace Product {
  /** Nested error structure matching the data shape */ export type Errors = {
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
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
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
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly name: FieldController<string>;
    readonly quickCode: FieldController<string>;
    readonly group: FieldController<string | null>;
    readonly subgroup: FieldController<string | null>;
    readonly unit: FieldController<string | null>;
    readonly active: FieldController<boolean>;
    readonly commission: FieldController<boolean>;
    readonly favorite: FieldController<boolean>;
    readonly defaults: FieldController<ProductDefaults>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Product;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Product, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Product>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Product>,
  ): Gigaform {
    let data = $state({ ...Product.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Product.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: () => data.name,
        set: (value: string) => {
          data.name = value;
        },
        getError: () => errors?.name,
        setError: (value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: () => tainted?.name ?? false,
        setTainted: (value: boolean) => {
          tainted.name = value;
        },
        validate: (): Array<string> => {
          const result = Product.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "name")
              .map((e) => e.message);
          }
          return [];
        },
      },
      quickCode: {
        path: ["quickCode"] as const,
        name: "quickCode",
        constraints: { required: true },

        get: () => data.quickCode,
        set: (value: string) => {
          data.quickCode = value;
        },
        getError: () => errors?.quickCode,
        setError: (value: Array<string> | undefined) => {
          errors.quickCode = value;
        },
        getTainted: () => tainted?.quickCode ?? false,
        setTainted: (value: boolean) => {
          tainted.quickCode = value;
        },
        validate: (): Array<string> => {
          const result = Product.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "quickCode")
              .map((e) => e.message);
          }
          return [];
        },
      },
      group: {
        path: ["group"] as const,
        name: "group",
        constraints: { required: true },

        get: () => data.group,
        set: (value: string | null) => {
          data.group = value;
        },
        getError: () => errors?.group,
        setError: (value: Array<string> | undefined) => {
          errors.group = value;
        },
        getTainted: () => tainted?.group ?? false,
        setTainted: (value: boolean) => {
          tainted.group = value;
        },
        validate: (): Array<string> => {
          const result = Product.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "group")
              .map((e) => e.message);
          }
          return [];
        },
      },
      subgroup: {
        path: ["subgroup"] as const,
        name: "subgroup",
        constraints: { required: true },

        get: () => data.subgroup,
        set: (value: string | null) => {
          data.subgroup = value;
        },
        getError: () => errors?.subgroup,
        setError: (value: Array<string> | undefined) => {
          errors.subgroup = value;
        },
        getTainted: () => tainted?.subgroup ?? false,
        setTainted: (value: boolean) => {
          tainted.subgroup = value;
        },
        validate: (): Array<string> => {
          const result = Product.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "subgroup")
              .map((e) => e.message);
          }
          return [];
        },
      },
      unit: {
        path: ["unit"] as const,
        name: "unit",
        constraints: { required: true },

        get: () => data.unit,
        set: (value: string | null) => {
          data.unit = value;
        },
        getError: () => errors?.unit,
        setError: (value: Array<string> | undefined) => {
          errors.unit = value;
        },
        getTainted: () => tainted?.unit ?? false,
        setTainted: (value: boolean) => {
          tainted.unit = value;
        },
        validate: (): Array<string> => {
          const result = Product.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "unit")
              .map((e) => e.message);
          }
          return [];
        },
      },
      active: {
        path: ["active"] as const,
        name: "active",
        constraints: { required: true },

        get: () => data.active,
        set: (value: boolean) => {
          data.active = value;
        },
        getError: () => errors?.active,
        setError: (value: Array<string> | undefined) => {
          errors.active = value;
        },
        getTainted: () => tainted?.active ?? false,
        setTainted: (value: boolean) => {
          tainted.active = value;
        },
        validate: (): Array<string> => {
          const result = Product.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "active")
              .map((e) => e.message);
          }
          return [];
        },
      },
      commission: {
        path: ["commission"] as const,
        name: "commission",
        constraints: { required: true },

        get: () => data.commission,
        set: (value: boolean) => {
          data.commission = value;
        },
        getError: () => errors?.commission,
        setError: (value: Array<string> | undefined) => {
          errors.commission = value;
        },
        getTainted: () => tainted?.commission ?? false,
        setTainted: (value: boolean) => {
          tainted.commission = value;
        },
        validate: (): Array<string> => {
          const result = Product.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "commission")
              .map((e) => e.message);
          }
          return [];
        },
      },
      favorite: {
        path: ["favorite"] as const,
        name: "favorite",
        constraints: { required: true },

        get: () => data.favorite,
        set: (value: boolean) => {
          data.favorite = value;
        },
        getError: () => errors?.favorite,
        setError: (value: Array<string> | undefined) => {
          errors.favorite = value;
        },
        getTainted: () => tainted?.favorite ?? false,
        setTainted: (value: boolean) => {
          tainted.favorite = value;
        },
        validate: (): Array<string> => {
          const result = Product.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "favorite")
              .map((e) => e.message);
          }
          return [];
        },
      },
      defaults: {
        path: ["defaults"] as const,
        name: "defaults",
        constraints: { required: true },

        get: () => data.defaults,
        set: (value: ProductDefaults) => {
          data.defaults = value;
        },
        getError: () => errors?.defaults,
        setError: (value: Array<string> | undefined) => {
          errors.defaults = value;
        },
        getTainted: () => tainted?.defaults ?? false,
        setTainted: (value: boolean) => {
          tainted.defaults = value;
        },
        validate: (): Array<string> => {
          const result = Product.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "defaults")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Product,
      Array<{ field: string; message: string }>
    > {
      return Product.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Product>): void {
      data = { ...Product.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Product, Array<{ field: string; message: string }>> {
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
    return Product.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<YearlyRecurrenceRule, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "YearlyRecurrenceRule.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "YearlyRecurrenceRule.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("quantityOfYears" in obj)) {
      errors.push({
        field: "quantityOfYears",
        message: "missing required field",
      });
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
      const __raw_quantityOfYears = obj["quantityOfYears"];
      instance.quantityOfYears = __raw_quantityOfYears;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as YearlyRecurrenceRule;
  }
}

export namespace YearlyRecurrenceRule {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    quantityOfYears?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { quantityOfYears?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly quantityOfYears: FieldController<number>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: YearlyRecurrenceRule;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<
      YearlyRecurrenceRule,
      Array<{ field: string; message: string }>
    >;
    reset(overrides?: Partial<YearlyRecurrenceRule>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<YearlyRecurrenceRule>,
  ): Gigaform {
    let data = $state({ ...YearlyRecurrenceRule.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      quantityOfYears: {
        path: ["quantityOfYears"] as const,
        name: "quantityOfYears",
        constraints: { required: true },

        get: () => data.quantityOfYears,
        set: (value: number) => {
          data.quantityOfYears = value;
        },
        getError: () => errors?.quantityOfYears,
        setError: (value: Array<string> | undefined) => {
          errors.quantityOfYears = value;
        },
        getTainted: () => tainted?.quantityOfYears ?? false,
        setTainted: (value: boolean) => {
          tainted.quantityOfYears = value;
        },
        validate: (): Array<string> => {
          const result = YearlyRecurrenceRule.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "quantityOfYears")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      YearlyRecurrenceRule,
      Array<{ field: string; message: string }>
    > {
      return YearlyRecurrenceRule.fromJSON(data);
    }
    function reset(newOverrides?: Partial<YearlyRecurrenceRule>): void {
      data = { ...YearlyRecurrenceRule.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<YearlyRecurrenceRule, Array<{ field: string; message: string }>> {
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
    return YearlyRecurrenceRule.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<
    AppointmentNotifications,
    Array<{ field: string; message: string }>
  > {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "AppointmentNotifications.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "AppointmentNotifications.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("personalScheduleChangeNotifications" in obj)) {
      errors.push({
        field: "personalScheduleChangeNotifications",
        message: "missing required field",
      });
    }
    if (!("allScheduleChangeNotifications" in obj)) {
      errors.push({
        field: "allScheduleChangeNotifications",
        message: "missing required field",
      });
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
      throw new DeserializeError(errors);
    }
    return instance as AppointmentNotifications;
  }
}

export namespace AppointmentNotifications {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    personalScheduleChangeNotifications?: Array<string>;
    allScheduleChangeNotifications?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
      personalScheduleChangeNotifications?: boolean;
      allScheduleChangeNotifications?: boolean;
    };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly personalScheduleChangeNotifications: FieldController<string>;
    readonly allScheduleChangeNotifications: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: AppointmentNotifications;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<
      AppointmentNotifications,
      Array<{ field: string; message: string }>
    >;
    reset(overrides?: Partial<AppointmentNotifications>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<AppointmentNotifications>,
  ): Gigaform {
    let data = $state({
      ...AppointmentNotifications.defaultValue(),
      ...overrides,
    });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      personalScheduleChangeNotifications: {
        path: ["personalScheduleChangeNotifications"] as const,
        name: "personalScheduleChangeNotifications",
        constraints: { required: true },

        get: () => data.personalScheduleChangeNotifications,
        set: (value: string) => {
          data.personalScheduleChangeNotifications = value;
        },
        getError: () => errors?.personalScheduleChangeNotifications,
        setError: (value: Array<string> | undefined) => {
          errors.personalScheduleChangeNotifications = value;
        },
        getTainted: () => tainted?.personalScheduleChangeNotifications ?? false,
        setTainted: (value: boolean) => {
          tainted.personalScheduleChangeNotifications = value;
        },
        validate: (): Array<string> => {
          const result = AppointmentNotifications.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "personalScheduleChangeNotifications")
              .map((e) => e.message);
          }
          return [];
        },
      },
      allScheduleChangeNotifications: {
        path: ["allScheduleChangeNotifications"] as const,
        name: "allScheduleChangeNotifications",
        constraints: { required: true },

        get: () => data.allScheduleChangeNotifications,
        set: (value: string) => {
          data.allScheduleChangeNotifications = value;
        },
        getError: () => errors?.allScheduleChangeNotifications,
        setError: (value: Array<string> | undefined) => {
          errors.allScheduleChangeNotifications = value;
        },
        getTainted: () => tainted?.allScheduleChangeNotifications ?? false,
        setTainted: (value: boolean) => {
          tainted.allScheduleChangeNotifications = value;
        },
        validate: (): Array<string> => {
          const result = AppointmentNotifications.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "allScheduleChangeNotifications")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      AppointmentNotifications,
      Array<{ field: string; message: string }>
    > {
      return AppointmentNotifications.fromJSON(data);
    }
    function reset(newOverrides?: Partial<AppointmentNotifications>): void {
      data = { ...AppointmentNotifications.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<
    AppointmentNotifications,
    Array<{ field: string; message: string }>
  > {
    const obj: Record<string, unknown> = {};
    obj.personalScheduleChangeNotifications =
      formData.get("personalScheduleChangeNotifications") ?? "";
    obj.allScheduleChangeNotifications =
      formData.get("allScheduleChangeNotifications") ?? "";
    return AppointmentNotifications.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<DirectionHue, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "DirectionHue.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "DirectionHue.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("bearing" in obj)) {
      errors.push({ field: "bearing", message: "missing required field" });
    }
    if (!("hue" in obj)) {
      errors.push({ field: "hue", message: "missing required field" });
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
      const __raw_bearing = obj["bearing"];
      instance.bearing = __raw_bearing;
    }
    {
      const __raw_hue = obj["hue"];
      instance.hue = __raw_hue;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as DirectionHue;
  }
}

export namespace DirectionHue {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    bearing?: Array<string>;
    hue?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { bearing?: boolean; hue?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly bearing: FieldController<number>;
    readonly hue: FieldController<number>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: DirectionHue;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<DirectionHue, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<DirectionHue>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<DirectionHue>,
  ): Gigaform {
    let data = $state({ ...DirectionHue.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      bearing: {
        path: ["bearing"] as const,
        name: "bearing",
        constraints: { required: true },

        get: () => data.bearing,
        set: (value: number) => {
          data.bearing = value;
        },
        getError: () => errors?.bearing,
        setError: (value: Array<string> | undefined) => {
          errors.bearing = value;
        },
        getTainted: () => tainted?.bearing ?? false,
        setTainted: (value: boolean) => {
          tainted.bearing = value;
        },
        validate: (): Array<string> => {
          const result = DirectionHue.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "bearing")
              .map((e) => e.message);
          }
          return [];
        },
      },
      hue: {
        path: ["hue"] as const,
        name: "hue",
        constraints: { required: true },

        get: () => data.hue,
        set: (value: number) => {
          data.hue = value;
        },
        getError: () => errors?.hue,
        setError: (value: Array<string> | undefined) => {
          errors.hue = value;
        },
        getTainted: () => tainted?.hue ?? false,
        setTainted: (value: boolean) => {
          tainted.hue = value;
        },
        validate: (): Array<string> => {
          const result = DirectionHue.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "hue")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      DirectionHue,
      Array<{ field: string; message: string }>
    > {
      return DirectionHue.fromJSON(data);
    }
    function reset(newOverrides?: Partial<DirectionHue>): void {
      data = { ...DirectionHue.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<DirectionHue, Array<{ field: string; message: string }>> {
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
    return DirectionHue.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<MonthlyRecurrenceRule, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "MonthlyRecurrenceRule.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "MonthlyRecurrenceRule.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("quantityOfMonths" in obj)) {
      errors.push({
        field: "quantityOfMonths",
        message: "missing required field",
      });
    }
    if (!("day" in obj)) {
      errors.push({ field: "day", message: "missing required field" });
    }
    if (!("name" in obj)) {
      errors.push({ field: "name", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as MonthlyRecurrenceRule;
  }
}

export namespace MonthlyRecurrenceRule {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    quantityOfMonths?: Array<string>;
    day?: Array<string>;
    name?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { quantityOfMonths?: boolean; day?: boolean; name?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly quantityOfMonths: FieldController<number>;
    readonly day: FieldController<number>;
    readonly name: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: MonthlyRecurrenceRule;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<
      MonthlyRecurrenceRule,
      Array<{ field: string; message: string }>
    >;
    reset(overrides?: Partial<MonthlyRecurrenceRule>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<MonthlyRecurrenceRule>,
  ): Gigaform {
    let data = $state({
      ...MonthlyRecurrenceRule.defaultValue(),
      ...overrides,
    });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      quantityOfMonths: {
        path: ["quantityOfMonths"] as const,
        name: "quantityOfMonths",
        constraints: { required: true },

        get: () => data.quantityOfMonths,
        set: (value: number) => {
          data.quantityOfMonths = value;
        },
        getError: () => errors?.quantityOfMonths,
        setError: (value: Array<string> | undefined) => {
          errors.quantityOfMonths = value;
        },
        getTainted: () => tainted?.quantityOfMonths ?? false,
        setTainted: (value: boolean) => {
          tainted.quantityOfMonths = value;
        },
        validate: (): Array<string> => {
          const result = MonthlyRecurrenceRule.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "quantityOfMonths")
              .map((e) => e.message);
          }
          return [];
        },
      },
      day: {
        path: ["day"] as const,
        name: "day",
        constraints: { required: true },

        get: () => data.day,
        set: (value: number) => {
          data.day = value;
        },
        getError: () => errors?.day,
        setError: (value: Array<string> | undefined) => {
          errors.day = value;
        },
        getTainted: () => tainted?.day ?? false,
        setTainted: (value: boolean) => {
          tainted.day = value;
        },
        validate: (): Array<string> => {
          const result = MonthlyRecurrenceRule.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "day")
              .map((e) => e.message);
          }
          return [];
        },
      },
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: () => data.name,
        set: (value: string) => {
          data.name = value;
        },
        getError: () => errors?.name,
        setError: (value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: () => tainted?.name ?? false,
        setTainted: (value: boolean) => {
          tainted.name = value;
        },
        validate: (): Array<string> => {
          const result = MonthlyRecurrenceRule.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "name")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      MonthlyRecurrenceRule,
      Array<{ field: string; message: string }>
    > {
      return MonthlyRecurrenceRule.fromJSON(data);
    }
    function reset(newOverrides?: Partial<MonthlyRecurrenceRule>): void {
      data = { ...MonthlyRecurrenceRule.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<MonthlyRecurrenceRule, Array<{ field: string; message: string }>> {
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
    return MonthlyRecurrenceRule.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): Represents {
    return { in: "", out: "", id: "", dateStarted: "" } as Represents;
  }
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
  ): Result<Represents, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Represents.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Represents.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("in" in obj)) {
      errors.push({ field: "in", message: "missing required field" });
    }
    if (!("out" in obj)) {
      errors.push({ field: "out", message: "missing required field" });
    }
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("dateStarted" in obj)) {
      errors.push({ field: "dateStarted", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Represents;
  }
}

export namespace Represents {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    in?: Array<string>;
    out?: Array<string>;
    id?: Array<string>;
    dateStarted?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { in?: boolean; out?: boolean; id?: boolean; dateStarted?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly in: FieldController<string | Employee>;
    readonly out: FieldController<string | Account>;
    readonly id: FieldController<string>;
    readonly dateStarted: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Represents;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Represents, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Represents>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Represents>,
  ): Gigaform {
    let data = $state({ ...Represents.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      in: {
        path: ["in"] as const,
        name: "in",
        constraints: { required: true },

        get: () => data.in,
        set: (value: string | Employee) => {
          data.in = value;
        },
        getError: () => errors?.in,
        setError: (value: Array<string> | undefined) => {
          errors.in = value;
        },
        getTainted: () => tainted?.in ?? false,
        setTainted: (value: boolean) => {
          tainted.in = value;
        },
        validate: (): Array<string> => {
          const result = Represents.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "in")
              .map((e) => e.message);
          }
          return [];
        },
      },
      out: {
        path: ["out"] as const,
        name: "out",
        constraints: { required: true },

        get: () => data.out,
        set: (value: string | Account) => {
          data.out = value;
        },
        getError: () => errors?.out,
        setError: (value: Array<string> | undefined) => {
          errors.out = value;
        },
        getTainted: () => tainted?.out ?? false,
        setTainted: (value: boolean) => {
          tainted.out = value;
        },
        validate: (): Array<string> => {
          const result = Represents.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "out")
              .map((e) => e.message);
          }
          return [];
        },
      },
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Represents.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      dateStarted: {
        path: ["dateStarted"] as const,
        name: "dateStarted",
        constraints: { required: true },

        get: () => data.dateStarted,
        set: (value: string) => {
          data.dateStarted = value;
        },
        getError: () => errors?.dateStarted,
        setError: (value: Array<string> | undefined) => {
          errors.dateStarted = value;
        },
        getTainted: () => tainted?.dateStarted ?? false,
        setTainted: (value: boolean) => {
          tainted.dateStarted = value;
        },
        validate: (): Array<string> => {
          const result = Represents.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "dateStarted")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Represents,
      Array<{ field: string; message: string }>
    > {
      return Represents.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Represents>): void {
      data = { ...Represents.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Represents, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.in = formData.get("in") ?? "";
    obj.out = formData.get("out") ?? "";
    obj.id = formData.get("id") ?? "";
    obj.dateStarted = formData.get("dateStarted") ?? "";
    return Represents.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Payment, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Payment.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Payment.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("date" in obj)) {
      errors.push({ field: "date", message: "missing required field" });
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
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_date = obj["date"];
      instance.date = __raw_date;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as Payment;
  }
}

export namespace Payment {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    date?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { id?: boolean; date?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly date: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Payment;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Payment, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Payment>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Payment>,
  ): Gigaform {
    let data = $state({ ...Payment.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Payment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      date: {
        path: ["date"] as const,
        name: "date",
        constraints: { required: true },

        get: () => data.date,
        set: (value: string) => {
          data.date = value;
        },
        getError: () => errors?.date,
        setError: (value: Array<string> | undefined) => {
          errors.date = value;
        },
        getTainted: () => tainted?.date ?? false,
        setTainted: (value: boolean) => {
          tainted.date = value;
        },
        validate: (): Array<string> => {
          const result = Payment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "date")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Payment,
      Array<{ field: string; message: string }>
    > {
      return Payment.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Payment>): void {
      data = { ...Payment.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Payment, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.date = formData.get("date") ?? "";
    return Payment.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): Settings {
    return {
      appointmentNotifications: null,
      commissions: null,
      scheduleSettings: ScheduleSettings.defaultValue(),
      accountOverviewSettings: OverviewSettings.defaultValue(),
      serviceOverviewSettings: OverviewSettings.defaultValue(),
      appointmentOverviewSettings: OverviewSettings.defaultValue(),
      leadOverviewSettings: OverviewSettings.defaultValue(),
      packageOverviewSettings: OverviewSettings.defaultValue(),
      productOverviewSettings: OverviewSettings.defaultValue(),
      orderOverviewSettings: OverviewSettings.defaultValue(),
      taxRateOverviewSettings: OverviewSettings.defaultValue(),
      homePage: "UserHome",
    } as Settings;
  }
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
  ): Result<Settings, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Settings.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Settings.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("appointmentNotifications" in obj)) {
      errors.push({
        field: "appointmentNotifications",
        message: "missing required field",
      });
    }
    if (!("commissions" in obj)) {
      errors.push({ field: "commissions", message: "missing required field" });
    }
    if (!("scheduleSettings" in obj)) {
      errors.push({
        field: "scheduleSettings",
        message: "missing required field",
      });
    }
    if (!("accountOverviewSettings" in obj)) {
      errors.push({
        field: "accountOverviewSettings",
        message: "missing required field",
      });
    }
    if (!("serviceOverviewSettings" in obj)) {
      errors.push({
        field: "serviceOverviewSettings",
        message: "missing required field",
      });
    }
    if (!("appointmentOverviewSettings" in obj)) {
      errors.push({
        field: "appointmentOverviewSettings",
        message: "missing required field",
      });
    }
    if (!("leadOverviewSettings" in obj)) {
      errors.push({
        field: "leadOverviewSettings",
        message: "missing required field",
      });
    }
    if (!("packageOverviewSettings" in obj)) {
      errors.push({
        field: "packageOverviewSettings",
        message: "missing required field",
      });
    }
    if (!("productOverviewSettings" in obj)) {
      errors.push({
        field: "productOverviewSettings",
        message: "missing required field",
      });
    }
    if (!("orderOverviewSettings" in obj)) {
      errors.push({
        field: "orderOverviewSettings",
        message: "missing required field",
      });
    }
    if (!("taxRateOverviewSettings" in obj)) {
      errors.push({
        field: "taxRateOverviewSettings",
        message: "missing required field",
      });
    }
    if (!("homePage" in obj)) {
      errors.push({ field: "homePage", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Settings;
  }
}

export namespace Settings {
  /** Nested error structure matching the data shape */ export type Errors = {
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
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
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
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly appointmentNotifications: FieldController<AppointmentNotifications | null>;
    readonly commissions: FieldController<Commissions | null>;
    readonly scheduleSettings: FieldController<ScheduleSettings>;
    readonly accountOverviewSettings: FieldController<OverviewSettings>;
    readonly serviceOverviewSettings: FieldController<OverviewSettings>;
    readonly appointmentOverviewSettings: FieldController<OverviewSettings>;
    readonly leadOverviewSettings: FieldController<OverviewSettings>;
    readonly packageOverviewSettings: FieldController<OverviewSettings>;
    readonly productOverviewSettings: FieldController<OverviewSettings>;
    readonly orderOverviewSettings: FieldController<OverviewSettings>;
    readonly taxRateOverviewSettings: FieldController<OverviewSettings>;
    readonly homePage: FieldController<Page>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Settings;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Settings, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Settings>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Settings>,
  ): Gigaform {
    let data = $state({ ...Settings.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      appointmentNotifications: {
        path: ["appointmentNotifications"] as const,
        name: "appointmentNotifications",
        constraints: { required: true },

        get: () => data.appointmentNotifications,
        set: (value: AppointmentNotifications | null) => {
          data.appointmentNotifications = value;
        },
        getError: () => errors?.appointmentNotifications,
        setError: (value: Array<string> | undefined) => {
          errors.appointmentNotifications = value;
        },
        getTainted: () => tainted?.appointmentNotifications ?? false,
        setTainted: (value: boolean) => {
          tainted.appointmentNotifications = value;
        },
        validate: (): Array<string> => {
          const result = Settings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "appointmentNotifications")
              .map((e) => e.message);
          }
          return [];
        },
      },
      commissions: {
        path: ["commissions"] as const,
        name: "commissions",
        constraints: { required: true },

        get: () => data.commissions,
        set: (value: Commissions | null) => {
          data.commissions = value;
        },
        getError: () => errors?.commissions,
        setError: (value: Array<string> | undefined) => {
          errors.commissions = value;
        },
        getTainted: () => tainted?.commissions ?? false,
        setTainted: (value: boolean) => {
          tainted.commissions = value;
        },
        validate: (): Array<string> => {
          const result = Settings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "commissions")
              .map((e) => e.message);
          }
          return [];
        },
      },
      scheduleSettings: {
        path: ["scheduleSettings"] as const,
        name: "scheduleSettings",
        constraints: { required: true },

        get: () => data.scheduleSettings,
        set: (value: ScheduleSettings) => {
          data.scheduleSettings = value;
        },
        getError: () => errors?.scheduleSettings,
        setError: (value: Array<string> | undefined) => {
          errors.scheduleSettings = value;
        },
        getTainted: () => tainted?.scheduleSettings ?? false,
        setTainted: (value: boolean) => {
          tainted.scheduleSettings = value;
        },
        validate: (): Array<string> => {
          const result = Settings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "scheduleSettings")
              .map((e) => e.message);
          }
          return [];
        },
      },
      accountOverviewSettings: {
        path: ["accountOverviewSettings"] as const,
        name: "accountOverviewSettings",
        constraints: { required: true },

        get: () => data.accountOverviewSettings,
        set: (value: OverviewSettings) => {
          data.accountOverviewSettings = value;
        },
        getError: () => errors?.accountOverviewSettings,
        setError: (value: Array<string> | undefined) => {
          errors.accountOverviewSettings = value;
        },
        getTainted: () => tainted?.accountOverviewSettings ?? false,
        setTainted: (value: boolean) => {
          tainted.accountOverviewSettings = value;
        },
        validate: (): Array<string> => {
          const result = Settings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "accountOverviewSettings")
              .map((e) => e.message);
          }
          return [];
        },
      },
      serviceOverviewSettings: {
        path: ["serviceOverviewSettings"] as const,
        name: "serviceOverviewSettings",
        constraints: { required: true },

        get: () => data.serviceOverviewSettings,
        set: (value: OverviewSettings) => {
          data.serviceOverviewSettings = value;
        },
        getError: () => errors?.serviceOverviewSettings,
        setError: (value: Array<string> | undefined) => {
          errors.serviceOverviewSettings = value;
        },
        getTainted: () => tainted?.serviceOverviewSettings ?? false,
        setTainted: (value: boolean) => {
          tainted.serviceOverviewSettings = value;
        },
        validate: (): Array<string> => {
          const result = Settings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "serviceOverviewSettings")
              .map((e) => e.message);
          }
          return [];
        },
      },
      appointmentOverviewSettings: {
        path: ["appointmentOverviewSettings"] as const,
        name: "appointmentOverviewSettings",
        constraints: { required: true },

        get: () => data.appointmentOverviewSettings,
        set: (value: OverviewSettings) => {
          data.appointmentOverviewSettings = value;
        },
        getError: () => errors?.appointmentOverviewSettings,
        setError: (value: Array<string> | undefined) => {
          errors.appointmentOverviewSettings = value;
        },
        getTainted: () => tainted?.appointmentOverviewSettings ?? false,
        setTainted: (value: boolean) => {
          tainted.appointmentOverviewSettings = value;
        },
        validate: (): Array<string> => {
          const result = Settings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "appointmentOverviewSettings")
              .map((e) => e.message);
          }
          return [];
        },
      },
      leadOverviewSettings: {
        path: ["leadOverviewSettings"] as const,
        name: "leadOverviewSettings",
        constraints: { required: true },

        get: () => data.leadOverviewSettings,
        set: (value: OverviewSettings) => {
          data.leadOverviewSettings = value;
        },
        getError: () => errors?.leadOverviewSettings,
        setError: (value: Array<string> | undefined) => {
          errors.leadOverviewSettings = value;
        },
        getTainted: () => tainted?.leadOverviewSettings ?? false,
        setTainted: (value: boolean) => {
          tainted.leadOverviewSettings = value;
        },
        validate: (): Array<string> => {
          const result = Settings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "leadOverviewSettings")
              .map((e) => e.message);
          }
          return [];
        },
      },
      packageOverviewSettings: {
        path: ["packageOverviewSettings"] as const,
        name: "packageOverviewSettings",
        constraints: { required: true },

        get: () => data.packageOverviewSettings,
        set: (value: OverviewSettings) => {
          data.packageOverviewSettings = value;
        },
        getError: () => errors?.packageOverviewSettings,
        setError: (value: Array<string> | undefined) => {
          errors.packageOverviewSettings = value;
        },
        getTainted: () => tainted?.packageOverviewSettings ?? false,
        setTainted: (value: boolean) => {
          tainted.packageOverviewSettings = value;
        },
        validate: (): Array<string> => {
          const result = Settings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "packageOverviewSettings")
              .map((e) => e.message);
          }
          return [];
        },
      },
      productOverviewSettings: {
        path: ["productOverviewSettings"] as const,
        name: "productOverviewSettings",
        constraints: { required: true },

        get: () => data.productOverviewSettings,
        set: (value: OverviewSettings) => {
          data.productOverviewSettings = value;
        },
        getError: () => errors?.productOverviewSettings,
        setError: (value: Array<string> | undefined) => {
          errors.productOverviewSettings = value;
        },
        getTainted: () => tainted?.productOverviewSettings ?? false,
        setTainted: (value: boolean) => {
          tainted.productOverviewSettings = value;
        },
        validate: (): Array<string> => {
          const result = Settings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "productOverviewSettings")
              .map((e) => e.message);
          }
          return [];
        },
      },
      orderOverviewSettings: {
        path: ["orderOverviewSettings"] as const,
        name: "orderOverviewSettings",
        constraints: { required: true },

        get: () => data.orderOverviewSettings,
        set: (value: OverviewSettings) => {
          data.orderOverviewSettings = value;
        },
        getError: () => errors?.orderOverviewSettings,
        setError: (value: Array<string> | undefined) => {
          errors.orderOverviewSettings = value;
        },
        getTainted: () => tainted?.orderOverviewSettings ?? false,
        setTainted: (value: boolean) => {
          tainted.orderOverviewSettings = value;
        },
        validate: (): Array<string> => {
          const result = Settings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "orderOverviewSettings")
              .map((e) => e.message);
          }
          return [];
        },
      },
      taxRateOverviewSettings: {
        path: ["taxRateOverviewSettings"] as const,
        name: "taxRateOverviewSettings",
        constraints: { required: true },

        get: () => data.taxRateOverviewSettings,
        set: (value: OverviewSettings) => {
          data.taxRateOverviewSettings = value;
        },
        getError: () => errors?.taxRateOverviewSettings,
        setError: (value: Array<string> | undefined) => {
          errors.taxRateOverviewSettings = value;
        },
        getTainted: () => tainted?.taxRateOverviewSettings ?? false,
        setTainted: (value: boolean) => {
          tainted.taxRateOverviewSettings = value;
        },
        validate: (): Array<string> => {
          const result = Settings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "taxRateOverviewSettings")
              .map((e) => e.message);
          }
          return [];
        },
      },
      homePage: {
        path: ["homePage"] as const,
        name: "homePage",
        constraints: { required: true },

        get: () => data.homePage,
        set: (value: Page) => {
          data.homePage = value;
        },
        getError: () => errors?.homePage,
        setError: (value: Array<string> | undefined) => {
          errors.homePage = value;
        },
        getTainted: () => tainted?.homePage ?? false,
        setTainted: (value: boolean) => {
          tainted.homePage = value;
        },
        validate: (): Array<string> => {
          const result = Settings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "homePage")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Settings,
      Array<{ field: string; message: string }>
    > {
      return Settings.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Settings>): void {
      data = { ...Settings.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Settings, Array<{ field: string; message: string }>> {
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
    return Settings.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Color, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Color.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "Color.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("red" in obj)) {
      errors.push({ field: "red", message: "missing required field" });
    }
    if (!("green" in obj)) {
      errors.push({ field: "green", message: "missing required field" });
    }
    if (!("blue" in obj)) {
      errors.push({ field: "blue", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Color;
  }
}

export namespace Color {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    red?: Array<string>;
    green?: Array<string>;
    blue?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { red?: boolean; green?: boolean; blue?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly red: FieldController<number>;
    readonly green: FieldController<number>;
    readonly blue: FieldController<number>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Color;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Color, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Color>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Color>,
  ): Gigaform {
    let data = $state({ ...Color.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      red: {
        path: ["red"] as const,
        name: "red",
        constraints: { required: true },

        get: () => data.red,
        set: (value: number) => {
          data.red = value;
        },
        getError: () => errors?.red,
        setError: (value: Array<string> | undefined) => {
          errors.red = value;
        },
        getTainted: () => tainted?.red ?? false,
        setTainted: (value: boolean) => {
          tainted.red = value;
        },
        validate: (): Array<string> => {
          const result = Color.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "red")
              .map((e) => e.message);
          }
          return [];
        },
      },
      green: {
        path: ["green"] as const,
        name: "green",
        constraints: { required: true },

        get: () => data.green,
        set: (value: number) => {
          data.green = value;
        },
        getError: () => errors?.green,
        setError: (value: Array<string> | undefined) => {
          errors.green = value;
        },
        getTainted: () => tainted?.green ?? false,
        setTainted: (value: boolean) => {
          tainted.green = value;
        },
        validate: (): Array<string> => {
          const result = Color.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "green")
              .map((e) => e.message);
          }
          return [];
        },
      },
      blue: {
        path: ["blue"] as const,
        name: "blue",
        constraints: { required: true },

        get: () => data.blue,
        set: (value: number) => {
          data.blue = value;
        },
        getError: () => errors?.blue,
        setError: (value: Array<string> | undefined) => {
          errors.blue = value;
        },
        getTainted: () => tainted?.blue ?? false,
        setTainted: (value: boolean) => {
          tainted.blue = value;
        },
        validate: (): Array<string> => {
          const result = Color.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "blue")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Color,
      Array<{ field: string; message: string }>
    > {
      return Color.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Color>): void {
      data = { ...Color.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Color, Array<{ field: string; message: string }>> {
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
    return Color.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<CompanyName, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "CompanyName.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "CompanyName.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("companyName" in obj)) {
      errors.push({ field: "companyName", message: "missing required field" });
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
      const __raw_companyName = obj["companyName"];
      instance.companyName = __raw_companyName;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as CompanyName;
  }
}

export namespace CompanyName {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    companyName?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { companyName?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly companyName: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: CompanyName;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<CompanyName, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<CompanyName>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<CompanyName>,
  ): Gigaform {
    let data = $state({ ...CompanyName.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      companyName: {
        path: ["companyName"] as const,
        name: "companyName",
        constraints: { required: true },

        get: () => data.companyName,
        set: (value: string) => {
          data.companyName = value;
        },
        getError: () => errors?.companyName,
        setError: (value: Array<string> | undefined) => {
          errors.companyName = value;
        },
        getTainted: () => tainted?.companyName ?? false,
        setTainted: (value: boolean) => {
          tainted.companyName = value;
        },
        validate: (): Array<string> => {
          const result = CompanyName.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "companyName")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      CompanyName,
      Array<{ field: string; message: string }>
    > {
      return CompanyName.fromJSON(data);
    }
    function reset(newOverrides?: Partial<CompanyName>): void {
      data = { ...CompanyName.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<CompanyName, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.companyName = formData.get("companyName") ?? "";
    return CompanyName.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): Appointment {
    return {
      id: "",
      title: "",
      status: "Scheduled",
      begins: "",
      duration: 0,
      timeZone: "",
      offsetMs: 0,
      allDay: false,
      multiDay: false,
      employees: [],
      location: "",
      description: null,
      colors: { main: "#000000", hover: "#333333", text: "#ffffff" },
      recurrenceRule: null,
    } as Appointment;
  }
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
  ): Result<Appointment, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Appointment.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Appointment.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("title" in obj)) {
      errors.push({ field: "title", message: "missing required field" });
    }
    if (!("status" in obj)) {
      errors.push({ field: "status", message: "missing required field" });
    }
    if (!("begins" in obj)) {
      errors.push({ field: "begins", message: "missing required field" });
    }
    if (!("duration" in obj)) {
      errors.push({ field: "duration", message: "missing required field" });
    }
    if (!("timeZone" in obj)) {
      errors.push({ field: "timeZone", message: "missing required field" });
    }
    if (!("offsetMs" in obj)) {
      errors.push({ field: "offsetMs", message: "missing required field" });
    }
    if (!("allDay" in obj)) {
      errors.push({ field: "allDay", message: "missing required field" });
    }
    if (!("multiDay" in obj)) {
      errors.push({ field: "multiDay", message: "missing required field" });
    }
    if (!("employees" in obj)) {
      errors.push({ field: "employees", message: "missing required field" });
    }
    if (!("location" in obj)) {
      errors.push({ field: "location", message: "missing required field" });
    }
    if (!("description" in obj)) {
      errors.push({ field: "description", message: "missing required field" });
    }
    if (!("colors" in obj)) {
      errors.push({ field: "colors", message: "missing required field" });
    }
    if (!("recurrenceRule" in obj)) {
      errors.push({
        field: "recurrenceRule",
        message: "missing required field",
      });
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
      throw new DeserializeError(errors);
    }
    return instance as Appointment;
  }
}

export namespace Appointment {
  /** Nested error structure matching the data shape */ export type Errors = {
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
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
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
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly title: FieldController<string>;
    readonly status: FieldController<Status>;
    readonly begins: FieldController<string>;
    readonly duration: FieldController<number>;
    readonly timeZone: FieldController<string>;
    readonly offsetMs: FieldController<number>;
    readonly allDay: FieldController<boolean>;
    readonly multiDay: FieldController<boolean>;
    readonly employees: FieldController<(string | Employee)[]>;
    readonly location: FieldController<string | Site>;
    readonly description: FieldController<string | null>;
    readonly colors: FieldController<Colors>;
    readonly recurrenceRule: FieldController<RecurrenceRule | null>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Appointment;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Appointment, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Appointment>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Appointment>,
  ): Gigaform {
    let data = $state({ ...Appointment.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Appointment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      title: {
        path: ["title"] as const,
        name: "title",
        constraints: { required: true },

        get: () => data.title,
        set: (value: string) => {
          data.title = value;
        },
        getError: () => errors?.title,
        setError: (value: Array<string> | undefined) => {
          errors.title = value;
        },
        getTainted: () => tainted?.title ?? false,
        setTainted: (value: boolean) => {
          tainted.title = value;
        },
        validate: (): Array<string> => {
          const result = Appointment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "title")
              .map((e) => e.message);
          }
          return [];
        },
      },
      status: {
        path: ["status"] as const,
        name: "status",
        constraints: { required: true },

        get: () => data.status,
        set: (value: Status) => {
          data.status = value;
        },
        getError: () => errors?.status,
        setError: (value: Array<string> | undefined) => {
          errors.status = value;
        },
        getTainted: () => tainted?.status ?? false,
        setTainted: (value: boolean) => {
          tainted.status = value;
        },
        validate: (): Array<string> => {
          const result = Appointment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "status")
              .map((e) => e.message);
          }
          return [];
        },
      },
      begins: {
        path: ["begins"] as const,
        name: "begins",
        constraints: { required: true },

        get: () => data.begins,
        set: (value: string) => {
          data.begins = value;
        },
        getError: () => errors?.begins,
        setError: (value: Array<string> | undefined) => {
          errors.begins = value;
        },
        getTainted: () => tainted?.begins ?? false,
        setTainted: (value: boolean) => {
          tainted.begins = value;
        },
        validate: (): Array<string> => {
          const result = Appointment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "begins")
              .map((e) => e.message);
          }
          return [];
        },
      },
      duration: {
        path: ["duration"] as const,
        name: "duration",
        constraints: { required: true },

        get: () => data.duration,
        set: (value: number) => {
          data.duration = value;
        },
        getError: () => errors?.duration,
        setError: (value: Array<string> | undefined) => {
          errors.duration = value;
        },
        getTainted: () => tainted?.duration ?? false,
        setTainted: (value: boolean) => {
          tainted.duration = value;
        },
        validate: (): Array<string> => {
          const result = Appointment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "duration")
              .map((e) => e.message);
          }
          return [];
        },
      },
      timeZone: {
        path: ["timeZone"] as const,
        name: "timeZone",
        constraints: { required: true },

        get: () => data.timeZone,
        set: (value: string) => {
          data.timeZone = value;
        },
        getError: () => errors?.timeZone,
        setError: (value: Array<string> | undefined) => {
          errors.timeZone = value;
        },
        getTainted: () => tainted?.timeZone ?? false,
        setTainted: (value: boolean) => {
          tainted.timeZone = value;
        },
        validate: (): Array<string> => {
          const result = Appointment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "timeZone")
              .map((e) => e.message);
          }
          return [];
        },
      },
      offsetMs: {
        path: ["offsetMs"] as const,
        name: "offsetMs",
        constraints: { required: true },

        get: () => data.offsetMs,
        set: (value: number) => {
          data.offsetMs = value;
        },
        getError: () => errors?.offsetMs,
        setError: (value: Array<string> | undefined) => {
          errors.offsetMs = value;
        },
        getTainted: () => tainted?.offsetMs ?? false,
        setTainted: (value: boolean) => {
          tainted.offsetMs = value;
        },
        validate: (): Array<string> => {
          const result = Appointment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "offsetMs")
              .map((e) => e.message);
          }
          return [];
        },
      },
      allDay: {
        path: ["allDay"] as const,
        name: "allDay",
        constraints: { required: true },

        get: () => data.allDay,
        set: (value: boolean) => {
          data.allDay = value;
        },
        getError: () => errors?.allDay,
        setError: (value: Array<string> | undefined) => {
          errors.allDay = value;
        },
        getTainted: () => tainted?.allDay ?? false,
        setTainted: (value: boolean) => {
          tainted.allDay = value;
        },
        validate: (): Array<string> => {
          const result = Appointment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "allDay")
              .map((e) => e.message);
          }
          return [];
        },
      },
      multiDay: {
        path: ["multiDay"] as const,
        name: "multiDay",
        constraints: { required: true },

        get: () => data.multiDay,
        set: (value: boolean) => {
          data.multiDay = value;
        },
        getError: () => errors?.multiDay,
        setError: (value: Array<string> | undefined) => {
          errors.multiDay = value;
        },
        getTainted: () => tainted?.multiDay ?? false,
        setTainted: (value: boolean) => {
          tainted.multiDay = value;
        },
        validate: (): Array<string> => {
          const result = Appointment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "multiDay")
              .map((e) => e.message);
          }
          return [];
        },
      },
      employees: {
        path: ["employees"] as const,
        name: "employees",
        constraints: { required: true },

        get: () => data.employees,
        set: (value: (string | Employee)[]) => {
          data.employees = value;
        },
        getError: () => errors?.employees,
        setError: (value: Array<string> | undefined) => {
          errors.employees = value;
        },
        getTainted: () => tainted?.employees ?? false,
        setTainted: (value: boolean) => {
          tainted.employees = value;
        },
        validate: (): Array<string> => {
          const result = Appointment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "employees")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["employees", index] as const,
          name: `employees.${index}`,
          constraints: { required: true },
          get: () => data.employees[index],
          set: (value: string | Employee) => {
            data.employees[index] = value;
          },
          getError: () =>
            (errors.employees as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.employees ??= {};
            (errors.employees as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.employees?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.employees ??= {};
            tainted.employees[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: string | Employee) => {
          data.employees.push(item);
        },
        remove: (index: number) => {
          data.employees.splice(index, 1);
        },
        swap: (a: number, b: number) => {
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

        get: () => data.location,
        set: (value: string | Site) => {
          data.location = value;
        },
        getError: () => errors?.location,
        setError: (value: Array<string> | undefined) => {
          errors.location = value;
        },
        getTainted: () => tainted?.location ?? false,
        setTainted: (value: boolean) => {
          tainted.location = value;
        },
        validate: (): Array<string> => {
          const result = Appointment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "location")
              .map((e) => e.message);
          }
          return [];
        },
      },
      description: {
        path: ["description"] as const,
        name: "description",
        constraints: { required: true },

        get: () => data.description,
        set: (value: string | null) => {
          data.description = value;
        },
        getError: () => errors?.description,
        setError: (value: Array<string> | undefined) => {
          errors.description = value;
        },
        getTainted: () => tainted?.description ?? false,
        setTainted: (value: boolean) => {
          tainted.description = value;
        },
        validate: (): Array<string> => {
          const result = Appointment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "description")
              .map((e) => e.message);
          }
          return [];
        },
      },
      colors: {
        path: ["colors"] as const,
        name: "colors",
        constraints: { required: true },

        get: () => data.colors,
        set: (value: Colors) => {
          data.colors = value;
        },
        getError: () => errors?.colors,
        setError: (value: Array<string> | undefined) => {
          errors.colors = value;
        },
        getTainted: () => tainted?.colors ?? false,
        setTainted: (value: boolean) => {
          tainted.colors = value;
        },
        validate: (): Array<string> => {
          const result = Appointment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "colors")
              .map((e) => e.message);
          }
          return [];
        },
      },
      recurrenceRule: {
        path: ["recurrenceRule"] as const,
        name: "recurrenceRule",
        constraints: { required: true },

        get: () => data.recurrenceRule,
        set: (value: RecurrenceRule | null) => {
          data.recurrenceRule = value;
        },
        getError: () => errors?.recurrenceRule,
        setError: (value: Array<string> | undefined) => {
          errors.recurrenceRule = value;
        },
        getTainted: () => tainted?.recurrenceRule ?? false,
        setTainted: (value: boolean) => {
          tainted.recurrenceRule = value;
        },
        validate: (): Array<string> => {
          const result = Appointment.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "recurrenceRule")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Appointment,
      Array<{ field: string; message: string }>
    > {
      return Appointment.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Appointment>): void {
      data = { ...Appointment.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Appointment, Array<{ field: string; message: string }>> {
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
    return Appointment.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Package, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Package.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Package.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("date" in obj)) {
      errors.push({ field: "date", message: "missing required field" });
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
      const __raw_id = obj["id"];
      instance.id = __raw_id;
    }
    {
      const __raw_date = obj["date"];
      instance.date = __raw_date;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as Package;
  }
}

export namespace Package {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    date?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { id?: boolean; date?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly date: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Package;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Package, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Package>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Package>,
  ): Gigaform {
    let data = $state({ ...Package.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Package.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      date: {
        path: ["date"] as const,
        name: "date",
        constraints: { required: true },

        get: () => data.date,
        set: (value: string) => {
          data.date = value;
        },
        getError: () => errors?.date,
        setError: (value: Array<string> | undefined) => {
          errors.date = value;
        },
        getTainted: () => tainted?.date ?? false,
        setTainted: (value: boolean) => {
          tainted.date = value;
        },
        validate: (): Array<string> => {
          const result = Package.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "date")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Package,
      Array<{ field: string; message: string }>
    > {
      return Package.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Package>): void {
      data = { ...Package.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Package, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.date = formData.get("date") ?? "";
    return Package.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): ScheduleSettings {
    return {
      daysPerWeek: 0,
      rowHeight: "Medium",
      visibleRoutes: [],
      detailedCards: false,
    } as ScheduleSettings;
  }
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
  ): Result<ScheduleSettings, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "ScheduleSettings.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "ScheduleSettings.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("daysPerWeek" in obj)) {
      errors.push({ field: "daysPerWeek", message: "missing required field" });
    }
    if (!("rowHeight" in obj)) {
      errors.push({ field: "rowHeight", message: "missing required field" });
    }
    if (!("visibleRoutes" in obj)) {
      errors.push({
        field: "visibleRoutes",
        message: "missing required field",
      });
    }
    if (!("detailedCards" in obj)) {
      errors.push({
        field: "detailedCards",
        message: "missing required field",
      });
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
      throw new DeserializeError(errors);
    }
    return instance as ScheduleSettings;
  }
}

export namespace ScheduleSettings {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    daysPerWeek?: Array<string>;
    rowHeight?: RowHeight.Errors;
    visibleRoutes?: { _errors?: Array<string>; [index: number]: Array<string> };
    detailedCards?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
      daysPerWeek?: boolean;
      rowHeight?: RowHeight.Tainted;
      visibleRoutes?: { [index: number]: boolean };
      detailedCards?: boolean;
    };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly daysPerWeek: FieldController<number>;
    readonly rowHeight: FieldController<RowHeight>;
    readonly visibleRoutes: FieldController<string[]>;
    readonly detailedCards: FieldController<boolean>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: ScheduleSettings;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<
      ScheduleSettings,
      Array<{ field: string; message: string }>
    >;
    reset(overrides?: Partial<ScheduleSettings>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<ScheduleSettings>,
  ): Gigaform {
    let data = $state({ ...ScheduleSettings.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      daysPerWeek: {
        path: ["daysPerWeek"] as const,
        name: "daysPerWeek",
        constraints: { required: true },

        get: () => data.daysPerWeek,
        set: (value: number) => {
          data.daysPerWeek = value;
        },
        getError: () => errors?.daysPerWeek,
        setError: (value: Array<string> | undefined) => {
          errors.daysPerWeek = value;
        },
        getTainted: () => tainted?.daysPerWeek ?? false,
        setTainted: (value: boolean) => {
          tainted.daysPerWeek = value;
        },
        validate: (): Array<string> => {
          const result = ScheduleSettings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "daysPerWeek")
              .map((e) => e.message);
          }
          return [];
        },
      },
      rowHeight: {
        path: ["rowHeight"] as const,
        name: "rowHeight",
        constraints: { required: true },

        get: () => data.rowHeight,
        set: (value: RowHeight) => {
          data.rowHeight = value;
        },
        getError: () => errors?.rowHeight,
        setError: (value: Array<string> | undefined) => {
          errors.rowHeight = value;
        },
        getTainted: () => tainted?.rowHeight ?? false,
        setTainted: (value: boolean) => {
          tainted.rowHeight = value;
        },
        validate: (): Array<string> => {
          const result = ScheduleSettings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "rowHeight")
              .map((e) => e.message);
          }
          return [];
        },
      },
      visibleRoutes: {
        path: ["visibleRoutes"] as const,
        name: "visibleRoutes",
        constraints: { required: true },

        get: () => data.visibleRoutes,
        set: (value: string[]) => {
          data.visibleRoutes = value;
        },
        getError: () => errors?.visibleRoutes,
        setError: (value: Array<string> | undefined) => {
          errors.visibleRoutes = value;
        },
        getTainted: () => tainted?.visibleRoutes ?? false,
        setTainted: (value: boolean) => {
          tainted.visibleRoutes = value;
        },
        validate: (): Array<string> => {
          const result = ScheduleSettings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "visibleRoutes")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["visibleRoutes", index] as const,
          name: `visibleRoutes.${index}`,
          constraints: { required: true },
          get: () => data.visibleRoutes[index],
          set: (value: string) => {
            data.visibleRoutes[index] = value;
          },
          getError: () =>
            (errors.visibleRoutes as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.visibleRoutes ??= {};
            (errors.visibleRoutes as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: () => tainted.visibleRoutes?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.visibleRoutes ??= {};
            tainted.visibleRoutes[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: string) => {
          data.visibleRoutes.push(item);
        },
        remove: (index: number) => {
          data.visibleRoutes.splice(index, 1);
        },
        swap: (a: number, b: number) => {
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

        get: () => data.detailedCards,
        set: (value: boolean) => {
          data.detailedCards = value;
        },
        getError: () => errors?.detailedCards,
        setError: (value: Array<string> | undefined) => {
          errors.detailedCards = value;
        },
        getTainted: () => tainted?.detailedCards ?? false,
        setTainted: (value: boolean) => {
          tainted.detailedCards = value;
        },
        validate: (): Array<string> => {
          const result = ScheduleSettings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "detailedCards")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      ScheduleSettings,
      Array<{ field: string; message: string }>
    > {
      return ScheduleSettings.fromJSON(data);
    }
    function reset(newOverrides?: Partial<ScheduleSettings>): void {
      data = { ...ScheduleSettings.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<ScheduleSettings, Array<{ field: string; message: string }>> {
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
    return ScheduleSettings.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<DailyRecurrenceRule, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "DailyRecurrenceRule.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "DailyRecurrenceRule.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("quantityOfDays" in obj)) {
      errors.push({
        field: "quantityOfDays",
        message: "missing required field",
      });
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
      const __raw_quantityOfDays = obj["quantityOfDays"];
      instance.quantityOfDays = __raw_quantityOfDays;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as DailyRecurrenceRule;
  }
}

export namespace DailyRecurrenceRule {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    quantityOfDays?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { quantityOfDays?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly quantityOfDays: FieldController<number>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: DailyRecurrenceRule;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<
      DailyRecurrenceRule,
      Array<{ field: string; message: string }>
    >;
    reset(overrides?: Partial<DailyRecurrenceRule>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<DailyRecurrenceRule>,
  ): Gigaform {
    let data = $state({ ...DailyRecurrenceRule.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      quantityOfDays: {
        path: ["quantityOfDays"] as const,
        name: "quantityOfDays",
        constraints: { required: true },

        get: () => data.quantityOfDays,
        set: (value: number) => {
          data.quantityOfDays = value;
        },
        getError: () => errors?.quantityOfDays,
        setError: (value: Array<string> | undefined) => {
          errors.quantityOfDays = value;
        },
        getTainted: () => tainted?.quantityOfDays ?? false,
        setTainted: (value: boolean) => {
          tainted.quantityOfDays = value;
        },
        validate: (): Array<string> => {
          const result = DailyRecurrenceRule.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "quantityOfDays")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      DailyRecurrenceRule,
      Array<{ field: string; message: string }>
    > {
      return DailyRecurrenceRule.fromJSON(data);
    }
    function reset(newOverrides?: Partial<DailyRecurrenceRule>): void {
      data = { ...DailyRecurrenceRule.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<DailyRecurrenceRule, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
      const quantityOfDaysStr = formData.get("quantityOfDays");
      obj.quantityOfDays = quantityOfDaysStr
        ? parseFloat(quantityOfDaysStr as string)
        : 0;
      if (obj.quantityOfDays !== undefined && Number.isNaN(obj.quantityOfDays))
        obj.quantityOfDays = 0;
    }
    return DailyRecurrenceRule.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): SignUpCredentials {
    return {
      firstName: FirstName.defaultValue(),
      lastName: LastName.defaultValue(),
      email: EmailParts.defaultValue(),
      password: Password.defaultValue(),
      rememberMe: false,
    } as SignUpCredentials;
  }
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
  ): Result<SignUpCredentials, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "SignUpCredentials.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "SignUpCredentials.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("firstName" in obj)) {
      errors.push({ field: "firstName", message: "missing required field" });
    }
    if (!("lastName" in obj)) {
      errors.push({ field: "lastName", message: "missing required field" });
    }
    if (!("email" in obj)) {
      errors.push({ field: "email", message: "missing required field" });
    }
    if (!("password" in obj)) {
      errors.push({ field: "password", message: "missing required field" });
    }
    if (!("rememberMe" in obj)) {
      errors.push({ field: "rememberMe", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as SignUpCredentials;
  }
}

export namespace SignUpCredentials {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    firstName?: FirstName.Errors;
    lastName?: LastName.Errors;
    email?: EmailParts.Errors;
    password?: Password.Errors;
    rememberMe?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
      firstName?: FirstName.Tainted;
      lastName?: LastName.Tainted;
      email?: EmailParts.Tainted;
      password?: Password.Tainted;
      rememberMe?: boolean;
    };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly firstName: FieldController<FirstName>;
    readonly lastName: FieldController<LastName>;
    readonly email: FieldController<EmailParts>;
    readonly password: FieldController<Password>;
    readonly rememberMe: FieldController<boolean>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: SignUpCredentials;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<
      SignUpCredentials,
      Array<{ field: string; message: string }>
    >;
    reset(overrides?: Partial<SignUpCredentials>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<SignUpCredentials>,
  ): Gigaform {
    let data = $state({ ...SignUpCredentials.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      firstName: {
        path: ["firstName"] as const,
        name: "firstName",
        constraints: { required: true },

        get: () => data.firstName,
        set: (value: FirstName) => {
          data.firstName = value;
        },
        getError: () => errors?.firstName,
        setError: (value: Array<string> | undefined) => {
          errors.firstName = value;
        },
        getTainted: () => tainted?.firstName ?? false,
        setTainted: (value: boolean) => {
          tainted.firstName = value;
        },
        validate: (): Array<string> => {
          const result = SignUpCredentials.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "firstName")
              .map((e) => e.message);
          }
          return [];
        },
      },
      lastName: {
        path: ["lastName"] as const,
        name: "lastName",
        constraints: { required: true },

        get: () => data.lastName,
        set: (value: LastName) => {
          data.lastName = value;
        },
        getError: () => errors?.lastName,
        setError: (value: Array<string> | undefined) => {
          errors.lastName = value;
        },
        getTainted: () => tainted?.lastName ?? false,
        setTainted: (value: boolean) => {
          tainted.lastName = value;
        },
        validate: (): Array<string> => {
          const result = SignUpCredentials.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "lastName")
              .map((e) => e.message);
          }
          return [];
        },
      },
      email: {
        path: ["email"] as const,
        name: "email",
        constraints: { required: true },

        get: () => data.email,
        set: (value: EmailParts) => {
          data.email = value;
        },
        getError: () => errors?.email,
        setError: (value: Array<string> | undefined) => {
          errors.email = value;
        },
        getTainted: () => tainted?.email ?? false,
        setTainted: (value: boolean) => {
          tainted.email = value;
        },
        validate: (): Array<string> => {
          const result = SignUpCredentials.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "email")
              .map((e) => e.message);
          }
          return [];
        },
      },
      password: {
        path: ["password"] as const,
        name: "password",
        constraints: { required: true },

        get: () => data.password,
        set: (value: Password) => {
          data.password = value;
        },
        getError: () => errors?.password,
        setError: (value: Array<string> | undefined) => {
          errors.password = value;
        },
        getTainted: () => tainted?.password ?? false,
        setTainted: (value: boolean) => {
          tainted.password = value;
        },
        validate: (): Array<string> => {
          const result = SignUpCredentials.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "password")
              .map((e) => e.message);
          }
          return [];
        },
      },
      rememberMe: {
        path: ["rememberMe"] as const,
        name: "rememberMe",
        constraints: { required: true },

        get: () => data.rememberMe,
        set: (value: boolean) => {
          data.rememberMe = value;
        },
        getError: () => errors?.rememberMe,
        setError: (value: Array<string> | undefined) => {
          errors.rememberMe = value;
        },
        getTainted: () => tainted?.rememberMe ?? false,
        setTainted: (value: boolean) => {
          tainted.rememberMe = value;
        },
        validate: (): Array<string> => {
          const result = SignUpCredentials.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "rememberMe")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      SignUpCredentials,
      Array<{ field: string; message: string }>
    > {
      return SignUpCredentials.fromJSON(data);
    }
    function reset(newOverrides?: Partial<SignUpCredentials>): void {
      data = { ...SignUpCredentials.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<SignUpCredentials, Array<{ field: string; message: string }>> {
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
    return SignUpCredentials.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): OverviewSettings {
    return {
      rowHeight: "Medium",
      cardOrRow: "Table",
      perPage: 0,
      columnConfigs: [],
    } as OverviewSettings;
  }
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
  ): Result<OverviewSettings, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "OverviewSettings.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "OverviewSettings.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("rowHeight" in obj)) {
      errors.push({ field: "rowHeight", message: "missing required field" });
    }
    if (!("cardOrRow" in obj)) {
      errors.push({ field: "cardOrRow", message: "missing required field" });
    }
    if (!("perPage" in obj)) {
      errors.push({ field: "perPage", message: "missing required field" });
    }
    if (!("columnConfigs" in obj)) {
      errors.push({
        field: "columnConfigs",
        message: "missing required field",
      });
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
      throw new DeserializeError(errors);
    }
    return instance as OverviewSettings;
  }
}

export namespace OverviewSettings {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    rowHeight?: RowHeight.Errors;
    cardOrRow?: OverviewDisplay.Errors;
    perPage?: Array<string>;
    columnConfigs?: {
      _errors?: Array<string>;
      [index: number]: ColumnConfig.Errors;
    };
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
      rowHeight?: RowHeight.Tainted;
      cardOrRow?: OverviewDisplay.Tainted;
      perPage?: boolean;
      columnConfigs?: { [index: number]: ColumnConfig.Tainted };
    };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly rowHeight: FieldController<RowHeight>;
    readonly cardOrRow: FieldController<OverviewDisplay>;
    readonly perPage: FieldController<number>;
    readonly columnConfigs: FieldController<ColumnConfig[]>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: OverviewSettings;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<
      OverviewSettings,
      Array<{ field: string; message: string }>
    >;
    reset(overrides?: Partial<OverviewSettings>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<OverviewSettings>,
  ): Gigaform {
    let data = $state({ ...OverviewSettings.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      rowHeight: {
        path: ["rowHeight"] as const,
        name: "rowHeight",
        constraints: { required: true },

        get: () => data.rowHeight,
        set: (value: RowHeight) => {
          data.rowHeight = value;
        },
        getError: () => errors?.rowHeight,
        setError: (value: Array<string> | undefined) => {
          errors.rowHeight = value;
        },
        getTainted: () => tainted?.rowHeight ?? false,
        setTainted: (value: boolean) => {
          tainted.rowHeight = value;
        },
        validate: (): Array<string> => {
          const result = OverviewSettings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "rowHeight")
              .map((e) => e.message);
          }
          return [];
        },
      },
      cardOrRow: {
        path: ["cardOrRow"] as const,
        name: "cardOrRow",
        constraints: { required: true },

        get: () => data.cardOrRow,
        set: (value: OverviewDisplay) => {
          data.cardOrRow = value;
        },
        getError: () => errors?.cardOrRow,
        setError: (value: Array<string> | undefined) => {
          errors.cardOrRow = value;
        },
        getTainted: () => tainted?.cardOrRow ?? false,
        setTainted: (value: boolean) => {
          tainted.cardOrRow = value;
        },
        validate: (): Array<string> => {
          const result = OverviewSettings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "cardOrRow")
              .map((e) => e.message);
          }
          return [];
        },
      },
      perPage: {
        path: ["perPage"] as const,
        name: "perPage",
        constraints: { required: true },

        get: () => data.perPage,
        set: (value: number) => {
          data.perPage = value;
        },
        getError: () => errors?.perPage,
        setError: (value: Array<string> | undefined) => {
          errors.perPage = value;
        },
        getTainted: () => tainted?.perPage ?? false,
        setTainted: (value: boolean) => {
          tainted.perPage = value;
        },
        validate: (): Array<string> => {
          const result = OverviewSettings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "perPage")
              .map((e) => e.message);
          }
          return [];
        },
      },
      columnConfigs: {
        path: ["columnConfigs"] as const,
        name: "columnConfigs",
        constraints: { required: true },

        get: () => data.columnConfigs,
        set: (value: ColumnConfig[]) => {
          data.columnConfigs = value;
        },
        getError: () => errors?.columnConfigs,
        setError: (value: Array<string> | undefined) => {
          errors.columnConfigs = value;
        },
        getTainted: () => tainted?.columnConfigs ?? false,
        setTainted: (value: boolean) => {
          tainted.columnConfigs = value;
        },
        validate: (): Array<string> => {
          const result = OverviewSettings.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "columnConfigs")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["columnConfigs", index] as const,
          name: `columnConfigs.${index}`,
          constraints: { required: true },
          get: () => data.columnConfigs[index],
          set: (value: ColumnConfig) => {
            data.columnConfigs[index] = value;
          },
          getError: () =>
            (errors.columnConfigs as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.columnConfigs ??= {};
            (errors.columnConfigs as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: () => tainted.columnConfigs?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.columnConfigs ??= {};
            tainted.columnConfigs[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: ColumnConfig) => {
          data.columnConfigs.push(item);
        },
        remove: (index: number) => {
          data.columnConfigs.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.columnConfigs[a], data.columnConfigs[b]] = [
            data.columnConfigs[b],
            data.columnConfigs[a],
          ];
        },
      },
    };
    function validate(): Result<
      OverviewSettings,
      Array<{ field: string; message: string }>
    > {
      return OverviewSettings.fromJSON(data);
    }
    function reset(newOverrides?: Partial<OverviewSettings>): void {
      data = { ...OverviewSettings.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<OverviewSettings, Array<{ field: string; message: string }>> {
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
    return OverviewSettings.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<FirstName, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "FirstName.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "FirstName.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("name" in obj)) {
      errors.push({ field: "name", message: "missing required field" });
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
      const __raw_name = obj["name"];
      instance.name = __raw_name;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as FirstName;
  }
}

export namespace FirstName {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    name?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { name?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly name: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: FirstName;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<FirstName, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<FirstName>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<FirstName>,
  ): Gigaform {
    let data = $state({ ...FirstName.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: () => data.name,
        set: (value: string) => {
          data.name = value;
        },
        getError: () => errors?.name,
        setError: (value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: () => tainted?.name ?? false,
        setTainted: (value: boolean) => {
          tainted.name = value;
        },
        validate: (): Array<string> => {
          const result = FirstName.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "name")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      FirstName,
      Array<{ field: string; message: string }>
    > {
      return FirstName.fromJSON(data);
    }
    function reset(newOverrides?: Partial<FirstName>): void {
      data = { ...FirstName.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<FirstName, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.name = formData.get("name") ?? "";
    return FirstName.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): Account {
    return {
      id: "",
      taxRate: "",
      site: "",
      salesRep: null,
      orders: [],
      activity: [],
      customFields: [],
      accountName: CompanyName.defaultValue(),
      sector: "Residential",
      memo: null,
      phones: [],
      email: Email.defaultValue(),
      leadSource: "",
      colors: Colors.defaultValue(),
      needsReview: false,
      hasAlert: false,
      accountType: "",
      subtype: "",
      isTaxExempt: false,
      paymentTerms: "",
      tags: [],
      dateAdded: "",
    } as Account;
  }
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
  ): Result<Account, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Account.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Account.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("taxRate" in obj)) {
      errors.push({ field: "taxRate", message: "missing required field" });
    }
    if (!("site" in obj)) {
      errors.push({ field: "site", message: "missing required field" });
    }
    if (!("salesRep" in obj)) {
      errors.push({ field: "salesRep", message: "missing required field" });
    }
    if (!("orders" in obj)) {
      errors.push({ field: "orders", message: "missing required field" });
    }
    if (!("activity" in obj)) {
      errors.push({ field: "activity", message: "missing required field" });
    }
    if (!("customFields" in obj)) {
      errors.push({ field: "customFields", message: "missing required field" });
    }
    if (!("accountName" in obj)) {
      errors.push({ field: "accountName", message: "missing required field" });
    }
    if (!("sector" in obj)) {
      errors.push({ field: "sector", message: "missing required field" });
    }
    if (!("memo" in obj)) {
      errors.push({ field: "memo", message: "missing required field" });
    }
    if (!("phones" in obj)) {
      errors.push({ field: "phones", message: "missing required field" });
    }
    if (!("email" in obj)) {
      errors.push({ field: "email", message: "missing required field" });
    }
    if (!("leadSource" in obj)) {
      errors.push({ field: "leadSource", message: "missing required field" });
    }
    if (!("colors" in obj)) {
      errors.push({ field: "colors", message: "missing required field" });
    }
    if (!("needsReview" in obj)) {
      errors.push({ field: "needsReview", message: "missing required field" });
    }
    if (!("hasAlert" in obj)) {
      errors.push({ field: "hasAlert", message: "missing required field" });
    }
    if (!("accountType" in obj)) {
      errors.push({ field: "accountType", message: "missing required field" });
    }
    if (!("subtype" in obj)) {
      errors.push({ field: "subtype", message: "missing required field" });
    }
    if (!("isTaxExempt" in obj)) {
      errors.push({ field: "isTaxExempt", message: "missing required field" });
    }
    if (!("paymentTerms" in obj)) {
      errors.push({ field: "paymentTerms", message: "missing required field" });
    }
    if (!("tags" in obj)) {
      errors.push({ field: "tags", message: "missing required field" });
    }
    if (!("dateAdded" in obj)) {
      errors.push({ field: "dateAdded", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Account;
  }
}

export namespace Account {
  /** Nested error structure matching the data shape */ export type Errors = {
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
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
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
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly taxRate: FieldController<string | TaxRate>;
    readonly site: FieldController<string | Site>;
    readonly salesRep: FieldController<Represents[] | null>;
    readonly orders: FieldController<Ordered[]>;
    readonly activity: FieldController<Did[]>;
    readonly customFields: FieldController<[string, string][]>;
    readonly accountName: FieldController<AccountName>;
    readonly sector: FieldController<Sector>;
    readonly memo: FieldController<string | null>;
    readonly phones: FieldController<PhoneNumber[]>;
    readonly email: FieldController<Email>;
    readonly leadSource: FieldController<string>;
    readonly colors: FieldController<Colors>;
    readonly needsReview: FieldController<boolean>;
    readonly hasAlert: FieldController<boolean>;
    readonly accountType: FieldController<string>;
    readonly subtype: FieldController<string>;
    readonly isTaxExempt: FieldController<boolean>;
    readonly paymentTerms: FieldController<string>;
    readonly tags: FieldController<string[]>;
    readonly dateAdded: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Account;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Account, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Account>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Account>,
  ): Gigaform {
    let data = $state({ ...Account.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      taxRate: {
        path: ["taxRate"] as const,
        name: "taxRate",
        constraints: { required: true },

        get: () => data.taxRate,
        set: (value: string | TaxRate) => {
          data.taxRate = value;
        },
        getError: () => errors?.taxRate,
        setError: (value: Array<string> | undefined) => {
          errors.taxRate = value;
        },
        getTainted: () => tainted?.taxRate ?? false,
        setTainted: (value: boolean) => {
          tainted.taxRate = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "taxRate")
              .map((e) => e.message);
          }
          return [];
        },
      },
      site: {
        path: ["site"] as const,
        name: "site",
        constraints: { required: true },

        get: () => data.site,
        set: (value: string | Site) => {
          data.site = value;
        },
        getError: () => errors?.site,
        setError: (value: Array<string> | undefined) => {
          errors.site = value;
        },
        getTainted: () => tainted?.site ?? false,
        setTainted: (value: boolean) => {
          tainted.site = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "site")
              .map((e) => e.message);
          }
          return [];
        },
      },
      salesRep: {
        path: ["salesRep"] as const,
        name: "salesRep",
        constraints: { required: true },

        get: () => data.salesRep,
        set: (value: Represents[] | null) => {
          data.salesRep = value;
        },
        getError: () => errors?.salesRep,
        setError: (value: Array<string> | undefined) => {
          errors.salesRep = value;
        },
        getTainted: () => tainted?.salesRep ?? false,
        setTainted: (value: boolean) => {
          tainted.salesRep = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "salesRep")
              .map((e) => e.message);
          }
          return [];
        },
      },
      orders: {
        path: ["orders"] as const,
        name: "orders",
        constraints: { required: true },

        get: () => data.orders,
        set: (value: Ordered[]) => {
          data.orders = value;
        },
        getError: () => errors?.orders,
        setError: (value: Array<string> | undefined) => {
          errors.orders = value;
        },
        getTainted: () => tainted?.orders ?? false,
        setTainted: (value: boolean) => {
          tainted.orders = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "orders")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["orders", index] as const,
          name: `orders.${index}`,
          constraints: { required: true },
          get: () => data.orders[index],
          set: (value: Ordered) => {
            data.orders[index] = value;
          },
          getError: () =>
            (errors.orders as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.orders ??= {};
            (errors.orders as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.orders?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.orders ??= {};
            tainted.orders[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: Ordered) => {
          data.orders.push(item);
        },
        remove: (index: number) => {
          data.orders.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.orders[a], data.orders[b]] = [data.orders[b], data.orders[a]];
        },
      },
      activity: {
        path: ["activity"] as const,
        name: "activity",
        constraints: { required: true },

        get: () => data.activity,
        set: (value: Did[]) => {
          data.activity = value;
        },
        getError: () => errors?.activity,
        setError: (value: Array<string> | undefined) => {
          errors.activity = value;
        },
        getTainted: () => tainted?.activity ?? false,
        setTainted: (value: boolean) => {
          tainted.activity = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "activity")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["activity", index] as const,
          name: `activity.${index}`,
          constraints: { required: true },
          get: () => data.activity[index],
          set: (value: Did) => {
            data.activity[index] = value;
          },
          getError: () =>
            (errors.activity as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.activity ??= {};
            (errors.activity as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.activity?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.activity ??= {};
            tainted.activity[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: Did) => {
          data.activity.push(item);
        },
        remove: (index: number) => {
          data.activity.splice(index, 1);
        },
        swap: (a: number, b: number) => {
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

        get: () => data.customFields,
        set: (value: [string, string][]) => {
          data.customFields = value;
        },
        getError: () => errors?.customFields,
        setError: (value: Array<string> | undefined) => {
          errors.customFields = value;
        },
        getTainted: () => tainted?.customFields ?? false,
        setTainted: (value: boolean) => {
          tainted.customFields = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "customFields")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["customFields", index] as const,
          name: `customFields.${index}`,
          constraints: { required: true },
          get: () => data.customFields[index],
          set: (value: [string, string]) => {
            data.customFields[index] = value;
          },
          getError: () =>
            (errors.customFields as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.customFields ??= {};
            (errors.customFields as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: () => tainted.customFields?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.customFields ??= {};
            tainted.customFields[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: [string, string]) => {
          data.customFields.push(item);
        },
        remove: (index: number) => {
          data.customFields.splice(index, 1);
        },
        swap: (a: number, b: number) => {
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

        get: () => data.accountName,
        set: (value: AccountName) => {
          data.accountName = value;
        },
        getError: () => errors?.accountName,
        setError: (value: Array<string> | undefined) => {
          errors.accountName = value;
        },
        getTainted: () => tainted?.accountName ?? false,
        setTainted: (value: boolean) => {
          tainted.accountName = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "accountName")
              .map((e) => e.message);
          }
          return [];
        },
      },
      sector: {
        path: ["sector"] as const,
        name: "sector",
        constraints: { required: true },

        get: () => data.sector,
        set: (value: Sector) => {
          data.sector = value;
        },
        getError: () => errors?.sector,
        setError: (value: Array<string> | undefined) => {
          errors.sector = value;
        },
        getTainted: () => tainted?.sector ?? false,
        setTainted: (value: boolean) => {
          tainted.sector = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "sector")
              .map((e) => e.message);
          }
          return [];
        },
      },
      memo: {
        path: ["memo"] as const,
        name: "memo",
        constraints: { required: true },

        get: () => data.memo,
        set: (value: string | null) => {
          data.memo = value;
        },
        getError: () => errors?.memo,
        setError: (value: Array<string> | undefined) => {
          errors.memo = value;
        },
        getTainted: () => tainted?.memo ?? false,
        setTainted: (value: boolean) => {
          tainted.memo = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "memo")
              .map((e) => e.message);
          }
          return [];
        },
      },
      phones: {
        path: ["phones"] as const,
        name: "phones",
        constraints: { required: true },

        get: () => data.phones,
        set: (value: PhoneNumber[]) => {
          data.phones = value;
        },
        getError: () => errors?.phones,
        setError: (value: Array<string> | undefined) => {
          errors.phones = value;
        },
        getTainted: () => tainted?.phones ?? false,
        setTainted: (value: boolean) => {
          tainted.phones = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "phones")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["phones", index] as const,
          name: `phones.${index}`,
          constraints: { required: true },
          get: () => data.phones[index],
          set: (value: PhoneNumber) => {
            data.phones[index] = value;
          },
          getError: () =>
            (errors.phones as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.phones ??= {};
            (errors.phones as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.phones?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.phones ??= {};
            tainted.phones[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: PhoneNumber) => {
          data.phones.push(item);
        },
        remove: (index: number) => {
          data.phones.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.phones[a], data.phones[b]] = [data.phones[b], data.phones[a]];
        },
      },
      email: {
        path: ["email"] as const,
        name: "email",
        constraints: { required: true },

        get: () => data.email,
        set: (value: Email) => {
          data.email = value;
        },
        getError: () => errors?.email,
        setError: (value: Array<string> | undefined) => {
          errors.email = value;
        },
        getTainted: () => tainted?.email ?? false,
        setTainted: (value: boolean) => {
          tainted.email = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "email")
              .map((e) => e.message);
          }
          return [];
        },
      },
      leadSource: {
        path: ["leadSource"] as const,
        name: "leadSource",
        constraints: { required: true },

        get: () => data.leadSource,
        set: (value: string) => {
          data.leadSource = value;
        },
        getError: () => errors?.leadSource,
        setError: (value: Array<string> | undefined) => {
          errors.leadSource = value;
        },
        getTainted: () => tainted?.leadSource ?? false,
        setTainted: (value: boolean) => {
          tainted.leadSource = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "leadSource")
              .map((e) => e.message);
          }
          return [];
        },
      },
      colors: {
        path: ["colors"] as const,
        name: "colors",
        constraints: { required: true },

        get: () => data.colors,
        set: (value: Colors) => {
          data.colors = value;
        },
        getError: () => errors?.colors,
        setError: (value: Array<string> | undefined) => {
          errors.colors = value;
        },
        getTainted: () => tainted?.colors ?? false,
        setTainted: (value: boolean) => {
          tainted.colors = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "colors")
              .map((e) => e.message);
          }
          return [];
        },
      },
      needsReview: {
        path: ["needsReview"] as const,
        name: "needsReview",
        constraints: { required: true },

        get: () => data.needsReview,
        set: (value: boolean) => {
          data.needsReview = value;
        },
        getError: () => errors?.needsReview,
        setError: (value: Array<string> | undefined) => {
          errors.needsReview = value;
        },
        getTainted: () => tainted?.needsReview ?? false,
        setTainted: (value: boolean) => {
          tainted.needsReview = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "needsReview")
              .map((e) => e.message);
          }
          return [];
        },
      },
      hasAlert: {
        path: ["hasAlert"] as const,
        name: "hasAlert",
        constraints: { required: true },

        get: () => data.hasAlert,
        set: (value: boolean) => {
          data.hasAlert = value;
        },
        getError: () => errors?.hasAlert,
        setError: (value: Array<string> | undefined) => {
          errors.hasAlert = value;
        },
        getTainted: () => tainted?.hasAlert ?? false,
        setTainted: (value: boolean) => {
          tainted.hasAlert = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "hasAlert")
              .map((e) => e.message);
          }
          return [];
        },
      },
      accountType: {
        path: ["accountType"] as const,
        name: "accountType",
        constraints: { required: true },

        get: () => data.accountType,
        set: (value: string) => {
          data.accountType = value;
        },
        getError: () => errors?.accountType,
        setError: (value: Array<string> | undefined) => {
          errors.accountType = value;
        },
        getTainted: () => tainted?.accountType ?? false,
        setTainted: (value: boolean) => {
          tainted.accountType = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "accountType")
              .map((e) => e.message);
          }
          return [];
        },
      },
      subtype: {
        path: ["subtype"] as const,
        name: "subtype",
        constraints: { required: true },

        get: () => data.subtype,
        set: (value: string) => {
          data.subtype = value;
        },
        getError: () => errors?.subtype,
        setError: (value: Array<string> | undefined) => {
          errors.subtype = value;
        },
        getTainted: () => tainted?.subtype ?? false,
        setTainted: (value: boolean) => {
          tainted.subtype = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "subtype")
              .map((e) => e.message);
          }
          return [];
        },
      },
      isTaxExempt: {
        path: ["isTaxExempt"] as const,
        name: "isTaxExempt",
        constraints: { required: true },

        get: () => data.isTaxExempt,
        set: (value: boolean) => {
          data.isTaxExempt = value;
        },
        getError: () => errors?.isTaxExempt,
        setError: (value: Array<string> | undefined) => {
          errors.isTaxExempt = value;
        },
        getTainted: () => tainted?.isTaxExempt ?? false,
        setTainted: (value: boolean) => {
          tainted.isTaxExempt = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "isTaxExempt")
              .map((e) => e.message);
          }
          return [];
        },
      },
      paymentTerms: {
        path: ["paymentTerms"] as const,
        name: "paymentTerms",
        constraints: { required: true },

        get: () => data.paymentTerms,
        set: (value: string) => {
          data.paymentTerms = value;
        },
        getError: () => errors?.paymentTerms,
        setError: (value: Array<string> | undefined) => {
          errors.paymentTerms = value;
        },
        getTainted: () => tainted?.paymentTerms ?? false,
        setTainted: (value: boolean) => {
          tainted.paymentTerms = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "paymentTerms")
              .map((e) => e.message);
          }
          return [];
        },
      },
      tags: {
        path: ["tags"] as const,
        name: "tags",
        constraints: { required: true },

        get: () => data.tags,
        set: (value: string[]) => {
          data.tags = value;
        },
        getError: () => errors?.tags,
        setError: (value: Array<string> | undefined) => {
          errors.tags = value;
        },
        getTainted: () => tainted?.tags ?? false,
        setTainted: (value: boolean) => {
          tainted.tags = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "tags")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["tags", index] as const,
          name: `tags.${index}`,
          constraints: { required: true },
          get: () => data.tags[index],
          set: (value: string) => {
            data.tags[index] = value;
          },
          getError: () =>
            (errors.tags as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.tags ??= {};
            (errors.tags as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.tags?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.tags ??= {};
            tainted.tags[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: string) => {
          data.tags.push(item);
        },
        remove: (index: number) => {
          data.tags.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.tags[a], data.tags[b]] = [data.tags[b], data.tags[a]];
        },
      },
      dateAdded: {
        path: ["dateAdded"] as const,
        name: "dateAdded",
        constraints: { required: true },

        get: () => data.dateAdded,
        set: (value: string) => {
          data.dateAdded = value;
        },
        getError: () => errors?.dateAdded,
        setError: (value: Array<string> | undefined) => {
          errors.dateAdded = value;
        },
        getTainted: () => tainted?.dateAdded ?? false,
        setTainted: (value: boolean) => {
          tainted.dateAdded = value;
        },
        validate: (): Array<string> => {
          const result = Account.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "dateAdded")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Account,
      Array<{ field: string; message: string }>
    > {
      return Account.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Account>): void {
      data = { ...Account.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Account, Array<{ field: string; message: string }>> {
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
    return Account.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Edited, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Edited.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "Edited.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("fieldName" in obj)) {
      errors.push({ field: "fieldName", message: "missing required field" });
    }
    if (!("oldValue" in obj)) {
      errors.push({ field: "oldValue", message: "missing required field" });
    }
    if (!("newValue" in obj)) {
      errors.push({ field: "newValue", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Edited;
  }
}

export namespace Edited {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    fieldName?: Array<string>;
    oldValue?: Array<string>;
    newValue?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { fieldName?: boolean; oldValue?: boolean; newValue?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly fieldName: FieldController<string>;
    readonly oldValue: FieldController<string | null>;
    readonly newValue: FieldController<string | null>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Edited;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Edited, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Edited>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Edited>,
  ): Gigaform {
    let data = $state({ ...Edited.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      fieldName: {
        path: ["fieldName"] as const,
        name: "fieldName",
        constraints: { required: true },

        get: () => data.fieldName,
        set: (value: string) => {
          data.fieldName = value;
        },
        getError: () => errors?.fieldName,
        setError: (value: Array<string> | undefined) => {
          errors.fieldName = value;
        },
        getTainted: () => tainted?.fieldName ?? false,
        setTainted: (value: boolean) => {
          tainted.fieldName = value;
        },
        validate: (): Array<string> => {
          const result = Edited.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "fieldName")
              .map((e) => e.message);
          }
          return [];
        },
      },
      oldValue: {
        path: ["oldValue"] as const,
        name: "oldValue",
        constraints: { required: true },

        get: () => data.oldValue,
        set: (value: string | null) => {
          data.oldValue = value;
        },
        getError: () => errors?.oldValue,
        setError: (value: Array<string> | undefined) => {
          errors.oldValue = value;
        },
        getTainted: () => tainted?.oldValue ?? false,
        setTainted: (value: boolean) => {
          tainted.oldValue = value;
        },
        validate: (): Array<string> => {
          const result = Edited.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "oldValue")
              .map((e) => e.message);
          }
          return [];
        },
      },
      newValue: {
        path: ["newValue"] as const,
        name: "newValue",
        constraints: { required: true },

        get: () => data.newValue,
        set: (value: string | null) => {
          data.newValue = value;
        },
        getError: () => errors?.newValue,
        setError: (value: Array<string> | undefined) => {
          errors.newValue = value;
        },
        getTainted: () => tainted?.newValue ?? false,
        setTainted: (value: boolean) => {
          tainted.newValue = value;
        },
        validate: (): Array<string> => {
          const result = Edited.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "newValue")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Edited,
      Array<{ field: string; message: string }>
    > {
      return Edited.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Edited>): void {
      data = { ...Edited.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Edited, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.fieldName = formData.get("fieldName") ?? "";
    obj.oldValue = formData.get("oldValue") ?? "";
    obj.newValue = formData.get("newValue") ?? "";
    return Edited.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): Order {
    return {
      id: "",
      account: "",
      stage: "Estimate",
      number: 0,
      payments: [],
      opportunity: "",
      reference: "",
      leadSource: "",
      salesRep: "",
      group: "",
      subgroup: "",
      isPosted: false,
      needsReview: false,
      actionItem: "",
      upsale: 0,
      dateCreated: "",
      appointment: "",
      lastTechs: [],
      package: null,
      promotion: null,
      balance: 0,
      due: "",
      total: 0,
      site: "",
      billedItems: [],
      memo: "",
      discount: 0,
      tip: 0,
      commissions: [],
    } as Order;
  }
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
  ): Result<Order, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Order.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "Order.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("account" in obj)) {
      errors.push({ field: "account", message: "missing required field" });
    }
    if (!("stage" in obj)) {
      errors.push({ field: "stage", message: "missing required field" });
    }
    if (!("number" in obj)) {
      errors.push({ field: "number", message: "missing required field" });
    }
    if (!("payments" in obj)) {
      errors.push({ field: "payments", message: "missing required field" });
    }
    if (!("opportunity" in obj)) {
      errors.push({ field: "opportunity", message: "missing required field" });
    }
    if (!("reference" in obj)) {
      errors.push({ field: "reference", message: "missing required field" });
    }
    if (!("leadSource" in obj)) {
      errors.push({ field: "leadSource", message: "missing required field" });
    }
    if (!("salesRep" in obj)) {
      errors.push({ field: "salesRep", message: "missing required field" });
    }
    if (!("group" in obj)) {
      errors.push({ field: "group", message: "missing required field" });
    }
    if (!("subgroup" in obj)) {
      errors.push({ field: "subgroup", message: "missing required field" });
    }
    if (!("isPosted" in obj)) {
      errors.push({ field: "isPosted", message: "missing required field" });
    }
    if (!("needsReview" in obj)) {
      errors.push({ field: "needsReview", message: "missing required field" });
    }
    if (!("actionItem" in obj)) {
      errors.push({ field: "actionItem", message: "missing required field" });
    }
    if (!("upsale" in obj)) {
      errors.push({ field: "upsale", message: "missing required field" });
    }
    if (!("dateCreated" in obj)) {
      errors.push({ field: "dateCreated", message: "missing required field" });
    }
    if (!("appointment" in obj)) {
      errors.push({ field: "appointment", message: "missing required field" });
    }
    if (!("lastTechs" in obj)) {
      errors.push({ field: "lastTechs", message: "missing required field" });
    }
    if (!("package" in obj)) {
      errors.push({ field: "package", message: "missing required field" });
    }
    if (!("promotion" in obj)) {
      errors.push({ field: "promotion", message: "missing required field" });
    }
    if (!("balance" in obj)) {
      errors.push({ field: "balance", message: "missing required field" });
    }
    if (!("due" in obj)) {
      errors.push({ field: "due", message: "missing required field" });
    }
    if (!("total" in obj)) {
      errors.push({ field: "total", message: "missing required field" });
    }
    if (!("site" in obj)) {
      errors.push({ field: "site", message: "missing required field" });
    }
    if (!("billedItems" in obj)) {
      errors.push({ field: "billedItems", message: "missing required field" });
    }
    if (!("memo" in obj)) {
      errors.push({ field: "memo", message: "missing required field" });
    }
    if (!("discount" in obj)) {
      errors.push({ field: "discount", message: "missing required field" });
    }
    if (!("tip" in obj)) {
      errors.push({ field: "tip", message: "missing required field" });
    }
    if (!("commissions" in obj)) {
      errors.push({ field: "commissions", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Order;
  }
}

export namespace Order {
  /** Nested error structure matching the data shape */ export type Errors = {
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
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
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
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly account: FieldController<string | Account>;
    readonly stage: FieldController<OrderStage>;
    readonly number: FieldController<number>;
    readonly payments: FieldController<(string | Payment)[]>;
    readonly opportunity: FieldController<string>;
    readonly reference: FieldController<string>;
    readonly leadSource: FieldController<string>;
    readonly salesRep: FieldController<string | Employee>;
    readonly group: FieldController<string>;
    readonly subgroup: FieldController<string>;
    readonly isPosted: FieldController<boolean>;
    readonly needsReview: FieldController<boolean>;
    readonly actionItem: FieldController<string>;
    readonly upsale: FieldController<number>;
    readonly dateCreated: FieldController<string>;
    readonly appointment: FieldController<string | Appointment>;
    readonly lastTechs: FieldController<(string | Employee)[]>;
    readonly package: FieldController<(string | Package)[] | null>;
    readonly promotion: FieldController<(string | Promotion)[] | null>;
    readonly balance: FieldController<number>;
    readonly due: FieldController<string>;
    readonly total: FieldController<number>;
    readonly site: FieldController<string | Site>;
    readonly billedItems: FieldController<BilledItem[]>;
    readonly memo: FieldController<string>;
    readonly discount: FieldController<number>;
    readonly tip: FieldController<number>;
    readonly commissions: FieldController<number[]>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Order;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Order, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Order>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Order>,
  ): Gigaform {
    let data = $state({ ...Order.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      account: {
        path: ["account"] as const,
        name: "account",
        constraints: { required: true },

        get: () => data.account,
        set: (value: string | Account) => {
          data.account = value;
        },
        getError: () => errors?.account,
        setError: (value: Array<string> | undefined) => {
          errors.account = value;
        },
        getTainted: () => tainted?.account ?? false,
        setTainted: (value: boolean) => {
          tainted.account = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "account")
              .map((e) => e.message);
          }
          return [];
        },
      },
      stage: {
        path: ["stage"] as const,
        name: "stage",
        constraints: { required: true },

        get: () => data.stage,
        set: (value: OrderStage) => {
          data.stage = value;
        },
        getError: () => errors?.stage,
        setError: (value: Array<string> | undefined) => {
          errors.stage = value;
        },
        getTainted: () => tainted?.stage ?? false,
        setTainted: (value: boolean) => {
          tainted.stage = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "stage")
              .map((e) => e.message);
          }
          return [];
        },
      },
      number: {
        path: ["number"] as const,
        name: "number",
        constraints: { required: true },

        get: () => data.number,
        set: (value: number) => {
          data.number = value;
        },
        getError: () => errors?.number,
        setError: (value: Array<string> | undefined) => {
          errors.number = value;
        },
        getTainted: () => tainted?.number ?? false,
        setTainted: (value: boolean) => {
          tainted.number = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "number")
              .map((e) => e.message);
          }
          return [];
        },
      },
      payments: {
        path: ["payments"] as const,
        name: "payments",
        constraints: { required: true },

        get: () => data.payments,
        set: (value: (string | Payment)[]) => {
          data.payments = value;
        },
        getError: () => errors?.payments,
        setError: (value: Array<string> | undefined) => {
          errors.payments = value;
        },
        getTainted: () => tainted?.payments ?? false,
        setTainted: (value: boolean) => {
          tainted.payments = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "payments")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["payments", index] as const,
          name: `payments.${index}`,
          constraints: { required: true },
          get: () => data.payments[index],
          set: (value: string | Payment) => {
            data.payments[index] = value;
          },
          getError: () =>
            (errors.payments as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.payments ??= {};
            (errors.payments as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.payments?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.payments ??= {};
            tainted.payments[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: string | Payment) => {
          data.payments.push(item);
        },
        remove: (index: number) => {
          data.payments.splice(index, 1);
        },
        swap: (a: number, b: number) => {
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

        get: () => data.opportunity,
        set: (value: string) => {
          data.opportunity = value;
        },
        getError: () => errors?.opportunity,
        setError: (value: Array<string> | undefined) => {
          errors.opportunity = value;
        },
        getTainted: () => tainted?.opportunity ?? false,
        setTainted: (value: boolean) => {
          tainted.opportunity = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "opportunity")
              .map((e) => e.message);
          }
          return [];
        },
      },
      reference: {
        path: ["reference"] as const,
        name: "reference",
        constraints: { required: true },

        get: () => data.reference,
        set: (value: string) => {
          data.reference = value;
        },
        getError: () => errors?.reference,
        setError: (value: Array<string> | undefined) => {
          errors.reference = value;
        },
        getTainted: () => tainted?.reference ?? false,
        setTainted: (value: boolean) => {
          tainted.reference = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "reference")
              .map((e) => e.message);
          }
          return [];
        },
      },
      leadSource: {
        path: ["leadSource"] as const,
        name: "leadSource",
        constraints: { required: true },

        get: () => data.leadSource,
        set: (value: string) => {
          data.leadSource = value;
        },
        getError: () => errors?.leadSource,
        setError: (value: Array<string> | undefined) => {
          errors.leadSource = value;
        },
        getTainted: () => tainted?.leadSource ?? false,
        setTainted: (value: boolean) => {
          tainted.leadSource = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "leadSource")
              .map((e) => e.message);
          }
          return [];
        },
      },
      salesRep: {
        path: ["salesRep"] as const,
        name: "salesRep",
        constraints: { required: true },

        get: () => data.salesRep,
        set: (value: string | Employee) => {
          data.salesRep = value;
        },
        getError: () => errors?.salesRep,
        setError: (value: Array<string> | undefined) => {
          errors.salesRep = value;
        },
        getTainted: () => tainted?.salesRep ?? false,
        setTainted: (value: boolean) => {
          tainted.salesRep = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "salesRep")
              .map((e) => e.message);
          }
          return [];
        },
      },
      group: {
        path: ["group"] as const,
        name: "group",
        constraints: { required: true },

        get: () => data.group,
        set: (value: string) => {
          data.group = value;
        },
        getError: () => errors?.group,
        setError: (value: Array<string> | undefined) => {
          errors.group = value;
        },
        getTainted: () => tainted?.group ?? false,
        setTainted: (value: boolean) => {
          tainted.group = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "group")
              .map((e) => e.message);
          }
          return [];
        },
      },
      subgroup: {
        path: ["subgroup"] as const,
        name: "subgroup",
        constraints: { required: true },

        get: () => data.subgroup,
        set: (value: string) => {
          data.subgroup = value;
        },
        getError: () => errors?.subgroup,
        setError: (value: Array<string> | undefined) => {
          errors.subgroup = value;
        },
        getTainted: () => tainted?.subgroup ?? false,
        setTainted: (value: boolean) => {
          tainted.subgroup = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "subgroup")
              .map((e) => e.message);
          }
          return [];
        },
      },
      isPosted: {
        path: ["isPosted"] as const,
        name: "isPosted",
        constraints: { required: true },

        get: () => data.isPosted,
        set: (value: boolean) => {
          data.isPosted = value;
        },
        getError: () => errors?.isPosted,
        setError: (value: Array<string> | undefined) => {
          errors.isPosted = value;
        },
        getTainted: () => tainted?.isPosted ?? false,
        setTainted: (value: boolean) => {
          tainted.isPosted = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "isPosted")
              .map((e) => e.message);
          }
          return [];
        },
      },
      needsReview: {
        path: ["needsReview"] as const,
        name: "needsReview",
        constraints: { required: true },

        get: () => data.needsReview,
        set: (value: boolean) => {
          data.needsReview = value;
        },
        getError: () => errors?.needsReview,
        setError: (value: Array<string> | undefined) => {
          errors.needsReview = value;
        },
        getTainted: () => tainted?.needsReview ?? false,
        setTainted: (value: boolean) => {
          tainted.needsReview = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "needsReview")
              .map((e) => e.message);
          }
          return [];
        },
      },
      actionItem: {
        path: ["actionItem"] as const,
        name: "actionItem",
        constraints: { required: true },

        get: () => data.actionItem,
        set: (value: string) => {
          data.actionItem = value;
        },
        getError: () => errors?.actionItem,
        setError: (value: Array<string> | undefined) => {
          errors.actionItem = value;
        },
        getTainted: () => tainted?.actionItem ?? false,
        setTainted: (value: boolean) => {
          tainted.actionItem = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "actionItem")
              .map((e) => e.message);
          }
          return [];
        },
      },
      upsale: {
        path: ["upsale"] as const,
        name: "upsale",
        constraints: { required: true },

        get: () => data.upsale,
        set: (value: number) => {
          data.upsale = value;
        },
        getError: () => errors?.upsale,
        setError: (value: Array<string> | undefined) => {
          errors.upsale = value;
        },
        getTainted: () => tainted?.upsale ?? false,
        setTainted: (value: boolean) => {
          tainted.upsale = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "upsale")
              .map((e) => e.message);
          }
          return [];
        },
      },
      dateCreated: {
        path: ["dateCreated"] as const,
        name: "dateCreated",
        constraints: { required: true },

        get: () => data.dateCreated,
        set: (value: string) => {
          data.dateCreated = value;
        },
        getError: () => errors?.dateCreated,
        setError: (value: Array<string> | undefined) => {
          errors.dateCreated = value;
        },
        getTainted: () => tainted?.dateCreated ?? false,
        setTainted: (value: boolean) => {
          tainted.dateCreated = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "dateCreated")
              .map((e) => e.message);
          }
          return [];
        },
      },
      appointment: {
        path: ["appointment"] as const,
        name: "appointment",
        constraints: { required: true },

        get: () => data.appointment,
        set: (value: string | Appointment) => {
          data.appointment = value;
        },
        getError: () => errors?.appointment,
        setError: (value: Array<string> | undefined) => {
          errors.appointment = value;
        },
        getTainted: () => tainted?.appointment ?? false,
        setTainted: (value: boolean) => {
          tainted.appointment = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "appointment")
              .map((e) => e.message);
          }
          return [];
        },
      },
      lastTechs: {
        path: ["lastTechs"] as const,
        name: "lastTechs",
        constraints: { required: true },

        get: () => data.lastTechs,
        set: (value: (string | Employee)[]) => {
          data.lastTechs = value;
        },
        getError: () => errors?.lastTechs,
        setError: (value: Array<string> | undefined) => {
          errors.lastTechs = value;
        },
        getTainted: () => tainted?.lastTechs ?? false,
        setTainted: (value: boolean) => {
          tainted.lastTechs = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "lastTechs")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["lastTechs", index] as const,
          name: `lastTechs.${index}`,
          constraints: { required: true },
          get: () => data.lastTechs[index],
          set: (value: string | Employee) => {
            data.lastTechs[index] = value;
          },
          getError: () =>
            (errors.lastTechs as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.lastTechs ??= {};
            (errors.lastTechs as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.lastTechs?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.lastTechs ??= {};
            tainted.lastTechs[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: string | Employee) => {
          data.lastTechs.push(item);
        },
        remove: (index: number) => {
          data.lastTechs.splice(index, 1);
        },
        swap: (a: number, b: number) => {
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

        get: () => data.package,
        set: (value: (string | Package)[] | null) => {
          data.package = value;
        },
        getError: () => errors?.package,
        setError: (value: Array<string> | undefined) => {
          errors.package = value;
        },
        getTainted: () => tainted?.package ?? false,
        setTainted: (value: boolean) => {
          tainted.package = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "package")
              .map((e) => e.message);
          }
          return [];
        },
      },
      promotion: {
        path: ["promotion"] as const,
        name: "promotion",
        constraints: { required: true },

        get: () => data.promotion,
        set: (value: (string | Promotion)[] | null) => {
          data.promotion = value;
        },
        getError: () => errors?.promotion,
        setError: (value: Array<string> | undefined) => {
          errors.promotion = value;
        },
        getTainted: () => tainted?.promotion ?? false,
        setTainted: (value: boolean) => {
          tainted.promotion = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "promotion")
              .map((e) => e.message);
          }
          return [];
        },
      },
      balance: {
        path: ["balance"] as const,
        name: "balance",
        constraints: { required: true },

        get: () => data.balance,
        set: (value: number) => {
          data.balance = value;
        },
        getError: () => errors?.balance,
        setError: (value: Array<string> | undefined) => {
          errors.balance = value;
        },
        getTainted: () => tainted?.balance ?? false,
        setTainted: (value: boolean) => {
          tainted.balance = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "balance")
              .map((e) => e.message);
          }
          return [];
        },
      },
      due: {
        path: ["due"] as const,
        name: "due",
        constraints: { required: true },

        get: () => data.due,
        set: (value: string) => {
          data.due = value;
        },
        getError: () => errors?.due,
        setError: (value: Array<string> | undefined) => {
          errors.due = value;
        },
        getTainted: () => tainted?.due ?? false,
        setTainted: (value: boolean) => {
          tainted.due = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "due")
              .map((e) => e.message);
          }
          return [];
        },
      },
      total: {
        path: ["total"] as const,
        name: "total",
        constraints: { required: true },

        get: () => data.total,
        set: (value: number) => {
          data.total = value;
        },
        getError: () => errors?.total,
        setError: (value: Array<string> | undefined) => {
          errors.total = value;
        },
        getTainted: () => tainted?.total ?? false,
        setTainted: (value: boolean) => {
          tainted.total = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "total")
              .map((e) => e.message);
          }
          return [];
        },
      },
      site: {
        path: ["site"] as const,
        name: "site",
        constraints: { required: true },

        get: () => data.site,
        set: (value: string | Site) => {
          data.site = value;
        },
        getError: () => errors?.site,
        setError: (value: Array<string> | undefined) => {
          errors.site = value;
        },
        getTainted: () => tainted?.site ?? false,
        setTainted: (value: boolean) => {
          tainted.site = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "site")
              .map((e) => e.message);
          }
          return [];
        },
      },
      billedItems: {
        path: ["billedItems"] as const,
        name: "billedItems",
        constraints: { required: true },

        get: () => data.billedItems,
        set: (value: BilledItem[]) => {
          data.billedItems = value;
        },
        getError: () => errors?.billedItems,
        setError: (value: Array<string> | undefined) => {
          errors.billedItems = value;
        },
        getTainted: () => tainted?.billedItems ?? false,
        setTainted: (value: boolean) => {
          tainted.billedItems = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "billedItems")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["billedItems", index] as const,
          name: `billedItems.${index}`,
          constraints: { required: true },
          get: () => data.billedItems[index],
          set: (value: BilledItem) => {
            data.billedItems[index] = value;
          },
          getError: () =>
            (errors.billedItems as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.billedItems ??= {};
            (errors.billedItems as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: () => tainted.billedItems?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.billedItems ??= {};
            tainted.billedItems[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: BilledItem) => {
          data.billedItems.push(item);
        },
        remove: (index: number) => {
          data.billedItems.splice(index, 1);
        },
        swap: (a: number, b: number) => {
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

        get: () => data.memo,
        set: (value: string) => {
          data.memo = value;
        },
        getError: () => errors?.memo,
        setError: (value: Array<string> | undefined) => {
          errors.memo = value;
        },
        getTainted: () => tainted?.memo ?? false,
        setTainted: (value: boolean) => {
          tainted.memo = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "memo")
              .map((e) => e.message);
          }
          return [];
        },
      },
      discount: {
        path: ["discount"] as const,
        name: "discount",
        constraints: { required: true },

        get: () => data.discount,
        set: (value: number) => {
          data.discount = value;
        },
        getError: () => errors?.discount,
        setError: (value: Array<string> | undefined) => {
          errors.discount = value;
        },
        getTainted: () => tainted?.discount ?? false,
        setTainted: (value: boolean) => {
          tainted.discount = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "discount")
              .map((e) => e.message);
          }
          return [];
        },
      },
      tip: {
        path: ["tip"] as const,
        name: "tip",
        constraints: { required: true },

        get: () => data.tip,
        set: (value: number) => {
          data.tip = value;
        },
        getError: () => errors?.tip,
        setError: (value: Array<string> | undefined) => {
          errors.tip = value;
        },
        getTainted: () => tainted?.tip ?? false,
        setTainted: (value: boolean) => {
          tainted.tip = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "tip")
              .map((e) => e.message);
          }
          return [];
        },
      },
      commissions: {
        path: ["commissions"] as const,
        name: "commissions",
        constraints: { required: true },

        get: () => data.commissions,
        set: (value: number[]) => {
          data.commissions = value;
        },
        getError: () => errors?.commissions,
        setError: (value: Array<string> | undefined) => {
          errors.commissions = value;
        },
        getTainted: () => tainted?.commissions ?? false,
        setTainted: (value: boolean) => {
          tainted.commissions = value;
        },
        validate: (): Array<string> => {
          const result = Order.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "commissions")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["commissions", index] as const,
          name: `commissions.${index}`,
          constraints: { required: true },
          get: () => data.commissions[index],
          set: (value: number) => {
            data.commissions[index] = value;
          },
          getError: () =>
            (errors.commissions as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.commissions ??= {};
            (errors.commissions as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: () => tainted.commissions?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.commissions ??= {};
            tainted.commissions[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: number) => {
          data.commissions.push(item);
        },
        remove: (index: number) => {
          data.commissions.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.commissions[a], data.commissions[b]] = [
            data.commissions[b],
            data.commissions[a],
          ];
        },
      },
    };
    function validate(): Result<
      Order,
      Array<{ field: string; message: string }>
    > {
      return Order.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Order>): void {
      data = { ...Order.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Order, Array<{ field: string; message: string }>> {
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
    return Order.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Commented, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Commented.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Commented.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("comment" in obj)) {
      errors.push({ field: "comment", message: "missing required field" });
    }
    if (!("replyTo" in obj)) {
      errors.push({ field: "replyTo", message: "missing required field" });
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
      const __raw_comment = obj["comment"];
      instance.comment = __raw_comment;
    }
    {
      const __raw_replyTo = obj["replyTo"];
      instance.replyTo = __raw_replyTo;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as Commented;
  }
}

export namespace Commented {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    comment?: Array<string>;
    replyTo?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { comment?: boolean; replyTo?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly comment: FieldController<string>;
    readonly replyTo: FieldController<string | null>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Commented;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Commented, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Commented>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Commented>,
  ): Gigaform {
    let data = $state({ ...Commented.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      comment: {
        path: ["comment"] as const,
        name: "comment",
        constraints: { required: true },

        get: () => data.comment,
        set: (value: string) => {
          data.comment = value;
        },
        getError: () => errors?.comment,
        setError: (value: Array<string> | undefined) => {
          errors.comment = value;
        },
        getTainted: () => tainted?.comment ?? false,
        setTainted: (value: boolean) => {
          tainted.comment = value;
        },
        validate: (): Array<string> => {
          const result = Commented.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "comment")
              .map((e) => e.message);
          }
          return [];
        },
      },
      replyTo: {
        path: ["replyTo"] as const,
        name: "replyTo",
        constraints: { required: true },

        get: () => data.replyTo,
        set: (value: string | null) => {
          data.replyTo = value;
        },
        getError: () => errors?.replyTo,
        setError: (value: Array<string> | undefined) => {
          errors.replyTo = value;
        },
        getTainted: () => tainted?.replyTo ?? false,
        setTainted: (value: boolean) => {
          tainted.replyTo = value;
        },
        validate: (): Array<string> => {
          const result = Commented.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "replyTo")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Commented,
      Array<{ field: string; message: string }>
    > {
      return Commented.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Commented>): void {
      data = { ...Commented.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Commented, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.comment = formData.get("comment") ?? "";
    obj.replyTo = formData.get("replyTo") ?? "";
    return Commented.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Custom, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Custom.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "Custom.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("mappings" in obj)) {
      errors.push({ field: "mappings", message: "missing required field" });
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
      const __raw_mappings = obj["mappings"];
      instance.mappings = __raw_mappings;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as Custom;
  }
}

export namespace Custom {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    mappings?: {
      _errors?: Array<string>;
      [index: number]: DirectionHue.Errors;
    };
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { mappings?: { [index: number]: DirectionHue.Tainted } };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly mappings: FieldController<DirectionHue[]>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Custom;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Custom, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Custom>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Custom>,
  ): Gigaform {
    let data = $state({ ...Custom.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      mappings: {
        path: ["mappings"] as const,
        name: "mappings",
        constraints: { required: true },

        get: () => data.mappings,
        set: (value: DirectionHue[]) => {
          data.mappings = value;
        },
        getError: () => errors?.mappings,
        setError: (value: Array<string> | undefined) => {
          errors.mappings = value;
        },
        getTainted: () => tainted?.mappings ?? false,
        setTainted: (value: boolean) => {
          tainted.mappings = value;
        },
        validate: (): Array<string> => {
          const result = Custom.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "mappings")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["mappings", index] as const,
          name: `mappings.${index}`,
          constraints: { required: true },
          get: () => data.mappings[index],
          set: (value: DirectionHue) => {
            data.mappings[index] = value;
          },
          getError: () =>
            (errors.mappings as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.mappings ??= {};
            (errors.mappings as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.mappings?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.mappings ??= {};
            tainted.mappings[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: DirectionHue) => {
          data.mappings.push(item);
        },
        remove: (index: number) => {
          data.mappings.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.mappings[a], data.mappings[b]] = [
            data.mappings[b],
            data.mappings[a],
          ];
        },
      },
    };
    function validate(): Result<
      Custom,
      Array<{ field: string; message: string }>
    > {
      return Custom.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Custom>): void {
      data = { ...Custom.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Custom, Array<{ field: string; message: string }>> {
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
    return Custom.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Colors, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Colors.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "Colors.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("main" in obj)) {
      errors.push({ field: "main", message: "missing required field" });
    }
    if (!("hover" in obj)) {
      errors.push({ field: "hover", message: "missing required field" });
    }
    if (!("active" in obj)) {
      errors.push({ field: "active", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Colors;
  }
}

export namespace Colors {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    main?: Array<string>;
    hover?: Array<string>;
    active?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { main?: boolean; hover?: boolean; active?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly main: FieldController<string>;
    readonly hover: FieldController<string>;
    readonly active: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Colors;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Colors, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Colors>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Colors>,
  ): Gigaform {
    let data = $state({ ...Colors.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      main: {
        path: ["main"] as const,
        name: "main",
        constraints: { required: true },

        get: () => data.main,
        set: (value: string) => {
          data.main = value;
        },
        getError: () => errors?.main,
        setError: (value: Array<string> | undefined) => {
          errors.main = value;
        },
        getTainted: () => tainted?.main ?? false,
        setTainted: (value: boolean) => {
          tainted.main = value;
        },
        validate: (): Array<string> => {
          const result = Colors.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "main")
              .map((e) => e.message);
          }
          return [];
        },
      },
      hover: {
        path: ["hover"] as const,
        name: "hover",
        constraints: { required: true },

        get: () => data.hover,
        set: (value: string) => {
          data.hover = value;
        },
        getError: () => errors?.hover,
        setError: (value: Array<string> | undefined) => {
          errors.hover = value;
        },
        getTainted: () => tainted?.hover ?? false,
        setTainted: (value: boolean) => {
          tainted.hover = value;
        },
        validate: (): Array<string> => {
          const result = Colors.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "hover")
              .map((e) => e.message);
          }
          return [];
        },
      },
      active: {
        path: ["active"] as const,
        name: "active",
        constraints: { required: true },

        get: () => data.active,
        set: (value: string) => {
          data.active = value;
        },
        getError: () => errors?.active,
        setError: (value: Array<string> | undefined) => {
          errors.active = value;
        },
        getTainted: () => tainted?.active ?? false,
        setTainted: (value: boolean) => {
          tainted.active = value;
        },
        validate: (): Array<string> => {
          const result = Colors.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "active")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Colors,
      Array<{ field: string; message: string }>
    > {
      return Colors.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Colors>): void {
      data = { ...Colors.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Colors, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.main = formData.get("main") ?? "";
    obj.hover = formData.get("hover") ?? "";
    obj.active = formData.get("active") ?? "";
    return Colors.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<ProductDefaults, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "ProductDefaults.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "ProductDefaults.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("price" in obj)) {
      errors.push({ field: "price", message: "missing required field" });
    }
    if (!("description" in obj)) {
      errors.push({ field: "description", message: "missing required field" });
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
      const __raw_price = obj["price"];
      instance.price = __raw_price;
    }
    {
      const __raw_description = obj["description"];
      instance.description = __raw_description;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as ProductDefaults;
  }
}

export namespace ProductDefaults {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    price?: Array<string>;
    description?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { price?: boolean; description?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly price: FieldController<number>;
    readonly description: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: ProductDefaults;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<
      ProductDefaults,
      Array<{ field: string; message: string }>
    >;
    reset(overrides?: Partial<ProductDefaults>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<ProductDefaults>,
  ): Gigaform {
    let data = $state({ ...ProductDefaults.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      price: {
        path: ["price"] as const,
        name: "price",
        constraints: { required: true },

        get: () => data.price,
        set: (value: number) => {
          data.price = value;
        },
        getError: () => errors?.price,
        setError: (value: Array<string> | undefined) => {
          errors.price = value;
        },
        getTainted: () => tainted?.price ?? false,
        setTainted: (value: boolean) => {
          tainted.price = value;
        },
        validate: (): Array<string> => {
          const result = ProductDefaults.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "price")
              .map((e) => e.message);
          }
          return [];
        },
      },
      description: {
        path: ["description"] as const,
        name: "description",
        constraints: { required: true },

        get: () => data.description,
        set: (value: string) => {
          data.description = value;
        },
        getError: () => errors?.description,
        setError: (value: Array<string> | undefined) => {
          errors.description = value;
        },
        getTainted: () => tainted?.description ?? false,
        setTainted: (value: boolean) => {
          tainted.description = value;
        },
        validate: (): Array<string> => {
          const result = ProductDefaults.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "description")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      ProductDefaults,
      Array<{ field: string; message: string }>
    > {
      return ProductDefaults.fromJSON(data);
    }
    function reset(newOverrides?: Partial<ProductDefaults>): void {
      data = { ...ProductDefaults.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<ProductDefaults, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
      const priceStr = formData.get("price");
      obj.price = priceStr ? parseFloat(priceStr as string) : 0;
      if (obj.price !== undefined && Number.isNaN(obj.price)) obj.price = 0;
    }
    obj.description = formData.get("description") ?? "";
    return ProductDefaults.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Viewed, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Viewed.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "Viewed.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("durationSeconds" in obj)) {
      errors.push({
        field: "durationSeconds",
        message: "missing required field",
      });
    }
    if (!("source" in obj)) {
      errors.push({ field: "source", message: "missing required field" });
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
      const __raw_durationSeconds = obj["durationSeconds"];
      instance.durationSeconds = __raw_durationSeconds;
    }
    {
      const __raw_source = obj["source"];
      instance.source = __raw_source;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as Viewed;
  }
}

export namespace Viewed {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    durationSeconds?: Array<string>;
    source?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { durationSeconds?: boolean; source?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly durationSeconds: FieldController<number | null>;
    readonly source: FieldController<string | null>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Viewed;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Viewed, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Viewed>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Viewed>,
  ): Gigaform {
    let data = $state({ ...Viewed.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      durationSeconds: {
        path: ["durationSeconds"] as const,
        name: "durationSeconds",
        constraints: { required: true },

        get: () => data.durationSeconds,
        set: (value: number | null) => {
          data.durationSeconds = value;
        },
        getError: () => errors?.durationSeconds,
        setError: (value: Array<string> | undefined) => {
          errors.durationSeconds = value;
        },
        getTainted: () => tainted?.durationSeconds ?? false,
        setTainted: (value: boolean) => {
          tainted.durationSeconds = value;
        },
        validate: (): Array<string> => {
          const result = Viewed.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "durationSeconds")
              .map((e) => e.message);
          }
          return [];
        },
      },
      source: {
        path: ["source"] as const,
        name: "source",
        constraints: { required: true },

        get: () => data.source,
        set: (value: string | null) => {
          data.source = value;
        },
        getError: () => errors?.source,
        setError: (value: Array<string> | undefined) => {
          errors.source = value;
        },
        getTainted: () => tainted?.source ?? false,
        setTainted: (value: boolean) => {
          tainted.source = value;
        },
        validate: (): Array<string> => {
          const result = Viewed.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "source")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Viewed,
      Array<{ field: string; message: string }>
    > {
      return Viewed.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Viewed>): void {
      data = { ...Viewed.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Viewed, Array<{ field: string; message: string }>> {
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
    return Viewed.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<WeeklyRecurrenceRule, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "WeeklyRecurrenceRule.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "WeeklyRecurrenceRule.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("quantityOfWeeks" in obj)) {
      errors.push({
        field: "quantityOfWeeks",
        message: "missing required field",
      });
    }
    if (!("weekdays" in obj)) {
      errors.push({ field: "weekdays", message: "missing required field" });
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
      const __raw_quantityOfWeeks = obj["quantityOfWeeks"];
      instance.quantityOfWeeks = __raw_quantityOfWeeks;
    }
    {
      const __raw_weekdays = obj["weekdays"];
      instance.weekdays = __raw_weekdays;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as WeeklyRecurrenceRule;
  }
}

export namespace WeeklyRecurrenceRule {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    quantityOfWeeks?: Array<string>;
    weekdays?: { _errors?: Array<string>; [index: number]: Weekday.Errors };
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
      quantityOfWeeks?: boolean;
      weekdays?: { [index: number]: Weekday.Tainted };
    };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly quantityOfWeeks: FieldController<number>;
    readonly weekdays: FieldController<Weekday[]>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: WeeklyRecurrenceRule;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<
      WeeklyRecurrenceRule,
      Array<{ field: string; message: string }>
    >;
    reset(overrides?: Partial<WeeklyRecurrenceRule>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<WeeklyRecurrenceRule>,
  ): Gigaform {
    let data = $state({ ...WeeklyRecurrenceRule.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      quantityOfWeeks: {
        path: ["quantityOfWeeks"] as const,
        name: "quantityOfWeeks",
        constraints: { required: true },

        get: () => data.quantityOfWeeks,
        set: (value: number) => {
          data.quantityOfWeeks = value;
        },
        getError: () => errors?.quantityOfWeeks,
        setError: (value: Array<string> | undefined) => {
          errors.quantityOfWeeks = value;
        },
        getTainted: () => tainted?.quantityOfWeeks ?? false,
        setTainted: (value: boolean) => {
          tainted.quantityOfWeeks = value;
        },
        validate: (): Array<string> => {
          const result = WeeklyRecurrenceRule.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "quantityOfWeeks")
              .map((e) => e.message);
          }
          return [];
        },
      },
      weekdays: {
        path: ["weekdays"] as const,
        name: "weekdays",
        constraints: { required: true },

        get: () => data.weekdays,
        set: (value: Weekday[]) => {
          data.weekdays = value;
        },
        getError: () => errors?.weekdays,
        setError: (value: Array<string> | undefined) => {
          errors.weekdays = value;
        },
        getTainted: () => tainted?.weekdays ?? false,
        setTainted: (value: boolean) => {
          tainted.weekdays = value;
        },
        validate: (): Array<string> => {
          const result = WeeklyRecurrenceRule.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "weekdays")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["weekdays", index] as const,
          name: `weekdays.${index}`,
          constraints: { required: true },
          get: () => data.weekdays[index],
          set: (value: Weekday) => {
            data.weekdays[index] = value;
          },
          getError: () =>
            (errors.weekdays as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.weekdays ??= {};
            (errors.weekdays as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.weekdays?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.weekdays ??= {};
            tainted.weekdays[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: Weekday) => {
          data.weekdays.push(item);
        },
        remove: (index: number) => {
          data.weekdays.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.weekdays[a], data.weekdays[b]] = [
            data.weekdays[b],
            data.weekdays[a],
          ];
        },
      },
    };
    function validate(): Result<
      WeeklyRecurrenceRule,
      Array<{ field: string; message: string }>
    > {
      return WeeklyRecurrenceRule.fromJSON(data);
    }
    function reset(newOverrides?: Partial<WeeklyRecurrenceRule>): void {
      data = { ...WeeklyRecurrenceRule.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<WeeklyRecurrenceRule, Array<{ field: string; message: string }>> {
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
    return WeeklyRecurrenceRule.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Paid, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Paid.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "Paid.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("amount" in obj)) {
      errors.push({ field: "amount", message: "missing required field" });
    }
    if (!("currency" in obj)) {
      errors.push({ field: "currency", message: "missing required field" });
    }
    if (!("paymentMethod" in obj)) {
      errors.push({
        field: "paymentMethod",
        message: "missing required field",
      });
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
      throw new DeserializeError(errors);
    }
    return instance as Paid;
  }
}

export namespace Paid {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    amount?: Array<string>;
    currency?: Array<string>;
    paymentMethod?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { amount?: boolean; currency?: boolean; paymentMethod?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly amount: FieldController<number | null>;
    readonly currency: FieldController<string | null>;
    readonly paymentMethod: FieldController<string | null>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Paid;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Paid, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Paid>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Paid>,
  ): Gigaform {
    let data = $state({ ...Paid.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      amount: {
        path: ["amount"] as const,
        name: "amount",
        constraints: { required: true },

        get: () => data.amount,
        set: (value: number | null) => {
          data.amount = value;
        },
        getError: () => errors?.amount,
        setError: (value: Array<string> | undefined) => {
          errors.amount = value;
        },
        getTainted: () => tainted?.amount ?? false,
        setTainted: (value: boolean) => {
          tainted.amount = value;
        },
        validate: (): Array<string> => {
          const result = Paid.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "amount")
              .map((e) => e.message);
          }
          return [];
        },
      },
      currency: {
        path: ["currency"] as const,
        name: "currency",
        constraints: { required: true },

        get: () => data.currency,
        set: (value: string | null) => {
          data.currency = value;
        },
        getError: () => errors?.currency,
        setError: (value: Array<string> | undefined) => {
          errors.currency = value;
        },
        getTainted: () => tainted?.currency ?? false,
        setTainted: (value: boolean) => {
          tainted.currency = value;
        },
        validate: (): Array<string> => {
          const result = Paid.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "currency")
              .map((e) => e.message);
          }
          return [];
        },
      },
      paymentMethod: {
        path: ["paymentMethod"] as const,
        name: "paymentMethod",
        constraints: { required: true },

        get: () => data.paymentMethod,
        set: (value: string | null) => {
          data.paymentMethod = value;
        },
        getError: () => errors?.paymentMethod,
        setError: (value: Array<string> | undefined) => {
          errors.paymentMethod = value;
        },
        getTainted: () => tainted?.paymentMethod ?? false,
        setTainted: (value: boolean) => {
          tainted.paymentMethod = value;
        },
        validate: (): Array<string> => {
          const result = Paid.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "paymentMethod")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Paid,
      Array<{ field: string; message: string }>
    > {
      return Paid.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Paid>): void {
      data = { ...Paid.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Paid, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
      const amountStr = formData.get("amount");
      obj.amount = amountStr ? parseFloat(amountStr as string) : 0;
      if (obj.amount !== undefined && Number.isNaN(obj.amount)) obj.amount = 0;
    }
    obj.currency = formData.get("currency") ?? "";
    obj.paymentMethod = formData.get("paymentMethod") ?? "";
    return Paid.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): TaxRate {
    return {
      id: "",
      name: "",
      taxAgency: "",
      zip: 0,
      city: "",
      county: "",
      state: "",
      isActive: false,
      description: "",
      taxComponents: {},
    } as TaxRate;
  }
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
  ): Result<TaxRate, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "TaxRate.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "TaxRate.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("name" in obj)) {
      errors.push({ field: "name", message: "missing required field" });
    }
    if (!("taxAgency" in obj)) {
      errors.push({ field: "taxAgency", message: "missing required field" });
    }
    if (!("zip" in obj)) {
      errors.push({ field: "zip", message: "missing required field" });
    }
    if (!("city" in obj)) {
      errors.push({ field: "city", message: "missing required field" });
    }
    if (!("county" in obj)) {
      errors.push({ field: "county", message: "missing required field" });
    }
    if (!("state" in obj)) {
      errors.push({ field: "state", message: "missing required field" });
    }
    if (!("isActive" in obj)) {
      errors.push({ field: "isActive", message: "missing required field" });
    }
    if (!("description" in obj)) {
      errors.push({ field: "description", message: "missing required field" });
    }
    if (!("taxComponents" in obj)) {
      errors.push({
        field: "taxComponents",
        message: "missing required field",
      });
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
      throw new DeserializeError(errors);
    }
    return instance as TaxRate;
  }
}

export namespace TaxRate {
  /** Nested error structure matching the data shape */ export type Errors = {
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
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
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
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly name: FieldController<string>;
    readonly taxAgency: FieldController<string>;
    readonly zip: FieldController<number>;
    readonly city: FieldController<string>;
    readonly county: FieldController<string>;
    readonly state: FieldController<string>;
    readonly isActive: FieldController<boolean>;
    readonly description: FieldController<string>;
    readonly taxComponents: FieldController<{ [key: string]: number }>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: TaxRate;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<TaxRate, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<TaxRate>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<TaxRate>,
  ): Gigaform {
    let data = $state({ ...TaxRate.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = TaxRate.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: () => data.name,
        set: (value: string) => {
          data.name = value;
        },
        getError: () => errors?.name,
        setError: (value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: () => tainted?.name ?? false,
        setTainted: (value: boolean) => {
          tainted.name = value;
        },
        validate: (): Array<string> => {
          const result = TaxRate.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "name")
              .map((e) => e.message);
          }
          return [];
        },
      },
      taxAgency: {
        path: ["taxAgency"] as const,
        name: "taxAgency",
        constraints: { required: true },

        get: () => data.taxAgency,
        set: (value: string) => {
          data.taxAgency = value;
        },
        getError: () => errors?.taxAgency,
        setError: (value: Array<string> | undefined) => {
          errors.taxAgency = value;
        },
        getTainted: () => tainted?.taxAgency ?? false,
        setTainted: (value: boolean) => {
          tainted.taxAgency = value;
        },
        validate: (): Array<string> => {
          const result = TaxRate.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "taxAgency")
              .map((e) => e.message);
          }
          return [];
        },
      },
      zip: {
        path: ["zip"] as const,
        name: "zip",
        constraints: { required: true },

        get: () => data.zip,
        set: (value: number) => {
          data.zip = value;
        },
        getError: () => errors?.zip,
        setError: (value: Array<string> | undefined) => {
          errors.zip = value;
        },
        getTainted: () => tainted?.zip ?? false,
        setTainted: (value: boolean) => {
          tainted.zip = value;
        },
        validate: (): Array<string> => {
          const result = TaxRate.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "zip")
              .map((e) => e.message);
          }
          return [];
        },
      },
      city: {
        path: ["city"] as const,
        name: "city",
        constraints: { required: true },

        get: () => data.city,
        set: (value: string) => {
          data.city = value;
        },
        getError: () => errors?.city,
        setError: (value: Array<string> | undefined) => {
          errors.city = value;
        },
        getTainted: () => tainted?.city ?? false,
        setTainted: (value: boolean) => {
          tainted.city = value;
        },
        validate: (): Array<string> => {
          const result = TaxRate.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "city")
              .map((e) => e.message);
          }
          return [];
        },
      },
      county: {
        path: ["county"] as const,
        name: "county",
        constraints: { required: true },

        get: () => data.county,
        set: (value: string) => {
          data.county = value;
        },
        getError: () => errors?.county,
        setError: (value: Array<string> | undefined) => {
          errors.county = value;
        },
        getTainted: () => tainted?.county ?? false,
        setTainted: (value: boolean) => {
          tainted.county = value;
        },
        validate: (): Array<string> => {
          const result = TaxRate.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "county")
              .map((e) => e.message);
          }
          return [];
        },
      },
      state: {
        path: ["state"] as const,
        name: "state",
        constraints: { required: true },

        get: () => data.state,
        set: (value: string) => {
          data.state = value;
        },
        getError: () => errors?.state,
        setError: (value: Array<string> | undefined) => {
          errors.state = value;
        },
        getTainted: () => tainted?.state ?? false,
        setTainted: (value: boolean) => {
          tainted.state = value;
        },
        validate: (): Array<string> => {
          const result = TaxRate.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "state")
              .map((e) => e.message);
          }
          return [];
        },
      },
      isActive: {
        path: ["isActive"] as const,
        name: "isActive",
        constraints: { required: true },

        get: () => data.isActive,
        set: (value: boolean) => {
          data.isActive = value;
        },
        getError: () => errors?.isActive,
        setError: (value: Array<string> | undefined) => {
          errors.isActive = value;
        },
        getTainted: () => tainted?.isActive ?? false,
        setTainted: (value: boolean) => {
          tainted.isActive = value;
        },
        validate: (): Array<string> => {
          const result = TaxRate.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "isActive")
              .map((e) => e.message);
          }
          return [];
        },
      },
      description: {
        path: ["description"] as const,
        name: "description",
        constraints: { required: true },

        get: () => data.description,
        set: (value: string) => {
          data.description = value;
        },
        getError: () => errors?.description,
        setError: (value: Array<string> | undefined) => {
          errors.description = value;
        },
        getTainted: () => tainted?.description ?? false,
        setTainted: (value: boolean) => {
          tainted.description = value;
        },
        validate: (): Array<string> => {
          const result = TaxRate.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "description")
              .map((e) => e.message);
          }
          return [];
        },
      },
      taxComponents: {
        path: ["taxComponents"] as const,
        name: "taxComponents",
        constraints: { required: true },

        get: () => data.taxComponents,
        set: (value: { [key: string]: number }) => {
          data.taxComponents = value;
        },
        getError: () => errors?.taxComponents,
        setError: (value: Array<string> | undefined) => {
          errors.taxComponents = value;
        },
        getTainted: () => tainted?.taxComponents ?? false,
        setTainted: (value: boolean) => {
          tainted.taxComponents = value;
        },
        validate: (): Array<string> => {
          const result = TaxRate.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "taxComponents")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      TaxRate,
      Array<{ field: string; message: string }>
    > {
      return TaxRate.fromJSON(data);
    }
    function reset(newOverrides?: Partial<TaxRate>): void {
      data = { ...TaxRate.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<TaxRate, Array<{ field: string; message: string }>> {
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
    return TaxRate.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Address, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Address.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Address.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("street" in obj)) {
      errors.push({ field: "street", message: "missing required field" });
    }
    if (!("city" in obj)) {
      errors.push({ field: "city", message: "missing required field" });
    }
    if (!("state" in obj)) {
      errors.push({ field: "state", message: "missing required field" });
    }
    if (!("zipcode" in obj)) {
      errors.push({ field: "zipcode", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Address;
  }
}

export namespace Address {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    street?: Array<string>;
    city?: Array<string>;
    state?: Array<string>;
    zipcode?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { street?: boolean; city?: boolean; state?: boolean; zipcode?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly street: FieldController<string>;
    readonly city: FieldController<string>;
    readonly state: FieldController<string>;
    readonly zipcode: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Address;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Address, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Address>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Address>,
  ): Gigaform {
    let data = $state({ ...Address.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      street: {
        path: ["street"] as const,
        name: "street",
        constraints: { required: true },

        get: () => data.street,
        set: (value: string) => {
          data.street = value;
        },
        getError: () => errors?.street,
        setError: (value: Array<string> | undefined) => {
          errors.street = value;
        },
        getTainted: () => tainted?.street ?? false,
        setTainted: (value: boolean) => {
          tainted.street = value;
        },
        validate: (): Array<string> => {
          const result = Address.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "street")
              .map((e) => e.message);
          }
          return [];
        },
      },
      city: {
        path: ["city"] as const,
        name: "city",
        constraints: { required: true },

        get: () => data.city,
        set: (value: string) => {
          data.city = value;
        },
        getError: () => errors?.city,
        setError: (value: Array<string> | undefined) => {
          errors.city = value;
        },
        getTainted: () => tainted?.city ?? false,
        setTainted: (value: boolean) => {
          tainted.city = value;
        },
        validate: (): Array<string> => {
          const result = Address.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "city")
              .map((e) => e.message);
          }
          return [];
        },
      },
      state: {
        path: ["state"] as const,
        name: "state",
        constraints: { required: true },

        get: () => data.state,
        set: (value: string) => {
          data.state = value;
        },
        getError: () => errors?.state,
        setError: (value: Array<string> | undefined) => {
          errors.state = value;
        },
        getTainted: () => tainted?.state ?? false,
        setTainted: (value: boolean) => {
          tainted.state = value;
        },
        validate: (): Array<string> => {
          const result = Address.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "state")
              .map((e) => e.message);
          }
          return [];
        },
      },
      zipcode: {
        path: ["zipcode"] as const,
        name: "zipcode",
        constraints: { required: true },

        get: () => data.zipcode,
        set: (value: string) => {
          data.zipcode = value;
        },
        getError: () => errors?.zipcode,
        setError: (value: Array<string> | undefined) => {
          errors.zipcode = value;
        },
        getTainted: () => tainted?.zipcode ?? false,
        setTainted: (value: boolean) => {
          tainted.zipcode = value;
        },
        validate: (): Array<string> => {
          const result = Address.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "zipcode")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Address,
      Array<{ field: string; message: string }>
    > {
      return Address.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Address>): void {
      data = { ...Address.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Address, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.street = formData.get("street") ?? "";
    obj.city = formData.get("city") ?? "";
    obj.state = formData.get("state") ?? "";
    obj.zipcode = formData.get("zipcode") ?? "";
    return Address.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): Lead {
    return {
      id: "",
      number: null,
      accepted: false,
      probability: 0,
      priority: "Medium",
      dueDate: null,
      closeDate: null,
      value: 0,
      stage: "Open",
      status: "",
      description: null,
      nextStep: "InitialContact",
      favorite: false,
      dateAdded: null,
      taxRate: null,
      sector: "Residential",
      leadName: CompanyName.defaultValue(),
      phones: [],
      email: Email.defaultValue(),
      leadSource: null,
      site: "",
      memo: "",
      needsReview: false,
      hasAlert: false,
      salesRep: null,
      color: null,
      accountType: "",
      subtype: "",
      isTaxExempt: false,
      paymentTerms: "",
      tags: [],
      customFields: [],
    } as Lead;
  }
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
  ): Result<Lead, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Lead.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "Lead.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("number" in obj)) {
      errors.push({ field: "number", message: "missing required field" });
    }
    if (!("accepted" in obj)) {
      errors.push({ field: "accepted", message: "missing required field" });
    }
    if (!("probability" in obj)) {
      errors.push({ field: "probability", message: "missing required field" });
    }
    if (!("priority" in obj)) {
      errors.push({ field: "priority", message: "missing required field" });
    }
    if (!("dueDate" in obj)) {
      errors.push({ field: "dueDate", message: "missing required field" });
    }
    if (!("closeDate" in obj)) {
      errors.push({ field: "closeDate", message: "missing required field" });
    }
    if (!("value" in obj)) {
      errors.push({ field: "value", message: "missing required field" });
    }
    if (!("stage" in obj)) {
      errors.push({ field: "stage", message: "missing required field" });
    }
    if (!("status" in obj)) {
      errors.push({ field: "status", message: "missing required field" });
    }
    if (!("description" in obj)) {
      errors.push({ field: "description", message: "missing required field" });
    }
    if (!("nextStep" in obj)) {
      errors.push({ field: "nextStep", message: "missing required field" });
    }
    if (!("favorite" in obj)) {
      errors.push({ field: "favorite", message: "missing required field" });
    }
    if (!("dateAdded" in obj)) {
      errors.push({ field: "dateAdded", message: "missing required field" });
    }
    if (!("taxRate" in obj)) {
      errors.push({ field: "taxRate", message: "missing required field" });
    }
    if (!("sector" in obj)) {
      errors.push({ field: "sector", message: "missing required field" });
    }
    if (!("leadName" in obj)) {
      errors.push({ field: "leadName", message: "missing required field" });
    }
    if (!("phones" in obj)) {
      errors.push({ field: "phones", message: "missing required field" });
    }
    if (!("email" in obj)) {
      errors.push({ field: "email", message: "missing required field" });
    }
    if (!("leadSource" in obj)) {
      errors.push({ field: "leadSource", message: "missing required field" });
    }
    if (!("site" in obj)) {
      errors.push({ field: "site", message: "missing required field" });
    }
    if (!("memo" in obj)) {
      errors.push({ field: "memo", message: "missing required field" });
    }
    if (!("needsReview" in obj)) {
      errors.push({ field: "needsReview", message: "missing required field" });
    }
    if (!("hasAlert" in obj)) {
      errors.push({ field: "hasAlert", message: "missing required field" });
    }
    if (!("salesRep" in obj)) {
      errors.push({ field: "salesRep", message: "missing required field" });
    }
    if (!("color" in obj)) {
      errors.push({ field: "color", message: "missing required field" });
    }
    if (!("accountType" in obj)) {
      errors.push({ field: "accountType", message: "missing required field" });
    }
    if (!("subtype" in obj)) {
      errors.push({ field: "subtype", message: "missing required field" });
    }
    if (!("isTaxExempt" in obj)) {
      errors.push({ field: "isTaxExempt", message: "missing required field" });
    }
    if (!("paymentTerms" in obj)) {
      errors.push({ field: "paymentTerms", message: "missing required field" });
    }
    if (!("tags" in obj)) {
      errors.push({ field: "tags", message: "missing required field" });
    }
    if (!("customFields" in obj)) {
      errors.push({ field: "customFields", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Lead;
  }
}

export namespace Lead {
  /** Nested error structure matching the data shape */ export type Errors = {
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
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
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
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly number: FieldController<number | null>;
    readonly accepted: FieldController<boolean>;
    readonly probability: FieldController<number>;
    readonly priority: FieldController<Priority>;
    readonly dueDate: FieldController<string | null>;
    readonly closeDate: FieldController<string | null>;
    readonly value: FieldController<number>;
    readonly stage: FieldController<LeadStage>;
    readonly status: FieldController<string>;
    readonly description: FieldController<string | null>;
    readonly nextStep: FieldController<NextStep>;
    readonly favorite: FieldController<boolean>;
    readonly dateAdded: FieldController<string | null>;
    readonly taxRate: FieldController<(string | TaxRate) | null>;
    readonly sector: FieldController<Sector>;
    readonly leadName: FieldController<AccountName>;
    readonly phones: FieldController<PhoneNumber[]>;
    readonly email: FieldController<Email>;
    readonly leadSource: FieldController<string | null>;
    readonly site: FieldController<string | Site>;
    readonly memo: FieldController<string>;
    readonly needsReview: FieldController<boolean>;
    readonly hasAlert: FieldController<boolean>;
    readonly salesRep: FieldController<Represents[] | null>;
    readonly color: FieldController<string | null>;
    readonly accountType: FieldController<string>;
    readonly subtype: FieldController<string>;
    readonly isTaxExempt: FieldController<boolean>;
    readonly paymentTerms: FieldController<string>;
    readonly tags: FieldController<string[]>;
    readonly customFields: FieldController<[string, string][]>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Lead;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Lead, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Lead>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Lead>,
  ): Gigaform {
    let data = $state({ ...Lead.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      number: {
        path: ["number"] as const,
        name: "number",
        constraints: { required: true },

        get: () => data.number,
        set: (value: number | null) => {
          data.number = value;
        },
        getError: () => errors?.number,
        setError: (value: Array<string> | undefined) => {
          errors.number = value;
        },
        getTainted: () => tainted?.number ?? false,
        setTainted: (value: boolean) => {
          tainted.number = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "number")
              .map((e) => e.message);
          }
          return [];
        },
      },
      accepted: {
        path: ["accepted"] as const,
        name: "accepted",
        constraints: { required: true },

        get: () => data.accepted,
        set: (value: boolean) => {
          data.accepted = value;
        },
        getError: () => errors?.accepted,
        setError: (value: Array<string> | undefined) => {
          errors.accepted = value;
        },
        getTainted: () => tainted?.accepted ?? false,
        setTainted: (value: boolean) => {
          tainted.accepted = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "accepted")
              .map((e) => e.message);
          }
          return [];
        },
      },
      probability: {
        path: ["probability"] as const,
        name: "probability",
        constraints: { required: true },

        get: () => data.probability,
        set: (value: number) => {
          data.probability = value;
        },
        getError: () => errors?.probability,
        setError: (value: Array<string> | undefined) => {
          errors.probability = value;
        },
        getTainted: () => tainted?.probability ?? false,
        setTainted: (value: boolean) => {
          tainted.probability = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "probability")
              .map((e) => e.message);
          }
          return [];
        },
      },
      priority: {
        path: ["priority"] as const,
        name: "priority",
        constraints: { required: true },

        get: () => data.priority,
        set: (value: Priority) => {
          data.priority = value;
        },
        getError: () => errors?.priority,
        setError: (value: Array<string> | undefined) => {
          errors.priority = value;
        },
        getTainted: () => tainted?.priority ?? false,
        setTainted: (value: boolean) => {
          tainted.priority = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "priority")
              .map((e) => e.message);
          }
          return [];
        },
      },
      dueDate: {
        path: ["dueDate"] as const,
        name: "dueDate",
        constraints: { required: true },

        get: () => data.dueDate,
        set: (value: string | null) => {
          data.dueDate = value;
        },
        getError: () => errors?.dueDate,
        setError: (value: Array<string> | undefined) => {
          errors.dueDate = value;
        },
        getTainted: () => tainted?.dueDate ?? false,
        setTainted: (value: boolean) => {
          tainted.dueDate = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "dueDate")
              .map((e) => e.message);
          }
          return [];
        },
      },
      closeDate: {
        path: ["closeDate"] as const,
        name: "closeDate",
        constraints: { required: true },

        get: () => data.closeDate,
        set: (value: string | null) => {
          data.closeDate = value;
        },
        getError: () => errors?.closeDate,
        setError: (value: Array<string> | undefined) => {
          errors.closeDate = value;
        },
        getTainted: () => tainted?.closeDate ?? false,
        setTainted: (value: boolean) => {
          tainted.closeDate = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "closeDate")
              .map((e) => e.message);
          }
          return [];
        },
      },
      value: {
        path: ["value"] as const,
        name: "value",
        constraints: { required: true },

        get: () => data.value,
        set: (value: number) => {
          data.value = value;
        },
        getError: () => errors?.value,
        setError: (value: Array<string> | undefined) => {
          errors.value = value;
        },
        getTainted: () => tainted?.value ?? false,
        setTainted: (value: boolean) => {
          tainted.value = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "value")
              .map((e) => e.message);
          }
          return [];
        },
      },
      stage: {
        path: ["stage"] as const,
        name: "stage",
        constraints: { required: true },

        get: () => data.stage,
        set: (value: LeadStage) => {
          data.stage = value;
        },
        getError: () => errors?.stage,
        setError: (value: Array<string> | undefined) => {
          errors.stage = value;
        },
        getTainted: () => tainted?.stage ?? false,
        setTainted: (value: boolean) => {
          tainted.stage = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "stage")
              .map((e) => e.message);
          }
          return [];
        },
      },
      status: {
        path: ["status"] as const,
        name: "status",
        constraints: { required: true },

        get: () => data.status,
        set: (value: string) => {
          data.status = value;
        },
        getError: () => errors?.status,
        setError: (value: Array<string> | undefined) => {
          errors.status = value;
        },
        getTainted: () => tainted?.status ?? false,
        setTainted: (value: boolean) => {
          tainted.status = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "status")
              .map((e) => e.message);
          }
          return [];
        },
      },
      description: {
        path: ["description"] as const,
        name: "description",
        constraints: { required: true },

        get: () => data.description,
        set: (value: string | null) => {
          data.description = value;
        },
        getError: () => errors?.description,
        setError: (value: Array<string> | undefined) => {
          errors.description = value;
        },
        getTainted: () => tainted?.description ?? false,
        setTainted: (value: boolean) => {
          tainted.description = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "description")
              .map((e) => e.message);
          }
          return [];
        },
      },
      nextStep: {
        path: ["nextStep"] as const,
        name: "nextStep",
        constraints: { required: true },

        get: () => data.nextStep,
        set: (value: NextStep) => {
          data.nextStep = value;
        },
        getError: () => errors?.nextStep,
        setError: (value: Array<string> | undefined) => {
          errors.nextStep = value;
        },
        getTainted: () => tainted?.nextStep ?? false,
        setTainted: (value: boolean) => {
          tainted.nextStep = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "nextStep")
              .map((e) => e.message);
          }
          return [];
        },
      },
      favorite: {
        path: ["favorite"] as const,
        name: "favorite",
        constraints: { required: true },

        get: () => data.favorite,
        set: (value: boolean) => {
          data.favorite = value;
        },
        getError: () => errors?.favorite,
        setError: (value: Array<string> | undefined) => {
          errors.favorite = value;
        },
        getTainted: () => tainted?.favorite ?? false,
        setTainted: (value: boolean) => {
          tainted.favorite = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "favorite")
              .map((e) => e.message);
          }
          return [];
        },
      },
      dateAdded: {
        path: ["dateAdded"] as const,
        name: "dateAdded",
        constraints: { required: true },

        get: () => data.dateAdded,
        set: (value: string | null) => {
          data.dateAdded = value;
        },
        getError: () => errors?.dateAdded,
        setError: (value: Array<string> | undefined) => {
          errors.dateAdded = value;
        },
        getTainted: () => tainted?.dateAdded ?? false,
        setTainted: (value: boolean) => {
          tainted.dateAdded = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "dateAdded")
              .map((e) => e.message);
          }
          return [];
        },
      },
      taxRate: {
        path: ["taxRate"] as const,
        name: "taxRate",
        constraints: { required: true },

        get: () => data.taxRate,
        set: (value: (string | TaxRate) | null) => {
          data.taxRate = value;
        },
        getError: () => errors?.taxRate,
        setError: (value: Array<string> | undefined) => {
          errors.taxRate = value;
        },
        getTainted: () => tainted?.taxRate ?? false,
        setTainted: (value: boolean) => {
          tainted.taxRate = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "taxRate")
              .map((e) => e.message);
          }
          return [];
        },
      },
      sector: {
        path: ["sector"] as const,
        name: "sector",
        constraints: { required: true },

        get: () => data.sector,
        set: (value: Sector) => {
          data.sector = value;
        },
        getError: () => errors?.sector,
        setError: (value: Array<string> | undefined) => {
          errors.sector = value;
        },
        getTainted: () => tainted?.sector ?? false,
        setTainted: (value: boolean) => {
          tainted.sector = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "sector")
              .map((e) => e.message);
          }
          return [];
        },
      },
      leadName: {
        path: ["leadName"] as const,
        name: "leadName",
        constraints: { required: true },

        get: () => data.leadName,
        set: (value: AccountName) => {
          data.leadName = value;
        },
        getError: () => errors?.leadName,
        setError: (value: Array<string> | undefined) => {
          errors.leadName = value;
        },
        getTainted: () => tainted?.leadName ?? false,
        setTainted: (value: boolean) => {
          tainted.leadName = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "leadName")
              .map((e) => e.message);
          }
          return [];
        },
      },
      phones: {
        path: ["phones"] as const,
        name: "phones",
        constraints: { required: true },

        get: () => data.phones,
        set: (value: PhoneNumber[]) => {
          data.phones = value;
        },
        getError: () => errors?.phones,
        setError: (value: Array<string> | undefined) => {
          errors.phones = value;
        },
        getTainted: () => tainted?.phones ?? false,
        setTainted: (value: boolean) => {
          tainted.phones = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "phones")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["phones", index] as const,
          name: `phones.${index}`,
          constraints: { required: true },
          get: () => data.phones[index],
          set: (value: PhoneNumber) => {
            data.phones[index] = value;
          },
          getError: () =>
            (errors.phones as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.phones ??= {};
            (errors.phones as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.phones?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.phones ??= {};
            tainted.phones[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: PhoneNumber) => {
          data.phones.push(item);
        },
        remove: (index: number) => {
          data.phones.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.phones[a], data.phones[b]] = [data.phones[b], data.phones[a]];
        },
      },
      email: {
        path: ["email"] as const,
        name: "email",
        constraints: { required: true },

        get: () => data.email,
        set: (value: Email) => {
          data.email = value;
        },
        getError: () => errors?.email,
        setError: (value: Array<string> | undefined) => {
          errors.email = value;
        },
        getTainted: () => tainted?.email ?? false,
        setTainted: (value: boolean) => {
          tainted.email = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "email")
              .map((e) => e.message);
          }
          return [];
        },
      },
      leadSource: {
        path: ["leadSource"] as const,
        name: "leadSource",
        constraints: { required: true },

        get: () => data.leadSource,
        set: (value: string | null) => {
          data.leadSource = value;
        },
        getError: () => errors?.leadSource,
        setError: (value: Array<string> | undefined) => {
          errors.leadSource = value;
        },
        getTainted: () => tainted?.leadSource ?? false,
        setTainted: (value: boolean) => {
          tainted.leadSource = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "leadSource")
              .map((e) => e.message);
          }
          return [];
        },
      },
      site: {
        path: ["site"] as const,
        name: "site",
        constraints: { required: true },

        get: () => data.site,
        set: (value: string | Site) => {
          data.site = value;
        },
        getError: () => errors?.site,
        setError: (value: Array<string> | undefined) => {
          errors.site = value;
        },
        getTainted: () => tainted?.site ?? false,
        setTainted: (value: boolean) => {
          tainted.site = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "site")
              .map((e) => e.message);
          }
          return [];
        },
      },
      memo: {
        path: ["memo"] as const,
        name: "memo",
        constraints: { required: true },

        get: () => data.memo,
        set: (value: string) => {
          data.memo = value;
        },
        getError: () => errors?.memo,
        setError: (value: Array<string> | undefined) => {
          errors.memo = value;
        },
        getTainted: () => tainted?.memo ?? false,
        setTainted: (value: boolean) => {
          tainted.memo = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "memo")
              .map((e) => e.message);
          }
          return [];
        },
      },
      needsReview: {
        path: ["needsReview"] as const,
        name: "needsReview",
        constraints: { required: true },

        get: () => data.needsReview,
        set: (value: boolean) => {
          data.needsReview = value;
        },
        getError: () => errors?.needsReview,
        setError: (value: Array<string> | undefined) => {
          errors.needsReview = value;
        },
        getTainted: () => tainted?.needsReview ?? false,
        setTainted: (value: boolean) => {
          tainted.needsReview = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "needsReview")
              .map((e) => e.message);
          }
          return [];
        },
      },
      hasAlert: {
        path: ["hasAlert"] as const,
        name: "hasAlert",
        constraints: { required: true },

        get: () => data.hasAlert,
        set: (value: boolean) => {
          data.hasAlert = value;
        },
        getError: () => errors?.hasAlert,
        setError: (value: Array<string> | undefined) => {
          errors.hasAlert = value;
        },
        getTainted: () => tainted?.hasAlert ?? false,
        setTainted: (value: boolean) => {
          tainted.hasAlert = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "hasAlert")
              .map((e) => e.message);
          }
          return [];
        },
      },
      salesRep: {
        path: ["salesRep"] as const,
        name: "salesRep",
        constraints: { required: true },

        get: () => data.salesRep,
        set: (value: Represents[] | null) => {
          data.salesRep = value;
        },
        getError: () => errors?.salesRep,
        setError: (value: Array<string> | undefined) => {
          errors.salesRep = value;
        },
        getTainted: () => tainted?.salesRep ?? false,
        setTainted: (value: boolean) => {
          tainted.salesRep = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "salesRep")
              .map((e) => e.message);
          }
          return [];
        },
      },
      color: {
        path: ["color"] as const,
        name: "color",
        constraints: { required: true },

        get: () => data.color,
        set: (value: string | null) => {
          data.color = value;
        },
        getError: () => errors?.color,
        setError: (value: Array<string> | undefined) => {
          errors.color = value;
        },
        getTainted: () => tainted?.color ?? false,
        setTainted: (value: boolean) => {
          tainted.color = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "color")
              .map((e) => e.message);
          }
          return [];
        },
      },
      accountType: {
        path: ["accountType"] as const,
        name: "accountType",
        constraints: { required: true },

        get: () => data.accountType,
        set: (value: string) => {
          data.accountType = value;
        },
        getError: () => errors?.accountType,
        setError: (value: Array<string> | undefined) => {
          errors.accountType = value;
        },
        getTainted: () => tainted?.accountType ?? false,
        setTainted: (value: boolean) => {
          tainted.accountType = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "accountType")
              .map((e) => e.message);
          }
          return [];
        },
      },
      subtype: {
        path: ["subtype"] as const,
        name: "subtype",
        constraints: { required: true },

        get: () => data.subtype,
        set: (value: string) => {
          data.subtype = value;
        },
        getError: () => errors?.subtype,
        setError: (value: Array<string> | undefined) => {
          errors.subtype = value;
        },
        getTainted: () => tainted?.subtype ?? false,
        setTainted: (value: boolean) => {
          tainted.subtype = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "subtype")
              .map((e) => e.message);
          }
          return [];
        },
      },
      isTaxExempt: {
        path: ["isTaxExempt"] as const,
        name: "isTaxExempt",
        constraints: { required: true },

        get: () => data.isTaxExempt,
        set: (value: boolean) => {
          data.isTaxExempt = value;
        },
        getError: () => errors?.isTaxExempt,
        setError: (value: Array<string> | undefined) => {
          errors.isTaxExempt = value;
        },
        getTainted: () => tainted?.isTaxExempt ?? false,
        setTainted: (value: boolean) => {
          tainted.isTaxExempt = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "isTaxExempt")
              .map((e) => e.message);
          }
          return [];
        },
      },
      paymentTerms: {
        path: ["paymentTerms"] as const,
        name: "paymentTerms",
        constraints: { required: true },

        get: () => data.paymentTerms,
        set: (value: string) => {
          data.paymentTerms = value;
        },
        getError: () => errors?.paymentTerms,
        setError: (value: Array<string> | undefined) => {
          errors.paymentTerms = value;
        },
        getTainted: () => tainted?.paymentTerms ?? false,
        setTainted: (value: boolean) => {
          tainted.paymentTerms = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "paymentTerms")
              .map((e) => e.message);
          }
          return [];
        },
      },
      tags: {
        path: ["tags"] as const,
        name: "tags",
        constraints: { required: true },

        get: () => data.tags,
        set: (value: string[]) => {
          data.tags = value;
        },
        getError: () => errors?.tags,
        setError: (value: Array<string> | undefined) => {
          errors.tags = value;
        },
        getTainted: () => tainted?.tags ?? false,
        setTainted: (value: boolean) => {
          tainted.tags = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "tags")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["tags", index] as const,
          name: `tags.${index}`,
          constraints: { required: true },
          get: () => data.tags[index],
          set: (value: string) => {
            data.tags[index] = value;
          },
          getError: () =>
            (errors.tags as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.tags ??= {};
            (errors.tags as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.tags?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.tags ??= {};
            tainted.tags[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: string) => {
          data.tags.push(item);
        },
        remove: (index: number) => {
          data.tags.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.tags[a], data.tags[b]] = [data.tags[b], data.tags[a]];
        },
      },
      customFields: {
        path: ["customFields"] as const,
        name: "customFields",
        constraints: { required: true },

        get: () => data.customFields,
        set: (value: [string, string][]) => {
          data.customFields = value;
        },
        getError: () => errors?.customFields,
        setError: (value: Array<string> | undefined) => {
          errors.customFields = value;
        },
        getTainted: () => tainted?.customFields ?? false,
        setTainted: (value: boolean) => {
          tainted.customFields = value;
        },
        validate: (): Array<string> => {
          const result = Lead.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "customFields")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["customFields", index] as const,
          name: `customFields.${index}`,
          constraints: { required: true },
          get: () => data.customFields[index],
          set: (value: [string, string]) => {
            data.customFields[index] = value;
          },
          getError: () =>
            (errors.customFields as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.customFields ??= {};
            (errors.customFields as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: () => tainted.customFields?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.customFields ??= {};
            tainted.customFields[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: [string, string]) => {
          data.customFields.push(item);
        },
        remove: (index: number) => {
          data.customFields.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.customFields[a], data.customFields[b]] = [
            data.customFields[b],
            data.customFields[a],
          ];
        },
      },
    };
    function validate(): Result<
      Lead,
      Array<{ field: string; message: string }>
    > {
      return Lead.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Lead>): void {
      data = { ...Lead.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Lead, Array<{ field: string; message: string }>> {
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
    return Lead.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<AppPermissions, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "AppPermissions.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "AppPermissions.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("applications" in obj)) {
      errors.push({ field: "applications", message: "missing required field" });
    }
    if (!("pages" in obj)) {
      errors.push({ field: "pages", message: "missing required field" });
    }
    if (!("data" in obj)) {
      errors.push({ field: "data", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as AppPermissions;
  }
}

export namespace AppPermissions {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    applications?: {
      _errors?: Array<string>;
      [index: number]: Applications.Errors;
    };
    pages?: { _errors?: Array<string>; [index: number]: Page.Errors };
    data?: { _errors?: Array<string>; [index: number]: Table.Errors };
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
      applications?: { [index: number]: Applications.Tainted };
      pages?: { [index: number]: Page.Tainted };
      data?: { [index: number]: Table.Tainted };
    };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly applications: FieldController<Applications[]>;
    readonly pages: FieldController<Page[]>;
    readonly data: FieldController<Table[]>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: AppPermissions;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<
      AppPermissions,
      Array<{ field: string; message: string }>
    >;
    reset(overrides?: Partial<AppPermissions>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<AppPermissions>,
  ): Gigaform {
    let data = $state({ ...AppPermissions.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      applications: {
        path: ["applications"] as const,
        name: "applications",
        constraints: { required: true },

        get: () => data.applications,
        set: (value: Applications[]) => {
          data.applications = value;
        },
        getError: () => errors?.applications,
        setError: (value: Array<string> | undefined) => {
          errors.applications = value;
        },
        getTainted: () => tainted?.applications ?? false,
        setTainted: (value: boolean) => {
          tainted.applications = value;
        },
        validate: (): Array<string> => {
          const result = AppPermissions.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "applications")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["applications", index] as const,
          name: `applications.${index}`,
          constraints: { required: true },
          get: () => data.applications[index],
          set: (value: Applications) => {
            data.applications[index] = value;
          },
          getError: () =>
            (errors.applications as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.applications ??= {};
            (errors.applications as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: () => tainted.applications?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.applications ??= {};
            tainted.applications[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: Applications) => {
          data.applications.push(item);
        },
        remove: (index: number) => {
          data.applications.splice(index, 1);
        },
        swap: (a: number, b: number) => {
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

        get: () => data.pages,
        set: (value: Page[]) => {
          data.pages = value;
        },
        getError: () => errors?.pages,
        setError: (value: Array<string> | undefined) => {
          errors.pages = value;
        },
        getTainted: () => tainted?.pages ?? false,
        setTainted: (value: boolean) => {
          tainted.pages = value;
        },
        validate: (): Array<string> => {
          const result = AppPermissions.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "pages")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["pages", index] as const,
          name: `pages.${index}`,
          constraints: { required: true },
          get: () => data.pages[index],
          set: (value: Page) => {
            data.pages[index] = value;
          },
          getError: () =>
            (errors.pages as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.pages ??= {};
            (errors.pages as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.pages?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.pages ??= {};
            tainted.pages[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: Page) => {
          data.pages.push(item);
        },
        remove: (index: number) => {
          data.pages.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.pages[a], data.pages[b]] = [data.pages[b], data.pages[a]];
        },
      },
      data: {
        path: ["data"] as const,
        name: "data",
        constraints: { required: true },

        get: () => data.data,
        set: (value: Table[]) => {
          data.data = value;
        },
        getError: () => errors?.data,
        setError: (value: Array<string> | undefined) => {
          errors.data = value;
        },
        getTainted: () => tainted?.data ?? false,
        setTainted: (value: boolean) => {
          tainted.data = value;
        },
        validate: (): Array<string> => {
          const result = AppPermissions.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "data")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["data", index] as const,
          name: `data.${index}`,
          constraints: { required: true },
          get: () => data.data[index],
          set: (value: Table) => {
            data.data[index] = value;
          },
          getError: () =>
            (errors.data as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.data ??= {};
            (errors.data as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.data?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.data ??= {};
            tainted.data[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: Table) => {
          data.data.push(item);
        },
        remove: (index: number) => {
          data.data.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.data[a], data.data[b]] = [data.data[b], data.data[a]];
        },
      },
    };
    function validate(): Result<
      AppPermissions,
      Array<{ field: string; message: string }>
    > {
      return AppPermissions.fromJSON(data);
    }
    function reset(newOverrides?: Partial<AppPermissions>): void {
      data = { ...AppPermissions.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<AppPermissions, Array<{ field: string; message: string }>> {
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
    return AppPermissions.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): Company {
    return {
      id: "",
      legalName: "",
      headquarters: "",
      phones: [],
      fax: "",
      email: "",
      website: "",
      taxId: "",
      referenceNumber: 0,
      postalCodeLookup: "",
      timeZone: "",
      defaultTax: "",
      defaultTaxLocation: "",
      defaultAreaCode: 0,
      defaultAccountType: "",
      lookupFormatting: "",
      accountNameFormat: "",
      merchantServiceProvider: null,
      dateDisplayStyle: "",
      hasAutoCommission: false,
      hasAutoDaylightSavings: false,
      hasAutoFmsTracking: false,
      hasNotifications: false,
      hasRequiredLeadSource: false,
      hasRequiredEmail: false,
      hasSortServiceItemsAlphabetically: false,
      hasAttachOrderToAppointmentEmails: false,
      scheduleInterval: 0,
      colorsConfig: Gradient.defaultValue(),
    } as Company;
  }
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
  ): Result<Company, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Company.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Company.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("legalName" in obj)) {
      errors.push({ field: "legalName", message: "missing required field" });
    }
    if (!("headquarters" in obj)) {
      errors.push({ field: "headquarters", message: "missing required field" });
    }
    if (!("phones" in obj)) {
      errors.push({ field: "phones", message: "missing required field" });
    }
    if (!("fax" in obj)) {
      errors.push({ field: "fax", message: "missing required field" });
    }
    if (!("email" in obj)) {
      errors.push({ field: "email", message: "missing required field" });
    }
    if (!("website" in obj)) {
      errors.push({ field: "website", message: "missing required field" });
    }
    if (!("taxId" in obj)) {
      errors.push({ field: "taxId", message: "missing required field" });
    }
    if (!("referenceNumber" in obj)) {
      errors.push({
        field: "referenceNumber",
        message: "missing required field",
      });
    }
    if (!("postalCodeLookup" in obj)) {
      errors.push({
        field: "postalCodeLookup",
        message: "missing required field",
      });
    }
    if (!("timeZone" in obj)) {
      errors.push({ field: "timeZone", message: "missing required field" });
    }
    if (!("defaultTax" in obj)) {
      errors.push({ field: "defaultTax", message: "missing required field" });
    }
    if (!("defaultTaxLocation" in obj)) {
      errors.push({
        field: "defaultTaxLocation",
        message: "missing required field",
      });
    }
    if (!("defaultAreaCode" in obj)) {
      errors.push({
        field: "defaultAreaCode",
        message: "missing required field",
      });
    }
    if (!("defaultAccountType" in obj)) {
      errors.push({
        field: "defaultAccountType",
        message: "missing required field",
      });
    }
    if (!("lookupFormatting" in obj)) {
      errors.push({
        field: "lookupFormatting",
        message: "missing required field",
      });
    }
    if (!("accountNameFormat" in obj)) {
      errors.push({
        field: "accountNameFormat",
        message: "missing required field",
      });
    }
    if (!("merchantServiceProvider" in obj)) {
      errors.push({
        field: "merchantServiceProvider",
        message: "missing required field",
      });
    }
    if (!("dateDisplayStyle" in obj)) {
      errors.push({
        field: "dateDisplayStyle",
        message: "missing required field",
      });
    }
    if (!("hasAutoCommission" in obj)) {
      errors.push({
        field: "hasAutoCommission",
        message: "missing required field",
      });
    }
    if (!("hasAutoDaylightSavings" in obj)) {
      errors.push({
        field: "hasAutoDaylightSavings",
        message: "missing required field",
      });
    }
    if (!("hasAutoFmsTracking" in obj)) {
      errors.push({
        field: "hasAutoFmsTracking",
        message: "missing required field",
      });
    }
    if (!("hasNotifications" in obj)) {
      errors.push({
        field: "hasNotifications",
        message: "missing required field",
      });
    }
    if (!("hasRequiredLeadSource" in obj)) {
      errors.push({
        field: "hasRequiredLeadSource",
        message: "missing required field",
      });
    }
    if (!("hasRequiredEmail" in obj)) {
      errors.push({
        field: "hasRequiredEmail",
        message: "missing required field",
      });
    }
    if (!("hasSortServiceItemsAlphabetically" in obj)) {
      errors.push({
        field: "hasSortServiceItemsAlphabetically",
        message: "missing required field",
      });
    }
    if (!("hasAttachOrderToAppointmentEmails" in obj)) {
      errors.push({
        field: "hasAttachOrderToAppointmentEmails",
        message: "missing required field",
      });
    }
    if (!("scheduleInterval" in obj)) {
      errors.push({
        field: "scheduleInterval",
        message: "missing required field",
      });
    }
    if (!("colorsConfig" in obj)) {
      errors.push({ field: "colorsConfig", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Company;
  }
}

export namespace Company {
  /** Nested error structure matching the data shape */ export type Errors = {
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
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
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
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly legalName: FieldController<string>;
    readonly headquarters: FieldController<string | Site>;
    readonly phones: FieldController<PhoneNumber[]>;
    readonly fax: FieldController<string>;
    readonly email: FieldController<string>;
    readonly website: FieldController<string>;
    readonly taxId: FieldController<string>;
    readonly referenceNumber: FieldController<number>;
    readonly postalCodeLookup: FieldController<string>;
    readonly timeZone: FieldController<string>;
    readonly defaultTax: FieldController<string | TaxRate>;
    readonly defaultTaxLocation: FieldController<string>;
    readonly defaultAreaCode: FieldController<number>;
    readonly defaultAccountType: FieldController<string>;
    readonly lookupFormatting: FieldController<string>;
    readonly accountNameFormat: FieldController<string>;
    readonly merchantServiceProvider: FieldController<string | null>;
    readonly dateDisplayStyle: FieldController<string>;
    readonly hasAutoCommission: FieldController<boolean>;
    readonly hasAutoDaylightSavings: FieldController<boolean>;
    readonly hasAutoFmsTracking: FieldController<boolean>;
    readonly hasNotifications: FieldController<boolean>;
    readonly hasRequiredLeadSource: FieldController<boolean>;
    readonly hasRequiredEmail: FieldController<boolean>;
    readonly hasSortServiceItemsAlphabetically: FieldController<boolean>;
    readonly hasAttachOrderToAppointmentEmails: FieldController<boolean>;
    readonly scheduleInterval: FieldController<number>;
    readonly colorsConfig: FieldController<ColorsConfig>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Company;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Company, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Company>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Company>,
  ): Gigaform {
    let data = $state({ ...Company.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      legalName: {
        path: ["legalName"] as const,
        name: "legalName",
        constraints: { required: true },

        get: () => data.legalName,
        set: (value: string) => {
          data.legalName = value;
        },
        getError: () => errors?.legalName,
        setError: (value: Array<string> | undefined) => {
          errors.legalName = value;
        },
        getTainted: () => tainted?.legalName ?? false,
        setTainted: (value: boolean) => {
          tainted.legalName = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "legalName")
              .map((e) => e.message);
          }
          return [];
        },
      },
      headquarters: {
        path: ["headquarters"] as const,
        name: "headquarters",
        constraints: { required: true },

        get: () => data.headquarters,
        set: (value: string | Site) => {
          data.headquarters = value;
        },
        getError: () => errors?.headquarters,
        setError: (value: Array<string> | undefined) => {
          errors.headquarters = value;
        },
        getTainted: () => tainted?.headquarters ?? false,
        setTainted: (value: boolean) => {
          tainted.headquarters = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "headquarters")
              .map((e) => e.message);
          }
          return [];
        },
      },
      phones: {
        path: ["phones"] as const,
        name: "phones",
        constraints: { required: true },

        get: () => data.phones,
        set: (value: PhoneNumber[]) => {
          data.phones = value;
        },
        getError: () => errors?.phones,
        setError: (value: Array<string> | undefined) => {
          errors.phones = value;
        },
        getTainted: () => tainted?.phones ?? false,
        setTainted: (value: boolean) => {
          tainted.phones = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "phones")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["phones", index] as const,
          name: `phones.${index}`,
          constraints: { required: true },
          get: () => data.phones[index],
          set: (value: PhoneNumber) => {
            data.phones[index] = value;
          },
          getError: () =>
            (errors.phones as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.phones ??= {};
            (errors.phones as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.phones?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.phones ??= {};
            tainted.phones[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: PhoneNumber) => {
          data.phones.push(item);
        },
        remove: (index: number) => {
          data.phones.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.phones[a], data.phones[b]] = [data.phones[b], data.phones[a]];
        },
      },
      fax: {
        path: ["fax"] as const,
        name: "fax",
        constraints: { required: true },

        get: () => data.fax,
        set: (value: string) => {
          data.fax = value;
        },
        getError: () => errors?.fax,
        setError: (value: Array<string> | undefined) => {
          errors.fax = value;
        },
        getTainted: () => tainted?.fax ?? false,
        setTainted: (value: boolean) => {
          tainted.fax = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "fax")
              .map((e) => e.message);
          }
          return [];
        },
      },
      email: {
        path: ["email"] as const,
        name: "email",
        constraints: { required: true },

        get: () => data.email,
        set: (value: string) => {
          data.email = value;
        },
        getError: () => errors?.email,
        setError: (value: Array<string> | undefined) => {
          errors.email = value;
        },
        getTainted: () => tainted?.email ?? false,
        setTainted: (value: boolean) => {
          tainted.email = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "email")
              .map((e) => e.message);
          }
          return [];
        },
      },
      website: {
        path: ["website"] as const,
        name: "website",
        constraints: { required: true },

        get: () => data.website,
        set: (value: string) => {
          data.website = value;
        },
        getError: () => errors?.website,
        setError: (value: Array<string> | undefined) => {
          errors.website = value;
        },
        getTainted: () => tainted?.website ?? false,
        setTainted: (value: boolean) => {
          tainted.website = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "website")
              .map((e) => e.message);
          }
          return [];
        },
      },
      taxId: {
        path: ["taxId"] as const,
        name: "taxId",
        constraints: { required: true },

        get: () => data.taxId,
        set: (value: string) => {
          data.taxId = value;
        },
        getError: () => errors?.taxId,
        setError: (value: Array<string> | undefined) => {
          errors.taxId = value;
        },
        getTainted: () => tainted?.taxId ?? false,
        setTainted: (value: boolean) => {
          tainted.taxId = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "taxId")
              .map((e) => e.message);
          }
          return [];
        },
      },
      referenceNumber: {
        path: ["referenceNumber"] as const,
        name: "referenceNumber",
        constraints: { required: true },

        get: () => data.referenceNumber,
        set: (value: number) => {
          data.referenceNumber = value;
        },
        getError: () => errors?.referenceNumber,
        setError: (value: Array<string> | undefined) => {
          errors.referenceNumber = value;
        },
        getTainted: () => tainted?.referenceNumber ?? false,
        setTainted: (value: boolean) => {
          tainted.referenceNumber = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "referenceNumber")
              .map((e) => e.message);
          }
          return [];
        },
      },
      postalCodeLookup: {
        path: ["postalCodeLookup"] as const,
        name: "postalCodeLookup",
        constraints: { required: true },

        get: () => data.postalCodeLookup,
        set: (value: string) => {
          data.postalCodeLookup = value;
        },
        getError: () => errors?.postalCodeLookup,
        setError: (value: Array<string> | undefined) => {
          errors.postalCodeLookup = value;
        },
        getTainted: () => tainted?.postalCodeLookup ?? false,
        setTainted: (value: boolean) => {
          tainted.postalCodeLookup = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "postalCodeLookup")
              .map((e) => e.message);
          }
          return [];
        },
      },
      timeZone: {
        path: ["timeZone"] as const,
        name: "timeZone",
        constraints: { required: true },

        get: () => data.timeZone,
        set: (value: string) => {
          data.timeZone = value;
        },
        getError: () => errors?.timeZone,
        setError: (value: Array<string> | undefined) => {
          errors.timeZone = value;
        },
        getTainted: () => tainted?.timeZone ?? false,
        setTainted: (value: boolean) => {
          tainted.timeZone = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "timeZone")
              .map((e) => e.message);
          }
          return [];
        },
      },
      defaultTax: {
        path: ["defaultTax"] as const,
        name: "defaultTax",
        constraints: { required: true },

        get: () => data.defaultTax,
        set: (value: string | TaxRate) => {
          data.defaultTax = value;
        },
        getError: () => errors?.defaultTax,
        setError: (value: Array<string> | undefined) => {
          errors.defaultTax = value;
        },
        getTainted: () => tainted?.defaultTax ?? false,
        setTainted: (value: boolean) => {
          tainted.defaultTax = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "defaultTax")
              .map((e) => e.message);
          }
          return [];
        },
      },
      defaultTaxLocation: {
        path: ["defaultTaxLocation"] as const,
        name: "defaultTaxLocation",
        constraints: { required: true },

        get: () => data.defaultTaxLocation,
        set: (value: string) => {
          data.defaultTaxLocation = value;
        },
        getError: () => errors?.defaultTaxLocation,
        setError: (value: Array<string> | undefined) => {
          errors.defaultTaxLocation = value;
        },
        getTainted: () => tainted?.defaultTaxLocation ?? false,
        setTainted: (value: boolean) => {
          tainted.defaultTaxLocation = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "defaultTaxLocation")
              .map((e) => e.message);
          }
          return [];
        },
      },
      defaultAreaCode: {
        path: ["defaultAreaCode"] as const,
        name: "defaultAreaCode",
        constraints: { required: true },

        get: () => data.defaultAreaCode,
        set: (value: number) => {
          data.defaultAreaCode = value;
        },
        getError: () => errors?.defaultAreaCode,
        setError: (value: Array<string> | undefined) => {
          errors.defaultAreaCode = value;
        },
        getTainted: () => tainted?.defaultAreaCode ?? false,
        setTainted: (value: boolean) => {
          tainted.defaultAreaCode = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "defaultAreaCode")
              .map((e) => e.message);
          }
          return [];
        },
      },
      defaultAccountType: {
        path: ["defaultAccountType"] as const,
        name: "defaultAccountType",
        constraints: { required: true },

        get: () => data.defaultAccountType,
        set: (value: string) => {
          data.defaultAccountType = value;
        },
        getError: () => errors?.defaultAccountType,
        setError: (value: Array<string> | undefined) => {
          errors.defaultAccountType = value;
        },
        getTainted: () => tainted?.defaultAccountType ?? false,
        setTainted: (value: boolean) => {
          tainted.defaultAccountType = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "defaultAccountType")
              .map((e) => e.message);
          }
          return [];
        },
      },
      lookupFormatting: {
        path: ["lookupFormatting"] as const,
        name: "lookupFormatting",
        constraints: { required: true },

        get: () => data.lookupFormatting,
        set: (value: string) => {
          data.lookupFormatting = value;
        },
        getError: () => errors?.lookupFormatting,
        setError: (value: Array<string> | undefined) => {
          errors.lookupFormatting = value;
        },
        getTainted: () => tainted?.lookupFormatting ?? false,
        setTainted: (value: boolean) => {
          tainted.lookupFormatting = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "lookupFormatting")
              .map((e) => e.message);
          }
          return [];
        },
      },
      accountNameFormat: {
        path: ["accountNameFormat"] as const,
        name: "accountNameFormat",
        constraints: { required: true },

        get: () => data.accountNameFormat,
        set: (value: string) => {
          data.accountNameFormat = value;
        },
        getError: () => errors?.accountNameFormat,
        setError: (value: Array<string> | undefined) => {
          errors.accountNameFormat = value;
        },
        getTainted: () => tainted?.accountNameFormat ?? false,
        setTainted: (value: boolean) => {
          tainted.accountNameFormat = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "accountNameFormat")
              .map((e) => e.message);
          }
          return [];
        },
      },
      merchantServiceProvider: {
        path: ["merchantServiceProvider"] as const,
        name: "merchantServiceProvider",
        constraints: { required: true },

        get: () => data.merchantServiceProvider,
        set: (value: string | null) => {
          data.merchantServiceProvider = value;
        },
        getError: () => errors?.merchantServiceProvider,
        setError: (value: Array<string> | undefined) => {
          errors.merchantServiceProvider = value;
        },
        getTainted: () => tainted?.merchantServiceProvider ?? false,
        setTainted: (value: boolean) => {
          tainted.merchantServiceProvider = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "merchantServiceProvider")
              .map((e) => e.message);
          }
          return [];
        },
      },
      dateDisplayStyle: {
        path: ["dateDisplayStyle"] as const,
        name: "dateDisplayStyle",
        constraints: { required: true },

        get: () => data.dateDisplayStyle,
        set: (value: string) => {
          data.dateDisplayStyle = value;
        },
        getError: () => errors?.dateDisplayStyle,
        setError: (value: Array<string> | undefined) => {
          errors.dateDisplayStyle = value;
        },
        getTainted: () => tainted?.dateDisplayStyle ?? false,
        setTainted: (value: boolean) => {
          tainted.dateDisplayStyle = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "dateDisplayStyle")
              .map((e) => e.message);
          }
          return [];
        },
      },
      hasAutoCommission: {
        path: ["hasAutoCommission"] as const,
        name: "hasAutoCommission",
        constraints: { required: true },

        get: () => data.hasAutoCommission,
        set: (value: boolean) => {
          data.hasAutoCommission = value;
        },
        getError: () => errors?.hasAutoCommission,
        setError: (value: Array<string> | undefined) => {
          errors.hasAutoCommission = value;
        },
        getTainted: () => tainted?.hasAutoCommission ?? false,
        setTainted: (value: boolean) => {
          tainted.hasAutoCommission = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "hasAutoCommission")
              .map((e) => e.message);
          }
          return [];
        },
      },
      hasAutoDaylightSavings: {
        path: ["hasAutoDaylightSavings"] as const,
        name: "hasAutoDaylightSavings",
        constraints: { required: true },

        get: () => data.hasAutoDaylightSavings,
        set: (value: boolean) => {
          data.hasAutoDaylightSavings = value;
        },
        getError: () => errors?.hasAutoDaylightSavings,
        setError: (value: Array<string> | undefined) => {
          errors.hasAutoDaylightSavings = value;
        },
        getTainted: () => tainted?.hasAutoDaylightSavings ?? false,
        setTainted: (value: boolean) => {
          tainted.hasAutoDaylightSavings = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "hasAutoDaylightSavings")
              .map((e) => e.message);
          }
          return [];
        },
      },
      hasAutoFmsTracking: {
        path: ["hasAutoFmsTracking"] as const,
        name: "hasAutoFmsTracking",
        constraints: { required: true },

        get: () => data.hasAutoFmsTracking,
        set: (value: boolean) => {
          data.hasAutoFmsTracking = value;
        },
        getError: () => errors?.hasAutoFmsTracking,
        setError: (value: Array<string> | undefined) => {
          errors.hasAutoFmsTracking = value;
        },
        getTainted: () => tainted?.hasAutoFmsTracking ?? false,
        setTainted: (value: boolean) => {
          tainted.hasAutoFmsTracking = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "hasAutoFmsTracking")
              .map((e) => e.message);
          }
          return [];
        },
      },
      hasNotifications: {
        path: ["hasNotifications"] as const,
        name: "hasNotifications",
        constraints: { required: true },

        get: () => data.hasNotifications,
        set: (value: boolean) => {
          data.hasNotifications = value;
        },
        getError: () => errors?.hasNotifications,
        setError: (value: Array<string> | undefined) => {
          errors.hasNotifications = value;
        },
        getTainted: () => tainted?.hasNotifications ?? false,
        setTainted: (value: boolean) => {
          tainted.hasNotifications = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "hasNotifications")
              .map((e) => e.message);
          }
          return [];
        },
      },
      hasRequiredLeadSource: {
        path: ["hasRequiredLeadSource"] as const,
        name: "hasRequiredLeadSource",
        constraints: { required: true },

        get: () => data.hasRequiredLeadSource,
        set: (value: boolean) => {
          data.hasRequiredLeadSource = value;
        },
        getError: () => errors?.hasRequiredLeadSource,
        setError: (value: Array<string> | undefined) => {
          errors.hasRequiredLeadSource = value;
        },
        getTainted: () => tainted?.hasRequiredLeadSource ?? false,
        setTainted: (value: boolean) => {
          tainted.hasRequiredLeadSource = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "hasRequiredLeadSource")
              .map((e) => e.message);
          }
          return [];
        },
      },
      hasRequiredEmail: {
        path: ["hasRequiredEmail"] as const,
        name: "hasRequiredEmail",
        constraints: { required: true },

        get: () => data.hasRequiredEmail,
        set: (value: boolean) => {
          data.hasRequiredEmail = value;
        },
        getError: () => errors?.hasRequiredEmail,
        setError: (value: Array<string> | undefined) => {
          errors.hasRequiredEmail = value;
        },
        getTainted: () => tainted?.hasRequiredEmail ?? false,
        setTainted: (value: boolean) => {
          tainted.hasRequiredEmail = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "hasRequiredEmail")
              .map((e) => e.message);
          }
          return [];
        },
      },
      hasSortServiceItemsAlphabetically: {
        path: ["hasSortServiceItemsAlphabetically"] as const,
        name: "hasSortServiceItemsAlphabetically",
        constraints: { required: true },

        get: () => data.hasSortServiceItemsAlphabetically,
        set: (value: boolean) => {
          data.hasSortServiceItemsAlphabetically = value;
        },
        getError: () => errors?.hasSortServiceItemsAlphabetically,
        setError: (value: Array<string> | undefined) => {
          errors.hasSortServiceItemsAlphabetically = value;
        },
        getTainted: () => tainted?.hasSortServiceItemsAlphabetically ?? false,
        setTainted: (value: boolean) => {
          tainted.hasSortServiceItemsAlphabetically = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "hasSortServiceItemsAlphabetically")
              .map((e) => e.message);
          }
          return [];
        },
      },
      hasAttachOrderToAppointmentEmails: {
        path: ["hasAttachOrderToAppointmentEmails"] as const,
        name: "hasAttachOrderToAppointmentEmails",
        constraints: { required: true },

        get: () => data.hasAttachOrderToAppointmentEmails,
        set: (value: boolean) => {
          data.hasAttachOrderToAppointmentEmails = value;
        },
        getError: () => errors?.hasAttachOrderToAppointmentEmails,
        setError: (value: Array<string> | undefined) => {
          errors.hasAttachOrderToAppointmentEmails = value;
        },
        getTainted: () => tainted?.hasAttachOrderToAppointmentEmails ?? false,
        setTainted: (value: boolean) => {
          tainted.hasAttachOrderToAppointmentEmails = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "hasAttachOrderToAppointmentEmails")
              .map((e) => e.message);
          }
          return [];
        },
      },
      scheduleInterval: {
        path: ["scheduleInterval"] as const,
        name: "scheduleInterval",
        constraints: { required: true },

        get: () => data.scheduleInterval,
        set: (value: number) => {
          data.scheduleInterval = value;
        },
        getError: () => errors?.scheduleInterval,
        setError: (value: Array<string> | undefined) => {
          errors.scheduleInterval = value;
        },
        getTainted: () => tainted?.scheduleInterval ?? false,
        setTainted: (value: boolean) => {
          tainted.scheduleInterval = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "scheduleInterval")
              .map((e) => e.message);
          }
          return [];
        },
      },
      colorsConfig: {
        path: ["colorsConfig"] as const,
        name: "colorsConfig",
        constraints: { required: true },

        get: () => data.colorsConfig,
        set: (value: ColorsConfig) => {
          data.colorsConfig = value;
        },
        getError: () => errors?.colorsConfig,
        setError: (value: Array<string> | undefined) => {
          errors.colorsConfig = value;
        },
        getTainted: () => tainted?.colorsConfig ?? false,
        setTainted: (value: boolean) => {
          tainted.colorsConfig = value;
        },
        validate: (): Array<string> => {
          const result = Company.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "colorsConfig")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Company,
      Array<{ field: string; message: string }>
    > {
      return Company.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Company>): void {
      data = { ...Company.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Company, Array<{ field: string; message: string }>> {
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
    return Company.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Ordinal, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Ordinal.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Ordinal.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("north" in obj)) {
      errors.push({ field: "north", message: "missing required field" });
    }
    if (!("northeast" in obj)) {
      errors.push({ field: "northeast", message: "missing required field" });
    }
    if (!("east" in obj)) {
      errors.push({ field: "east", message: "missing required field" });
    }
    if (!("southeast" in obj)) {
      errors.push({ field: "southeast", message: "missing required field" });
    }
    if (!("south" in obj)) {
      errors.push({ field: "south", message: "missing required field" });
    }
    if (!("southwest" in obj)) {
      errors.push({ field: "southwest", message: "missing required field" });
    }
    if (!("west" in obj)) {
      errors.push({ field: "west", message: "missing required field" });
    }
    if (!("northwest" in obj)) {
      errors.push({ field: "northwest", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Ordinal;
  }
}

export namespace Ordinal {
  /** Nested error structure matching the data shape */ export type Errors = {
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
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
      north?: boolean;
      northeast?: boolean;
      east?: boolean;
      southeast?: boolean;
      south?: boolean;
      southwest?: boolean;
      west?: boolean;
      northwest?: boolean;
    };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly north: FieldController<number>;
    readonly northeast: FieldController<number>;
    readonly east: FieldController<number>;
    readonly southeast: FieldController<number>;
    readonly south: FieldController<number>;
    readonly southwest: FieldController<number>;
    readonly west: FieldController<number>;
    readonly northwest: FieldController<number>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Ordinal;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Ordinal, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Ordinal>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Ordinal>,
  ): Gigaform {
    let data = $state({ ...Ordinal.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      north: {
        path: ["north"] as const,
        name: "north",
        constraints: { required: true },

        get: () => data.north,
        set: (value: number) => {
          data.north = value;
        },
        getError: () => errors?.north,
        setError: (value: Array<string> | undefined) => {
          errors.north = value;
        },
        getTainted: () => tainted?.north ?? false,
        setTainted: (value: boolean) => {
          tainted.north = value;
        },
        validate: (): Array<string> => {
          const result = Ordinal.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "north")
              .map((e) => e.message);
          }
          return [];
        },
      },
      northeast: {
        path: ["northeast"] as const,
        name: "northeast",
        constraints: { required: true },

        get: () => data.northeast,
        set: (value: number) => {
          data.northeast = value;
        },
        getError: () => errors?.northeast,
        setError: (value: Array<string> | undefined) => {
          errors.northeast = value;
        },
        getTainted: () => tainted?.northeast ?? false,
        setTainted: (value: boolean) => {
          tainted.northeast = value;
        },
        validate: (): Array<string> => {
          const result = Ordinal.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "northeast")
              .map((e) => e.message);
          }
          return [];
        },
      },
      east: {
        path: ["east"] as const,
        name: "east",
        constraints: { required: true },

        get: () => data.east,
        set: (value: number) => {
          data.east = value;
        },
        getError: () => errors?.east,
        setError: (value: Array<string> | undefined) => {
          errors.east = value;
        },
        getTainted: () => tainted?.east ?? false,
        setTainted: (value: boolean) => {
          tainted.east = value;
        },
        validate: (): Array<string> => {
          const result = Ordinal.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "east")
              .map((e) => e.message);
          }
          return [];
        },
      },
      southeast: {
        path: ["southeast"] as const,
        name: "southeast",
        constraints: { required: true },

        get: () => data.southeast,
        set: (value: number) => {
          data.southeast = value;
        },
        getError: () => errors?.southeast,
        setError: (value: Array<string> | undefined) => {
          errors.southeast = value;
        },
        getTainted: () => tainted?.southeast ?? false,
        setTainted: (value: boolean) => {
          tainted.southeast = value;
        },
        validate: (): Array<string> => {
          const result = Ordinal.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "southeast")
              .map((e) => e.message);
          }
          return [];
        },
      },
      south: {
        path: ["south"] as const,
        name: "south",
        constraints: { required: true },

        get: () => data.south,
        set: (value: number) => {
          data.south = value;
        },
        getError: () => errors?.south,
        setError: (value: Array<string> | undefined) => {
          errors.south = value;
        },
        getTainted: () => tainted?.south ?? false,
        setTainted: (value: boolean) => {
          tainted.south = value;
        },
        validate: (): Array<string> => {
          const result = Ordinal.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "south")
              .map((e) => e.message);
          }
          return [];
        },
      },
      southwest: {
        path: ["southwest"] as const,
        name: "southwest",
        constraints: { required: true },

        get: () => data.southwest,
        set: (value: number) => {
          data.southwest = value;
        },
        getError: () => errors?.southwest,
        setError: (value: Array<string> | undefined) => {
          errors.southwest = value;
        },
        getTainted: () => tainted?.southwest ?? false,
        setTainted: (value: boolean) => {
          tainted.southwest = value;
        },
        validate: (): Array<string> => {
          const result = Ordinal.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "southwest")
              .map((e) => e.message);
          }
          return [];
        },
      },
      west: {
        path: ["west"] as const,
        name: "west",
        constraints: { required: true },

        get: () => data.west,
        set: (value: number) => {
          data.west = value;
        },
        getError: () => errors?.west,
        setError: (value: Array<string> | undefined) => {
          errors.west = value;
        },
        getTainted: () => tainted?.west ?? false,
        setTainted: (value: boolean) => {
          tainted.west = value;
        },
        validate: (): Array<string> => {
          const result = Ordinal.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "west")
              .map((e) => e.message);
          }
          return [];
        },
      },
      northwest: {
        path: ["northwest"] as const,
        name: "northwest",
        constraints: { required: true },

        get: () => data.northwest,
        set: (value: number) => {
          data.northwest = value;
        },
        getError: () => errors?.northwest,
        setError: (value: Array<string> | undefined) => {
          errors.northwest = value;
        },
        getTainted: () => tainted?.northwest ?? false,
        setTainted: (value: boolean) => {
          tainted.northwest = value;
        },
        validate: (): Array<string> => {
          const result = Ordinal.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "northwest")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Ordinal,
      Array<{ field: string; message: string }>
    > {
      return Ordinal.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Ordinal>): void {
      data = { ...Ordinal.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Ordinal, Array<{ field: string; message: string }>> {
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
    return Ordinal.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Password, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Password.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Password.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("password" in obj)) {
      errors.push({ field: "password", message: "missing required field" });
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
      const __raw_password = obj["password"];
      instance.password = __raw_password;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as Password;
  }
}

export namespace Password {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    password?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { password?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly password: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Password;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Password, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Password>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Password>,
  ): Gigaform {
    let data = $state({ ...Password.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      password: {
        path: ["password"] as const,
        name: "password",
        constraints: { required: true },

        get: () => data.password,
        set: (value: string) => {
          data.password = value;
        },
        getError: () => errors?.password,
        setError: (value: Array<string> | undefined) => {
          errors.password = value;
        },
        getTainted: () => tainted?.password ?? false,
        setTainted: (value: boolean) => {
          tainted.password = value;
        },
        validate: (): Array<string> => {
          const result = Password.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "password")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Password,
      Array<{ field: string; message: string }>
    > {
      return Password.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Password>): void {
      data = { ...Password.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Password, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.password = formData.get("password") ?? "";
    return Password.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Created, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Created.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Created.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("initialData" in obj)) {
      errors.push({ field: "initialData", message: "missing required field" });
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
      const __raw_initialData = obj["initialData"];
      instance.initialData = __raw_initialData;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as Created;
  }
}

export namespace Created {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    initialData?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { initialData?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly initialData: FieldController<string | null>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Created;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Created, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Created>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Created>,
  ): Gigaform {
    let data = $state({ ...Created.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      initialData: {
        path: ["initialData"] as const,
        name: "initialData",
        constraints: { required: true },

        get: () => data.initialData,
        set: (value: string | null) => {
          data.initialData = value;
        },
        getError: () => errors?.initialData,
        setError: (value: Array<string> | undefined) => {
          errors.initialData = value;
        },
        getTainted: () => tainted?.initialData ?? false,
        setTainted: (value: boolean) => {
          tainted.initialData = value;
        },
        validate: (): Array<string> => {
          const result = Created.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "initialData")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Created,
      Array<{ field: string; message: string }>
    > {
      return Created.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Created>): void {
      data = { ...Created.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Created, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.initialData = formData.get("initialData") ?? "";
    return Created.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): Employee {
    return {
      id: "",
      imageUrl: null,
      name: "",
      phones: [],
      role: "",
      title: "Technician",
      email: Email.defaultValue(),
      address: "",
      username: "",
      route: "",
      ratePerHour: 0,
      active: false,
      isTechnician: false,
      isSalesRep: false,
      description: null,
      linkedinUrl: null,
      attendance: [],
      settings: Settings.defaultValue(),
    } as Employee;
  }
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
  ): Result<Employee, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Employee.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Employee.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("imageUrl" in obj)) {
      errors.push({ field: "imageUrl", message: "missing required field" });
    }
    if (!("name" in obj)) {
      errors.push({ field: "name", message: "missing required field" });
    }
    if (!("phones" in obj)) {
      errors.push({ field: "phones", message: "missing required field" });
    }
    if (!("role" in obj)) {
      errors.push({ field: "role", message: "missing required field" });
    }
    if (!("title" in obj)) {
      errors.push({ field: "title", message: "missing required field" });
    }
    if (!("email" in obj)) {
      errors.push({ field: "email", message: "missing required field" });
    }
    if (!("address" in obj)) {
      errors.push({ field: "address", message: "missing required field" });
    }
    if (!("username" in obj)) {
      errors.push({ field: "username", message: "missing required field" });
    }
    if (!("route" in obj)) {
      errors.push({ field: "route", message: "missing required field" });
    }
    if (!("ratePerHour" in obj)) {
      errors.push({ field: "ratePerHour", message: "missing required field" });
    }
    if (!("active" in obj)) {
      errors.push({ field: "active", message: "missing required field" });
    }
    if (!("isTechnician" in obj)) {
      errors.push({ field: "isTechnician", message: "missing required field" });
    }
    if (!("isSalesRep" in obj)) {
      errors.push({ field: "isSalesRep", message: "missing required field" });
    }
    if (!("description" in obj)) {
      errors.push({ field: "description", message: "missing required field" });
    }
    if (!("linkedinUrl" in obj)) {
      errors.push({ field: "linkedinUrl", message: "missing required field" });
    }
    if (!("attendance" in obj)) {
      errors.push({ field: "attendance", message: "missing required field" });
    }
    if (!("settings" in obj)) {
      errors.push({ field: "settings", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Employee;
  }
}

export namespace Employee {
  /** Nested error structure matching the data shape */ export type Errors = {
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
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
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
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly imageUrl: FieldController<string | null>;
    readonly name: FieldController<string>;
    readonly phones: FieldController<PhoneNumber[]>;
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
    readonly attendance: FieldController<string[]>;
    readonly settings: FieldController<Settings>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Employee;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Employee, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Employee>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Employee>,
  ): Gigaform {
    let data = $state({ ...Employee.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      imageUrl: {
        path: ["imageUrl"] as const,
        name: "imageUrl",
        constraints: { required: true },

        get: () => data.imageUrl,
        set: (value: string | null) => {
          data.imageUrl = value;
        },
        getError: () => errors?.imageUrl,
        setError: (value: Array<string> | undefined) => {
          errors.imageUrl = value;
        },
        getTainted: () => tainted?.imageUrl ?? false,
        setTainted: (value: boolean) => {
          tainted.imageUrl = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "imageUrl")
              .map((e) => e.message);
          }
          return [];
        },
      },
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: () => data.name,
        set: (value: string) => {
          data.name = value;
        },
        getError: () => errors?.name,
        setError: (value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: () => tainted?.name ?? false,
        setTainted: (value: boolean) => {
          tainted.name = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "name")
              .map((e) => e.message);
          }
          return [];
        },
      },
      phones: {
        path: ["phones"] as const,
        name: "phones",
        constraints: { required: true },

        get: () => data.phones,
        set: (value: PhoneNumber[]) => {
          data.phones = value;
        },
        getError: () => errors?.phones,
        setError: (value: Array<string> | undefined) => {
          errors.phones = value;
        },
        getTainted: () => tainted?.phones ?? false,
        setTainted: (value: boolean) => {
          tainted.phones = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "phones")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["phones", index] as const,
          name: `phones.${index}`,
          constraints: { required: true },
          get: () => data.phones[index],
          set: (value: PhoneNumber) => {
            data.phones[index] = value;
          },
          getError: () =>
            (errors.phones as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.phones ??= {};
            (errors.phones as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.phones?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.phones ??= {};
            tainted.phones[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: PhoneNumber) => {
          data.phones.push(item);
        },
        remove: (index: number) => {
          data.phones.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.phones[a], data.phones[b]] = [data.phones[b], data.phones[a]];
        },
      },
      role: {
        path: ["role"] as const,
        name: "role",
        constraints: { required: true },

        get: () => data.role,
        set: (value: string) => {
          data.role = value;
        },
        getError: () => errors?.role,
        setError: (value: Array<string> | undefined) => {
          errors.role = value;
        },
        getTainted: () => tainted?.role ?? false,
        setTainted: (value: boolean) => {
          tainted.role = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "role")
              .map((e) => e.message);
          }
          return [];
        },
      },
      title: {
        path: ["title"] as const,
        name: "title",
        constraints: { required: true },

        get: () => data.title,
        set: (value: JobTitle) => {
          data.title = value;
        },
        getError: () => errors?.title,
        setError: (value: Array<string> | undefined) => {
          errors.title = value;
        },
        getTainted: () => tainted?.title ?? false,
        setTainted: (value: boolean) => {
          tainted.title = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "title")
              .map((e) => e.message);
          }
          return [];
        },
      },
      email: {
        path: ["email"] as const,
        name: "email",
        constraints: { required: true },

        get: () => data.email,
        set: (value: Email) => {
          data.email = value;
        },
        getError: () => errors?.email,
        setError: (value: Array<string> | undefined) => {
          errors.email = value;
        },
        getTainted: () => tainted?.email ?? false,
        setTainted: (value: boolean) => {
          tainted.email = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "email")
              .map((e) => e.message);
          }
          return [];
        },
      },
      address: {
        path: ["address"] as const,
        name: "address",
        constraints: { required: true },

        get: () => data.address,
        set: (value: string) => {
          data.address = value;
        },
        getError: () => errors?.address,
        setError: (value: Array<string> | undefined) => {
          errors.address = value;
        },
        getTainted: () => tainted?.address ?? false,
        setTainted: (value: boolean) => {
          tainted.address = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "address")
              .map((e) => e.message);
          }
          return [];
        },
      },
      username: {
        path: ["username"] as const,
        name: "username",
        constraints: { required: true },

        get: () => data.username,
        set: (value: string) => {
          data.username = value;
        },
        getError: () => errors?.username,
        setError: (value: Array<string> | undefined) => {
          errors.username = value;
        },
        getTainted: () => tainted?.username ?? false,
        setTainted: (value: boolean) => {
          tainted.username = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "username")
              .map((e) => e.message);
          }
          return [];
        },
      },
      route: {
        path: ["route"] as const,
        name: "route",
        constraints: { required: true },

        get: () => data.route,
        set: (value: string | Route) => {
          data.route = value;
        },
        getError: () => errors?.route,
        setError: (value: Array<string> | undefined) => {
          errors.route = value;
        },
        getTainted: () => tainted?.route ?? false,
        setTainted: (value: boolean) => {
          tainted.route = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "route")
              .map((e) => e.message);
          }
          return [];
        },
      },
      ratePerHour: {
        path: ["ratePerHour"] as const,
        name: "ratePerHour",
        constraints: { required: true },

        get: () => data.ratePerHour,
        set: (value: number) => {
          data.ratePerHour = value;
        },
        getError: () => errors?.ratePerHour,
        setError: (value: Array<string> | undefined) => {
          errors.ratePerHour = value;
        },
        getTainted: () => tainted?.ratePerHour ?? false,
        setTainted: (value: boolean) => {
          tainted.ratePerHour = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "ratePerHour")
              .map((e) => e.message);
          }
          return [];
        },
      },
      active: {
        path: ["active"] as const,
        name: "active",
        constraints: { required: true },

        get: () => data.active,
        set: (value: boolean) => {
          data.active = value;
        },
        getError: () => errors?.active,
        setError: (value: Array<string> | undefined) => {
          errors.active = value;
        },
        getTainted: () => tainted?.active ?? false,
        setTainted: (value: boolean) => {
          tainted.active = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "active")
              .map((e) => e.message);
          }
          return [];
        },
      },
      isTechnician: {
        path: ["isTechnician"] as const,
        name: "isTechnician",
        constraints: { required: true },

        get: () => data.isTechnician,
        set: (value: boolean) => {
          data.isTechnician = value;
        },
        getError: () => errors?.isTechnician,
        setError: (value: Array<string> | undefined) => {
          errors.isTechnician = value;
        },
        getTainted: () => tainted?.isTechnician ?? false,
        setTainted: (value: boolean) => {
          tainted.isTechnician = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "isTechnician")
              .map((e) => e.message);
          }
          return [];
        },
      },
      isSalesRep: {
        path: ["isSalesRep"] as const,
        name: "isSalesRep",
        constraints: { required: true },

        get: () => data.isSalesRep,
        set: (value: boolean) => {
          data.isSalesRep = value;
        },
        getError: () => errors?.isSalesRep,
        setError: (value: Array<string> | undefined) => {
          errors.isSalesRep = value;
        },
        getTainted: () => tainted?.isSalesRep ?? false,
        setTainted: (value: boolean) => {
          tainted.isSalesRep = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "isSalesRep")
              .map((e) => e.message);
          }
          return [];
        },
      },
      description: {
        path: ["description"] as const,
        name: "description",
        constraints: { required: true },

        get: () => data.description,
        set: (value: string | null) => {
          data.description = value;
        },
        getError: () => errors?.description,
        setError: (value: Array<string> | undefined) => {
          errors.description = value;
        },
        getTainted: () => tainted?.description ?? false,
        setTainted: (value: boolean) => {
          tainted.description = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "description")
              .map((e) => e.message);
          }
          return [];
        },
      },
      linkedinUrl: {
        path: ["linkedinUrl"] as const,
        name: "linkedinUrl",
        constraints: { required: true },

        get: () => data.linkedinUrl,
        set: (value: string | null) => {
          data.linkedinUrl = value;
        },
        getError: () => errors?.linkedinUrl,
        setError: (value: Array<string> | undefined) => {
          errors.linkedinUrl = value;
        },
        getTainted: () => tainted?.linkedinUrl ?? false,
        setTainted: (value: boolean) => {
          tainted.linkedinUrl = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "linkedinUrl")
              .map((e) => e.message);
          }
          return [];
        },
      },
      attendance: {
        path: ["attendance"] as const,
        name: "attendance",
        constraints: { required: true },

        get: () => data.attendance,
        set: (value: string[]) => {
          data.attendance = value;
        },
        getError: () => errors?.attendance,
        setError: (value: Array<string> | undefined) => {
          errors.attendance = value;
        },
        getTainted: () => tainted?.attendance ?? false,
        setTainted: (value: boolean) => {
          tainted.attendance = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "attendance")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["attendance", index] as const,
          name: `attendance.${index}`,
          constraints: { required: true },
          get: () => data.attendance[index],
          set: (value: string) => {
            data.attendance[index] = value;
          },
          getError: () =>
            (errors.attendance as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.attendance ??= {};
            (errors.attendance as Record<number, Array<string>>)[index] =
              value!;
          },
          getTainted: () => tainted.attendance?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.attendance ??= {};
            tainted.attendance[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: string) => {
          data.attendance.push(item);
        },
        remove: (index: number) => {
          data.attendance.splice(index, 1);
        },
        swap: (a: number, b: number) => {
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

        get: () => data.settings,
        set: (value: Settings) => {
          data.settings = value;
        },
        getError: () => errors?.settings,
        setError: (value: Array<string> | undefined) => {
          errors.settings = value;
        },
        getTainted: () => tainted?.settings ?? false,
        setTainted: (value: boolean) => {
          tainted.settings = value;
        },
        validate: (): Array<string> => {
          const result = Employee.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "settings")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Employee,
      Array<{ field: string; message: string }>
    > {
      return Employee.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Employee>): void {
      data = { ...Employee.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Employee, Array<{ field: string; message: string }>> {
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
    return Employee.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Commissions, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Commissions.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Commissions.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("technician" in obj)) {
      errors.push({ field: "technician", message: "missing required field" });
    }
    if (!("salesRep" in obj)) {
      errors.push({ field: "salesRep", message: "missing required field" });
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
      const __raw_technician = obj["technician"];
      instance.technician = __raw_technician;
    }
    {
      const __raw_salesRep = obj["salesRep"];
      instance.salesRep = __raw_salesRep;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as Commissions;
  }
}

export namespace Commissions {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    technician?: Array<string>;
    salesRep?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { technician?: boolean; salesRep?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly technician: FieldController<string>;
    readonly salesRep: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Commissions;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Commissions, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Commissions>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Commissions>,
  ): Gigaform {
    let data = $state({ ...Commissions.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      technician: {
        path: ["technician"] as const,
        name: "technician",
        constraints: { required: true },

        get: () => data.technician,
        set: (value: string) => {
          data.technician = value;
        },
        getError: () => errors?.technician,
        setError: (value: Array<string> | undefined) => {
          errors.technician = value;
        },
        getTainted: () => tainted?.technician ?? false,
        setTainted: (value: boolean) => {
          tainted.technician = value;
        },
        validate: (): Array<string> => {
          const result = Commissions.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "technician")
              .map((e) => e.message);
          }
          return [];
        },
      },
      salesRep: {
        path: ["salesRep"] as const,
        name: "salesRep",
        constraints: { required: true },

        get: () => data.salesRep,
        set: (value: string) => {
          data.salesRep = value;
        },
        getError: () => errors?.salesRep,
        setError: (value: Array<string> | undefined) => {
          errors.salesRep = value;
        },
        getTainted: () => tainted?.salesRep ?? false,
        setTainted: (value: boolean) => {
          tainted.salesRep = value;
        },
        validate: (): Array<string> => {
          const result = Commissions.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "salesRep")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Commissions,
      Array<{ field: string; message: string }>
    > {
      return Commissions.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Commissions>): void {
      data = { ...Commissions.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Commissions, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.technician = formData.get("technician") ?? "";
    obj.salesRep = formData.get("salesRep") ?? "";
    return Commissions.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Number, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Number.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "Number.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("countryCode" in obj)) {
      errors.push({ field: "countryCode", message: "missing required field" });
    }
    if (!("areaCode" in obj)) {
      errors.push({ field: "areaCode", message: "missing required field" });
    }
    if (!("localNumber" in obj)) {
      errors.push({ field: "localNumber", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Number;
  }
}

export namespace Number {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    countryCode?: Array<string>;
    areaCode?: Array<string>;
    localNumber?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { countryCode?: boolean; areaCode?: boolean; localNumber?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly countryCode: FieldController<string>;
    readonly areaCode: FieldController<string>;
    readonly localNumber: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Number;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Number, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Number>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Number>,
  ): Gigaform {
    let data = $state({ ...Number.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      countryCode: {
        path: ["countryCode"] as const,
        name: "countryCode",
        constraints: { required: true },

        get: () => data.countryCode,
        set: (value: string) => {
          data.countryCode = value;
        },
        getError: () => errors?.countryCode,
        setError: (value: Array<string> | undefined) => {
          errors.countryCode = value;
        },
        getTainted: () => tainted?.countryCode ?? false,
        setTainted: (value: boolean) => {
          tainted.countryCode = value;
        },
        validate: (): Array<string> => {
          const result = Number.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "countryCode")
              .map((e) => e.message);
          }
          return [];
        },
      },
      areaCode: {
        path: ["areaCode"] as const,
        name: "areaCode",
        constraints: { required: true },

        get: () => data.areaCode,
        set: (value: string) => {
          data.areaCode = value;
        },
        getError: () => errors?.areaCode,
        setError: (value: Array<string> | undefined) => {
          errors.areaCode = value;
        },
        getTainted: () => tainted?.areaCode ?? false,
        setTainted: (value: boolean) => {
          tainted.areaCode = value;
        },
        validate: (): Array<string> => {
          const result = Number.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "areaCode")
              .map((e) => e.message);
          }
          return [];
        },
      },
      localNumber: {
        path: ["localNumber"] as const,
        name: "localNumber",
        constraints: { required: true },

        get: () => data.localNumber,
        set: (value: string) => {
          data.localNumber = value;
        },
        getError: () => errors?.localNumber,
        setError: (value: Array<string> | undefined) => {
          errors.localNumber = value;
        },
        getTainted: () => tainted?.localNumber ?? false,
        setTainted: (value: boolean) => {
          tainted.localNumber = value;
        },
        validate: (): Array<string> => {
          const result = Number.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "localNumber")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Number,
      Array<{ field: string; message: string }>
    > {
      return Number.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Number>): void {
      data = { ...Number.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Number, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.countryCode = formData.get("countryCode") ?? "";
    obj.areaCode = formData.get("areaCode") ?? "";
    obj.localNumber = formData.get("localNumber") ?? "";
    return Number.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<DataPath, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "DataPath.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "DataPath.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("path" in obj)) {
      errors.push({ field: "path", message: "missing required field" });
    }
    if (!("formatter" in obj)) {
      errors.push({ field: "formatter", message: "missing required field" });
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
      const __raw_path = obj["path"];
      instance.path = __raw_path;
    }
    {
      const __raw_formatter = obj["formatter"];
      instance.formatter = __raw_formatter;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as DataPath;
  }
}

export namespace DataPath {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    path?: { _errors?: Array<string>; [index: number]: Array<string> };
    formatter?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { path?: { [index: number]: boolean }; formatter?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly path: FieldController<string[]>;
    readonly formatter: FieldController<string | null>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: DataPath;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<DataPath, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<DataPath>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<DataPath>,
  ): Gigaform {
    let data = $state({ ...DataPath.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      path: {
        path: ["path"] as const,
        name: "path",
        constraints: { required: true },

        get: () => data.path,
        set: (value: string[]) => {
          data.path = value;
        },
        getError: () => errors?.path,
        setError: (value: Array<string> | undefined) => {
          errors.path = value;
        },
        getTainted: () => tainted?.path ?? false,
        setTainted: (value: boolean) => {
          tainted.path = value;
        },
        validate: (): Array<string> => {
          const result = DataPath.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "path")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["path", index] as const,
          name: `path.${index}`,
          constraints: { required: true },
          get: () => data.path[index],
          set: (value: string) => {
            data.path[index] = value;
          },
          getError: () =>
            (errors.path as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.path ??= {};
            (errors.path as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.path?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.path ??= {};
            tainted.path[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: string) => {
          data.path.push(item);
        },
        remove: (index: number) => {
          data.path.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.path[a], data.path[b]] = [data.path[b], data.path[a]];
        },
      },
      formatter: {
        path: ["formatter"] as const,
        name: "formatter",
        constraints: { required: true },

        get: () => data.formatter,
        set: (value: string | null) => {
          data.formatter = value;
        },
        getError: () => errors?.formatter,
        setError: (value: Array<string> | undefined) => {
          errors.formatter = value;
        },
        getTainted: () => tainted?.formatter ?? false,
        setTainted: (value: boolean) => {
          tainted.formatter = value;
        },
        validate: (): Array<string> => {
          const result = DataPath.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "formatter")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      DataPath,
      Array<{ field: string; message: string }>
    > {
      return DataPath.fromJSON(data);
    }
    function reset(newOverrides?: Partial<DataPath>): void {
      data = { ...DataPath.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<DataPath, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.path = formData.getAll("path") as Array<string>;
    obj.formatter = formData.get("formatter") ?? "";
    return DataPath.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Route, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Route.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "Route.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("techs" in obj)) {
      errors.push({ field: "techs", message: "missing required field" });
    }
    if (!("active" in obj)) {
      errors.push({ field: "active", message: "missing required field" });
    }
    if (!("name" in obj)) {
      errors.push({ field: "name", message: "missing required field" });
    }
    if (!("phone" in obj)) {
      errors.push({ field: "phone", message: "missing required field" });
    }
    if (!("position" in obj)) {
      errors.push({ field: "position", message: "missing required field" });
    }
    if (!("serviceRoute" in obj)) {
      errors.push({ field: "serviceRoute", message: "missing required field" });
    }
    if (!("defaultDurationHours" in obj)) {
      errors.push({
        field: "defaultDurationHours",
        message: "missing required field",
      });
    }
    if (!("tags" in obj)) {
      errors.push({ field: "tags", message: "missing required field" });
    }
    if (!("icon" in obj)) {
      errors.push({ field: "icon", message: "missing required field" });
    }
    if (!("color" in obj)) {
      errors.push({ field: "color", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Route;
  }
}

export namespace Route {
  /** Nested error structure matching the data shape */ export type Errors = {
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
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
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
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly techs: FieldController<(string | Employee)[] | null>;
    readonly active: FieldController<boolean>;
    readonly name: FieldController<string>;
    readonly phone: FieldController<string>;
    readonly position: FieldController<string>;
    readonly serviceRoute: FieldController<boolean>;
    readonly defaultDurationHours: FieldController<number>;
    readonly tags: FieldController<string[]>;
    readonly icon: FieldController<string | null>;
    readonly color: FieldController<string | null>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Route;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Route, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Route>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Route>,
  ): Gigaform {
    let data = $state({ ...Route.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Route.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      techs: {
        path: ["techs"] as const,
        name: "techs",
        constraints: { required: true },

        get: () => data.techs,
        set: (value: (string | Employee)[] | null) => {
          data.techs = value;
        },
        getError: () => errors?.techs,
        setError: (value: Array<string> | undefined) => {
          errors.techs = value;
        },
        getTainted: () => tainted?.techs ?? false,
        setTainted: (value: boolean) => {
          tainted.techs = value;
        },
        validate: (): Array<string> => {
          const result = Route.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "techs")
              .map((e) => e.message);
          }
          return [];
        },
      },
      active: {
        path: ["active"] as const,
        name: "active",
        constraints: { required: true },

        get: () => data.active,
        set: (value: boolean) => {
          data.active = value;
        },
        getError: () => errors?.active,
        setError: (value: Array<string> | undefined) => {
          errors.active = value;
        },
        getTainted: () => tainted?.active ?? false,
        setTainted: (value: boolean) => {
          tainted.active = value;
        },
        validate: (): Array<string> => {
          const result = Route.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "active")
              .map((e) => e.message);
          }
          return [];
        },
      },
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: () => data.name,
        set: (value: string) => {
          data.name = value;
        },
        getError: () => errors?.name,
        setError: (value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: () => tainted?.name ?? false,
        setTainted: (value: boolean) => {
          tainted.name = value;
        },
        validate: (): Array<string> => {
          const result = Route.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "name")
              .map((e) => e.message);
          }
          return [];
        },
      },
      phone: {
        path: ["phone"] as const,
        name: "phone",
        constraints: { required: true },

        get: () => data.phone,
        set: (value: string) => {
          data.phone = value;
        },
        getError: () => errors?.phone,
        setError: (value: Array<string> | undefined) => {
          errors.phone = value;
        },
        getTainted: () => tainted?.phone ?? false,
        setTainted: (value: boolean) => {
          tainted.phone = value;
        },
        validate: (): Array<string> => {
          const result = Route.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "phone")
              .map((e) => e.message);
          }
          return [];
        },
      },
      position: {
        path: ["position"] as const,
        name: "position",
        constraints: { required: true },

        get: () => data.position,
        set: (value: string) => {
          data.position = value;
        },
        getError: () => errors?.position,
        setError: (value: Array<string> | undefined) => {
          errors.position = value;
        },
        getTainted: () => tainted?.position ?? false,
        setTainted: (value: boolean) => {
          tainted.position = value;
        },
        validate: (): Array<string> => {
          const result = Route.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "position")
              .map((e) => e.message);
          }
          return [];
        },
      },
      serviceRoute: {
        path: ["serviceRoute"] as const,
        name: "serviceRoute",
        constraints: { required: true },

        get: () => data.serviceRoute,
        set: (value: boolean) => {
          data.serviceRoute = value;
        },
        getError: () => errors?.serviceRoute,
        setError: (value: Array<string> | undefined) => {
          errors.serviceRoute = value;
        },
        getTainted: () => tainted?.serviceRoute ?? false,
        setTainted: (value: boolean) => {
          tainted.serviceRoute = value;
        },
        validate: (): Array<string> => {
          const result = Route.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "serviceRoute")
              .map((e) => e.message);
          }
          return [];
        },
      },
      defaultDurationHours: {
        path: ["defaultDurationHours"] as const,
        name: "defaultDurationHours",
        constraints: { required: true },

        get: () => data.defaultDurationHours,
        set: (value: number) => {
          data.defaultDurationHours = value;
        },
        getError: () => errors?.defaultDurationHours,
        setError: (value: Array<string> | undefined) => {
          errors.defaultDurationHours = value;
        },
        getTainted: () => tainted?.defaultDurationHours ?? false,
        setTainted: (value: boolean) => {
          tainted.defaultDurationHours = value;
        },
        validate: (): Array<string> => {
          const result = Route.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "defaultDurationHours")
              .map((e) => e.message);
          }
          return [];
        },
      },
      tags: {
        path: ["tags"] as const,
        name: "tags",
        constraints: { required: true },

        get: () => data.tags,
        set: (value: string[]) => {
          data.tags = value;
        },
        getError: () => errors?.tags,
        setError: (value: Array<string> | undefined) => {
          errors.tags = value;
        },
        getTainted: () => tainted?.tags ?? false,
        setTainted: (value: boolean) => {
          tainted.tags = value;
        },
        validate: (): Array<string> => {
          const result = Route.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "tags")
              .map((e) => e.message);
          }
          return [];
        },
        at: (index: number) => ({
          path: ["tags", index] as const,
          name: `tags.${index}`,
          constraints: { required: true },
          get: () => data.tags[index],
          set: (value: string) => {
            data.tags[index] = value;
          },
          getError: () =>
            (errors.tags as Record<number, Array<string>>)?.[index],
          setError: (value: Array<string> | undefined) => {
            errors.tags ??= {};
            (errors.tags as Record<number, Array<string>>)[index] = value!;
          },
          getTainted: () => tainted.tags?.[index] ?? false,
          setTainted: (value: boolean) => {
            tainted.tags ??= {};
            tainted.tags[index] = value;
          },
          validate: (): Array<string> => [],
        }),
        push: (item: string) => {
          data.tags.push(item);
        },
        remove: (index: number) => {
          data.tags.splice(index, 1);
        },
        swap: (a: number, b: number) => {
          [data.tags[a], data.tags[b]] = [data.tags[b], data.tags[a]];
        },
      },
      icon: {
        path: ["icon"] as const,
        name: "icon",
        constraints: { required: true },

        get: () => data.icon,
        set: (value: string | null) => {
          data.icon = value;
        },
        getError: () => errors?.icon,
        setError: (value: Array<string> | undefined) => {
          errors.icon = value;
        },
        getTainted: () => tainted?.icon ?? false,
        setTainted: (value: boolean) => {
          tainted.icon = value;
        },
        validate: (): Array<string> => {
          const result = Route.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "icon")
              .map((e) => e.message);
          }
          return [];
        },
      },
      color: {
        path: ["color"] as const,
        name: "color",
        constraints: { required: true },

        get: () => data.color,
        set: (value: string | null) => {
          data.color = value;
        },
        getError: () => errors?.color,
        setError: (value: Array<string> | undefined) => {
          errors.color = value;
        },
        getTainted: () => tainted?.color ?? false,
        setTainted: (value: boolean) => {
          tainted.color = value;
        },
        validate: (): Array<string> => {
          const result = Route.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "color")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Route,
      Array<{ field: string; message: string }>
    > {
      return Route.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Route>): void {
      data = { ...Route.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Route, Array<{ field: string; message: string }>> {
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
    return Route.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<EmailParts, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "EmailParts.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "EmailParts.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("local" in obj)) {
      errors.push({ field: "local", message: "missing required field" });
    }
    if (!("domainName" in obj)) {
      errors.push({ field: "domainName", message: "missing required field" });
    }
    if (!("topLevelDomain" in obj)) {
      errors.push({
        field: "topLevelDomain",
        message: "missing required field",
      });
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
      throw new DeserializeError(errors);
    }
    return instance as EmailParts;
  }
}

export namespace EmailParts {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    local?: Array<string>;
    domainName?: Array<string>;
    topLevelDomain?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { local?: boolean; domainName?: boolean; topLevelDomain?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly local: FieldController<string>;
    readonly domainName: FieldController<string>;
    readonly topLevelDomain: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: EmailParts;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<EmailParts, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<EmailParts>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<EmailParts>,
  ): Gigaform {
    let data = $state({ ...EmailParts.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      local: {
        path: ["local"] as const,
        name: "local",
        constraints: { required: true },

        get: () => data.local,
        set: (value: string) => {
          data.local = value;
        },
        getError: () => errors?.local,
        setError: (value: Array<string> | undefined) => {
          errors.local = value;
        },
        getTainted: () => tainted?.local ?? false,
        setTainted: (value: boolean) => {
          tainted.local = value;
        },
        validate: (): Array<string> => {
          const result = EmailParts.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "local")
              .map((e) => e.message);
          }
          return [];
        },
      },
      domainName: {
        path: ["domainName"] as const,
        name: "domainName",
        constraints: { required: true },

        get: () => data.domainName,
        set: (value: string) => {
          data.domainName = value;
        },
        getError: () => errors?.domainName,
        setError: (value: Array<string> | undefined) => {
          errors.domainName = value;
        },
        getTainted: () => tainted?.domainName ?? false,
        setTainted: (value: boolean) => {
          tainted.domainName = value;
        },
        validate: (): Array<string> => {
          const result = EmailParts.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "domainName")
              .map((e) => e.message);
          }
          return [];
        },
      },
      topLevelDomain: {
        path: ["topLevelDomain"] as const,
        name: "topLevelDomain",
        constraints: { required: true },

        get: () => data.topLevelDomain,
        set: (value: string) => {
          data.topLevelDomain = value;
        },
        getError: () => errors?.topLevelDomain,
        setError: (value: Array<string> | undefined) => {
          errors.topLevelDomain = value;
        },
        getTainted: () => tainted?.topLevelDomain ?? false,
        setTainted: (value: boolean) => {
          tainted.topLevelDomain = value;
        },
        validate: (): Array<string> => {
          const result = EmailParts.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "topLevelDomain")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      EmailParts,
      Array<{ field: string; message: string }>
    > {
      return EmailParts.fromJSON(data);
    }
    function reset(newOverrides?: Partial<EmailParts>): void {
      data = { ...EmailParts.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<EmailParts, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.local = formData.get("local") ?? "";
    obj.domainName = formData.get("domainName") ?? "";
    obj.topLevelDomain = formData.get("topLevelDomain") ?? "";
    return EmailParts.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Sent, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Sent.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "Sent.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("recipient" in obj)) {
      errors.push({ field: "recipient", message: "missing required field" });
    }
    if (!("method" in obj)) {
      errors.push({ field: "method", message: "missing required field" });
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
      const __raw_recipient = obj["recipient"];
      instance.recipient = __raw_recipient;
    }
    {
      const __raw_method = obj["method"];
      instance.method = __raw_method;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as Sent;
  }
}

export namespace Sent {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    recipient?: Array<string>;
    method?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { recipient?: boolean; method?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly recipient: FieldController<string | null>;
    readonly method: FieldController<string | null>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Sent;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Sent, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Sent>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Sent>,
  ): Gigaform {
    let data = $state({ ...Sent.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      recipient: {
        path: ["recipient"] as const,
        name: "recipient",
        constraints: { required: true },

        get: () => data.recipient,
        set: (value: string | null) => {
          data.recipient = value;
        },
        getError: () => errors?.recipient,
        setError: (value: Array<string> | undefined) => {
          errors.recipient = value;
        },
        getTainted: () => tainted?.recipient ?? false,
        setTainted: (value: boolean) => {
          tainted.recipient = value;
        },
        validate: (): Array<string> => {
          const result = Sent.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "recipient")
              .map((e) => e.message);
          }
          return [];
        },
      },
      method: {
        path: ["method"] as const,
        name: "method",
        constraints: { required: true },

        get: () => data.method,
        set: (value: string | null) => {
          data.method = value;
        },
        getError: () => errors?.method,
        setError: (value: Array<string> | undefined) => {
          errors.method = value;
        },
        getTainted: () => tainted?.method ?? false,
        setTainted: (value: boolean) => {
          tainted.method = value;
        },
        validate: (): Array<string> => {
          const result = Sent.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "method")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Sent,
      Array<{ field: string; message: string }>
    > {
      return Sent.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Sent>): void {
      data = { ...Sent.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Sent, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.recipient = formData.get("recipient") ?? "";
    obj.method = formData.get("method") ?? "";
    return Sent.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): BilledItem {
    return { item: "", quantity: 0, taxed: false, upsale: false } as BilledItem;
  }
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
  ): Result<BilledItem, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "BilledItem.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "BilledItem.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("item" in obj)) {
      errors.push({ field: "item", message: "missing required field" });
    }
    if (!("quantity" in obj)) {
      errors.push({ field: "quantity", message: "missing required field" });
    }
    if (!("taxed" in obj)) {
      errors.push({ field: "taxed", message: "missing required field" });
    }
    if (!("upsale" in obj)) {
      errors.push({ field: "upsale", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as BilledItem;
  }
}

export namespace BilledItem {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    item?: Item.Errors;
    quantity?: Array<string>;
    taxed?: Array<string>;
    upsale?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
      item?: Item.Tainted;
      quantity?: boolean;
      taxed?: boolean;
      upsale?: boolean;
    };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly item: FieldController<Item>;
    readonly quantity: FieldController<number>;
    readonly taxed: FieldController<boolean>;
    readonly upsale: FieldController<boolean>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: BilledItem;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<BilledItem, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<BilledItem>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<BilledItem>,
  ): Gigaform {
    let data = $state({ ...BilledItem.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      item: {
        path: ["item"] as const,
        name: "item",
        constraints: { required: true },

        get: () => data.item,
        set: (value: Item) => {
          data.item = value;
        },
        getError: () => errors?.item,
        setError: (value: Array<string> | undefined) => {
          errors.item = value;
        },
        getTainted: () => tainted?.item ?? false,
        setTainted: (value: boolean) => {
          tainted.item = value;
        },
        validate: (): Array<string> => {
          const result = BilledItem.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "item")
              .map((e) => e.message);
          }
          return [];
        },
      },
      quantity: {
        path: ["quantity"] as const,
        name: "quantity",
        constraints: { required: true },

        get: () => data.quantity,
        set: (value: number) => {
          data.quantity = value;
        },
        getError: () => errors?.quantity,
        setError: (value: Array<string> | undefined) => {
          errors.quantity = value;
        },
        getTainted: () => tainted?.quantity ?? false,
        setTainted: (value: boolean) => {
          tainted.quantity = value;
        },
        validate: (): Array<string> => {
          const result = BilledItem.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "quantity")
              .map((e) => e.message);
          }
          return [];
        },
      },
      taxed: {
        path: ["taxed"] as const,
        name: "taxed",
        constraints: { required: true },

        get: () => data.taxed,
        set: (value: boolean) => {
          data.taxed = value;
        },
        getError: () => errors?.taxed,
        setError: (value: Array<string> | undefined) => {
          errors.taxed = value;
        },
        getTainted: () => tainted?.taxed ?? false,
        setTainted: (value: boolean) => {
          tainted.taxed = value;
        },
        validate: (): Array<string> => {
          const result = BilledItem.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "taxed")
              .map((e) => e.message);
          }
          return [];
        },
      },
      upsale: {
        path: ["upsale"] as const,
        name: "upsale",
        constraints: { required: true },

        get: () => data.upsale,
        set: (value: boolean) => {
          data.upsale = value;
        },
        getError: () => errors?.upsale,
        setError: (value: Array<string> | undefined) => {
          errors.upsale = value;
        },
        getTainted: () => tainted?.upsale ?? false,
        setTainted: (value: boolean) => {
          tainted.upsale = value;
        },
        validate: (): Array<string> => {
          const result = BilledItem.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "upsale")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      BilledItem,
      Array<{ field: string; message: string }>
    > {
      return BilledItem.fromJSON(data);
    }
    function reset(newOverrides?: Partial<BilledItem>): void {
      data = { ...BilledItem.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<BilledItem, Array<{ field: string; message: string }>> {
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
    return BilledItem.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Coordinates, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Coordinates.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Coordinates.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("lat" in obj)) {
      errors.push({ field: "lat", message: "missing required field" });
    }
    if (!("lng" in obj)) {
      errors.push({ field: "lng", message: "missing required field" });
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
      const __raw_lat = obj["lat"];
      instance.lat = __raw_lat;
    }
    {
      const __raw_lng = obj["lng"];
      instance.lng = __raw_lng;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as Coordinates;
  }
}

export namespace Coordinates {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    lat?: Array<string>;
    lng?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { lat?: boolean; lng?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly lat: FieldController<number>;
    readonly lng: FieldController<number>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Coordinates;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Coordinates, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Coordinates>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Coordinates>,
  ): Gigaform {
    let data = $state({ ...Coordinates.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      lat: {
        path: ["lat"] as const,
        name: "lat",
        constraints: { required: true },

        get: () => data.lat,
        set: (value: number) => {
          data.lat = value;
        },
        getError: () => errors?.lat,
        setError: (value: Array<string> | undefined) => {
          errors.lat = value;
        },
        getTainted: () => tainted?.lat ?? false,
        setTainted: (value: boolean) => {
          tainted.lat = value;
        },
        validate: (): Array<string> => {
          const result = Coordinates.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "lat")
              .map((e) => e.message);
          }
          return [];
        },
      },
      lng: {
        path: ["lng"] as const,
        name: "lng",
        constraints: { required: true },

        get: () => data.lng,
        set: (value: number) => {
          data.lng = value;
        },
        getError: () => errors?.lng,
        setError: (value: Array<string> | undefined) => {
          errors.lng = value;
        },
        getTainted: () => tainted?.lng ?? false,
        setTainted: (value: boolean) => {
          tainted.lng = value;
        },
        validate: (): Array<string> => {
          const result = Coordinates.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "lng")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Coordinates,
      Array<{ field: string; message: string }>
    > {
      return Coordinates.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Coordinates>): void {
      data = { ...Coordinates.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Coordinates, Array<{ field: string; message: string }>> {
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
    return Coordinates.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): Ordered {
    return { id: "", in: "", out: "", date: "" } as Ordered;
  }
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
  ): Result<Ordered, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Ordered.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Ordered.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("id" in obj)) {
      errors.push({ field: "id", message: "missing required field" });
    }
    if (!("in" in obj)) {
      errors.push({ field: "in", message: "missing required field" });
    }
    if (!("out" in obj)) {
      errors.push({ field: "out", message: "missing required field" });
    }
    if (!("date" in obj)) {
      errors.push({ field: "date", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Ordered;
  }
}

export namespace Ordered {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    id?: Array<string>;
    in?: Array<string>;
    out?: Array<string>;
    date?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { id?: boolean; in?: boolean; out?: boolean; date?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly id: FieldController<string>;
    readonly in: FieldController<string | Account>;
    readonly out: FieldController<string | Order>;
    readonly date: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Ordered;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Ordered, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Ordered>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Ordered>,
  ): Gigaform {
    let data = $state({ ...Ordered.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      id: {
        path: ["id"] as const,
        name: "id",
        constraints: { required: true },

        get: () => data.id,
        set: (value: string) => {
          data.id = value;
        },
        getError: () => errors?.id,
        setError: (value: Array<string> | undefined) => {
          errors.id = value;
        },
        getTainted: () => tainted?.id ?? false,
        setTainted: (value: boolean) => {
          tainted.id = value;
        },
        validate: (): Array<string> => {
          const result = Ordered.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "id")
              .map((e) => e.message);
          }
          return [];
        },
      },
      in: {
        path: ["in"] as const,
        name: "in",
        constraints: { required: true },

        get: () => data.in,
        set: (value: string | Account) => {
          data.in = value;
        },
        getError: () => errors?.in,
        setError: (value: Array<string> | undefined) => {
          errors.in = value;
        },
        getTainted: () => tainted?.in ?? false,
        setTainted: (value: boolean) => {
          tainted.in = value;
        },
        validate: (): Array<string> => {
          const result = Ordered.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "in")
              .map((e) => e.message);
          }
          return [];
        },
      },
      out: {
        path: ["out"] as const,
        name: "out",
        constraints: { required: true },

        get: () => data.out,
        set: (value: string | Order) => {
          data.out = value;
        },
        getError: () => errors?.out,
        setError: (value: Array<string> | undefined) => {
          errors.out = value;
        },
        getTainted: () => tainted?.out ?? false,
        setTainted: (value: boolean) => {
          tainted.out = value;
        },
        validate: (): Array<string> => {
          const result = Ordered.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "out")
              .map((e) => e.message);
          }
          return [];
        },
      },
      date: {
        path: ["date"] as const,
        name: "date",
        constraints: { required: true },

        get: () => data.date,
        set: (value: string) => {
          data.date = value;
        },
        getError: () => errors?.date,
        setError: (value: Array<string> | undefined) => {
          errors.date = value;
        },
        getTainted: () => tainted?.date ?? false,
        setTainted: (value: boolean) => {
          tainted.date = value;
        },
        validate: (): Array<string> => {
          const result = Ordered.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "date")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Ordered,
      Array<{ field: string; message: string }>
    > {
      return Ordered.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Ordered>): void {
      data = { ...Ordered.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Ordered, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get("id") ?? "";
    obj.in = formData.get("in") ?? "";
    obj.out = formData.get("out") ?? "";
    obj.date = formData.get("date") ?? "";
    return Ordered.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Email, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Email.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        { field: "_root", message: "Email.__deserialize: expected an object" },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("canEmail" in obj)) {
      errors.push({ field: "canEmail", message: "missing required field" });
    }
    if (!("emailString" in obj)) {
      errors.push({ field: "emailString", message: "missing required field" });
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
      const __raw_canEmail = obj["canEmail"];
      instance.canEmail = __raw_canEmail;
    }
    {
      const __raw_emailString = obj["emailString"];
      instance.emailString = __raw_emailString;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as Email;
  }
}

export namespace Email {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    canEmail?: Array<string>;
    emailString?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { canEmail?: boolean; emailString?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly canEmail: FieldController<boolean>;
    readonly emailString: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Email;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Email, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Email>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Email>,
  ): Gigaform {
    let data = $state({ ...Email.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      canEmail: {
        path: ["canEmail"] as const,
        name: "canEmail",
        constraints: { required: true },

        get: () => data.canEmail,
        set: (value: boolean) => {
          data.canEmail = value;
        },
        getError: () => errors?.canEmail,
        setError: (value: Array<string> | undefined) => {
          errors.canEmail = value;
        },
        getTainted: () => tainted?.canEmail ?? false,
        setTainted: (value: boolean) => {
          tainted.canEmail = value;
        },
        validate: (): Array<string> => {
          const result = Email.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "canEmail")
              .map((e) => e.message);
          }
          return [];
        },
      },
      emailString: {
        path: ["emailString"] as const,
        name: "emailString",
        constraints: { required: true },

        get: () => data.emailString,
        set: (value: string) => {
          data.emailString = value;
        },
        getError: () => errors?.emailString,
        setError: (value: Array<string> | undefined) => {
          errors.emailString = value;
        },
        getTainted: () => tainted?.emailString ?? false,
        setTainted: (value: boolean) => {
          tainted.emailString = value;
        },
        validate: (): Array<string> => {
          const result = Email.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "emailString")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Email,
      Array<{ field: string; message: string }>
    > {
      return Email.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Email>): void {
      data = { ...Email.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Email, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
      const canEmailVal = formData.get("canEmail");
      obj.canEmail =
        canEmailVal === "true" || canEmailVal === "on" || canEmailVal === "1";
    }
    obj.emailString = formData.get("emailString") ?? "";
    return Email.fromStringifiedJSON(JSON.stringify(obj));
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
  export function defaultValue(): RecurrenceRule {
    return {
      interval: DailyRecurrenceRule.defaultValue(),
      recurrenceBegins: "",
      recurrenceEnds: null,
      cancelledInstances: null,
      additionalInstances: null,
    } as RecurrenceRule;
  }
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
  ): Result<RecurrenceRule, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "RecurrenceRule.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "RecurrenceRule.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("interval" in obj)) {
      errors.push({ field: "interval", message: "missing required field" });
    }
    if (!("recurrenceBegins" in obj)) {
      errors.push({
        field: "recurrenceBegins",
        message: "missing required field",
      });
    }
    if (!("recurrenceEnds" in obj)) {
      errors.push({
        field: "recurrenceEnds",
        message: "missing required field",
      });
    }
    if (!("cancelledInstances" in obj)) {
      errors.push({
        field: "cancelledInstances",
        message: "missing required field",
      });
    }
    if (!("additionalInstances" in obj)) {
      errors.push({
        field: "additionalInstances",
        message: "missing required field",
      });
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
      throw new DeserializeError(errors);
    }
    return instance as RecurrenceRule;
  }
}

export namespace RecurrenceRule {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    interval?: Interval.Errors;
    recurrenceBegins?: Array<string>;
    recurrenceEnds?: Array<string>;
    cancelledInstances?: Array<string>;
    additionalInstances?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    {
      interval?: Interval.Tainted;
      recurrenceBegins?: boolean;
      recurrenceEnds?: boolean;
      cancelledInstances?: boolean;
      additionalInstances?: boolean;
    };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly interval: FieldController<Interval>;
    readonly recurrenceBegins: FieldController<string>;
    readonly recurrenceEnds: FieldController<RecurrenceEnd | null>;
    readonly cancelledInstances: FieldController<string[] | null>;
    readonly additionalInstances: FieldController<string[] | null>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: RecurrenceRule;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<
      RecurrenceRule,
      Array<{ field: string; message: string }>
    >;
    reset(overrides?: Partial<RecurrenceRule>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<RecurrenceRule>,
  ): Gigaform {
    let data = $state({ ...RecurrenceRule.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      interval: {
        path: ["interval"] as const,
        name: "interval",
        constraints: { required: true },

        get: () => data.interval,
        set: (value: Interval) => {
          data.interval = value;
        },
        getError: () => errors?.interval,
        setError: (value: Array<string> | undefined) => {
          errors.interval = value;
        },
        getTainted: () => tainted?.interval ?? false,
        setTainted: (value: boolean) => {
          tainted.interval = value;
        },
        validate: (): Array<string> => {
          const result = RecurrenceRule.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "interval")
              .map((e) => e.message);
          }
          return [];
        },
      },
      recurrenceBegins: {
        path: ["recurrenceBegins"] as const,
        name: "recurrenceBegins",
        constraints: { required: true },

        get: () => data.recurrenceBegins,
        set: (value: string) => {
          data.recurrenceBegins = value;
        },
        getError: () => errors?.recurrenceBegins,
        setError: (value: Array<string> | undefined) => {
          errors.recurrenceBegins = value;
        },
        getTainted: () => tainted?.recurrenceBegins ?? false,
        setTainted: (value: boolean) => {
          tainted.recurrenceBegins = value;
        },
        validate: (): Array<string> => {
          const result = RecurrenceRule.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "recurrenceBegins")
              .map((e) => e.message);
          }
          return [];
        },
      },
      recurrenceEnds: {
        path: ["recurrenceEnds"] as const,
        name: "recurrenceEnds",
        constraints: { required: true },

        get: () => data.recurrenceEnds,
        set: (value: RecurrenceEnd | null) => {
          data.recurrenceEnds = value;
        },
        getError: () => errors?.recurrenceEnds,
        setError: (value: Array<string> | undefined) => {
          errors.recurrenceEnds = value;
        },
        getTainted: () => tainted?.recurrenceEnds ?? false,
        setTainted: (value: boolean) => {
          tainted.recurrenceEnds = value;
        },
        validate: (): Array<string> => {
          const result = RecurrenceRule.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "recurrenceEnds")
              .map((e) => e.message);
          }
          return [];
        },
      },
      cancelledInstances: {
        path: ["cancelledInstances"] as const,
        name: "cancelledInstances",
        constraints: { required: true },

        get: () => data.cancelledInstances,
        set: (value: string[] | null) => {
          data.cancelledInstances = value;
        },
        getError: () => errors?.cancelledInstances,
        setError: (value: Array<string> | undefined) => {
          errors.cancelledInstances = value;
        },
        getTainted: () => tainted?.cancelledInstances ?? false,
        setTainted: (value: boolean) => {
          tainted.cancelledInstances = value;
        },
        validate: (): Array<string> => {
          const result = RecurrenceRule.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "cancelledInstances")
              .map((e) => e.message);
          }
          return [];
        },
      },
      additionalInstances: {
        path: ["additionalInstances"] as const,
        name: "additionalInstances",
        constraints: { required: true },

        get: () => data.additionalInstances,
        set: (value: string[] | null) => {
          data.additionalInstances = value;
        },
        getError: () => errors?.additionalInstances,
        setError: (value: Array<string> | undefined) => {
          errors.additionalInstances = value;
        },
        getTainted: () => tainted?.additionalInstances ?? false,
        setTainted: (value: boolean) => {
          tainted.additionalInstances = value;
        },
        validate: (): Array<string> => {
          const result = RecurrenceRule.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "additionalInstances")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      RecurrenceRule,
      Array<{ field: string; message: string }>
    > {
      return RecurrenceRule.fromJSON(data);
    }
    function reset(newOverrides?: Partial<RecurrenceRule>): void {
      data = { ...RecurrenceRule.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<RecurrenceRule, Array<{ field: string; message: string }>> {
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
    return RecurrenceRule.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<LastName, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "LastName.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "LastName.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("name" in obj)) {
      errors.push({ field: "name", message: "missing required field" });
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
      const __raw_name = obj["name"];
      instance.name = __raw_name;
    }
    if (errors.length > 0) {
      throw new DeserializeError(errors);
    }
    return instance as LastName;
  }
}

export namespace LastName {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    name?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { name?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly name: FieldController<string>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: LastName;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<LastName, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<LastName>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<LastName>,
  ): Gigaform {
    let data = $state({ ...LastName.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      name: {
        path: ["name"] as const,
        name: "name",
        constraints: { required: true },

        get: () => data.name,
        set: (value: string) => {
          data.name = value;
        },
        getError: () => errors?.name,
        setError: (value: Array<string> | undefined) => {
          errors.name = value;
        },
        getTainted: () => tainted?.name ?? false,
        setTainted: (value: boolean) => {
          tainted.name = value;
        },
        validate: (): Array<string> => {
          const result = LastName.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "name")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      LastName,
      Array<{ field: string; message: string }>
    > {
      return LastName.fromJSON(data);
    }
    function reset(newOverrides?: Partial<LastName>): void {
      data = { ...LastName.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<LastName, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.name = formData.get("name") ?? "";
    return LastName.fromStringifiedJSON(JSON.stringify(obj));
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
  ): Result<Cardinal, Array<{ field: string; message: string }>> {
    try {
      const ctx = DeserializeContext.create();
      const raw = JSON.parse(json);
      const resultOrRef = __deserialize(raw, ctx);
      if (PendingRef.is(resultOrRef)) {
        return Result.err([
          {
            field: "_root",
            message:
              "Cardinal.fromStringifiedJSON: root cannot be a forward reference",
          },
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
      return Result.err([{ field: "_root", message }]);
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
      throw new DeserializeError([
        {
          field: "_root",
          message: "Cardinal.__deserialize: expected an object",
        },
      ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!("north" in obj)) {
      errors.push({ field: "north", message: "missing required field" });
    }
    if (!("east" in obj)) {
      errors.push({ field: "east", message: "missing required field" });
    }
    if (!("south" in obj)) {
      errors.push({ field: "south", message: "missing required field" });
    }
    if (!("west" in obj)) {
      errors.push({ field: "west", message: "missing required field" });
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
      throw new DeserializeError(errors);
    }
    return instance as Cardinal;
  }
}

export namespace Cardinal {
  /** Nested error structure matching the data shape */ export type Errors = {
    _errors?: Array<string>;
    north?: Array<string>;
    east?: Array<string>;
    south?: Array<string>;
    west?: Array<string>;
  };
  /** Nested boolean structure for tracking touched/dirty fields */ export type Tainted =
    { north?: boolean; east?: boolean; south?: boolean; west?: boolean };
  /** Field controller interface for a single field */ export interface FieldController<
    T,
  > {
    readonly path: ReadonlyArray<string | number>;
    readonly name: string;
    readonly constraints: Record<string, unknown>;
    readonly label?: string;
    readonly description?: string;
    readonly placeholder?: string;
    readonly disabled?: boolean;
    readonly readonly?: boolean;
    get(): T;
    set(value: T): void;
    getError(): Array<string> | undefined;
    setError(value: Array<string> | undefined): void;
    getTainted(): boolean;
    setTainted(value: boolean): void;
    validate(): Array<string>;
  }
  /** Type-safe field controllers for this form */ export interface FieldControllers {
    readonly north: FieldController<number>;
    readonly east: FieldController<number>;
    readonly south: FieldController<number>;
    readonly west: FieldController<number>;
  }
  /** Gigaform instance containing reactive state and field controllers */ export interface Gigaform {
    readonly data: Cardinal;
    readonly errors: Errors;
    readonly tainted: Tainted;
    readonly fields: FieldControllers;
    validate(): Result<Cardinal, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Cardinal>): void;
  }
  /** Creates a new Gigaform instance with reactive state and field controllers. */ export function createForm(
    overrides?: Partial<Cardinal>,
  ): Gigaform {
    let data = $state({ ...Cardinal.defaultValue(), ...overrides });
    let errors = $state<Errors>({});
    let tainted = $state<Tainted>({});
    const fields: FieldControllers = {
      north: {
        path: ["north"] as const,
        name: "north",
        constraints: { required: true },

        get: () => data.north,
        set: (value: number) => {
          data.north = value;
        },
        getError: () => errors?.north,
        setError: (value: Array<string> | undefined) => {
          errors.north = value;
        },
        getTainted: () => tainted?.north ?? false,
        setTainted: (value: boolean) => {
          tainted.north = value;
        },
        validate: (): Array<string> => {
          const result = Cardinal.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "north")
              .map((e) => e.message);
          }
          return [];
        },
      },
      east: {
        path: ["east"] as const,
        name: "east",
        constraints: { required: true },

        get: () => data.east,
        set: (value: number) => {
          data.east = value;
        },
        getError: () => errors?.east,
        setError: (value: Array<string> | undefined) => {
          errors.east = value;
        },
        getTainted: () => tainted?.east ?? false,
        setTainted: (value: boolean) => {
          tainted.east = value;
        },
        validate: (): Array<string> => {
          const result = Cardinal.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "east")
              .map((e) => e.message);
          }
          return [];
        },
      },
      south: {
        path: ["south"] as const,
        name: "south",
        constraints: { required: true },

        get: () => data.south,
        set: (value: number) => {
          data.south = value;
        },
        getError: () => errors?.south,
        setError: (value: Array<string> | undefined) => {
          errors.south = value;
        },
        getTainted: () => tainted?.south ?? false,
        setTainted: (value: boolean) => {
          tainted.south = value;
        },
        validate: (): Array<string> => {
          const result = Cardinal.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "south")
              .map((e) => e.message);
          }
          return [];
        },
      },
      west: {
        path: ["west"] as const,
        name: "west",
        constraints: { required: true },

        get: () => data.west,
        set: (value: number) => {
          data.west = value;
        },
        getError: () => errors?.west,
        setError: (value: Array<string> | undefined) => {
          errors.west = value;
        },
        getTainted: () => tainted?.west ?? false,
        setTainted: (value: boolean) => {
          tainted.west = value;
        },
        validate: (): Array<string> => {
          const result = Cardinal.fromJSON(data);
          if (result.isErr()) {
            const allErrors = result.unwrapErr();
            return allErrors
              .filter((e) => e.field === "west")
              .map((e) => e.message);
          }
          return [];
        },
      },
    };
    function validate(): Result<
      Cardinal,
      Array<{ field: string; message: string }>
    > {
      return Cardinal.fromJSON(data);
    }
    function reset(newOverrides?: Partial<Cardinal>): void {
      data = { ...Cardinal.defaultValue(), ...newOverrides };
      errors = {};
      tainted = {};
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
      reset,
    };
  }
  /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */ export function fromFormData(
    formData: FormData,
  ): Result<Cardinal, Array<{ field: string; message: string }>> {
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
    return Cardinal.fromStringifiedJSON(JSON.stringify(obj));
  }
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
