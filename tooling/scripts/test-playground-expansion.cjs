#!/usr/bin/env node

/**
 * Test playground macro expansion.
 *
 * This script:
 * 1. Expands macros in playground TypeScript files using the CLI
 * 2. Formats expanded files with biome (validates code correctness)
 * 3. Runs macroforge tsc wrapper for type checking
 *
 * Exit code 1 if any step fails.
 */

const { program } = require('commander');
const fs = require("node:fs");
const path = require("node:path");
const { execSync } = require("node:child_process");

program
	.name('test-playground-expansion')
	.description('Test playground macro expansion, formatting, and type checking');

const rootDir = path.join(__dirname, "..");

// Path to the CLI binary (release preferred)
const cliBinary = (() => {
	const release = path.join(rootDir, "crates", "target", "release", "macroforge");
	const debug = path.join(rootDir, "crates", "target", "debug", "macroforge");
	if (fs.existsSync(release)) return release;
	if (fs.existsSync(debug)) return debug;
	console.error("ERROR: macroforge CLI binary not found. Run `cargo build --release -p macroforge_ts` first.");
	process.exit(1);
})();

// Playground configurations
const playgrounds = [
	{
		name: "svelte",
		dir: path.join(rootDir, "playground", "svelte"),
		scanDirs: [
			path.join(rootDir, "playground", "svelte", "src", "lib", "demo"),
			path.join(rootDir, "playground", "svelte", "src", "lib", "demo", "types"),
		],
	},
	{
		name: "vanilla",
		dir: path.join(rootDir, "playground", "vanilla"),
		scanDirs: [
			path.join(rootDir, "playground", "vanilla", "src"),
		],
	},
];

function run(cmd, cwd, description) {
	console.log(`  ${description}...`);
	try {
		execSync(cmd, { cwd, stdio: "pipe" });
		return { success: true };
	} catch (err) {
		const stderr = err.stderr?.toString() || "";
		const stdout = err.stdout?.toString() || "";
		return { success: false, output: stderr || stdout || err.message };
	}
}

function main() {
	console.log("Testing playground macro expansion\n");
	console.log(`Using CLI: ${cliBinary}\n`);

	let hasErrors = false;

	for (const playground of playgrounds) {
		if (!fs.existsSync(playground.dir)) {
			console.log(`Skipping ${playground.name} (not found)`);
			continue;
		}

		console.log(`[${playground.name}]`);

		// Step 1: Expand macros
		for (const scanDir of playground.scanDirs) {
			if (!fs.existsSync(scanDir)) continue;
			const result = run(
				`"${cliBinary}" expand --scan "${scanDir}"`,
				rootDir,
				`Expanding ${path.relative(rootDir, scanDir)}`
			);
			if (!result.success) {
				console.error(`\n  ERROR: Expansion failed`);
				console.error(result.output.split('\n').map(l => '    ' + l).join('\n'));
				hasErrors = true;
			}
		}

		// Step 2: Format with biome
		const formatResult = run(
			"npx biome format --write src",
			playground.dir,
			"Formatting expanded files"
		);
		if (!formatResult.success) {
			console.error(`\n  ERROR: Formatting failed`);
			console.error(formatResult.output.split('\n').map(l => '    ' + l).join('\n'));
			hasErrors = true;
		}

		// Step 3: Type check with macroforge tsc
		const tsconfigPath = path.join(playground.dir, "tsconfig.json");
		if (fs.existsSync(tsconfigPath)) {
			const tscResult = run(
				`"${cliBinary}" tsc -p "${tsconfigPath}"`,
				playground.dir,
				"Type checking"
			);
			if (!tscResult.success) {
				console.error(`\n  ERROR: Type checking failed`);
				console.error(tscResult.output.split('\n').map(l => '    ' + l).join('\n'));
				hasErrors = true;
			}
		} else {
			console.log(`  Skipping type check (no tsconfig.json)`);
		}

		console.log();
	}

	if (hasErrors) {
		console.error("FAILED: Some tests failed");
		process.exit(1);
	} else {
		console.log("PASSED: All tests passed");
	}
}

program.action(main);
program.parse();
