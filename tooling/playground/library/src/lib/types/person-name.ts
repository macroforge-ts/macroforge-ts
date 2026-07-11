/** @derive(Default, Serialize, Deserialize) */
export interface PersonName {
    /** @serde({ validate: ["nonEmpty"] }) */
    firstName: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    lastName: string;
}
