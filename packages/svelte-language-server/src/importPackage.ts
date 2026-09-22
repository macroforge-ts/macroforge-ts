import { createRequire } from 'module';
import { dirname, resolve } from 'path';
import * as prettier from 'prettier';
import * as svelte from 'svelte/compiler';
import { fileURLToPath } from 'url';
import { Logger } from './logger.ts';

/** Loads and resolves modules the way `require` does from this package. */
export const packageRequire = createRequire(import.meta.url);

/** A directory inside this package: the fallback root for its bundled dependencies. */
export const packageDir = dirname(fileURLToPath(import.meta.url));

/**
 * Whether or not the current workspace can be trusted.
 * TODO rework this to a class which depends on the LsConfigManager
 * and inject that class into all places where it's needed (Document etc.)
 */
let isTrusted = true;

export function setIsTrusted(_isTrusted: boolean) {
    isTrusted = _isTrusted;
}

/**
 * This function encapsulates the require call in one place
 * so we can replace its content inside rollup builds
 * so it's not transformed.
 */
function dynamicRequire(dynamicFileToRequire: string): any {
    // prettier-ignore
    return packageRequire(dynamicFileToRequire);
}

function getPackageInfo(
    packageName: string,
    fromPath: string,
    use_fallback = true
) {
    const paths: string[] = [];
    if (isTrusted) {
        paths.push(fromPath);
    }
    if (use_fallback) {
        paths.push(packageDir);
    }

    const packageJSONPath = packageRequire.resolve(`${packageName}/package.json`, {
        paths
    });
    const { version } = dynamicRequire(packageJSONPath);
    const [major, minor, patch] = version.split('.');

    return {
        path: dirname(packageJSONPath),
        version: {
            full: version,
            major: Number(major),
            minor: Number(minor),
            patch: Number(patch)
        }
    };
}

function importPrettier(fromPath: string): typeof prettier {
    const pkg = packageLoader.getPackageInfo('prettier', fromPath);
    const main = resolve(pkg.path);
    Logger.debug('Using Prettier v' + pkg.version.full, 'from', main);
    return dynamicRequire(main);
}

function importSvelte(fromPath: string): typeof svelte {
    const pkg = packageLoader.getPackageInfo('svelte', fromPath);
    const main = resolve(pkg.path, 'compiler');
    Logger.debug('Using Svelte v' + pkg.version.full, 'from', main);
    if (pkg.version.major === 4) {
        return dynamicRequire(main + '.cjs');
    } else {
        return dynamicRequire(main);
    }
}

/** Can throw because no fallback guaranteed */
function importSveltePreprocess(fromPath: string): any {
    const pkg = packageLoader.getPackageInfo(
        'svelte-preprocess',
        fromPath,
        false // svelte-language-server doesn't have a dependency on svelte-preprocess so we can't provide a fallback
    );
    const main = resolve(pkg.path);
    Logger.debug('Using svelte-preprocess v' + pkg.version.full, 'from', main);
    return dynamicRequire(main);
}

/**
 * Locates and loads the packages the language server takes from the user's
 * workspace. Every caller goes through this object, so a lookup can be
 * replaced as a whole.
 */
export const packageLoader = {
    getPackageInfo,
    importPrettier,
    importSvelte,
    importSveltePreprocess
};
