/** import macro { FieldController } from "@playground/macro"; */

/**
 * Example class using the FieldController macro
 *
 * Usage:
 * - Add @derive(FieldController) on the class
 * - Add @FieldController(TextAreaController) on fields you want to generate controllers for
 */

export class FormModel {
  
  memo: string | null;

  username: string;

  
  description: string;

  constructor(memo: string | null, username: string, description: string) {
    this.memo = memo;
    this.username = username;
    this.description = description;
  }

  makeFormModelBaseProps<D extends number, const P extends DeepPath<FormModel, D>, V = DeepValue<FormModel, P, never, D>>(superForm: SuperForm<FormModel>, path: P, overrides?: BasePropsOverrides<FormModel, V, D>): BaseFieldProps<FormModel, V, D> {
    const proxy = formFieldProxy(superForm, path);
    const baseProps = {
        fieldPath: path,
        ...(overrides ?? {}),
        value: proxy.value,
        errors: proxy.errors,
        superForm
    };
    return baseProps;
}

  ;

  memoFieldPath = [
    "memo"
];

  memoFieldController(superForm: SuperForm<FormModel>): MemoFieldController<FormModel, string | null, 1> {
    const fieldPath = this.memoFieldPath;
    return {
        fieldPath,
        baseProps: this.makeFormModelBaseProps(superForm, fieldPath, {
            labelText: "Memo"
        })
    };
}

  descriptionFieldPath = [
    "description"
];

  descriptionFieldController(superForm: SuperForm<FormModel>): DescriptionFieldController<FormModel, string, 1> {
    const fieldPath = this.descriptionFieldPath;
    return {
        fieldPath,
        baseProps: this.makeFormModelBaseProps(superForm, fieldPath, {
            labelText: "Description"
        })
    };
}
}

let formy = new FormModel("sdfsdf", "dfsdf", "sdfsdf");

let controller = formy.memoFieldController;