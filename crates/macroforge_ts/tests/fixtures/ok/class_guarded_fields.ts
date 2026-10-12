/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
class Guarded {
    name: string = '';
    protected level: number = 1;
    private secret: boolean = true;
    private readonly sealed: string = 'sealed';
}
