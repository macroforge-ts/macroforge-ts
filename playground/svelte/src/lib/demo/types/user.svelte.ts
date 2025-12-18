/** import macro {Gigaform} from "@playground/macro"; */

import type { DateTime } from 'effect';
import type { Option } from 'effect/Option';
import type { Metadata } from './metadata.svelte';
import type { Settings } from './settings.svelte';
import type { AppPermissions } from './app-permissions.svelte';
import type { UserRole } from './user-role.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface User {
    id: string;
    email: Option<string>;
    /** @serde({ validate: ["nonEmpty"] }) */
    firstName: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    lastName: string;
    password: Option<string>;
    metadata: Option<Metadata>;
    settings: Settings;
    /** @default("Administrator") */
    role: UserRole;
    emailVerified: boolean;
    verificationToken: Option<string>;
    verificationExpires: Option<DateTime.DateTime>;
    passwordResetToken: Option<string>;
    passwordResetExpires: Option<DateTime.DateTime>;
    permissions: AppPermissions;
    createdAt: DateTime.DateTime;
    lastLoginAt: Option<DateTime.DateTime>;
}
