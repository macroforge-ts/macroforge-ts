<script lang="ts">
	interface Props {
		size?: 'sm' | 'md' | 'lg';
		showText?: boolean;
	}

	let { size = 'md', showText = true }: Props = $props();

	const sizes = {
		sm: 'h-6 w-6',
		md: 'h-8 w-8',
		lg: 'h-10 w-10'
	};

	const textSizes = {
		sm: 'text-base',
		md: 'text-lg',
		lg: 'text-xl'
	};
</script>

<div class="flex items-center gap-2">
	<!-- Forge Flame + Sparkle Icon -->
	<div class="flex items-center justify-center rounded-lg bg-primary text-primary-foreground {sizes[size]}">
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="w-full h-full"
		>
			<!-- Flame -->
			<path d="M12 3q1 4 4 6.5t3 5.5a1 1 0 0 1-14 0 5 5 0 0 1 1-3 1 1 0 0 0 5 0c0-2-1.5-3-1.5-5q0-2 2.5-4" />
			<!-- Small 4-point sparkle (top right) -->
			<path d="M19.5 2l0.25 1.3a0.5 0.5 0 0 0 0.4 0.4l1.3 0.25-1.3 0.25a0.5 0.5 0 0 0-0.4 0.4L19.5 5.9l-0.25-1.3a0.5 0.5 0 0 0-0.4-0.4L17.55 3.95l1.3-0.25a0.5 0.5 0 0 0 0.4-0.4z" stroke-width="1.2" />
			<!-- Hollow circle (top left) -->
			<circle cx="4.5" cy="4" r="1.5" fill="none" />
		</svg>
	</div>

	{#if showText}
		<span class="font-semibold text-foreground {textSizes[size]}">
			Macroforge
		</span>
	{/if}
</div>
