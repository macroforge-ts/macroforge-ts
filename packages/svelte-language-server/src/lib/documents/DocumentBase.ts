import { Position, Range, TextDocument } from 'vscode-languageserver';
import { getLineOffsets, offsetAt, positionAt } from './utils';

function nextLineOffset(text: string, lineOffsets: number[], line: number): number {
    return line + 1 < lineOffsets.length ? lineOffsets[line + 1] : text.length;
}

/** Offset where a line's content ends, before any carriage return or newline. */
function endOfLineOffset(text: string, lineOffsets: number[], line: number): number {
    let offset = nextLineOffset(text, lineOffsets, line);
    while (offset > lineOffsets[line]) {
        const code = text.charCodeAt(offset - 1);
        if (code !== 10 && code !== 13) {
            break;
        }
        offset--;
    }
    return offset;
}

/**
 * Represents a textual document.
 */
export abstract class ReadableDocument implements TextDocument {
    /**
     * Get the text content of the document
     */
    abstract getText(range?: Range): string;

    /**
     * Returns the url of the document
     */
    abstract getURL(): string;

    /**
     * Returns the file path if the url scheme is file
     */
    abstract getFilePath(): string | null;

    /**
     * Current version of the document.
     */
    public version = 0;

    /**
     * Should be cleared when there's an update to the text
     */
    protected lineOffsets?: number[];

    /**
     * Get the length of the document's content
     */
    getTextLength(): number {
        return this.getText().length;
    }

    /**
     * Get the line and character based on the offset
     * @param offset The index of the position
     */
    positionAt(offset: number): Position {
        return positionAt(offset, this.getText(), this.getLineOffsets());
    }

    /**
     * Get the index of the line and character position
     * @param position Line and character position
     */
    offsetAt(position: Position): number {
        return offsetAt(position, this.getText(), this.getLineOffsets());
    }

    /**
     * Get the range covering an entire line, excluding its terminator.
     *
     * A line past the end collapses onto the last line, and a negative line
     * onto an empty range at the start.
     */
    getLineRange(line: number): Range {
        const lineOffsets = this.getLineOffsets();
        if (line >= lineOffsets.length) {
            const lastLine = lineOffsets.length - 1;
            return Range.create(
                lastLine,
                0,
                lastLine,
                this.getTextLength() - lineOffsets[lastLine]
            );
        }
        if (line < 0) {
            return Range.create(0, 0, 0, 0);
        }
        const text = this.getText();
        return Range.create(
            line,
            0,
            line,
            endOfLineOffset(text, lineOffsets, line) - lineOffsets[line]
        );
    }

    /**
     * Get the terminator of a line, or an empty string when out of bounds.
     */
    getEOLCharacters(line: number): string {
        const lineOffsets = this.getLineOffsets();
        if (line < 0 || line >= lineOffsets.length) {
            return '';
        }
        const text = this.getText();
        return text.substring(
            endOfLineOffset(text, lineOffsets, line),
            nextLineOffset(text, lineOffsets, line)
        );
    }

    private getLineOffsets() {
        if (!this.lineOffsets) {
            this.lineOffsets = getLineOffsets(this.getText());
        }
        return this.lineOffsets;
    }

    /**
     * Implements TextDocument
     */
    get uri(): string {
        return this.getURL();
    }

    get lineCount(): number {
        return this.getText().split(/\r?\n/).length;
    }

    abstract languageId: string;
}

/**
 * Represents a textual document that can be manipulated.
 */
export abstract class WritableDocument extends ReadableDocument {
    /**
     * Set the text content of the document.
     * Implementers should set `lineOffsets` to `undefined` here.
     * @param text The new text content
     */
    abstract setText(text: string): void;

    /**
     * Update the text between two positions.
     * @param text The new text slice
     * @param start Start offset of the new text
     * @param end End offset of the new text
     */
    update(text: string, start: number, end: number): void {
        this.lineOffsets = undefined;
        const content = this.getText();
        this.setText(content.slice(0, start) + text + content.slice(end));
    }
}
