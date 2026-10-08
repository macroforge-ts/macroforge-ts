// An inline object member with no tag field is recognised by shape alone:
// every required field must be present, and optional fields are copied only
// when the input carries them.

/** @derive(Encode, Decode) */
export type Link = string | { id: string; label?: string };
