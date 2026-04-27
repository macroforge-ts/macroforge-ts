// Foreign type DateTime.Utc whose default body uses Option, where Option
// is NOT imported at the top of macroforge.config.ts but IS itself a
// configured foreign type. The engine must still emit
// `import { Option as __mf_Option } from "effect"` by recovering the
// module from the Option foreign-type entry's `from` list.
import { DateTime } from 'effect';

export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            serialize: (v) => DateTime.formatIso(v),
            deserialize: (raw) => DateTime.make(raw as string),
            default: () =>
                Option.match(DateTime.make(new Date()), {
                    onSome: (dt) => dt,
                    onNone: () => DateTime.make(0),
                }),
        },
        'Option': {
            from: ['effect'],
            serialize: (v) => v,
            deserialize: (raw) => raw,
            default: () => null,
        },
    },
};
