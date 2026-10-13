## References

A `{ "__ref": n }` stands for the object registered as `n`, and has to be read as the type that
object was decoded as: input cannot stand an object of one type in for another. A value decoded by
`nameDecodePartialWithContext` or flattened into its parent is not registered at all.

Validation (see `validation`) runs during decoding and reports failures through the `errors` array
of the result union. Field-level options are parsed by `field`; see the parent endec module for the
option and validator reference.
