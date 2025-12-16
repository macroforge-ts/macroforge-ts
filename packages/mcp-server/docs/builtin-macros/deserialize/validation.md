## Validation

The macro supports 30+ validators via `@serde(validate(...))`:

### String Validators
- `email`, `url`, `uuid` - Format validation
- `minLength(n)`, `maxLength(n)`, `length(n)` - Length constraints
- `pattern("regex")` - Regular expression matching
- `nonEmpty`, `trimmed`, `lowercase`, `uppercase` - String properties

### Number Validators
- `gt(n)`, `gte(n)`, `lt(n)`, `lte(n)`, `between(min, max)` - Range checks
- `int`, `positive`, `nonNegative`, `finite` - Number properties

### Array Validators
- `minItems(n)`, `maxItems(n)`, `itemsCount(n)` - Collection size

### Date Validators
- `validDate`, `afterDate("ISO")`, `beforeDate("ISO")` - Date validation

## Field-Level Options

The `@serde` decorator supports:

- `skip` / `skipDeserializing` - Exclude field from deserialization
- `rename = "jsonKey"` - Read from different JSON property
- `default` / `default = expr` - Use default value if missing
- `flatten` - Read fields from parent object level
- `validate(...)` - Apply validators

## Container-Level Options

- `denyUnknownFields` - Error on unrecognized JSON properties
- `renameAll = "camelCase"` - Apply naming convention to all fields