/**
 * Runs the plugin at argv[2] over the source file at argv[3], with the cwd as
 * the project root. Spawned in a directory whose `typescript` has no compiler
 * API, which the plugin under test cannot see from inside this repository.
 */
import fs from 'node:fs';
import { pathToFileURL } from 'node:url';

const { default: macroforge } = await import(pathToFileURL(process.argv[2]).href);
const plugin = await macroforge();
plugin.configResolved({ root: process.cwd(), command: 'build', mode: 'production' });

const context = {
    error(message) {
        throw new Error(message);
    },
    warn(message) {
        console.warn(message);
    }
};
const result = await plugin.transform.call(
    context,
    fs.readFileSync(process.argv[3], 'utf8'),
    process.argv[3]
);
process.stdout.write(result?.code ?? '');
