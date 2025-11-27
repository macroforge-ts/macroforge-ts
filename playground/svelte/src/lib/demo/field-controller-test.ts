import { Derive } from "@ts-macros/macros";
import {
  FieldController,
  fieldController,
  textAreaController,
} from "@playground/macro";

/**
 * Example class using the FieldController macro
 *
 * Usage:
 * @Derive(FieldController) on the class
 * @FieldController(TextAreaController) on fields you want to generate controllers for
 */
@Derive(FieldController)
class FormModel {
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

export { FormModel };
