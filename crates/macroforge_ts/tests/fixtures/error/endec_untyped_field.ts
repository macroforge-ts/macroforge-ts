declare function makeTags(): string[];

/** @derive(Encode, Decode) */
class Loose {
    count = 0;
    tags = makeTags();
    items = [];
    /** @endec({ skip: true }) */
    scratch = makeTags();
    /** @endec({ encodeWith: (tags: string[]) => tags.join(','), decodeWith: (raw: unknown) => String(raw).split(',') }) */
    joined = makeTags();

    constructor(public bag = {}) {}
}
