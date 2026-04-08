/** @derive(Deserialize) */
type ContactInfo = {
    /** @serde(email) */
    primaryEmail: string;

    /** @serde(minLength(1), maxLength(100)) */
    address: string;
};
