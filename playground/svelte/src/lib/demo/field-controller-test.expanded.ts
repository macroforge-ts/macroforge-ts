/** import macro { FieldController } from '@playground/macro'; */

/** @derive(FieldController) */
export interface FormModel {
  /** @fieldController(textAreaController)*/
  memo: string | null;
  username: string;
  /** @fieldController(textAreaController)*/
  description: string;
}

export namespace FormModel {
  export function make(
    memo: string | null,
    username: string,
    description: string,
  ) {
    return {
      memo,
      username,
      description,
    };
  }
}
