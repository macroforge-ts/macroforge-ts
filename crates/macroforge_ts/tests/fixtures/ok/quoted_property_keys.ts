/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
interface Headers {
    'content-type': string;
    'x request id': number;
    0: boolean;
    class: string;
}
