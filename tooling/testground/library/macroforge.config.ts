import { DateTime, Option } from 'effect';

export default {
    foreignTypes: {
        'DateTime.DateTime': {
            from: ['effect'],
            encode: (value: DateTime.DateTime) => DateTime.formatIso(value),
            decode: (raw: unknown) =>
                Option.getOrThrowWith(
                    typeof raw === 'string' ? DateTime.make(raw) : Option.none(),
                    () => new Error(`Not a date-time: ${JSON.stringify(raw)}`)
                ),
            default: () => DateTime.unsafeNow()
        }
    }
};
