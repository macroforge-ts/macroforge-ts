/**
 * A class whose fields are runes. The derives that only read an instance
 * work over it, and read through the runes like any other code does.
 */

/** @derive(Debug, PartialEq, Hash, Encode) */
export class RuneCounter {
    label: string = $state('clicks');
    count: number = $state(0);
    doubled: number = $derived(this.count * 2);

    increment(): void {
        this.count += 1;
    }
}
