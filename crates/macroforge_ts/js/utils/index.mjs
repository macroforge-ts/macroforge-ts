// js/utils/index.ts
import { Result, Option } from "@rydshift/mirror/declarative";
var DeserializeResult;
((DeserializeResult) => {
  function ok(value) {
    return { success: true, value };
  }
  DeserializeResult.ok = ok;
  function err(errors) {
    return { success: false, errors };
  }
  DeserializeResult.err = err;
  function isOk(result) {
    return result.success;
  }
  DeserializeResult.isOk = isOk;
  function isErr(result) {
    return !result.success;
  }
  DeserializeResult.isErr = isErr;
  function unwrap(result) {
    if (result.success)
      return result.value;
    throw new Error(`Deserialization failed: ${JSON.stringify(result.errors)}`);
  }
  DeserializeResult.unwrap = unwrap;
  function unwrapOr(result, defaultValue) {
    return result.success ? result.value : defaultValue;
  }
  DeserializeResult.unwrapOr = unwrapOr;
})(DeserializeResult ||= {});
export {
  Result,
  Option,
  DeserializeResult
};
