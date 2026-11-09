/**
 * Declarations by where and how a module writes them: types it keeps to
 * itself or exports apart from their declaration, classes with static
 * members, a superclass that derives nothing, overloads and optional methods.
 */

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
interface ModulePrivate {
    a: string;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
class ModulePrivateClass {
    a: string = '';
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
type ModulePrivateAlias = { a: string };

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
enum ModulePrivateEnum {
    /** @default */
    A = 'a',
    B = 'b'
}

export function usePrivate(): string {
    const one: ModulePrivate = { a: 'x' };
    const two: ModulePrivateAlias = { a: 'y' };
    return [
        modulePrivateToString(one),
        ModulePrivateClass.toString(ModulePrivateClass.defaultValue()),
        modulePrivateAliasToString(two),
        modulePrivateEnumToString(ModulePrivateEnum.B)
    ].join(' ');
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
interface ModuleLater {
    a: string;
}
export type { ModuleLater };

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ModuleStatics {
    static count = 0;
    static #made = 0;
    static {
        ModuleStatics.#made += 1;
    }
    name: string = '';

    static made(): number {
        return ModuleStatics.#made;
    }
}

export interface ModuleNamed {
    name: string;
    describe(): string;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ModuleImplements implements ModuleNamed {
    name: string = '';

    describe(): string {
        return this.name;
    }
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ModuleBase<T> {
    held: T | null = null;
}

export class ModulePlainBase {
    plain: string = 'p';
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ModuleExtendsPlain extends ModulePlainBase {
    extra: number = 0;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ModuleOverloads {
    name: string = '';

    pick(index: number): string;
    pick(key: string): string;
    pick(which: number | string): string {
        return `${this.name}:${which}`;
    }
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export abstract class ModuleAbstract {
    name: string = '';
    abstract area(): number;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ModuleOptionalMethods {
    name: string = '';
    handler?(): void;
    readonly tags: readonly string[] = [];
    readonly pair: readonly [string, number] = ['a', 1];
}
