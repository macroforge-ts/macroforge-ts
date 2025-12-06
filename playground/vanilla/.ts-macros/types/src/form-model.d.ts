/**
 * Example class using the FieldController macro
 *
 * Usage:
 * @Derive(FieldController) on the class
 * @FieldController(TextAreaController) on fields you want to generate controllers for
 */
export declare class FormModel {
    memo: string | null;
    username: string;
    description: string;
    constructor(memo: string | null, username: string, description: string);
    /** ts-macros warning: Failed to parse macro output for @playground/macro::FieldController: Parse("Error { error: (21..39, Unexpected { got: \"FormModelBaseProps\", expected: \"* for generator, private key, identifier or async\" }) }") */
    make: any;
    FormModelBaseProps<D extends number, const P extends DeepPath<FormModel, D>, V = DeepValue<FormModel, P, never, D>>(superForm: SuperForm<FormModel>, path: P, overrides?: BasePropsOverrides<FormModel, V, D>): BaseFieldProps<FormModel, V, D>;
    memoFieldController(superForm: SuperForm<FormModel>): "Memo";
    FieldController<FormModel, string>(): any;
}
