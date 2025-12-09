export interface NavItem {
	title: string;
	href: string;
}

export interface NavSection {
	title: string;
	items: NavItem[];
}

export const navigation: NavSection[] = [
	{
		title: 'Getting Started',
		items: [
			{ title: 'Installation', href: '/docs/getting-started' },
			{ title: 'First Macro', href: '/docs/getting-started/first-macro' }
		]
	},
	{
		title: 'Core Concepts',
		items: [
			{ title: 'How Macros Work', href: '/docs/concepts' },
			{ title: 'The Derive System', href: '/docs/concepts/derive-system' },
			{ title: 'Architecture', href: '/docs/concepts/architecture' }
		]
	},
	{
		title: 'Built-in Macros',
		items: [
			{ title: 'Overview', href: '/docs/builtin-macros' },
			{ title: 'Debug', href: '/docs/builtin-macros/debug' },
			{ title: 'Clone', href: '/docs/builtin-macros/clone' },
			{ title: 'Default', href: '/docs/builtin-macros/default' },
			{ title: 'Hash', href: '/docs/builtin-macros/hash' },
			{ title: 'Ord', href: '/docs/builtin-macros/ord' },
			{ title: 'PartialEq', href: '/docs/builtin-macros/partial-eq' },
			{ title: 'PartialOrd', href: '/docs/builtin-macros/partial-ord' },
			{ title: 'Serialize', href: '/docs/builtin-macros/serialize' },
			{ title: 'Deserialize', href: '/docs/builtin-macros/deserialize' }
		]
	},
	{
		title: 'Custom Macros',
		items: [
			{ title: 'Overview', href: '/docs/custom-macros' },
			{ title: 'Rust Setup', href: '/docs/custom-macros/rust-setup' },
			{ title: '#[ts_macro_derive]', href: '/docs/custom-macros/ts-macro-derive' },
			{ title: 'Template Syntax', href: '/docs/custom-macros/ts-quote' }
		]
	},
	{
		title: 'Integration',
		items: [
			{ title: 'Overview', href: '/docs/integration' },
			{ title: 'CLI', href: '/docs/integration/cli' },
			{ title: 'TypeScript Plugin', href: '/docs/integration/typescript-plugin' },
			{ title: 'Vite Plugin', href: '/docs/integration/vite-plugin' },
			{ title: 'Svelte Preprocessor', href: '/docs/integration/svelte-preprocessor' },
			{ title: 'MCP Server', href: '/docs/integration/mcp-server' },
			{ title: 'Configuration', href: '/docs/integration/configuration' }
		]
	},
	{
		title: 'Language Servers',
		items: [
			{ title: 'Overview', href: '/docs/language-servers' },
			{ title: 'Svelte', href: '/docs/language-servers/svelte' },
			{ title: 'Zed Extensions', href: '/docs/language-servers/zed' }
		]
	},
	{
		title: 'API Reference',
		items: [
			{ title: 'Overview', href: '/docs/api' },
			{ title: 'expandSync()', href: '/docs/api/expand-sync' },
			{ title: 'transformSync()', href: '/docs/api/transform-sync' },
			{ title: 'NativePlugin', href: '/docs/api/native-plugin' },
			{ title: 'PositionMapper', href: '/docs/api/position-mapper' }
		]
	},
	{
		title: 'Roadmap',
		items: [{ title: 'Roadmap', href: '/docs/roadmap' }]
	}
];

// Flatten navigation for prev/next functionality
export function getFlatNavigation(): NavItem[] {
	return navigation.flatMap((section) => section.items);
}

// Get previous and next items for a given path
export function getPrevNext(currentPath: string): { prev: NavItem | null; next: NavItem | null } {
	const flat = getFlatNavigation();
	const currentIndex = flat.findIndex((item) => item.href === currentPath);

	return {
		prev: currentIndex > 0 ? flat[currentIndex - 1] : null,
		next: currentIndex < flat.length - 1 ? flat[currentIndex + 1] : null
	};
}
