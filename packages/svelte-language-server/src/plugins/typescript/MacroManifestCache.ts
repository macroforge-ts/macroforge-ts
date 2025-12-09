import { Logger } from '../../logger';

/**
 * Macro info from the manifest
 */
export interface MacroInfo {
    name: string;
    kind: string;
    description: string;
    package: string;
}

/**
 * Decorator info from the manifest
 */
export interface DecoratorInfo {
    name: string;
    module: string;
    kind: string;
    docs: string;
}

/**
 * Cache for macro manifest data
 * Provides quick lookup for hover info
 */
export class MacroManifestCache {
    private macros: Map<string, MacroInfo> = new Map();
    private decorators: Map<string, DecoratorInfo> = new Map();
    private initialized = false;

    /**
     * Initialize the cache from macroforge native module
     */
    initialize(): void {
        if (this.initialized) {
            return;
        }

        try {
            // eslint-disable-next-line @typescript-eslint/no-var-requires
            const macroforge = require('macroforge');
            if (typeof macroforge.__macroforgeGetManifest !== 'function') {
                Logger.debug('MacroManifestCache: __macroforgeGetManifest not available');
                return;
            }

            const manifest = macroforge.__macroforgeGetManifest();

            // Cache macros
            for (const m of manifest.macros) {
                this.macros.set(m.name.toLowerCase(), {
                    name: m.name,
                    kind: m.kind,
                    description: m.description,
                    package: m.package
                });
            }

            // Cache decorators
            for (const d of manifest.decorators) {
                this.decorators.set(d.export.toLowerCase(), {
                    name: d.export,
                    module: d.module,
                    kind: d.kind,
                    docs: d.docs
                });
            }

            this.initialized = true;
            Logger.log(
                `MacroManifestCache: Loaded ${this.macros.size} macros and ${this.decorators.size} decorators`
            );
        } catch (e) {
            Logger.debug('MacroManifestCache: Failed to load manifest:', e);
        }
    }

    /**
     * Get macro info by name (case-insensitive)
     */
    getMacroInfo(name: string): MacroInfo | undefined {
        this.initialize();
        return this.macros.get(name.toLowerCase());
    }

    /**
     * Get decorator info by name (case-insensitive)
     */
    getDecoratorInfo(name: string): DecoratorInfo | undefined {
        this.initialize();
        return this.decorators.get(name.toLowerCase());
    }

    /**
     * Check if a name is a known macro
     */
    isMacro(name: string): boolean {
        this.initialize();
        return this.macros.has(name.toLowerCase());
    }

    /**
     * Check if a name is a known decorator
     */
    isDecorator(name: string): boolean {
        this.initialize();
        return this.decorators.has(name.toLowerCase());
    }

    /**
     * Get all macro names
     */
    getMacroNames(): string[] {
        this.initialize();
        return Array.from(this.macros.values()).map((m) => m.name);
    }

    /**
     * Get all decorator names
     */
    getDecoratorNames(): string[] {
        this.initialize();
        return Array.from(this.decorators.values()).map((d) => d.name);
    }
}

// Singleton instance
let instance: MacroManifestCache | undefined;

/**
 * Get the singleton MacroManifestCache instance
 */
export function getMacroManifestCache(): MacroManifestCache {
    if (!instance) {
        instance = new MacroManifestCache();
    }
    return instance;
}
