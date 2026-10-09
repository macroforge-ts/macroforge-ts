/**
 * The member of `options` a `<select>` reported. The `<option>`s are rendered
 * from the same list, so any other value is a bug worth failing on.
 */
export function pickOption<Choice extends string>(
    options: readonly Choice[],
    value: string
): Choice {
    const choice = options.find((option) => option === value);
    if (choice === undefined) {
        throw new Error(`"${value}" is not one of this select's options`);
    }
    return choice;
}
