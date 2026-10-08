import { Metadata } from './metadata.svelte';

/** @derive(Default, Encode, Decode) */
export interface User {
    metadata: Metadata;
}
