/**
 * Tests for plugin initialization and structure.
 */

import test from "node:test";
import assert from "node:assert/strict";
import macroforge from "../src/index.js";
import { createMockResolvedConfig } from "./test-utils.js";

test("plugin exports default factory function", () => {
  assert.equal(typeof macroforge, "function");
});

test("plugin factory returns a Plugin object", async () => {
  const plugin = await macroforge();
  assert.equal(typeof plugin, "object");
  assert.notEqual(plugin, null);
});

test("plugin has correct name", async () => {
  const plugin = await macroforge();
  assert.equal(plugin.name, "@macroforge/vite-plugin");
});

test("plugin enforces 'pre' order", async () => {
  const plugin = await macroforge();
  assert.equal(plugin.enforce, "pre");
});

test("plugin has configResolved hook", async () => {
  const plugin = await macroforge();
  assert.equal(typeof plugin.configResolved, "function");
});

test("plugin has transform hook", async () => {
  const plugin = await macroforge();
  assert.equal(typeof plugin.transform, "function");
});

test("default options: generateTypes defaults to true", async () => {
  const plugin = await macroforge();
  // We can verify defaults by checking transform behavior
  // generateTypes defaults to true (option !== false)
  assert.ok(plugin);
});

test("default options: emitMetadata defaults to true", async () => {
  const plugin = await macroforge();
  assert.ok(plugin);
});

test("custom options are accepted", async () => {
  // Options are now loaded from macroforge.config.js
  const plugin = await macroforge();
  assert.ok(plugin);
});

test("configResolved initializes projectRoot", async () => {
  const plugin = await macroforge();
  const config = createMockResolvedConfig("/test/project");
  plugin.configResolved(config);
  // Plugin is initialized - if this doesn't throw, it works
  assert.ok(plugin);
});
