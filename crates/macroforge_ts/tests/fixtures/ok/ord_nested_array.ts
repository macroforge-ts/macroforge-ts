/** @derive(PartialEq, Eq, PartialOrd, Ord) */
export class Version {
    major: number = 0;
}

/** @derive(PartialEq, Eq, PartialOrd, Ord) */
export class Release {
    versions: Version[] = [];
    tags: string[] = [];
}
