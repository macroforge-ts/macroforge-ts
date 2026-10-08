/** @derive(Decode) */
interface UserProfile {
    /** @endec(email) */
    email: string;

    /** @endec(minLength(2), maxLength(50)) */
    username: string;

    /** @endec(positive) */
    age?: number;
}
