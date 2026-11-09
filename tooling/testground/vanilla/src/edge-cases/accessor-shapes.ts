/**
 * A class that keeps fields behind `accessor`, under the derives that read
 * an instance. It has a module of its own because a runtime that takes no
 * `accessor` as written cannot load one that declares it.
 */

/** @derive(Debug, PartialEq, Hash, Encode) */
export class ClassAccessors {
    accessor count = 0;
    accessor label: string = 'a';
    plain: number = 1;
}
