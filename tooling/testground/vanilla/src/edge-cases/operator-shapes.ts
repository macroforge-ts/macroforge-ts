/**
 * Fields typed by an operator over a type from another module: `keyof` it,
 * and an indexed access whose type that module names and this one does not
 * import.
 */

import type { OperatorItem } from './operator-types';

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface OperatorHolder {
    owner: OperatorItem['owner'];
    owners: OperatorItem['owners'];
    label: OperatorItem['label'];
    weight: OperatorItem['weight'];
    key: keyof OperatorItem;
}
