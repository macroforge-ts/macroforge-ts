/** import macro { FieldController } from '@playground/macro'; */

/**  */
export interface FormModel {
  
  memo: string | null;
  username: string;
  
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