// The default and decode handlers read Option, which the config imports and
// the target file does not. The expanded config keeps that import, so the
// generated file imports only the handlers and never Option.
import { DateTime, Option } from 'effect';

export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            encode: (v: DateTime.Utc) => DateTime.formatIso(v),
            decode: (raw: unknown) =>
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
