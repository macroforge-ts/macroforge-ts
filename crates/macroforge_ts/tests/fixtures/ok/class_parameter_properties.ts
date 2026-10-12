/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
class Point {
    constructor(
        public x: number,
        readonly y: number,
        private label: string = 'origin'
    ) {}
}
