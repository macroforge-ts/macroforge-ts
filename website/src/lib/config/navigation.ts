import { resolve } from '$app/paths';
import type { Pathname, ResolvedPathname } from '$app/types';

export interface NavItem {
	title: string;
	href: Pathname;
}

export interface NavSection {
	title: string;
	items: NavItem[];
}

// Type-safe path resolution using switch for literal narrowing
export function resolvePath(path: Pathname): ResolvedPathname {
	switch (path) {
		// Getting Started
		case '/docs/getting-started':
			return resolve('/docs/getting-started');
		case '/docs/getting-started/first-macro':
			return resolve('/docs/getting-started/first-macro');

		// Core Concepts
		case '/docs/concepts':
			return resolve('/docs/concepts');
		case '/docs/concepts/derive-system':
			return resolve('/docs/concepts/derive-system');
		case '/docs/concepts/architecture':
			return resolve('/docs/concepts/architecture');

		// Built-in Macros
		case '/docs/builtin-macros':
			return resolve('/docs/builtin-macros');
		case '/docs/builtin-macros/debug':
			return resolve('/docs/builtin-macros/debug');
		case '/docs/builtin-macros/clone':
			return resolve('/docs/builtin-macros/clone');
		case '/docs/builtin-macros/default':
			return resolve('/docs/builtin-macros/default');
		case '/docs/builtin-macros/hash':
			return resolve('/docs/builtin-macros/hash');
		case '/docs/builtin-macros/ord':
			return resolve('/docs/builtin-macros/ord');
		case '/docs/builtin-macros/partial-eq':
			return resolve('/docs/builtin-macros/partial-eq');
		case '/docs/builtin-macros/partial-ord':
			return resolve('/docs/builtin-macros/partial-ord');
		case '/docs/builtin-macros/serialize':
			return resolve('/docs/builtin-macros/serialize');
		case '/docs/builtin-macros/deserialize':
			return resolve('/docs/builtin-macros/deserialize');

		// Custom Macros
		case '/docs/custom-macros':
			return resolve('/docs/custom-macros');
		case '/docs/custom-macros/rust-setup':
			return resolve('/docs/custom-macros/rust-setup');
		case '/docs/custom-macros/ts-macro-derive':
			return resolve('/docs/custom-macros/ts-macro-derive');
		case '/docs/custom-macros/ts-quote':
			return resolve('/docs/custom-macros/ts-quote');

		// Integration
		case '/docs/integration':
			return resolve('/docs/integration');
		case '/docs/integration/cli':
			return resolve('/docs/integration/cli');
		case '/docs/integration/typescript-plugin':
			return resolve('/docs/integration/typescript-plugin');
		case '/docs/integration/vite-plugin':
			return resolve('/docs/integration/vite-plugin');
		case '/docs/integration/svelte-preprocessor':
			return resolve('/docs/integration/svelte-preprocessor');
		case '/docs/integration/mcp-server':
			return resolve('/docs/integration/mcp-server');
		case '/docs/integration/configuration':
			return resolve('/docs/integration/configuration');

		// Language Servers
		case '/docs/language-servers':
			return resolve('/docs/language-servers');
		case '/docs/language-servers/svelte':
			return resolve('/docs/language-servers/svelte');
		case '/docs/language-servers/zed':
			return resolve('/docs/language-servers/zed');

		// API Reference
		case '/docs/api':
			return resolve('/docs/api');
		case '/docs/api/expand-sync':
			return resolve('/docs/api/expand-sync');
		case '/docs/api/transform-sync':
			return resolve('/docs/api/transform-sync');
		case '/docs/api/native-plugin':
			return resolve('/docs/api/native-plugin');
		case '/docs/api/position-mapper':
			return resolve('/docs/api/position-mapper');

		// Roadmap
		case '/docs/roadmap':
			return resolve('/docs/roadmap');

		// Root
		case '/':
			return resolve('/');

		default:
			throw new Error(`Unknown route: ${path}`);
	}
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
