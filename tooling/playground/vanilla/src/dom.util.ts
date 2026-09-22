/** The element with `id`, which the page renders before asking for it. */
export function requireElement(id: string): HTMLElement {
    const element = document.getElementById(id);
    if (!element) throw new Error(`#${id} is missing from the page`);
    return element;
}

export function requireForm(id: string): HTMLFormElement {
    const element = requireElement(id);
    if (!(element instanceof HTMLFormElement)) throw new Error(`#${id} is not a form`);
    return element;
}

/** A text field's value; file fields and absent names read as empty. */
export function formText(form: FormData, name: string): string {
    const value = form.get(name);
    return typeof value === 'string' ? value : '';
}
