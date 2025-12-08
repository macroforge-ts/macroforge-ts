import { test, expect } from "@playwright/test";

test.describe("Svelte Playground Macro Tests", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    // Wait for Svelte to hydrate
    await page.waitForSelector("main.page");
  });

  test("page loads with macro-generated content", async ({ page }) => {
    // Verify MacroUser toString() is displayed
    const userSummary = page.locator('[data-testid="macro-user-summary"]');
    await expect(userSummary).toContainText("MacroUser {");
    await expect(userSummary).toContainText("userId:"); // renamed field

    // Verify MacroUser toJSON() is displayed
    const userJson = page.locator('[data-testid="macro-user-json"]');
    await expect(userJson).toContainText('"id"');
    await expect(userJson).toContainText('"name"');
  });

  test("test button is visible and clickable", async ({ page }) => {
    const testButton = page.locator('[data-testid="test-all-macros"]');
    await expect(testButton).toBeVisible();
    await expect(testButton).toHaveText("Run All Macro Tests");
  });

  test("clicking test button runs all macro tests", async ({ page }) => {
    const testButton = page.locator('[data-testid="test-all-macros"]');
    await testButton.click();

    // Wait for tests to complete
    await page.waitForSelector('[data-tests-complete="true"]');

    // Verify test results container shows completion
    const testResults = page.locator('[data-testid="test-results"]');
    await expect(testResults).toHaveAttribute("data-tests-complete", "true");
  });

  test("Debug macro generates toString() in Svelte", async ({ page }) => {
    await page.click('[data-testid="test-all-macros"]');
    await page.waitForSelector('[data-tests-complete="true"]');

    const debugResult = page.locator('[data-testid="result-debug"]');
    await expect(debugResult).toContainText("Debug (toString)");
    await expect(debugResult).toContainText("SvelteAllMacrosTest {");
    await expect(debugResult).toContainText("testId:"); // renamed from 'id'
    await expect(debugResult).not.toContainText("apiKey"); // skipped field
  });

  test("Serialize macro generates toJSON() in Svelte", async ({ page }) => {
    await page.click('[data-testid="test-all-macros"]');
    await page.waitForSelector('[data-tests-complete="true"]');

    const serializeResult = page.locator('[data-testid="result-serialize"]');
    await expect(serializeResult).toContainText("Serialize (toJSON)");
    await expect(serializeResult).toContainText('"id"');
    await expect(serializeResult).toContainText('"title"');
  });

  test("Deserialize macro result is displayed in Svelte", async ({ page }) => {
    await page.click('[data-testid="test-all-macros"]');
    await page.waitForSelector('[data-tests-complete="true"]');

    const deserializeResult = page.locator('[data-testid="result-deserialize"]');
    const content = await deserializeResult.textContent();
    expect(
      content?.includes("Deserialized") || content?.includes("Not available")
    ).toBe(true);
  });

  test("all macro results are displayed after button click", async ({ page }) => {
    await page.click('[data-testid="test-all-macros"]');
    await page.waitForSelector('[data-tests-complete="true"]');

    // Verify all result sections are rendered
    const resultSelectors = [
      '[data-testid="result-debug"]',
      '[data-testid="result-clone"]',
      '[data-testid="result-equals"]',
      '[data-testid="result-hashcode"]',
      '[data-testid="result-serialize"]',
      '[data-testid="result-deserialize"]',
    ];

    for (const selector of resultSelectors) {
      await expect(page.locator(selector)).toBeVisible();
    }
  });

  test("Clone macro result is displayed in Svelte", async ({ page }) => {
    await page.click('[data-testid="test-all-macros"]');
    await page.waitForSelector('[data-tests-complete="true"]');

    const cloneResult = page.locator('[data-testid="result-clone"]');
    const content = await cloneResult.textContent();
    expect(content).toContain("Clone:");
  });

  test("Eq macro results are displayed in Svelte", async ({ page }) => {
    await page.click('[data-testid="test-all-macros"]');
    await page.waitForSelector('[data-tests-complete="true"]');

    const equalsResult = page.locator('[data-testid="result-equals"]');
    const hashCodeResult = page.locator('[data-testid="result-hashcode"]');

    await expect(equalsResult).toContainText("Equals");
    await expect(hashCodeResult).toContainText("HashCode");
  });
});
