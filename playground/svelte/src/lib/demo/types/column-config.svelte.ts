/** import macro {Gigaform} from "@playground/macro"; */

import type { DataPath } from './data-path.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface ColumnConfig {
    /** @serde({ validate: ["nonEmpty"] }) */
    heading: string;
    dataPath: DataPath;
}
