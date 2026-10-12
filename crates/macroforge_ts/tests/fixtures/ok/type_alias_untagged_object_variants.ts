/** @derive(Encode, Decode) */
type Outcome = { ok: true; at: Date; tags: Set<string> } | { ok: false; reason: string };

/** @derive(Encode, Decode) */
/** @endec({ untagged: true }) */
type Loose = { kind: 'a'; when: Date } | { kind: 'b'; count: number; note?: string };
