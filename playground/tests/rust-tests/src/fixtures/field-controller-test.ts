import { Derive } from "@ts-macros/swc-napi";
import {
  FieldController,
  fieldController,
  textAreaController,
} from "@playground/macro";

/**
 * Example class using the FieldController macro
 *
 * Usage:
 * Use a `@derive(FieldController)` macro comment above the class
 * Use `@fieldController(textAreaController)` on fields you want to generate controllers for
 */
/** @derive(FieldController) */
export class FormModel {
  @fieldController(textAreaController)
  memo: string | null;

  username: string;

  @fieldController(textAreaController)
  description: string;

  constructor(memo: string | null, username: string, description: string) {
    this.memo = memo;
    this.username = username;
    this.description = description;
  }
}
