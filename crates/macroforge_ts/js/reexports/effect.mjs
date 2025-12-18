// js/reexports/effect.ts
import { succeed, fail, isSuccess } from "effect/Exit";
import { some, none, isNone } from "effect/Option";
export {
  some as optionSome,
  none as optionNone,
  isNone as optionIsNone,
  succeed as exitSucceed,
  isSuccess as exitIsSuccess,
  fail as exitFail
};
