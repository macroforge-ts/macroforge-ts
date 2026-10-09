import { DateTime, Option } from 'effect';

const isIsoString = (value: unknown): value is string =>
    typeof value === 'string' && !Number.isNaN(Date.parse(value));

export default {
    keepDecorators: false,
    cfg: {
        features: ['testground'],
        target: 'web'
    },
    foreignTypes: {
        // Use the fully qualified type name as the key
        // This matches the type annotation "DateTime.DateTime" in TypeScript
        'DateTime.DateTime': {
            from: ['effect'],
            aliases: [{ name: 'DateTime', from: 'effect/DateTime' }],
            encode: (v: DateTime.DateTime) => DateTime.formatIso(v),
            // Form validation passes live values, so a DateTime must pass through as-is.
            decode: (raw: unknown) => DateTime.unsafeMake(raw as DateTime.DateTime.Input),
            default: () => DateTime.unsafeNow(),
            // Encode tells a union's members apart on live values, decode on raw ones.
            hasShape: (value: unknown): value is DateTime.DateTime | string =>
                DateTime.isDateTime(value) || isIsoString(value)
        },
        'Option.Option': {
            from: ['effect'],
            aliases: [{ name: 'Option', from: 'effect/Option' }],
            encode: (v: Option.Option<unknown>) => Option.getOrNull(v),
            decode: (
                raw: unknown
            ) => (Option.isOption(raw) ? raw : Option.fromNullable(raw)),
            default: () => Option.none()
        }
    }
};
