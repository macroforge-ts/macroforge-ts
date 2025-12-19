#!/usr/bin/env node

/**
 * Diagnostic script for manually inspecting macro expansion output
 * This is NOT part of the automated test suite - it's for debugging/development
 */

const { program } = require('commander');
const { root, resolve } = require("./env.cjs");
const { expandSync } = require(resolve("crates", "macroforge_ts"));
const fs = require("fs");

program
	.name('test-plugin-expansion')
	.description('Diagnostic tool for manually inspecting macro expansion output');

function main() {
  // Test the User class from vanilla playground
  const userPath = resolve("tooling", "playground", "vanilla", "src", "user.ts");
  const userContent = fs.readFileSync(userPath, "utf8");

  console.log("=== Testing User.ts expansion ===\n");
  console.log("Original content:");
  console.log(userContent);
  console.log("\n--- Running expandSync ---\n");

  try {
    const result = expandSync(userContent, "user.ts");

    console.log("Expansion result:");
    console.log("- Has types:", !!result.types);
    console.log("- Types length:", result.types?.length || 0);
    console.log("- Diagnostics count:", result.diagnostics?.length || 0);
    console.log("- Diagnostics:", JSON.stringify(result.diagnostics, null, 2));

    if (result.types) {
      console.log("\n--- Expanded Types Output ---");
      console.log(result.types);
      console.log("\n--- Line-by-line comparison ---");
      const originalLines = userContent.split("\n");
      const expandedLines = result.types.split("\n");

      console.log(`Original: ${originalLines.length} lines`);
      console.log(`Expanded: ${expandedLines.length} lines`);
      console.log(
        `Difference: ${expandedLines.length - originalLines.length} lines`,
      );

      // Show line differences
      const maxLines = Math.max(originalLines.length, expandedLines.length);
      for (let i = 0; i < Math.min(15, maxLines); i++) {
        const orig = originalLines[i] || "";
        const exp = expandedLines[i] || "";
        if (orig !== exp) {
          console.log(`\nLine ${i + 1} differs:`);
          console.log(`  Original: ${orig}`);
          console.log(`  Expanded: ${exp}`);
        }
      }
    } else {
      console.log("\nNo types output generated!");
    }
  } catch (e) {
    console.error("Expansion failed:", e);
  }

  // Test MacroUser from svelte playground
  console.log("\n\n=== Testing MacroUser.ts expansion ===\n");
  const macroUserPath = resolve("tooling", "playground", "svelte", "src", "lib", "demo", "macro-user.ts");
  const macroUserContent = fs.readFileSync(macroUserPath, "utf8");

  try {
    const result = expandSync(macroUserContent, "macro-user.ts");

    console.log("Expansion result:");
    console.log("- Has types:", !!result.types);
    console.log("- Types length:", result.types?.length || 0);
    console.log("- Diagnostics count:", result.diagnostics?.length || 0);

    if (result.diagnostics && result.diagnostics.length > 0) {
      console.log("\n--- Diagnostics ---");
      result.diagnostics.forEach((d, i) => {
        console.log(`${i + 1}. [${d.level}] ${d.message}`);
        if (d.start !== undefined) {
          console.log(`   Position: ${d.start} - ${d.end}`);
        }
      });
    }
  } catch (e) {
    console.error("Expansion failed:", e);
  }
}

program.action(main);
program.parse();
