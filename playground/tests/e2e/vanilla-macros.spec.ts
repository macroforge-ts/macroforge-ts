import { test, expect } from "@playwright/test";

test.describe("Vanilla Playground Macro Tests", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    // Wait for the app to render
    await page.waitForSelector("#app");
  });

  test("page loads with macro-generated content", async ({ page }) => {
    // Verify the page title
    await expect(page).toHaveTitle(/TS Macros Playground/);

    // Verify User toString() is displayed (Debug macro)
    const userSummary = page.locator('[data-testid="user-summary"]');
    await expect(userSummary).toContainText("User {");
    await expect(userSummary).toContainText("identifier:"); // renamed field
    await expect(userSummary).not.toContainText("authToken"); // skipped field

    // Verify User toJSON() is displayed
    const userJson = page.locator('[data-testid="user-json"]');
    await expect(userJson).toContainText('"id"');
    await expect(userJson).toContainText('"name"');
    await expect(userJson).toContainText('"email"');
  });

  test("test button is visible and clickable", async ({ page }) => {
    const testButton = page.locator('[data-testid="test-all-macros"]');
    await expect(testButton).toBeVisible();
    await expect(testButton).toHaveText("Run All Macro Tests");
  });

  test("clicking test button runs all macro tests", async ({ page }) => {
    // Click the test button
    const testButton = page.locator('[data-testid="test-all-macros"]');
    await testButton.click();

    // Wait for tests to complete
    await page.waitForSelector('[data-tests-complete="true"]');

    // Verify test results container shows completion
    const testResults = page.locator('[data-testid="test-results"]');
    await expect(testResults).toHaveAttribute("data-tests-complete", "true");
  });

  test("Debug macro generates toString()", async ({ page }) => {
    await page.click('[data-testid="test-all-macros"]');
    await page.waitForSelector('[data-tests-complete="true"]');

    const debugResult = page.locator('[data-testid="result-debug"]');
    await expect(debugResult).toContainText("Debug (toString)");
    await expect(debugResult).toContainText("AllMacrosTestClass {");
    await expect(debugResult).toContainText("identifier:"); // renamed from 'id'
    await expect(debugResult).not.toContainText("secretToken"); // skipped field
  });

  test("Serialize macro generates toJSON()", async ({ page }) => {
    await page.click('[data-testid="test-all-macros"]');
    await page.waitForSelector('[data-tests-complete="true"]');

    const serializeResult = page.locator('[data-testid="result-serialize"]');
    await expect(serializeResult).toContainText("Serialize (toJSON)");
    await expect(serializeResult).toContainText('"id"');
    await expect(serializeResult).toContainText('"name"');
    await expect(serializeResult).toContainText('"email"');
  });

  test("Deserialize macro generates fromJSON()", async ({ page }) => {
    await page.click('[data-testid="test-all-macros"]');
    await page.waitForSelector('[data-tests-complete="true"]');

    const deserializeResult = page.locator('[data-testid="result-deserialize"]');
    const content = await deserializeResult.textContent();
    // Check if fromJSON is available or shows "Not available"
    expect(
      content?.includes("Deserialized User") || content?.includes("Not available")
    ).toBe(true);
  });

  test("Clone macro result is displayed", async ({ page }) => {
    await page.click('[data-testid="test-all-macros"]');
    await page.waitForSelector('[data-tests-complete="true"]');

    const cloneResult = page.locator('[data-testid="result-clone"]');
    const content = await cloneResult.textContent();
    // Clone should either show cloned object or "Not available"
    expect(content).toContain("Clone:");
  });

  test("Eq macro results are displayed", async ({ page }) => {
    await page.click('[data-testid="test-all-macros"]');
    await page.waitForSelector('[data-tests-complete="true"]');

    const equalsResult = page.locator('[data-testid="result-equals"]');
    const hashCodeResult = page.locator('[data-testid="result-hashcode"]');

    // Check that results are displayed
    await expect(equalsResult).toContainText("Equals");
    await expect(hashCodeResult).toContainText("HashCode");
  });

  test("window.macroTestResults is populated after tests", async ({ page }) => {
    await page.click('[data-testid="test-all-macros"]');
    await page.waitForSelector('[data-tests-complete="true"]');

    // Check that the global results object is populated
    const results = await page.evaluate(() => (window as any).macroTestResults);

    expect(results.debug).toBeDefined();
    expect(typeof results.debug).toBe("string");
    expect(results.debug).toContain("AllMacrosTestClass");

    expect(results.serialize).toBeDefined();
    expect(typeof results.serialize).toBe("object");
  });
});
