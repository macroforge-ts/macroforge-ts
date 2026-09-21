<script lang="ts">
    // The GitLab tanuki, as a single filled path so it takes `currentColor`
    // from whatever links to it.
    let { class: className = "h-5 w-5" }: { class?: string } = $props();
</script>

<svg
    class={className}
    fill="currentColor"
    viewBox="0 0 24 24"
    role="img"
    aria-hidden="true"
>
    <path
        d="M23.955 13.587l-1.342-4.135-2.664-8.189a.455.455 0 00-.867 0L16.418 9.45H7.582L4.919 1.263a.455.455 0 00-.867 0L1.388 9.452.045 13.587a.924.924 0 00.331 1.023L12 23.054l11.624-8.443a.92.92 0 00.331-1.024"
    />
</svg>
