/** @derive(Decode) */
type ContactInfo = {
    /** @endec(email) */
    primaryEmail: string;

    /** @endec(minLength(1), maxLength(100)) */
    address: string;
};
