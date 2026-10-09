// Handlers that read JS globals (Math, Array, console, Date). The globals
// stay inside the expanded handlers; the generated file imports only the
// handler exports.
import { DateTime } from 'effect';

export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            encode: (v: DateTime.Utc) => DateTime.formatIso(v),
            decode: (raw: unknown) => {
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
            hasShape: (v: unknown) => typeof v === 'string'
        }
    }
};
