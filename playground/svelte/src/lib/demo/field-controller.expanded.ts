/** import macro { FieldController } from '@playground/macro'; */

export interface FormModel {
    memo: string | null;
    username: string;

    description: string;
}

export function makeFormModelBaseProps<
    D extends number,
    const P extends DeepPath<FormModel, D>,
    V = DeepValue<FormModel, P, never, D>
>(
    superForm: SuperForm<FormModel>,
    path: P,
    overrides?: BasePropsOverrides<FormModel, V, D>
): BaseFieldProps<FormModel, V, D> {
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
export const memoFieldPath = ['memo'] as const;
export function memoFieldController(
    superForm: SuperForm<FormModel>
): MemoFieldController<FormModel, string | null, 1> {
    const fieldPath = memoFieldPath;
    return {
        fieldPath,
        baseProps: makeFormModelBaseProps(superForm, fieldPath, { labelText: 'Memo' })
    };
}
export const descriptionFieldPath = ['description'] as const;
export function descriptionFieldController(
    superForm: SuperForm<FormModel>
): DescriptionFieldController<FormModel, string, 1> {
    const fieldPath = descriptionFieldPath;
    return {
        fieldPath,
        baseProps: makeFormModelBaseProps(superForm, fieldPath, { labelText: 'Description' })
    };
}

export namespace FormModel {
    export function toString(self: FormModel): string {
        const parts: string[] = [];
        parts.push('memo: ' + self.memo);
        parts.push('username: ' + self.username);
        parts.push('description: ' + self.description);
        return 'FormModel { ' + parts.join(', ') + ' }';
    }
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
