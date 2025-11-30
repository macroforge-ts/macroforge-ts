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
 * @Derive(FieldController) on the class
 * @FieldController(TextAreaController) on fields you want to generate controllers for
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
/** ts-macros warning: Failed to parse macro output for @playground/macro::FieldController: Parse("Error { error: (21..39, Unexpected { got: \"FormModelBaseProps\", expected: \"* for generator, private key, identifier or async\" }) }") */
make FormModelBaseProps < D extends number , const P extends DeepPath <FormModel, D >, V = DeepValue <FormModel, P , never , D >> (superForm : SuperForm <FormModel>, path : P , overrides ?: BasePropsOverrides <FormModel, V , D > ): BaseFieldProps <FormModel, V , D > {const proxy = formFieldProxy (superForm , path ); const baseProps = {fieldPath : path , ... (overrides ?? {}), value : proxy . value , errors : proxy . errors , superForm }; return baseProps ; }; static {this . prototype .memoFieldPath= ["memo"]; }memoFieldController(superForm : SuperForm <FormModel> ):"Memo"FieldController<FormModel, string | null;, 1 > {const fieldPath = this .memoFieldPath; return {fieldPath , baseProps : this .makeFormModelBaseProps(superForm , fieldPath , {labelText :"Memo"})}; }; static {this . prototype .descriptionFieldPath= ["description"]; }descriptionFieldController(superForm : SuperForm <FormModel> ):"Description"FieldController<FormModel, string;, 1 > {const fieldPath = this .descriptionFieldPath; return {fieldPath , baseProps : this .makeFormModelBaseProps(superForm , fieldPath , {labelText :"Description"})}; }; }