import { expect, type Page, test } from '@playwright/test';
import { readVanillaSlice } from './vanilla-playground.ts';

test.describe('Vanilla Validator Form E2E Tests', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/validator-form.html');
        await page.waitForSelector('#app');
    });

    test.describe('User Registration Form', () => {
        test('page loads with user registration form', async ({ page }) => {
            await expect(page).toHaveTitle(/Validator Form/);
            await expect(page.locator('[data-testid="user-registration-form"]'))
                .toBeVisible();
        });

        test('validates valid user registration data', async ({ page }) => {
            // Fill in valid data
            await page.fill('[data-testid="user-email"]', 'test@example.com');
            await page.fill('[data-testid="user-username"]', 'johndoe123');
            await page.fill('[data-testid="user-password"]', 'securepassword123');
            await page.fill('[data-testid="user-age"]', '25');
            await page.fill('[data-testid="user-website"]', 'https://example.com');

            // Submit form
            await page.click('[data-testid="submit-user-registration"]');

            // Wait for result
            const result = page.locator('[data-testid="user-registration-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'true');
            await expect(result).toHaveClass(/success/);
        });

        test('rejects invalid email', async ({ page }) => {
            await page.fill('[data-testid="user-email"]', 'not-an-email');
            await page.fill('[data-testid="user-username"]', 'johndoe123');
            await page.fill('[data-testid="user-password"]', 'securepassword123');
            await page.fill('[data-testid="user-age"]', '25');
            await page.fill('[data-testid="user-website"]', 'https://example.com');

            await page.click('[data-testid="submit-user-registration"]');

            const result = page.locator('[data-testid="user-registration-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('email');
        });

        test('rejects password too short', async ({ page }) => {
            await page.fill('[data-testid="user-email"]', 'test@example.com');
            await page.fill('[data-testid="user-username"]', 'johndoe123');
            await page.fill('[data-testid="user-password"]', 'short'); // Only 5 chars
            await page.fill('[data-testid="user-age"]', '25');
            await page.fill('[data-testid="user-website"]', 'https://example.com');

            await page.click('[data-testid="submit-user-registration"]');

            const result = page.locator('[data-testid="user-registration-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('at least 8');
        });

        test('rejects username with uppercase letters', async ({ page }) => {
            await page.fill('[data-testid="user-email"]', 'test@example.com');
            await page.fill('[data-testid="user-username"]', 'JohnDoe'); // Uppercase not allowed
            await page.fill('[data-testid="user-password"]', 'securepassword123');
            await page.fill('[data-testid="user-age"]', '25');
            await page.fill('[data-testid="user-website"]', 'https://example.com');

            await page.click('[data-testid="submit-user-registration"]');

            const result = page.locator('[data-testid="user-registration-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('lowercase');
        });

        test('rejects age below minimum', async ({ page }) => {
            await page.fill('[data-testid="user-email"]', 'test@example.com');
            await page.fill('[data-testid="user-username"]', 'johndoe123');
            await page.fill('[data-testid="user-password"]', 'securepassword123');
            await page.fill('[data-testid="user-age"]', '15'); // Below 18
            await page.fill('[data-testid="user-website"]', 'https://example.com');

            await page.click('[data-testid="submit-user-registration"]');

            const result = page.locator('[data-testid="user-registration-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('between');
        });

        test('rejects invalid URL', async ({ page }) => {
            await page.fill('[data-testid="user-email"]', 'test@example.com');
            await page.fill('[data-testid="user-username"]', 'johndoe123');
            await page.fill('[data-testid="user-password"]', 'securepassword123');
            await page.fill('[data-testid="user-age"]', '25');
            await page.fill('[data-testid="user-website"]', 'not-a-url');

            await page.click('[data-testid="submit-user-registration"]');

            const result = page.locator('[data-testid="user-registration-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('URL');
        });
    });

    test.describe('Product Form', () => {
        test('validates valid product data', async ({ page }) => {
            await page.fill('[data-testid="product-name"]', 'Awesome Widget');
            await page.fill(
                '[data-testid="product-sku"]',
                '123e4567-e89b-12d3-a456-426614174000'
            );
            await page.fill('[data-testid="product-price"]', '29.99');
            await page.fill('[data-testid="product-quantity"]', '100');
            await page.fill('[data-testid="product-tags"]', 'electronics, gadget');

            await page.click('[data-testid="submit-product"]');

            const result = page.locator('[data-testid="product-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'true');
            await expect(result).toHaveClass(/success/);
        });

        test('rejects empty product name', async ({ page }) => {
            await page.fill('[data-testid="product-name"]', '');
            await page.fill(
                '[data-testid="product-sku"]',
                '123e4567-e89b-12d3-a456-426614174000'
            );
            await page.fill('[data-testid="product-price"]', '29.99');
            await page.fill('[data-testid="product-quantity"]', '100');
            await page.fill('[data-testid="product-tags"]', 'electronics');

            await page.click('[data-testid="submit-product"]');

            const result = page.locator('[data-testid="product-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('empty');
        });

        test('rejects invalid UUID for SKU', async ({ page }) => {
            await page.fill('[data-testid="product-name"]', 'Awesome Widget');
            await page.fill('[data-testid="product-sku"]', 'not-a-uuid');
            await page.fill('[data-testid="product-price"]', '29.99');
            await page.fill('[data-testid="product-quantity"]', '100');
            await page.fill('[data-testid="product-tags"]', 'electronics');

            await page.click('[data-testid="submit-product"]');

            const result = page.locator('[data-testid="product-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('UUID');
        });

        test('rejects negative price', async ({ page }) => {
            await page.fill('[data-testid="product-name"]', 'Awesome Widget');
            await page.fill(
                '[data-testid="product-sku"]',
                '123e4567-e89b-12d3-a456-426614174000'
            );
            await page.fill('[data-testid="product-price"]', '-10');
            await page.fill('[data-testid="product-quantity"]', '100');
            await page.fill('[data-testid="product-tags"]', 'electronics');

            await page.click('[data-testid="submit-product"]');

            const result = page.locator('[data-testid="product-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('positive');
        });

        test('rejects negative quantity', async ({ page }) => {
            await page.fill('[data-testid="product-name"]', 'Awesome Widget');
            await page.fill(
                '[data-testid="product-sku"]',
                '123e4567-e89b-12d3-a456-426614174000'
            );
            await page.fill('[data-testid="product-price"]', '29.99');
            await page.fill('[data-testid="product-quantity"]', '-5');
            await page.fill('[data-testid="product-tags"]', 'electronics');

            await page.click('[data-testid="submit-product"]');

            const result = page.locator('[data-testid="product-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('non-negative');
        });

        test('rejects too many tags', async ({ page }) => {
            await page.fill('[data-testid="product-name"]', 'Awesome Widget');
            await page.fill(
                '[data-testid="product-sku"]',
                '123e4567-e89b-12d3-a456-426614174000'
            );
            await page.fill('[data-testid="product-price"]', '29.99');
            await page.fill('[data-testid="product-quantity"]', '100');
            await page.fill(
                '[data-testid="product-tags"]',
                'one, two, three, four, five, six'
            ); // 6 tags, max is 5

            await page.click('[data-testid="submit-product"]');

            const result = page.locator('[data-testid="product-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('at most 5');
        });

        test('rejects empty tags array', async ({ page }) => {
            await page.fill('[data-testid="product-name"]', 'Awesome Widget');
            await page.fill(
                '[data-testid="product-sku"]',
                '123e4567-e89b-12d3-a456-426614174000'
            );
            await page.fill('[data-testid="product-price"]', '29.99');
            await page.fill('[data-testid="product-quantity"]', '100');
            await page.fill('[data-testid="product-tags"]', ''); // No tags

            await page.click('[data-testid="submit-product"]');

            const result = page.locator('[data-testid="product-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('at least 1');
        });
    });

    test.describe('Event Form', () => {
        test('validates valid event data', async ({ page }) => {
            await page.fill('[data-testid="event-title"]', 'Annual Conference');
            await page.fill('[data-testid="event-start"]', '2025-06-15');
            await page.fill('[data-testid="event-end"]', '2025-06-17');
            await page.fill('[data-testid="event-attendees"]', '200');

            await page.click('[data-testid="submit-event"]');

            const result = page.locator('[data-testid="event-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'true');
            await expect(result).toHaveClass(/success/);
        });

        test('rejects untrimmed title', async ({ page }) => {
            await page.fill('[data-testid="event-title"]', '  Annual Conference  '); // Whitespace
            await page.fill('[data-testid="event-start"]', '2025-06-15');
            await page.fill('[data-testid="event-end"]', '2025-06-17');
            await page.fill('[data-testid="event-attendees"]', '200');

            await page.click('[data-testid="submit-event"]');

            const result = page.locator('[data-testid="event-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('trimmed');
        });

        test('rejects invalid date format', async ({ page }) => {
            await page.fill('[data-testid="event-title"]', 'Annual Conference');
            await page.fill('[data-testid="event-start"]', 'not-a-date');
            await page.fill('[data-testid="event-end"]', '2025-06-17');
            await page.fill('[data-testid="event-attendees"]', '200');

            await page.click('[data-testid="submit-event"]');

            const result = page.locator('[data-testid="event-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('must be a valid date');
        });

        test('rejects date before 2020', async ({ page }) => {
            await page.fill('[data-testid="event-title"]', 'Annual Conference');
            await page.fill('[data-testid="event-start"]', '2019-06-15'); // Before 2020-01-01
            await page.fill('[data-testid="event-end"]', '2019-06-17');
            await page.fill('[data-testid="event-attendees"]', '200');

            await page.click('[data-testid="submit-event"]');

            const result = page.locator('[data-testid="event-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('after');
        });

        test('rejects max attendees below minimum', async ({ page }) => {
            await page.fill('[data-testid="event-title"]', 'Annual Conference');
            await page.fill('[data-testid="event-start"]', '2025-06-15');
            await page.fill('[data-testid="event-end"]', '2025-06-17');
            await page.fill('[data-testid="event-attendees"]', '0'); // Below 1

            await page.click('[data-testid="submit-event"]');

            const result = page.locator('[data-testid="event-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('between');
        });

        test('rejects max attendees above maximum', async ({ page }) => {
            await page.fill('[data-testid="event-title"]', 'Annual Conference');
            await page.fill('[data-testid="event-start"]', '2025-06-15');
            await page.fill('[data-testid="event-end"]', '2025-06-17');
            await page.fill('[data-testid="event-attendees"]', '5000'); // Above 1000

            await page.click('[data-testid="submit-event"]');

            const result = page.locator('[data-testid="event-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'false');
            await expect(result).toContainText('between');
        });
    });

    test.describe('Window Results Object', () => {
        test('validation results are stored in window object', async ({ page }) => {
            // Submit valid user registration
            await page.fill('[data-testid="user-email"]', 'test@example.com');
            await page.fill('[data-testid="user-username"]', 'johndoe123');
            await page.fill('[data-testid="user-password"]', 'securepassword123');
            await page.fill('[data-testid="user-age"]', '25');
            await page.fill('[data-testid="user-website"]', 'https://example.com');
            await page.click('[data-testid="submit-user-registration"]');

            const { userRegistration } = await readVanillaSlice(page, 'validatorForm');
            if (!userRegistration?.success) {
                throw new Error(
                    `registration did not validate: ${JSON.stringify(userRegistration)}`
                );
            }
            expect(userRegistration.value.email).toBe('test@example.com');
        });

        test('validation errors are stored in window object', async ({ page }) => {
            // Submit invalid product
            await page.fill('[data-testid="product-name"]', '');
            await page.fill('[data-testid="product-sku"]', 'invalid');
            await page.fill('[data-testid="product-price"]', '-10');
            await page.fill('[data-testid="product-quantity"]', '-5');
            await page.fill('[data-testid="product-tags"]', '');
            await page.click('[data-testid="submit-product"]');

            const { product } = await readVanillaSlice(page, 'validatorForm');
            if (product === undefined || product.success) {
                throw new Error(
                    `product should have failed validation: ${JSON.stringify(product)}`
                );
            }
            expect(product.errors.length).toBeGreaterThan(0);
        });
    });
});

test.describe('Vanilla Measurement Form (newtypes)', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await page.getByTestId('nav-validator-form').click();
        await expect(page.getByTestId('measurement-form')).toBeVisible();
    });

    async function submitMeasurement(
        page: Page,
        values: { runner: string; distance: string; cents: string; contact: string }
    ) {
        await page.getByTestId('measurement-runner').fill(values.runner);
        await page.getByTestId('measurement-distance').fill(values.distance);
        await page.getByTestId('measurement-cents').fill(values.cents);
        await page.getByTestId('measurement-contact').fill(values.contact);
        await page.getByTestId('submit-measurement').click();
        return page.getByTestId('measurement-result');
    }

    const valid = {
        runner: 'ada',
        distance: '12.5',
        cents: '123456789012345678901',
        contact: 'ada@example.com'
    };

    test('accepts valid newtype values and keeps every bigint digit', async ({ page }) => {
        const result = await submitMeasurement(page, valid);
        await expect(result).toHaveAttribute('data-validation-success', 'true');
        await expect(result).toContainText('"cents": "123456789012345678901"');
        await expect(result).toContainText('"distance": 12.5');
    });

    test('rejects a negative distance through the Meters newtype', async ({ page }) => {
        const result = await submitMeasurement(page, { ...valid, distance: '-3' });
        await expect(result).toHaveAttribute('data-validation-success', 'false');
        await expect(result).toContainText('distance:');
        await expect(result).toContainText('Meters must be non-negative');
    });

    test('rejects an empty distance as the wrong type', async ({ page }) => {
        const result = await submitMeasurement(page, { ...valid, distance: '' });
        await expect(result).toHaveAttribute('data-validation-success', 'false');
        await expect(result).toContainText('expected number');
    });

    test('rejects an empty runner through the Username newtype', async ({ page }) => {
        const result = await submitMeasurement(page, { ...valid, runner: '' });
        await expect(result).toHaveAttribute('data-validation-success', 'false');
        await expect(result).toContainText('runner:');
        await expect(result).toContainText('must not be empty');
    });

    test('rejects a fractional fee through the Cents newtype', async ({ page }) => {
        const result = await submitMeasurement(page, { ...valid, cents: '12.5' });
        await expect(result).toHaveAttribute('data-validation-success', 'false');
        await expect(result).toContainText('cents:');
        await expect(result).toContainText('expected bigint');
    });

    test('rejects a negative fee through the Cents validators', async ({ page }) => {
        const result = await submitMeasurement(page, { ...valid, cents: '-1' });
        await expect(result).toHaveAttribute('data-validation-success', 'false');
        await expect(result).toContainText('Cents must be non-negative');
    });

    test('rejects a contact the union arm validator refuses', async ({ page }) => {
        const result = await submitMeasurement(page, { ...valid, contact: 'nobody' });
        await expect(result).toHaveAttribute('data-validation-success', 'false');
        await expect(result).toContainText('contact:');
        await expect(result).toContainText('valid email');
    });
});
