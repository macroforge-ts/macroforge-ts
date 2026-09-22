import type { Node } from 'estree';
import type { TemplateNode } from 'svelte/types/compiler/interfaces';

export interface SvelteNode {
    start: number;
    end: number;
    type: string;
    parent?: SvelteNode;
    [key: string]: any;
}

type HTMLLike = 'Element' | 'InlineComponent' | 'Body' | 'Window';

export interface AwaitBlock extends SvelteNode {
    type: 'AwaitBlock';
    expression: SvelteNode & Node;
    value: (SvelteNode & Node) | null;
    error: (SvelteNode & Node) | null;
    pending: AwaitSubBlock;
    then: AwaitSubBlock;
    catch: AwaitSubBlock;
}

export interface AwaitSubBlock extends SvelteNode {
    skip: boolean;
    children: SvelteNode[];
}

export interface EachBlock extends SvelteNode {
    type: 'EachBlock';
    expression: SvelteNode & Node;
    context: SvelteNode & Node;
    key?: SvelteNode & Node;
    else?: SvelteNode;
    children: SvelteNode[];
}

function matchesOnly(
    type: string | undefined,
    only?: 'Element' | 'InlineComponent'
): boolean {
    return (
        !only ||
        // We hide the detail that body/window are also like elements in the context of this usage
        (only === 'Element' &&
            ['Element', 'Body', 'Window'].includes(type as HTMLLike)) ||
        (only === 'InlineComponent' && type === 'InlineComponent')
    );
}

/**
 * Returns true if given node is a component or html element, or if the offset is at the end of the node
 * and its parent is a component or html element.
 */
export function isInTag(
    node: SvelteNode | null | undefined,
    offset: number
): boolean {
    return (
        node?.type === 'InlineComponent' ||
        node?.type === 'Element' ||
        (node?.end === offset &&
            (node?.parent?.type === 'InlineComponent' ||
                node?.parent?.type === 'Element'))
    );
}

/**
 * Returns when given node represents an HTML Attribute.
 * Example: The `class` in `<div class=".."`.
 * Note: This method returns `false` for shorthands like `<div {foo}`.
 */
export function isAttributeName(
    node: SvelteNode | null | undefined,
    only?: 'Element' | 'InlineComponent'
): boolean {
    return !!node && node.type === 'Attribute' &&
        matchesOnly(node.parent?.type, only);
}

/**
 * Returns when given node represents an HTML Attribute shorthand or is inside one.
 * Example: The `{foo}` in `<div {foo}`
 */
export function isAttributeShorthand(
    node: SvelteNode | null | undefined,
    only?: 'Element' | 'InlineComponent'
): boolean {
    if (!node) {
        return false;
    }
    do {
        // We could get the expression, or the shorthand, or the attribute
        // Be pragmatic and just go upwards until we can't anymore
        if (isAttributeName(node, only)) {
            return true;
        }
        node = node.parent!;
    } while (node);
    return false;
}

/**
 * Returns when given node represents an HTML Attribute shorthand or is inside one.
 * Example: The `on:click={foo}` in `<div on:click={foo}`
 */
export function isEventHandler(
    node: SvelteNode | null | undefined,
    only?: 'Element' | 'InlineComponent'
) {
    return !!node && node.type === 'EventHandler' &&
        matchesOnly(node.parent?.type, only);
}

export function isElseBlockWithElseIf(node: SvelteNode | null | undefined) {
    return (
        !!node &&
        node.type === 'ElseBlock' &&
        'children' in node &&
        Array.isArray(node.children) &&
        node.children.length === 1 &&
        node.children[0].type === 'IfBlock'
    );
}

export function hasElseBlock(
    node: SvelteNode
): node is SvelteNode & { else: SvelteNode } {
    return 'else' in node && !!node.else;
}

export function findElseBlockTagStart(
    documentText: string,
    elseBlock: SvelteNode
) {
    return documentText.lastIndexOf(
        '{',
        documentText.lastIndexOf(':else', elseBlock.start)
    );
}

export function findIfBlockEndTagStart(
    documentText: string,
    ifBlock: SvelteNode
) {
    return documentText.lastIndexOf(
        '{',
        documentText.lastIndexOf('/if', ifBlock.end)
    );
}

/** What a walker's handlers can do to the traversal. */
export interface SvelteWalkContext {
    /** Skip the current node's children (and, from `enter`, its `leave`). */
    skip: () => void;
}

/** A handler called for each node of a Svelte template AST. */
export type SvelteNodeHandler = (
    this: SvelteWalkContext,
    node: SvelteNode,
    parent: SvelteNode | null,
    key: string | null,
    index: number | null
) => void;

export interface SvelteNodeWalker {
    enter?: SvelteNodeHandler;
    leave?: SvelteNodeHandler;
}

/**
 * Walk a Svelte template AST depth first. Every property holding a node, or an
 * array of nodes, is a child; `parent` links added by callers are not.
 */
export function walkSvelteAst(root: SvelteNode, walker: SvelteNodeWalker) {
    visitSvelteNode(root, null, null, null, walker);
}

function visitSvelteNode(
    node: SvelteNode,
    parent: SvelteNode | null,
    key: string | null,
    index: number | null,
    walker: SvelteNodeWalker
) {
    let skipped = false;
    const context: SvelteWalkContext = {
        skip: () => {
            skipped = true;
        }
    };
    walker.enter?.call(context, node, parent, key, index);
    if (skipped) {
        return;
    }
    for (const [childKey, value] of Object.entries(node)) {
        if (childKey === 'parent') {
            continue;
        }
        if (Array.isArray(value)) {
            value.forEach((item, itemIndex) => {
                if (isSvelteNode(item)) {
                    visitSvelteNode(item, node, childKey, itemIndex, walker);
                }
            });
        } else if (isSvelteNode(value)) {
            visitSvelteNode(value, node, childKey, null, walker);
        }
    }
    walker.leave?.call(context, node, parent, key, index);
}

function isSvelteNode(value: unknown): value is SvelteNode {
    return typeof value === 'object' && value !== null && 'type' in value &&
        typeof value.type === 'string';
}

export function isAwaitBlock(node: SvelteNode): node is AwaitBlock {
    return node.type === 'AwaitBlock';
}

export function isEachBlock(node: SvelteNode): node is EachBlock {
    return node.type === 'EachBlock';
}
