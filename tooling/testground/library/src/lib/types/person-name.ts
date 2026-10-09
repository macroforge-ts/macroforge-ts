/** @derive(Default, Encode, Decode) */
export interface PersonName {
    /** @endec({ validate: ["nonEmpty"] }) */
    firstName: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    lastName: string;
}
