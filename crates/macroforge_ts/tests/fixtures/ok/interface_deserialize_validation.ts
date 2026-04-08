/** @derive(Deserialize) */
interface UserProfile {
    /** @serde(email) */
    email: string;

    /** @serde(minLength(2), maxLength(50)) */
    username: string;

    /** @serde(positive) */
    age?: number;
}
