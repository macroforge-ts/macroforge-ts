## Example

```typescript before
/** @derive(Encode) */
class User {
    id: number;

    /** @endec({ rename: "userName" }) */
    name: string;

    /** @endec({ skipEncoding: true }) */
    password: string;

    /** @endec({ flatten: true }) */
    metadata: UserMetadata;
}
```

```typescript after
import {
    EncodeContext as __mf_EncodeContext,
    encodedText as __mf_encodedText,
    encodedTextWithoutMetadata as __mf_encodedTextWithoutMetadata,
    encodeSelection as __mf_encodeSelection,
    fieldRequirement as __mf_fieldRequirement
} from '@macroforge/core/endec';

class User {
    id: number;
    name: string;
    password: string;
    metadata: UserMetadata;

    static encode(value: User, keepMetadata?: boolean): string {
        return userEncode(value, keepMetadata);
    }

    static encodeWithContext(value: User, ctx: __mf_EncodeContext): Record<string, unknown> {
        const existingId = ctx.getId(value);
        if (existingId !== undefined) {
            return { __ref: existingId };
        }
        const result: Record<string, unknown> = { __type: 'User', __id: ctx.register(value) };
        User.#encodeFields(value, ctx, result, undefined);
        return result;
    }

    static encodePartialWithContext(
        value: Partial<User>,
        ctx: __mf_EncodeContext,
        fields?
    ): Record<string, unknown> {
        return __mf_encodeSelection(ctx, value, () => {
            const result: Record<string, unknown> = { __type: 'User' };
            User.#encodeFields(
                value as User,
                ctx,
                result,
                fields === undefined ? undefined : __mf_fieldRequirement(fields)
            );
            return result;
        });
    }

    // Writes each field the value has and the selection keeps.
    static #encodeFields(
        value: User,
        ctx: __mf_EncodeContext,
        result: Record<string, unknown>,
        __fields
    ): void {
        if (__fields === undefined || __fields.keeps('id')) {
            if (value.id !== undefined) {
                result['id'] = value.id;
            }
        }
        if (__fields === undefined || __fields.keeps('name')) {
            if (value.name !== undefined) {
                result['userName'] = value.name;
            }
        }
        if (__fields === undefined || __fields.keeps('metadata')) {
            if (value.metadata !== undefined) {
                const __flattened = userMetadataEncodePartialWithContext(value.metadata, ctx);
                const { ['__type']: _, __id: __, ...rest } = __flattened as Record<string, unknown>;
                Object.assign(result, rest);
            }
        }
    }
}

export function userEncode(value: User, keepMetadata?: boolean): string {
    const ctx = __mf_EncodeContext.create();
    const __raw = userEncodeWithContext(value, ctx);
    return keepMetadata
        ? __mf_encodedText(__raw)
        : __mf_encodedTextWithoutMetadata(__raw, '__type');
}

export function userEncodeWithContext(
    value: User,
    ctx: __mf_EncodeContext
): Record<string, unknown> {
    return User.encodeWithContext(value, ctx);
}

export function userEncodePartialWithContext(
    value: Partial<User>,
    ctx: __mf_EncodeContext,
    fields?
): Record<string, unknown> {
    return User.encodePartialWithContext(value, ctx, fields);
}
```

A class's encoders are static methods, where they reach its `private` and `protected` fields, and
the standalone functions run them. An interface or a type alias has the same three functions beside
it, with a module-level `nameEncodeFields`. A subclass encodes the fields of its superclasses too.

## Required Imports

The generated code automatically imports `EncodeContext` and the helpers that give an encoded
value's JSON text from `@macroforge/core/endec`. `encode` throws a `TypeError` for a value JSON
cannot represent, such as a top-level `undefined`.
