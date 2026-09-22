/**
 * @module @macroforge/shared
 *
 * Shared utilities for Macroforge plugins.
 *
 * This package provides common functionality used by both `@macroforge/vite-plugin`
 * and `@macroforge/typescript-plugin`, ensuring consistent behavior across
 * different build tools.
 *
 * @packageDocumentation
 */

export {
    clearExternalManifestCache,
    type DecoratorManifestEntry,
    type ExpandOptions,
    getExternalDecoratorInfo,
    getExternalMacroInfo,
    getExternalManifest,
    type MacroManifest,
    type MacroManifestEntry,
    type RequireFunction
} from './external-manifest.ts';

export {
    type CfgFlags,
    CONFIG_FILES,
    type ConfigLoader,
    type ConfigLoadResult,
    type DeprecatedConfig,
    findConfigFile,
    loadMacroConfig,
    type MacroConfig,
    type MustUseConfig,
    type NonExhaustiveConfig,
    type VitePluginConfig
} from './config.ts';
