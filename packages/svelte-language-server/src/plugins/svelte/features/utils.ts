import { Position } from 'vscode-languageserver';
import { isInTag } from '../../../lib/documents/index.ts';
import type { AttributeContext } from '../../../lib/documents/parseHtml.ts';
import { possiblyComponent } from '../../../utils.ts';
import { SvelteDocument } from '../SvelteDocument.ts';

export function attributeCanHaveEventModifier(
    attributeContext: AttributeContext
) {
    return (
        !attributeContext.inValue &&
        !possiblyComponent(attributeContext.elementTag) &&
        attributeContext.name.startsWith('on:') &&
        attributeContext.name.includes('|')
    );
}

export function inStyleOrScript(svelteDoc: SvelteDocument, position: Position) {
    return (
        isInTag(position, svelteDoc.style) ||
        isInTag(position, svelteDoc.script) ||
        isInTag(position, svelteDoc.moduleScript)
    );
}
