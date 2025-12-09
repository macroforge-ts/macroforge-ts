import ts from 'typescript';
import { Hover, Position } from 'vscode-languageserver';
import { Document, getWordAt, mapObjWithRangeToOriginal } from '../../../lib/documents';
import { HoverProvider } from '../../interfaces';
import { SvelteDocumentSnapshot } from '../DocumentSnapshot';
import { LSAndTSDocResolver } from '../LSAndTSDocResolver';
import { getMacroManifestCache } from '../MacroManifestCache';
import { getMarkdownDocumentation } from '../previewer';
import { convertRange } from '../utils';
import { getComponentAtPosition } from './utils';

export class HoverProviderImpl implements HoverProvider {
    constructor(private readonly lsAndTsDocResolver: LSAndTSDocResolver) {}

    async doHover(document: Document, position: Position): Promise<Hover | null> {
        // Check for macro hover first (JSDoc @derive comments and decorators)
        const macroHover = this.getMacroHover(document, position);
        if (macroHover) {
            return macroHover;
        }

        const { lang, tsDoc, userPreferences } = await this.getLSAndTSDoc(document);

        const eventHoverInfo = this.getEventHoverInfo(lang, document, tsDoc, position);
        if (eventHoverInfo) {
            return eventHoverInfo;
        }

        const offset = tsDoc.offsetAt(tsDoc.getGeneratedPosition(position));
        const info = lang.getQuickInfoAtPosition(
            tsDoc.filePath,
            offset,
            userPreferences.maximumHoverLength
        );
        if (!info) {
            return null;
        }

        let declaration = ts.displayPartsToString(info.displayParts);
        if (
            tsDoc.isSvelte5Plus &&
            declaration.includes('(alias)') &&
            declaration.includes('__sveltets_2_IsomorphicComponent')
        ) {
            // info ends with "import ComponentName"
            declaration = declaration.substring(declaration.lastIndexOf('import'));
        }

        const documentation = getMarkdownDocumentation(info.documentation, info.tags);

        // https://microsoft.github.io/language-server-protocol/specification#textDocument_hover
        const contents = ['```typescript', declaration, '```']
            .concat(documentation ? ['---', documentation] : [])
            .join('\n');

        return mapObjWithRangeToOriginal(tsDoc, {
            range: convertRange(tsDoc, info.textSpan),
            contents
        });
    }

    private getEventHoverInfo(
        lang: ts.LanguageService,
        doc: Document,
        tsDoc: SvelteDocumentSnapshot,
        originalPosition: Position
    ): Hover | null {
        const possibleEventName = getWordAt(doc.getText(), doc.offsetAt(originalPosition), {
            left: /\S+$/,
            right: /[\s=]/
        });
        if (!possibleEventName.startsWith('on:')) {
            return null;
        }

        const component = getComponentAtPosition(lang, doc, tsDoc, originalPosition);
        if (!component) {
            return null;
        }

        const eventName = possibleEventName.substr('on:'.length);
        const event = component.getEvents().find((event) => event.name === eventName);
        if (!event) {
            return null;
        }

        return {
            contents: [
                '```typescript',
                `${event.name}: ${event.type}`,
                '```',
                event.doc || ''
            ].join('\n')
        };
    }

    private async getLSAndTSDoc(document: Document) {
        return this.lsAndTsDocResolver.getLSAndTSDoc(document);
    }

