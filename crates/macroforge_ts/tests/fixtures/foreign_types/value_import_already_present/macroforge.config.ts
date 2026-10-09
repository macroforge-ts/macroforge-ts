// The target file already imports DateTime as a value. The generated file
// still imports only the handler exports, adding no alias of DateTime.
import { DateTime } from 'effect';

export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            encode: (v: DateTime.Utc) => DateTime.formatIso(v),
            decode: (raw: unknown) => DateTime.make(raw as string),
            default: () => DateTime.make(new Date()),
            hasShape: (v: unknown) => typeof v === 'string'
        }
    }
};
