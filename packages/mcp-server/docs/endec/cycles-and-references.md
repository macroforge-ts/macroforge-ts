# Cycles & Forward References

`JSON.stringify` throws on a cycle. Macroforge's generated encoders handle cyclic graphs by
assigning identities and emitting back-references.

```typescript
/** @derive(Encode, Decode) */
class Node {
    name: string;
    parent?: Node;
    children: Node[];
}

const root = new Node();
const child = new Node();
root.children = [child];
child.parent = root; // cycle

const json = Node.encode(root, true); // keepMetadata
```

## Metadata

Cycle detection works by tagging objects with `__type` and `__id` as they're visited; a repeat visit
emits a reference to the existing id instead of recursing.

Those fields are **stripped from the output by default**, and with them goes what a reference would
name. A value held in several places is then written out at each of them, and decodes as that many
equal values. A value that holds itself has no end to write out, so `encode` throws a `TypeError`
for it. Pass `keepMetadata: true` to keep the ids and the references that reconnect a graph.

```typescript
const shared = new Node();
const tree = new Node();
tree.children = [shared, shared];

Node.encode(tree); // clean payload, `shared` written out twice
Node.encode(tree, true); // includes __type/__id, the second `shared` is a __ref
Node.encode(root, true); // a cycle needs its metadata
Node.encode(root); // TypeError: root holds itself
```

For API responses you usually want the default. For persistence or transferring an object graph,
keep the metadata.

## What a reference stands for

A reference stands for one whole value of one type. Three rules follow from that:

- A value held more than once is one object after decoding, wherever it is held: in a field, an
  array, a `Set`, a `Map` or a `Record`.
- A `{ "__ref": n }` has to refer to an object that was decoded as the type the reference is read
  as. Input that points a `User` field at a registered `Order` fails to decode.
- A `Partial<T>`, `Pick<T, K>` or `Omit<T, K>` is written out in full wherever it is held, and is
  never referred to: an object lacking fields cannot stand in for a whole `T`. One that holds itself
  has no encoding, and `encode` throws a `TypeError`.

## The context object

`encodeWithContext` / `decodeWithContext` take a context that carries the identity map. The
top-level `encode` / `decode` create one for you; you only need the context form when composing
encoders by hand.

## Forward references

While decoding, a reference may point at an object that hasn't been constructed yet. Rather than
failing, the decoder records a `PendingRef` placeholder and patches it once the target exists.

Two consequences worth knowing:

- The field is temporarily `null` between the placeholder being recorded and patches being applied.
  If you read the graph mid-decoding you may observe that.
- `ctx.applyPatches()` performs the fix-up. The top-level `decode` calls it for you.

## `freeze`

Pass `freeze: true` to deep-freeze every object produced:

```typescript
const r = Node.decode(input, { freeze: true });
```

Freezing happens after patches are applied, so cyclic graphs freeze correctly rather than being
frozen mid-construction.

## Errors

Structural failures inside `decodeWithContext` throw `DecodeError`. The top-level `decode` catches
these and returns `{ success: false, errors }` instead: so the entry point never throws, while the
internal context form does.
