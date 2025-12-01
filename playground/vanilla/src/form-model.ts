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
 * /** @derive(FieldController) */ on the class
 * @FieldController(TextAreaController) on fields you want to generate controllers for
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

let formy = new FormModel("sdfsdf", "dfsdf", "sdfsdf");

//let controller = formy.memoFieldController;
