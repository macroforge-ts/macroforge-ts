/** import macro { FieldController } from "@playground/macro"; */

/**
 * Example class using the FieldController macro
 *
 * Usage:
 * - Add @derive(FieldController) on the class
 * - Add @FieldController(TextAreaController) on fields you want to generate controllers for
 */
/** @derive(FieldController) */
export class FormModel {
    /** @fieldController(textAreaController) */
    memo: string | null;

    username: string;

    /** @fieldController(textAreaController) */
    description: string;

    constructor(memo: string | null, username: string, description: string) {
        this.memo = memo;
        this.username = username;
        this.description = description;
    }
}

let formy = new FormModel('sdfsdf', 'dfsdf', 'sdfsdf');

let controller = formy.memoFieldController;
