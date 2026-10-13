## Generic Types

`nameDecodeWithContext` and `nameDecodePartialWithContext` of a generic type take one optional
`Decoder` for each type parameter, after their other parameters, and a field written with the type's
arguments hands in a decoder made from each: `Box<Date>` decodes the dates its `T` holds, and
`Page<User>` its users, checked and built like any other. A union alias hands the decoders on to a
member written with type arguments. The top-level `nameDecode` and a class's static `decode` take
the same decoders after their options, `boxDecode(text, undefined, decodeDate)`. Without a decoder a
value of the parameter is kept as it is. A type argument a field leaves to its default, `Box` for
`Box<T = Date>`, is decoded and checked as that default.

## Reserved Keys

`__id` and `__ref` mark a value held in more than one place, and the tag key (`__type`, or the
container's `tag`) holds the type's name. A field read from or written under one of them is an
expansion error: give it another key with `@endec({ rename: "..." })`. So are two fields under one
key, whether by `rename`, `renameAll` or a flattened type's own fields, and two flattened `Record`s,
each of which holds every key nothing else reads.
