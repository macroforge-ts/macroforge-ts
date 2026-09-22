import { expandForDisplay } from '../macroforge-expand.js';
import { highlightCode } from '../shiki-highlighter.js';

export interface ExpandedExample {
    before: string;
    after: string;
    beforeHtml: string;
    afterHtml: string;
}

/**
 * Expands macro code at build time and returns both before/after
 * Strips the import statement from the output for cleaner display
 */
export async function expandExample(
    code: string,
    filename = 'example.ts'
): Promise<ExpandedExample> {
    const after = await expandForDisplay(code, filename);

    // Pre-highlight with Shiki
    const [beforeHtml, afterHtml] = await Promise.all([
        highlightCode(code.trim()),
        highlightCode(after.trim())
    ]);

    return {
        before: code.trim(),
        after: after.trim(),
        beforeHtml,
        afterHtml
    };
}

/**
 * Expand multiple examples at once
 */
export async function expandExamples(
    examples: Record<string, string>
): Promise<Record<string, ExpandedExample>> {
    const expanded = await Promise.all(
        Object.entries(examples).map(
            async ([key, code]): Promise<[string, ExpandedExample]> => [
                key,
                await expandExample(code, `${key}.ts`)
            ]
        )
    );
    return Object.fromEntries(expanded);
}
