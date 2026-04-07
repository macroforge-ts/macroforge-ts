<script lang="ts">
    import { siteConfig } from "$lib/config/site";
    const currentYear = new Date().getFullYear();
    const copyrightStatement =
        currentYear === 2025 ? "2025" : `2025 - ${currentYear}`;
</script>

<footer class="border-t border-border bg-muted">
    <div class="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div
            class="flex flex-col md:flex-row items-center justify-between gap-4"
        >
            <div class="flex items-center gap-2 text-sm text-muted-foreground">
                <span
                    >&copy; {copyrightStatement}
                    {siteConfig.name}.</span
                >
            </div>

            <div class="flex items-center gap-6">
                <a
                    href={siteConfig.links.github}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-sm text-muted-foreground hover:text-foreground transition-colors"
                >
                    GitHub
                </a>
                <a
                    href={siteConfig.links.npm}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-sm text-muted-foreground hover:text-foreground transition-colors"
                >
                    npm
                </a>
                <a
                    href="{siteConfig.links.github}/blob/main/LICENSE"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-sm text-muted-foreground hover:text-foreground transition-colors"
                >
                    MIT License
                </a>
            </div>
        </div>
    </div>
</footer>
