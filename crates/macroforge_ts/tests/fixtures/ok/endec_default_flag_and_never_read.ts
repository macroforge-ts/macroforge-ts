/** @derive(Encode, Decode) */
interface Settings {
    /** @endec({ default: true }) */
    retries: number;
    /** @endec({ skipDecoding: true, default: "\"computed\"" }) */
    origin: string;
    /** @endec({ default: "true" }) */
    enabled: boolean;
}
