import { visit } from 'unist-util-visit';

/**
 * Escape code for use in a Svelte expression.
 * Uses JSON.stringify to properly escape all special characters.
 * @param {string} str - String to escape
 * @returns {string} - JSON-encoded string (including quotes)
 */
function escapeForSvelte(str) {
	return JSON.stringify(str);
}

/**
 * Generate script import for components used in the document.
 * @param {boolean} hasMacroExample - Whether MacroExample is used
 * @param {boolean} hasInteractiveMacro - Whether InteractiveMacro is used
 * @returns {string} - Script tag with imports
 */
function generateImportScript(hasMacroExample, hasInteractiveMacro) {
	const imports = [];
	if (hasMacroExample) {
		imports.push("import MacroExample from '$lib/components/ui/MacroExample.svelte'");
	}
	if (hasInteractiveMacro) {
		imports.push("import InteractiveMacro from '$lib/components/ui/InteractiveMacro.svelte'");
	}
	if (imports.length === 0) return '';
	return `<script>\n${imports.join(';\n')};\n</script>\n\n`;
}

/**
 * @typedef {Object} ReplacementEntry
 * @property {any} parent - Parent node
 * @property {number} index - Index in parent children
 * @property {any} node - Replacement node
 */

/**
 * Remark plugin to transform consecutive before/after code blocks into MacroExample components.
 *
 * Input:
 * ```ts before
 * code
 * ```
 *
 * ```ts after
 * code
 * ```
 *
 * Output:
 * <MacroExample before={`code`} after={`code`} />
 * @returns {(tree: any) => void}
 */
export function remarkMacroBlocks() {
	return (/** @type {any} */ tree) => {
		/** @type {Set<any>} */
		const nodesToRemove = new Set();
		/** @type {ReplacementEntry[]} */
		const replacements = [];
		let hasMacroExample = false;
		let hasInteractiveMacro = false;

		visit(tree, 'code', (/** @type {any} */ node, /** @type {number | undefined} */ index, /** @type {any} */ parent) => {
			if (!parent || index === undefined || nodesToRemove.has(node)) return;

			const meta = node.meta || '';
			const metaLower = meta.toLowerCase();

			// Check if this is a "before" block
			if (metaLower.includes('before')) {
				// Look for the next code block that's an "after" block
				let afterIndex = index + 1;
				/** @type {any} */
				let afterNode = null;

				// Skip any non-code nodes (like empty paragraphs) between before and after
				while (afterIndex < parent.children.length) {
					const candidate = parent.children[afterIndex];

					if (candidate.type === 'code') {
						const candidateMeta = (candidate.meta || '').toLowerCase();
						if (candidateMeta.includes('after')) {
							afterNode = candidate;
							break;
						} else {
							// Found another code block that's not an "after" - stop looking
							break;
						}
					} else if (candidate.type === 'paragraph' || candidate.type === 'text') {
						// Skip whitespace/empty paragraphs
						afterIndex++;
					} else {
						// Stop if we hit any other node type
						break;
					}
				}

				if (afterNode) {
					const beforeCode = escapeForSvelte(node.value);
					const afterCode = escapeForSvelte(afterNode.value);

					// Mark both nodes for removal
					nodesToRemove.add(node);
					nodesToRemove.add(afterNode);

					// Create a replacement HTML node with the MacroExample component
					// JSON.stringify already includes quotes, so we use {beforeCode} directly
					replacements.push({
						parent,
						index,
						node: {
							type: 'html',
							value: `<MacroExample before={${beforeCode}} after={${afterCode}} />`
						}
					});
					hasMacroExample = true;
				}
			}

			// Handle interactive flag - convert to InteractiveMacro
			if (metaLower.includes('interactive') && !nodesToRemove.has(node)) {
				const code = escapeForSvelte(node.value);
				nodesToRemove.add(node);

				replacements.push({
					parent,
					index,
					node: {
						type: 'html',
						value: `<InteractiveMacro code={${code}} />`
					}
				});
				hasInteractiveMacro = true;
			}
		});

		// Apply replacements in reverse order to preserve indices
		replacements.sort((a, b) => b.index - a.index);

		for (const { parent, index, node } of replacements) {
			// Find all nodes to remove between the before and the replacement
			/** @type {number[]} */
			const toRemove = [];
			for (let i = index; i < parent.children.length; i++) {
				if (nodesToRemove.has(parent.children[i])) {
					toRemove.push(i);
				} else if (toRemove.length > 0) {
					// Stop after we've passed the last node to remove
					break;
				}
			}

			// Remove nodes and insert replacement
			if (toRemove.length > 0) {
				const removeCount = toRemove[toRemove.length - 1] - toRemove[0] + 1;
				parent.children.splice(toRemove[0], removeCount, node);
			}
		}

		// Add import script at the beginning if components are used
		if (hasMacroExample || hasInteractiveMacro) {
			const importScript = generateImportScript(hasMacroExample, hasInteractiveMacro);
			tree.children.unshift({
				type: 'html',
				value: importScript
			});
		}
	};
}

export default remarkMacroBlocks;
