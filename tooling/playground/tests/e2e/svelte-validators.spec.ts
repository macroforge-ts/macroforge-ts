import { test, expect } from "@playwright/test";

test.describe("Svelte Validator Form E2E Tests", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/validator-form");
    // Wait for SvelteKit hydration to complete before interacting with forms
    await page.waitForSelector("body.hydrated", { timeout: 10000 });
    await page.waitForSelector('[data-testid="user-registration-form"]');
  });

  test.describe("User Registration Form", () => {
    test("page loads with user registration form", async ({ page }) => {
      await expect(page).toHaveTitle(/Validator Form/);
      await expect(page.locator('[data-testid="user-registration-form"]')).toBeVisible();
    });

    test("validates valid user registration data", async ({ page }) => {
      // Fill in valid data
      await page.fill('[data-testid="user-email"]', "test@example.com");
      await page.fill('[data-testid="user-username"]', "johndoe123");
      await page.fill('[data-testid="user-password"]', "securepassword123");
      await page.fill('[data-testid="user-age"]', "25");
      await page.fill('[data-testid="user-website"]', "https://example.com");

      // Submit form
      await page.click('[data-testid="submit-user-registration"]');

      // Wait for result
      const result = page.locator('[data-testid="user-registration-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "true");
      await expect(result).toHaveClass(/success/);
    });

    test("rejects invalid email", async ({ page }) => {
      await page.fill('[data-testid="user-email"]', "not-an-email");
      await page.fill('[data-testid="user-username"]', "johndoe123");
      await page.fill('[data-testid="user-password"]', "securepassword123");
      await page.fill('[data-testid="user-age"]', "25");
      await page.fill('[data-testid="user-website"]', "https://example.com");

      await page.click('[data-testid="submit-user-registration"]');

      const result = page.locator('[data-testid="user-registration-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("email");
    });

    test("rejects password too short", async ({ page }) => {
      await page.fill('[data-testid="user-email"]', "test@example.com");
      await page.fill('[data-testid="user-username"]', "johndoe123");
      await page.fill('[data-testid="user-password"]', "short"); // Only 5 chars
      await page.fill('[data-testid="user-age"]', "25");
      await page.fill('[data-testid="user-website"]', "https://example.com");

      await page.click('[data-testid="submit-user-registration"]');

      const result = page.locator('[data-testid="user-registration-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("at least 8");
    });

    test("rejects username with uppercase letters", async ({ page }) => {
      await page.fill('[data-testid="user-email"]', "test@example.com");
      await page.fill('[data-testid="user-username"]', "JohnDoe"); // Uppercase not allowed
      await page.fill('[data-testid="user-password"]', "securepassword123");
      await page.fill('[data-testid="user-age"]', "25");
      await page.fill('[data-testid="user-website"]', "https://example.com");

      await page.click('[data-testid="submit-user-registration"]');

      const result = page.locator('[data-testid="user-registration-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("lowercase");
    });

    test("rejects age below minimum", async ({ page }) => {
      await page.fill('[data-testid="user-email"]', "test@example.com");
      await page.fill('[data-testid="user-username"]', "johndoe123");
      await page.fill('[data-testid="user-password"]', "securepassword123");
      await page.fill('[data-testid="user-age"]', "15"); // Below 18
      await page.fill('[data-testid="user-website"]', "https://example.com");

      await page.click('[data-testid="submit-user-registration"]');

      const result = page.locator('[data-testid="user-registration-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("between");
    });

    test("rejects invalid URL", async ({ page }) => {
      await page.fill('[data-testid="user-email"]', "test@example.com");
      await page.fill('[data-testid="user-username"]', "johndoe123");
      await page.fill('[data-testid="user-password"]', "securepassword123");
      await page.fill('[data-testid="user-age"]', "25");
      await page.fill('[data-testid="user-website"]', "not-a-url");

      await page.click('[data-testid="submit-user-registration"]');

      const result = page.locator('[data-testid="user-registration-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("URL");
    });
  });

  test.describe("Product Form", () => {
    test("validates valid product data", async ({ page }) => {
      await page.fill('[data-testid="product-name"]', "Awesome Widget");
      await page.fill('[data-testid="product-sku"]', "123e4567-e89b-12d3-a456-426614174000");
      await page.fill('[data-testid="product-price"]', "29.99");
      await page.fill('[data-testid="product-quantity"]', "100");
      await page.fill('[data-testid="product-tags"]', "electronics, gadget");

      await page.click('[data-testid="submit-product"]');

      const result = page.locator('[data-testid="product-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "true");
      await expect(result).toHaveClass(/success/);
    });

    test("rejects empty product name", async ({ page }) => {
      await page.fill('[data-testid="product-name"]', "");
      await page.fill('[data-testid="product-sku"]', "123e4567-e89b-12d3-a456-426614174000");
      await page.fill('[data-testid="product-price"]', "29.99");
      await page.fill('[data-testid="product-quantity"]', "100");
      await page.fill('[data-testid="product-tags"]', "electronics");

      await page.click('[data-testid="submit-product"]');

      const result = page.locator('[data-testid="product-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("empty");
    });

    test("rejects invalid UUID for SKU", async ({ page }) => {
      await page.fill('[data-testid="product-name"]', "Awesome Widget");
      await page.fill('[data-testid="product-sku"]', "not-a-uuid");
      await page.fill('[data-testid="product-price"]', "29.99");
      await page.fill('[data-testid="product-quantity"]', "100");
      await page.fill('[data-testid="product-tags"]', "electronics");

      await page.click('[data-testid="submit-product"]');

      const result = page.locator('[data-testid="product-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("UUID");
    });

    test("rejects negative price", async ({ page }) => {
      await page.fill('[data-testid="product-name"]', "Awesome Widget");
      await page.fill('[data-testid="product-sku"]', "123e4567-e89b-12d3-a456-426614174000");
      await page.fill('[data-testid="product-price"]', "-10");
      await page.fill('[data-testid="product-quantity"]', "100");
      await page.fill('[data-testid="product-tags"]', "electronics");

      await page.click('[data-testid="submit-product"]');

      const result = page.locator('[data-testid="product-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("positive");
    });

    test("rejects negative quantity", async ({ page }) => {
      await page.fill('[data-testid="product-name"]', "Awesome Widget");
      await page.fill('[data-testid="product-sku"]', "123e4567-e89b-12d3-a456-426614174000");
      await page.fill('[data-testid="product-price"]', "29.99");
      await page.fill('[data-testid="product-quantity"]', "-5");
      await page.fill('[data-testid="product-tags"]', "electronics");

      await page.click('[data-testid="submit-product"]');

      const result = page.locator('[data-testid="product-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("non-negative");
    });

    test("rejects too many tags", async ({ page }) => {
      await page.fill('[data-testid="product-name"]', "Awesome Widget");
      await page.fill('[data-testid="product-sku"]', "123e4567-e89b-12d3-a456-426614174000");
      await page.fill('[data-testid="product-price"]', "29.99");
      await page.fill('[data-testid="product-quantity"]', "100");
      await page.fill('[data-testid="product-tags"]', "one, two, three, four, five, six"); // 6 tags, max is 5

      await page.click('[data-testid="submit-product"]');

      const result = page.locator('[data-testid="product-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("at most 5");
    });

    test("rejects empty tags array", async ({ page }) => {
      await page.fill('[data-testid="product-name"]', "Awesome Widget");
      await page.fill('[data-testid="product-sku"]', "123e4567-e89b-12d3-a456-426614174000");
      await page.fill('[data-testid="product-price"]', "29.99");
      await page.fill('[data-testid="product-quantity"]', "100");
      await page.fill('[data-testid="product-tags"]', ""); // No tags

      await page.click('[data-testid="submit-product"]');

      const result = page.locator('[data-testid="product-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("at least 1");
    });
  });

  test.describe("Event Form", () => {
    test("validates valid event data", async ({ page }) => {
      await page.fill('[data-testid="event-title"]', "Annual Conference");
      await page.fill('[data-testid="event-start"]', "2025-06-15");
      await page.fill('[data-testid="event-end"]', "2025-06-17");
      await page.fill('[data-testid="event-attendees"]', "200");

      await page.click('[data-testid="submit-event"]');

      const result = page.locator('[data-testid="event-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "true");
      await expect(result).toHaveClass(/success/);
    });

    test("rejects untrimmed title", async ({ page }) => {
      await page.fill('[data-testid="event-title"]', "  Annual Conference  "); // Whitespace
      await page.fill('[data-testid="event-start"]', "2025-06-15");
      await page.fill('[data-testid="event-end"]', "2025-06-17");
      await page.fill('[data-testid="event-attendees"]', "200");

      await page.click('[data-testid="submit-event"]');

      const result = page.locator('[data-testid="event-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("trimmed");
    });

    test("rejects invalid date format", async ({ page }) => {
      await page.fill('[data-testid="event-title"]', "Annual Conference");
      await page.fill('[data-testid="event-start"]', "not-a-date");
      await page.fill('[data-testid="event-end"]', "2025-06-17");
      await page.fill('[data-testid="event-attendees"]', "200");

      await page.click('[data-testid="submit-event"]');

      const result = page.locator('[data-testid="event-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("valid date");
    });

    test("rejects date before 2020", async ({ page }) => {
      await page.fill('[data-testid="event-title"]', "Annual Conference");
      await page.fill('[data-testid="event-start"]', "2019-06-15"); // Before 2020-01-01
      await page.fill('[data-testid="event-end"]', "2019-06-17");
      await page.fill('[data-testid="event-attendees"]', "200");

      await page.click('[data-testid="submit-event"]');

      const result = page.locator('[data-testid="event-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("after");
    });

    test("rejects max attendees below minimum", async ({ page }) => {
      await page.fill('[data-testid="event-title"]', "Annual Conference");
      await page.fill('[data-testid="event-start"]', "2025-06-15");
      await page.fill('[data-testid="event-end"]', "2025-06-17");
      await page.fill('[data-testid="event-attendees"]', "0"); // Below 1

      await page.click('[data-testid="submit-event"]');

      const result = page.locator('[data-testid="event-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("between");
    });

    test("rejects max attendees above maximum", async ({ page }) => {
      await page.fill('[data-testid="event-title"]', "Annual Conference");
      await page.fill('[data-testid="event-start"]', "2025-06-15");
      await page.fill('[data-testid="event-end"]', "2025-06-17");
      await page.fill('[data-testid="event-attendees"]', "5000"); // Above 1000

      await page.click('[data-testid="submit-event"]');

      const result = page.locator('[data-testid="event-result"]');
      await expect(result).toHaveAttribute("data-validation-success", "false");
      await expect(result).toContainText("between");
    });
  });

  test.describe("Window Results Object", () => {
    test("validation results are stored in window object", async ({ page }) => {
      // Submit valid user registration
      await page.fill('[data-testid="user-email"]', "test@example.com");
      await page.fill('[data-testid="user-username"]', "johndoe123");
      await page.fill('[data-testid="user-password"]', "securepassword123");
      await page.fill('[data-testid="user-age"]', "25");
      await page.fill('[data-testid="user-website"]', "https://example.com");
      await page.click('[data-testid="submit-user-registration"]');

      // Check window object
      const results = await page.evaluate(() => (window as any).validatorFormResults);
      expect(results.userRegistration).toBeDefined();
      expect(results.userRegistration.success).toBe(true);
      expect(results.userRegistration.data).toBeDefined();
      expect(results.userRegistration.data.email).toBe("test@example.com");
    });

    test("validation errors are stored in window object", async ({ page }) => {
      // Submit invalid product
      await page.fill('[data-testid="product-name"]', "");
      await page.fill('[data-testid="product-sku"]', "invalid");
      await page.fill('[data-testid="product-price"]', "-10");
      await page.fill('[data-testid="product-quantity"]', "-5");
      await page.fill('[data-testid="product-tags"]', "");
      await page.click('[data-testid="submit-product"]');

      // Check window object
      const results = await page.evaluate(() => (window as any).validatorFormResults);
      expect(results.product).toBeDefined();
      expect(results.product.success).toBe(false);
      expect(results.product.errors).toBeDefined();
      expect(results.product.errors.length).toBeGreaterThan(0);
    });
  });
});
