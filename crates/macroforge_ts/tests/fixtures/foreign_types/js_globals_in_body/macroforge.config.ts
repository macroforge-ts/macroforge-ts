// Foreign type whose expression bodies use JS globals (Math, Array,
// console, Date). Globals must NEVER be aliased or imported — they live
// in the JS runtime. The cache must reference them unchanged.
import { DateTime } from 'effect';

export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            serialize: (v: DateTime.Utc) => DateTime.formatIso(v),
            deserialize: (raw: unknown) => {
                if (!Array.isArray(raw) && typeof raw !== 'string') {
                    console.error('bad DateTime.Utc payload', raw);
                    return DateTime.make(0);
                }
                return DateTime.make(raw as string);
            },
            default: () => {
                const now = Math.floor(Date.now() / 1000);
                return DateTime.make(now);
            },
            hasShape: (v: unknown) => typeof v === 'string',
        },
    },
};
