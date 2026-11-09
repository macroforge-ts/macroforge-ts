/** @derive(Decode) */
/** @endec({ denyUnknownFields: true }) */
interface Settings {
    name: string;
    /** @endec(flatten) */
    extra: Record<string, number>;
}
