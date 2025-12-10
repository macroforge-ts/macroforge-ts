## All Options

### Container Options (on class/interface)

| `rename_all` 
| `string` 
| Apply naming convention to all fields 

| `deny_unknown_fields` 
| `boolean` 
| Throw error if JSON has unknown keys

### Field Options (on properties)

| `rename` 
| `string` 
| Use a different JSON key 

| `skip` 
| `boolean` 
| Exclude from serialization and deserialization 

| `skip_deserializing` 
| `boolean` 
| Exclude from deserialization only 

| `default` 
| `boolean | string` 
| Use TypeScript default or custom expression if missing 

| `flatten` 
| `boolean` 
| Merge nested object fields from parent