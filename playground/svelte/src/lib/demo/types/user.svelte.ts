/** import macro {Gigaform} from "@playground/macro"; */

import type { Metadata } from './metadata.svelte';
import type { Settings } from './settings.svelte';
import type { AppPermissions } from './app-permissions.svelte';
import type { UserRole } from './user-role.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface User {
    id: string;
    email: string | null;
    /** @serde({ validate: ["nonEmpty"] }) */
    firstName: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    lastName: string;
    password: string | null;
    metadata: Metadata | null;
    settings: Settings;
    /** @default("Administrator") */
    role: UserRole;
    emailVerified: boolean;
    verificationToken: string | null;
    verificationExpires: string | null;
    passwordResetToken: string | null;
    passwordResetExpires: string | null;
    permissions: AppPermissions;
}
