// Target file imports DateTime as a value at the top, so the inlined
// expression can reference `DateTime.foo` directly — the engine must
// NOT add a redundant `__mf_DateTime` alias.
import { DateTime } from 'effect';

export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            serialize: (v: DateTime.Utc) => DateTime.formatIso(v),
            deserialize: (raw: unknown) => DateTime.make(raw as string),
            default: () => DateTime.make(new Date()),
            hasShape: (v: unknown) => typeof v === 'string',
        },
    },
};
