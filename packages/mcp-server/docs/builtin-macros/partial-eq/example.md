## Example

```typescript before
/** @derive(PartialEq, Hash) */
class User {
    id: number;
    name: string;

    @partialEq(skip) // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;
}
```

```typescript after
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        return hash;
    }
}
```

Generated output:

```typescript before
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        return hash;
    }
}
```

```typescript after
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        return hash;
    }
}
```

Generated output:

```typescript before
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        return hash;
    }
}
```

```typescript after
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        return hash;
    }
}
```

Generated output:

```typescript before
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        return hash;
    }
}
```

```typescript after
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        return hash;
    }
}
```

Generated output:

```typescript before
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        return hash;
    }
}
```

```typescript after
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        return hash;
    }
}
```

Generated output:

```typescript before
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        return hash;
    }
}
```

```typescript after
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split('')
                          .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
            0;
        return hash;
    }
}
```

## Equality Contract

When implementing `PartialEq`, consider also implementing `Hash`:

- **Reflexivity**: `a.equals(a)` is always true
- **Symmetry**: `a.equals(b)` implies `b.equals(a)`
- **Hash consistency**: Equal objects must have equal hash codes