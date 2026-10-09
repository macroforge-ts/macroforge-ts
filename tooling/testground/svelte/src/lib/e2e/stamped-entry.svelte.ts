/**
 * Fields typed by `Stamped<T>` from another module, in each position a field
 * can take it: alone, nullable and as array elements. A literal default that
 * a type has to decode, for a newtype, a generic alias or an object holding a
 * newtype, takes it through that type's decoder.
 */

import type { Slug, Stamped } from './stamp';

/** @derive(Default, Encode, Decode) */
export interface Note {
    text: string;
}

/** @derive(Default, Encode, Decode) */
export interface Label {
    slug: Slug;
}

/** @derive(Default, Encode, Decode) */
export interface StampedEntry {
    at: Stamped<Note>;
    previous: Stamped<Note> | null;
    history: Array<Stamped<Note>>;
    /** @default("home") */
    slug: Slug;
    /** @default({ slug: "pinned" }) */
    label: Label;
    /** @default("2024-01-01T00:00:00.000Z") */
    since: Stamped<Note>;
}
