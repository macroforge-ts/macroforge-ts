/** @derive(Decode) */
interface Account {
    id: string;
    balance: number;
    opened: Date;
    tags: string[];
    nickname?: string;
    note: string | undefined;
}
