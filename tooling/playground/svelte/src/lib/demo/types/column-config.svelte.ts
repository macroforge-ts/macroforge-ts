/** import macro {Gigaform} from "@playground/macro"; */
import type { DataPath } from './data-path.svelte';

/** @derive(Default, Encode, Decode, Gigaform) */
export interface ColumnConfig {
    /** @endec({ validate: ["nonEmpty"] }) */
    heading: string;
    dataPath: DataPath;
}
