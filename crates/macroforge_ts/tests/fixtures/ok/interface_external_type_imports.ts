import { Metadata } from "./metadata.svelte";

/** @derive(Default, Serialize, Deserialize) */
export interface User {
    metadata: Metadata;
}
