/**
 * Example class using the FieldController macro
 *
 * Usage:
 * @Derive(FieldController) on the class
 * @FieldController(TextAreaController) on fields you want to generate controllers for
 */
declare class FormModel {
    memo: string | null;
    username: string;
    description: string;
    constructor(memo: string | null, username: string, description: string);
}
export { FormModel };
