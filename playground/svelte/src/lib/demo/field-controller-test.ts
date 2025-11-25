import { Derive, FieldController } from "$lib/macros";

/**
 * Example class using the FieldController macro
 *
 * Usage:
 * @Derive(FieldController) on the class
 * @Field({ component: "TextArea" }) on fields you want to generate controllers for
 */
@Derive("FieldController")
class FormModel {
  @FieldController({ component: "TextArea" })
  memo: string | null;

  @FieldController({ component: "TextField" })
  username: string;

  @FieldController({ component: "TextArea" })
  description: string;

  constructor(memo: string | null, username: string, description: string) {
    this.memo = memo;
    this.username = username;
    this.description = description;
  }
}

export { FormModel };
