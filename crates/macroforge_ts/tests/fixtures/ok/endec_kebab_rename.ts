/** @derive(Encode, Decode) */
/** @endec({ renameAll: "kebab-case" }) */
interface Profile {
    firstName: string;
    isActive: boolean;
}
