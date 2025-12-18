/** import macro { FieldController } from '@playground/macro'; */

export interface FormModel {
    memo: string | null;
    username: string;

    description: string;
}

export function formModelToString(value: FormModel): string {
    const parts: string[] = [];
    parts.push('memo: ' + value.memo);
    parts.push('username: ' + value.username);
    parts.push('description: ' + value.description);
    return 'FormModel { ' + parts.join(', ') + ' }';
}

export namespace FormModel {
    export function make(memo: string | null, username: string, description: string) {
        return {
            memo,
            username,
            description
        };
    }
}
