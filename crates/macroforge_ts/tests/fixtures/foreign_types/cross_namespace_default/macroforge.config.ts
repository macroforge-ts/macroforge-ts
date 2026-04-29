// Foreign type DateTime.Utc whose default body references a *different*
// namespace (Option) that IS imported at the top of macroforge.config.ts.
// Target file imports only DateTime, so Option must be auto-aliased and
// imported under `__mf_Option`.
import { DateTime, Option } from 'effect';

export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            serialize: (v: DateTime.Utc) => DateTime.formatIso(v),
            deserialize: (raw: unknown) =>
                Option.match(DateTime.make(raw as string), {
                    onSome: (dt) => dt,
                    onNone: () => Option.getOrElse(DateTime.make(0), () => null as never)
                }),
            default: () =>
                Option.match(DateTime.make(new Date()), {
                    onSome: (dt) => dt,
                    onNone: () => Option.getOrElse(DateTime.make(0), () => null as never)
                }),
            hasShape: (v: unknown) => typeof v === 'string'
        }
    }
};
