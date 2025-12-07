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
			{ title: 'Eq', href: '/docs/builtin-macros/eq' }
		]
	},
	{
		title: 'Custom Macros',
		items: [
			{ title: 'Overview', href: '/docs/custom-macros' },
			{ title: 'Rust Setup', href: '/docs/custom-macros/rust-setup' },
			{ title: 'ts_macro_derive', href: '/docs/custom-macros/ts-macro-derive' },
			{ title: 'Template Syntax', href: '/docs/custom-macros/ts-quote' }
		]
	},
	{
		title: 'Integration',
		items: [
			{ title: 'Overview', href: '/docs/integration' },
			{ title: 'TypeScript Plugin', href: '/docs/integration/typescript-plugin' },
			{ title: 'Vite Plugin', href: '/docs/integration/vite-plugin' },
			{ title: 'Configuration', href: '/docs/integration/configuration' }
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
