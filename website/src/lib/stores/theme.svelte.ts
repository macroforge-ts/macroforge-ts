import { browser } from '$app/environment';

function createThemeStore() {
	let theme = $state<'light' | 'dark'>('light');

	if (browser) {
		const stored = localStorage.getItem('theme') as 'light' | 'dark' | null;
		const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
		theme = stored ?? (prefersDark ? 'dark' : 'light');
	}

	function toggle() {
		theme = theme === 'light' ? 'dark' : 'light';
		if (browser) {
			localStorage.setItem('theme', theme);
			document.documentElement.classList.toggle('dark', theme === 'dark');
		}
	}

	function set(value: 'light' | 'dark') {
		theme = value;
		if (browser) {
			localStorage.setItem('theme', theme);
			document.documentElement.classList.toggle('dark', theme === 'dark');
		}
	}

	return {
		get current() {
			return theme;
		},
		get isDark() {
			return theme === 'dark';
		},
		toggle,
		set
	};
}

export const themeStore = createThemeStore();
