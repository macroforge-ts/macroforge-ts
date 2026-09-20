import { DateTime, Option } from 'effect';

export default {
    keepDecorators: false,
    cfg: {
        features: ['playground'],
        target: 'web'
    },
    foreignTypes: {
        // Use the fully qualified type name as the key
        // This matches the type annotation "DateTime.DateTime" in TypeScript
        'DateTime.DateTime': {
            from: ['effect'],
            aliases: [{ name: 'DateTime', from: 'effect/DateTime' }],
            serialize: (v: DateTime.DateTime) => DateTime.formatIso(v),
            // Form validation passes live values, so a DateTime must pass through as-is.
            deserialize: (raw: unknown) => DateTime.unsafeMake(raw as DateTime.DateTime.Input),
            default: () => DateTime.unsafeNow()
        },
        'Option.Option': {
            from: ['effect'],
            aliases: [{ name: 'Option', from: 'effect/Option' }],
            serialize: (v: Option.Option<unknown>) => Option.getOrNull(v),
            deserialize: (
                raw: unknown
            ) => (Option.isOption(raw) ? raw : Option.fromNullable(raw)),
            default: () => Option.none()
        }
    }
};
