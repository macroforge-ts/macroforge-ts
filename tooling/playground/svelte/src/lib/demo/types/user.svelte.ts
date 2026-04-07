/** import macro {Gigaform} from "@playground/macro"; */

import { type DateTime, Option } from 'effect';
import type { AppPermissions } from './app-permissions.svelte';
import type { Metadata } from './metadata.svelte';
import type { Settings } from './settings.svelte';
import type { UserRole } from './user-role.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface User {
    id: string;
    email: Option.Option<string>;
    /** @serde({ validate: ["nonEmpty"] }) */
    firstName: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    lastName: string;
    password: Option.Option<string>;
    metadata: Option.Option<Metadata>;
    settings: Settings;
    /** @default("Administrator") */
    role: UserRole;
    emailVerified: boolean;
    verificationToken: Option.Option<string>;
    verificationExpires: Option.Option<DateTime.DateTime>;
    passwordResetToken: Option.Option<string>;
    passwordResetExpires: Option.Option<DateTime.DateTime>;
    permissions: AppPermissions;
    createdAt: DateTime.DateTime;
    lastLoginAt: Option.Option<DateTime.DateTime>;
}
