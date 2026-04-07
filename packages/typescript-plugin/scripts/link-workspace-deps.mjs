import { lstat, mkdir, rm, symlink } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(scriptDir, '..');

const links = [
    ['macroforge', path.resolve(packageRoot, '../../crates/macroforge_ts')],
    ['@macroforge/shared', path.resolve(packageRoot, '../shared')]
];

async function pathExists(target) {
    try {
        await lstat(target);
        return true;
    } catch {
        return false;
    }
}

async function linkDependency(specifier, target) {
    if (!(await pathExists(target))) {
        return;
    }

    const linkPath = path.resolve(packageRoot, 'node_modules', ...specifier.split('/'));
    await mkdir(path.dirname(linkPath), { recursive: true });
    await rm(linkPath, { force: true, recursive: true });
    await symlink(target, linkPath, 'dir');
}

for (const [specifier, target] of links) {
    await linkDependency(specifier, target);
}
