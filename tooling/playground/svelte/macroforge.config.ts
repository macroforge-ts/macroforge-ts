import { DateTime, Option } from "effect";

export default {
  keepDecorators: false,
  foreignTypes: {
    // Use the fully qualified type name as the key
    // This matches the type annotation "DateTime.DateTime" in TypeScript
    "DateTime.DateTime": {
      from: ["effect"],
      aliases: [{ name: "DateTime", from: "effect/DateTime" }],
      serialize: (v: DateTime.DateTime) => DateTime.formatIso(v),
      deserialize: (raw: unknown) =>
        DateTime.unsafeFromDate(new Date(raw as string)),
      default: () => DateTime.unsafeNow(),
    },
    Option: {
      from: ["effect/Option"],
      serialize: (v: Option.Option<unknown>) => Option.getOrNull(v),
      deserialize: (raw: unknown) =>
        raw === null ? Option.none() : Option.some(raw),
      default: () => Option.none(),
    },
  },
};
