/** import macro { FieldController } from '@playground/macro'; */

/** @derive(FieldController, Debug) */
export interface FormModel {
  /** @fieldController(textAreaController)*/
  memo: string | null;
  username: string;
  /** @fieldController(textAreaController)*/
  description: string;
}

export const FormModel = {
  make: (memo: string | null, username: string, description: string) => {
    return {
      memo,
      username,
      description,
    };
  },
};
