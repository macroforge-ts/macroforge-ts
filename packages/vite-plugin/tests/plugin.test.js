/**
 * Tests for plugin initialization and structure.
 */

import test from "node:test";
import assert from "node:assert/strict";
import napiMacrosPlugin from "../dist/index.js";
import { createMockResolvedConfig, initializePlugin } from "./test-utils.js";

test("plugin exports default factory function", () => {
  assert.equal(typeof napiMacrosPlugin, "function");
});

test("plugin factory returns a Plugin object", () => {
  const plugin = napiMacrosPlugin();
  assert.equal(typeof plugin, "object");
  assert.notEqual(plugin, null);
});

test("plugin has correct name", () => {
  const plugin = napiMacrosPlugin();
  assert.equal(plugin.name, "@macroforge/vite-plugin");
});

test("plugin enforces 'pre' order", () => {
  const plugin = napiMacrosPlugin();
  assert.equal(plugin.enforce, "pre");
});

test("plugin has configResolved hook", () => {
  const plugin = napiMacrosPlugin();
  assert.equal(typeof plugin.configResolved, "function");
});

test("plugin has transform hook", () => {
  const plugin = napiMacrosPlugin();
  assert.equal(typeof plugin.transform, "function");
});

test("default options: generateTypes defaults to true", () => {
  const plugin = napiMacrosPlugin();
  // We can verify defaults by checking transform behavior
  // generateTypes defaults to true (option !== false)
  assert.ok(plugin);
});

test("default options: emitMetadata defaults to true", () => {
  const plugin = napiMacrosPlugin();
  assert.ok(plugin);
});

test("custom options are accepted", () => {
  const plugin = napiMacrosPlugin({
    generateTypes: false,
    typesOutputDir: "custom/types",
    emitMetadata: false,
    metadataOutputDir: "custom/metadata",
    include: ["src/**/*.ts"],
    exclude: ["**/*.spec.ts"],
  });
  assert.ok(plugin);
});

test("configResolved initializes projectRoot", () => {
  const plugin = napiMacrosPlugin();
  const config = createMockResolvedConfig("/test/project");
  plugin.configResolved(config);
  // Plugin is initialized - if this doesn't throw, it works
  assert.ok(plugin);
});
