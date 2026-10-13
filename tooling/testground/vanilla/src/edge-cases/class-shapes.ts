/**
 * Class members a derive has to tell from plain fields: visibility
 * modifiers, readonly and static properties, accessors, methods, optional and
 * definite fields, a constructor of the class's own, parameter properties,
 * fields typed by their initializers and inherited fields, of a class and of
 * an interface, and a static a superclass writes that a derive generates.
 */

import type { EdgeItem } from './edge-types';

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ClassMembers {
    static created: number = 0;
    public open: string = 'open';
    readonly fixed: string = 'fixed';
    optional?: string;
    definite!: number;
    items: EdgeItem[] = [];

    get label(): string {
        return `${this.open}:${this.fixed}`;
    }

    set label(text: string) {
        this.open = text;
    }

    describe(): string {
        return `${this.open} (${this.items.length})`;
    }
}

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export class ClassGuarded {
    name: string = '';
    protected guarded: number = 1;
    private hidden: boolean = true;
    private readonly sealed: string = 'sealed';

    reveal(): [number, boolean, string] {
        return [this.guarded, this.hidden, this.sealed];
    }
}

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export class ClassConstructed {
    label: string;
    weight: number;

    constructor(label: string, weight: number) {
        this.label = label;
        this.weight = weight;
    }
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ClassParameters {
    constructor(
        public name: string,
        readonly rank: number,
        private secret: string = 'secret'
    ) {}

    reveal(): string {
        return this.secret;
    }
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ClassBase {
    id: string = '';
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ClassDerived extends ClassBase {
    extra: number = 0;
}

export class ClassOwnStatic {
    id: string = '';

    static toString(value: ClassOwnStatic): string {
        return `own ${value.id}`;
    }
}

/** @derive(Debug, Clone) */
export class ClassOverStatic extends ClassOwnStatic {
    note: string = '';
}

/** @derive(Debug, Clone, PartialEq, Hash, Default) */
export abstract class ClassAbstract {
    kind: string = 'abstract';
    abstract area(): number;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ClassHolder {
    card: ClassMembers = ClassMembers.defaultValue();
    cards: ClassMembers[] = [];
    byName: Map<string, ClassBase> = new Map();
    maybe: ClassBase | null = null;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ClassInferred {
    count = 0;
    label = 'none';
    active = false;
    big = 10n;
    offset = -1;
    note = `n${1}`;
    at = new Date(0);
    tags = new Set<string>();
    ratio = 0.5 as number;
    readonly kind = 'inferred';
    readonly rank = 3;

    constructor(
        public id = '',
        private secret = 7
    ) {}

    reveal(): number {
        return this.secret;
    }
}

/** @derive(Debug, Clone, PartialEq, Hash, Default) */
export class ClassCallback {
    name: string = '';
    onChange: (name: string) => void = () => {};
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface ClassEntity {
    id: string;
    kind: string;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface ClassDated {
    at: Date;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface ClassAccount extends ClassEntity, ClassDated {
    kind: 'account';
    balance: number;
}

/** A base that leaves a field for its subclasses to give. */
export abstract class ClassShapeBase {
    abstract kind: string;
    id: string = '';
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ClassCircle extends ClassShapeBase {
    kind: string = 'circle';
    radius: number = 1;
}
