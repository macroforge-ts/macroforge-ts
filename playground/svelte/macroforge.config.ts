import { DateTime } from "effect";

export default {
  foreignTypes: {
    // Use the fully qualified type name as the key
    // This matches the type annotation "DateTime.DateTime" in TypeScript
    "DateTime.DateTime": {
      from: ["effect"],
      serialize: (v: DateTime.DateTime) => DateTime.formatIso(v),
      deserialize: (raw: unknown) => DateTime.unsafeFromDate(new Date(raw as string)),
      default: () => DateTime.unsafeNow()
    }
  }
}