    /**
     * Get hover info for macro-related tokens:
     * - JSDoc @derive(MacroName) comments
     * - @decorator(...) decorators like @serde, @debug
     */
    private getMacroHover(document: Document, position: Position): Hover | null {
        const text = document.getText();
        const offset = document.offsetAt(position);
        const cache = getMacroManifestCache();

        // Check if hovering in a JSDoc @derive comment
        // Pattern: /** @derive(Debug, Serialize) */
        const deriveMatch = this.findDeriveAtPosition(text, offset);
        if (deriveMatch) {
            const macroInfo = cache.getMacroInfo(deriveMatch.macroName);
            if (macroInfo) {
                return {
                    contents: {
                        kind: 'markdown',
                        value: `**@derive(${macroInfo.name})**\n\n${macroInfo.description || 'No description available.'}`
                    }
                };
            }
        }

        // Check if hovering on a decorator like @serde or @debug
        const decoratorMatch = this.findDecoratorAtPosition(text, offset);
        if (decoratorMatch) {
            // First check if it's a macro (like @Debug used as decorator)
            const macroInfo = cache.getMacroInfo(decoratorMatch.name);
            if (macroInfo) {
                return {
                    contents: {
                        kind: 'markdown',
                        value: `**@${macroInfo.name}**\n\n${macroInfo.description || 'No description available.'}`
                    }
                };
            }

            // Then check if it's a field decorator (like @serde, @debug)
            const decoratorInfo = cache.getDecoratorInfo(decoratorMatch.name);
            if (decoratorInfo && decoratorInfo.docs) {
                return {
                    contents: {
                        kind: 'markdown',
                        value: `**@${decoratorInfo.name}**\n\n${decoratorInfo.docs}`
                    }
                };
            }
        }

        return null;
    }

    /**
     * Find if the cursor is on a macro name inside @derive(...)
     * Returns the macro name if found
     */
    private findDeriveAtPosition(
        text: string,
        offset: number
    ): { macroName: string; start: number; end: number } | null {
        // Look for @derive patterns in JSDoc comments
        // Pattern: @derive(Name1, Name2, ...)
        const derivePattern = /@derive\s*\(\s*([^)]+)\s*\)/gi;
        let match: RegExpExecArray | null;

        while ((match = derivePattern.exec(text)) !== null) {
            const deriveStart = match.index;
            const deriveEnd = deriveStart + match[0].length;

            // Check if offset is within this @derive(...) block
            if (offset >= deriveStart && offset <= deriveEnd) {
                // Parse the macro names inside the parentheses
                const argsStart = text.indexOf('(', deriveStart) + 1;
                const argsEnd = text.indexOf(')', argsStart);
                const argsContent = text.substring(argsStart, argsEnd);

                // Split by comma and find which macro name the cursor is on
                let currentPos = argsStart;
                const macroNames = argsContent.split(',');

                for (const rawName of macroNames) {
                    const trimmedName = rawName.trim();
                    const nameStartInArgs = rawName.indexOf(trimmedName);
                    const nameStart = currentPos + nameStartInArgs;
                    const nameEnd = nameStart + trimmedName.length;

                    if (offset >= nameStart && offset <= nameEnd) {
                        return {
                            macroName: trimmedName,
                            start: nameStart,
                            end: nameEnd
                        };
                    }

                    currentPos += rawName.length + 1; // +1 for comma
                }
            }
        }

        return null;
    }

    /**
     * Find if the cursor is on a decorator name like @serde or @debug
     * Returns the decorator name if found
     */
    private findDecoratorAtPosition(
        text: string,
        offset: number
    ): { name: string; start: number; end: number } | null {
        // Look for @decoratorName patterns (not in JSDoc comments)
        // Pattern: @identifier (optionally followed by (...))
        const decoratorPattern = /@([a-zA-Z_$][a-zA-Z0-9_$]*)/g;
        let match: RegExpExecArray | null;

        while ((match = decoratorPattern.exec(text)) !== null) {
            const atSign = match.index;
            const nameStart = atSign + 1;
            const nameEnd = nameStart + match[1].length;

            // Check if offset is on the decorator name (including the @)
            if (offset >= atSign && offset <= nameEnd) {
                // Skip if this is inside a JSDoc comment starting with @derive
                // We want @serde/@debug decorators, not the @derive in comments
                if (match[1].toLowerCase() === 'derive') {
                    // Check if we're inside a JSDoc comment
                    const beforeMatch = text.substring(0, atSign);
                    const lastCommentStart = beforeMatch.lastIndexOf('/**');
                    const lastCommentEnd = beforeMatch.lastIndexOf('*/');
                    if (lastCommentStart > lastCommentEnd) {
                        // Inside a JSDoc comment - let findDeriveAtPosition handle it
                        continue;
                    }
                }

                return {
                    name: match[1],
                    start: atSign,
                    end: nameEnd
                };
            }
        }

        return null;
    }
}
