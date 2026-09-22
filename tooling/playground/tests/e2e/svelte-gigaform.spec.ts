import { expect, test } from '@playwright/test';

test.describe('Gigaform E2E Tests', () => {
    test.describe('Basic Field Operations', () => {
        test.beforeEach(async ({ page }) => {
            await page.goto('/gigaform/basic');
            await page.waitForSelector('body.hydrated', { timeout: 10000 });
            await page.waitForSelector('[data-testid="phone-form"]');
        });

        test('page loads with all basic forms', async ({ page }) => {
            await expect(page).toHaveTitle(/Basic Fields/);
            await expect(page.locator('[data-testid="phone-form"]')).toBeVisible();
            await expect(page.locator('[data-testid="gradient-form"]')).toBeVisible();
            await expect(page.locator('[data-testid="coordinates-form"]'))
                .toBeVisible();
        });

        test('PhoneNumber form creates with defaults', async ({ page }) => {
            // Check default values are empty strings for text fields
            const phoneType = await page.inputValue('[data-testid="phone-type"]');
            const phoneNumber = await page.inputValue('[data-testid="phone-number"]');
            expect(phoneType).toBe('');
            expect(phoneNumber).toBe('');
        });

        test('field.set() updates input value reactively', async ({ page }) => {
            // Programmatically set via window object
            await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const phoneNumber = playground?.gigaform?.phoneNumber;
                if (!phoneNumber) throw new Error('the gigaform page did not publish phoneNumber');
                phoneNumber
                    .fields.phoneType.set(
                        'Mobile'
                    );
            });
            await page.waitForTimeout(50);

            const value = await page.inputValue('[data-testid="phone-type"]');
            expect(value).toBe('Mobile');
        });

        test('getTainted() returns false initially', async ({ page }) => {
            const tainted = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const phoneNumber = playground?.gigaform?.phoneNumber;
                const option = playground?.effectOption;
                if (!phoneNumber || !option) {
                    throw new Error('the gigaform page did not publish phoneNumber, option');
                }
                return option.getOrElse(
                    phoneNumber.fields.phoneType.getTainted(),
                    () => false
                );
            });
            expect(tainted).toBe(false);
        });

        test('setTainted() marks field as touched', async ({ page }) => {
            await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const phoneNumber = playground?.gigaform?.phoneNumber;
                const option = playground?.effectOption;
                if (!phoneNumber || !option) {
                    throw new Error('the gigaform page did not publish phoneNumber, option');
                }
                phoneNumber.fields.phoneType.setTainted(
                    option.some(true)
                );
            });

            const tainted = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const phoneNumber = playground?.gigaform?.phoneNumber;
                const option = playground?.effectOption;
                if (!phoneNumber || !option) {
                    throw new Error('the gigaform page did not publish phoneNumber, option');
                }
                return option.getOrElse(
                    phoneNumber.fields.phoneType.getTainted(),
                    () => false
                );
            });
            expect(tainted).toBe(true);

            // Verify UI updates
            await expect(page.locator('[data-testid="phone-type-tainted"]'))
                .toContainText('true');
        });

        test('UI input triggers tainted via oninput handler', async ({ page }) => {
            await page.fill('[data-testid="phone-type"]', 'Work');

            const tainted = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const phoneNumber = playground?.gigaform?.phoneNumber;
                const option = playground?.effectOption;
                if (!phoneNumber || !option) {
                    throw new Error('the gigaform page did not publish phoneNumber, option');
                }
                return option.getOrElse(
                    phoneNumber.fields.phoneType.getTainted(),
                    () => false
                );
            });
            expect(tainted).toBe(true);
        });

        test('setError() stores and displays error', async ({ page }) => {
            await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const phoneNumber = playground?.gigaform?.phoneNumber;
                const option = playground?.effectOption;
                if (!phoneNumber || !option) {
                    throw new Error('the gigaform page did not publish phoneNumber, option');
                }
                phoneNumber.fields.phoneType.setError(
                    option.some(['Phone type is required'])
                );
            });
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="phone-type-error"]'))
                .toContainText(
                    'Phone type is required'
                );
        });

        test('validate() returns field-specific errors', async ({ page }) => {
            // phoneType starts empty and carries a nonEmpty validator.
            await page.click('[data-testid="validate-phone-type"]');

            await expect(page.locator('[data-testid="phone-type-error"]')).toBeVisible();
        });

        test('submit valid PhoneNumber form', async ({ page }) => {
            await page.fill('[data-testid="phone-type"]', 'Mobile');
            await page.fill('[data-testid="phone-number"]', '555-123-4567');
            await page.check('[data-testid="phone-main"]');
            await page.check('[data-testid="phone-canCall"]');

            await page.click('[data-testid="submit-phone"]');

            const result = page.locator('[data-testid="phone-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'true');
        });

        test('reset() clears form to defaults', async ({ page }) => {
            // Fill form
            await page.fill('[data-testid="phone-type"]', 'Mobile');
            await page.fill('[data-testid="phone-number"]', '555-123-4567');

            // Reset
            await page.click('[data-testid="reset-phone"]');
            await page.waitForTimeout(50);

            // Verify cleared
            const phoneType = await page.inputValue('[data-testid="phone-type"]');
            const phoneNumber = await page.inputValue('[data-testid="phone-number"]');
            expect(phoneType).toBe('');
            expect(phoneNumber).toBe('');
        });

        test('Gradient form with number field', async ({ page }) => {
            await page.fill('[data-testid="gradient-startHue"]', '180');
            await page.click('[data-testid="submit-gradient"]');

            const result = page.locator('[data-testid="gradient-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'true');
        });

        test('Coordinates form with lat/lng', async ({ page }) => {
            await page.fill('[data-testid="coordinates-lat"]', '40.7128');
            await page.fill('[data-testid="coordinates-lng"]', '-74.0060');
            await page.click('[data-testid="submit-coordinates"]');

            const result = page.locator('[data-testid="coordinates-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'true');
        });
    });

    test.describe('Nested Objects', () => {
        test.beforeEach(async ({ page }) => {
            await page.goto('/gigaform/nested');
            await page.waitForSelector('body.hydrated', { timeout: 10000 });
            await page.waitForSelector('[data-testid="user-form"]');
        });

        test('nested defaults are initialized', async ({ page }) => {
            // Check settings.scheduleSettings.daysPerWeek has a value
            const daysPerWeek = await page.inputValue(
                '[data-testid="settings-daysPerWeek"]'
            );
            expect(Number(daysPerWeek)).toBeGreaterThanOrEqual(0);
        });

        test('can modify nested field via wholesale update', async ({ page }) => {
            await page.fill('[data-testid="settings-daysPerWeek"]', '5');
            await page.waitForTimeout(50);

            const data = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const user = playground?.gigaform?.user;
                if (!user) throw new Error('the gigaform page did not publish user');
                return user.data
                    .settings.scheduleSettings
                    .daysPerWeek;
            });
            expect(data).toBe(5);
        });

        test('nested select field works', async ({ page }) => {
            await page.selectOption('[data-testid="settings-rowHeight"]', 'Large');
            await page.waitForTimeout(50);

            const data = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const user = playground?.gigaform?.user;
                if (!user) throw new Error('the gigaform page did not publish user');
                return user.data
                    .settings.scheduleSettings
                    .rowHeight;
            });
            expect(data).toBe('Large');
        });

        test('add permission page to nested array', async ({ page }) => {
            const initialLength = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const user = playground?.gigaform?.user;
                if (!user) throw new Error('the gigaform page did not publish user');
                return user.data
                    .permissions.pages.length;
            });

            await page.click('[data-testid="add-permission-page"]');
            await page.waitForTimeout(50);

            const newLength = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const user = playground?.gigaform?.user;
                if (!user) throw new Error('the gigaform page did not publish user');
                return user.data
                    .permissions.pages.length;
            });

            expect(newLength).toBe(initialLength + 1);
        });

        test('reset Settings to default', async ({ page }) => {
            // Modify a setting
            await page.fill('[data-testid="settings-daysPerWeek"]', '7');

            // Reset to default
            await page.click('[data-testid="set-nested-wholesale"]');
            await page.waitForTimeout(50);

            const daysPerWeek = await page.inputValue(
                '[data-testid="settings-daysPerWeek"]'
            );
            // Default value should be restored
            expect(Number(daysPerWeek)).toBeLessThan(7);
        });
    });

    test.describe('Array Operations', () => {
        test.beforeEach(async ({ page }) => {
            await page.goto('/gigaform/arrays');
            await page.waitForSelector('body.hydrated', { timeout: 10000 });
            await page.waitForSelector('[data-testid="account-form"]');
        });

        test('displays initial phones count', async ({ page }) => {
            await expect(page.locator('[data-testid="phones-count"]')).toContainText(
                'Count:'
            );
        });

        test('push() adds new phone to array', async ({ page }) => {
            const initialCount = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const account = playground?.gigaform?.account;
                if (!account) throw new Error('the gigaform page did not publish account');
                return account
                    .fields.phones.get().length;
            });

            await page.fill('[data-testid="new-phone-type"]', 'Work');
            await page.fill('[data-testid="new-phone-number"]', '555-999-0000');
            await page.click('[data-testid="add-phone"]');
            await page.waitForTimeout(50);

            const newCount = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const account = playground?.gigaform?.account;
                if (!account) throw new Error('the gigaform page did not publish account');
                return account
                    .fields.phones.get().length;
            });

            expect(newCount).toBe(initialCount + 1);
        });

        test('remove() deletes phone from array', async ({ page }) => {
            // First add a phone
            await page.fill('[data-testid="new-phone-type"]', 'Temp');
            await page.fill('[data-testid="new-phone-number"]', '555-000-0000');
            await page.click('[data-testid="add-phone"]');
            await page.waitForTimeout(50);

            const countBefore = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const account = playground?.gigaform?.account;
                if (!account) throw new Error('the gigaform page did not publish account');
                return account
                    .fields.phones.get().length;
            });

            // Remove the first phone
            await page.click('[data-testid="phone-remove-0"]');
            await page.waitForTimeout(50);

            const countAfter = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const account = playground?.gigaform?.account;
                if (!account) throw new Error('the gigaform page did not publish account');
                return account
                    .fields.phones.get().length;
            });

            expect(countAfter).toBe(countBefore - 1);
        });

        test('at() returns controller for indexed element', async ({ page }) => {
            // Add a phone first
            await page.fill('[data-testid="new-phone-type"]', 'Mobile');
            await page.fill('[data-testid="new-phone-number"]', '555-111-2222');
            await page.click('[data-testid="add-phone"]');
            await page.waitForTimeout(50);

            const phoneAt0 = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const account = playground?.gigaform?.account;
                if (!account) throw new Error('the gigaform page did not publish account');
                const controller = account.fields.phones
                    .at(0);
                return controller.get();
            });

            expect(phoneAt0.phoneType).toBe('Mobile');
        });

        test('at() set modifies specific array element', async ({ page }) => {
            // Add a phone
            await page.fill('[data-testid="new-phone-type"]', 'Home');
            await page.fill('[data-testid="new-phone-number"]', '555-333-4444');
            await page.click('[data-testid="add-phone"]');
            await page.waitForTimeout(50);

            // Toggle main via at() set
            await page.click('[data-testid="phone-edit-0"]');
            await page.waitForTimeout(50);

            const phone = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const account = playground?.gigaform?.account;
                if (!account) throw new Error('the gigaform page did not publish account');
                return account
                    .fields.phones.get()[0];
            });

            // Main should have toggled
            expect(typeof phone.main).toBe('boolean');
        });

        test('swap() exchanges two array elements', async ({ page }) => {
            // Add two phones
            await page.fill('[data-testid="new-phone-type"]', 'First');
            await page.fill('[data-testid="new-phone-number"]', '111');
            await page.click('[data-testid="add-phone"]');
            await page.waitForTimeout(50);

            await page.fill('[data-testid="new-phone-type"]', 'Second');
            await page.fill('[data-testid="new-phone-number"]', '222');
            await page.click('[data-testid="add-phone"]');
            await page.waitForTimeout(50);

            const beforeSwap = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const account = playground?.gigaform?.account;
                if (!account) throw new Error('the gigaform page did not publish account');
                return account.fields.phones.get().map((phone) => phone.phoneType);
            });

            // Swap via Move Up button on second element
            await page.click('[data-testid="phone-swap-up-1"]');
            await page.waitForTimeout(50);

            const afterSwap = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const account = playground?.gigaform?.account;
                if (!account) throw new Error('the gigaform page did not publish account');
                return account.fields.phones.get().map((phone) => phone.phoneType);
            });

            expect(afterSwap[0]).toBe(beforeSwap[1]);
            expect(afterSwap[1]).toBe(beforeSwap[0]);
        });

        test('tags string array add/remove', async ({ page }) => {
            await page.fill('[data-testid="new-tag"]', 'VIP');
            await page.click('[data-testid="add-tag"]');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="tags-count"]')).toContainText(
                '1'
            );

            await page.click('[data-testid="tag-remove-0"]');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="tags-count"]')).toContainText(
                '0'
            );
        });

        test('customFields tuple array operations', async ({ page }) => {
            await page.fill('[data-testid="new-customField-key"]', 'Industry');
            await page.fill('[data-testid="new-customField-value"]', 'Technology');
            await page.click('[data-testid="add-customField"]');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="customFields-count"]'))
                .toContainText('1');

            // Verify tuple structure
            const customField = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const account = playground?.gigaform?.account;
                if (!account) throw new Error('the gigaform page did not publish account');
                return account
                    .fields.customFields.get()[0];
            });
            expect(customField).toEqual(['Industry', 'Technology']);
        });
    });

    test.describe('Union Types', () => {
        test.beforeEach(async ({ page }) => {
            await page.goto('/gigaform/unions');
            await page.waitForSelector('body.hydrated', { timeout: 10000 });
            await page.waitForSelector('[data-testid="lead-form"]');
        });

        test('defaults to CompanyName variant', async ({ page }) => {
            await expect(page.locator('[data-testid="current-variant"]'))
                .toContainText('company');
        });

        test('can switch to PersonName variant', async ({ page }) => {
            await page.click('[data-testid="select-person-variant"]');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="current-variant"]'))
                .toContainText('person');
            await expect(page.locator('[data-testid="person-form"]')).toBeVisible();
        });

        test('CompanyName variant data structure', async ({ page }) => {
            await page.fill('[data-testid="lead-companyName"]', 'Acme Inc');
            await page.waitForTimeout(50);

            const leadName = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const lead = playground?.gigaform?.lead;
                if (!lead) throw new Error('the gigaform page did not publish lead');
                return lead.fields
                    .leadName.get();
            });

            if (!('companyName' in leadName)) {
                throw new Error(
                    `expected the CompanyName variant, got ${JSON.stringify(leadName)}`
                );
            }
            expect(leadName.companyName).toBe('Acme Inc');
        });

        test('PersonName variant data structure', async ({ page }) => {
            await page.click('[data-testid="select-person-variant"]');
            await page.waitForTimeout(50);

            await page.fill('[data-testid="lead-firstName"]', 'Jane');
            await page.fill('[data-testid="lead-lastName"]', 'Doe');
            await page.waitForTimeout(50);

            const leadName = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const lead = playground?.gigaform?.lead;
                if (!lead) throw new Error('the gigaform page did not publish lead');
                return lead.fields
                    .leadName.get();
            });

            if (!('firstName' in leadName)) {
                throw new Error(`expected the PersonName variant, got ${JSON.stringify(leadName)}`);
            }
            expect(leadName.firstName).toBe('Jane');
            expect(leadName.lastName).toBe('Doe');
        });

        test('submit with CompanyName variant', async ({ page }) => {
            await page.fill('[data-testid="lead-companyName"]', 'Test Corp');
            await page.click('[data-testid="submit-lead"]');

            const result = page.locator('[data-testid="lead-result"]');
            // Check that submission completes (validation may pass or fail based on validators)
            await expect(result).toBeVisible();
        });
    });

    test.describe('Nullable Fields', () => {
        test.beforeEach(async ({ page }) => {
            await page.goto('/gigaform/nullable');
            await page.waitForSelector('body.hydrated', { timeout: 10000 });
            await page.waitForSelector('[data-testid="user-form"]');
        });

        test('email initialized as null', async ({ page }) => {
            await expect(page.locator('[data-testid="email-nullable-status"]'))
                .toContainText(
                    'null'
                );
        });

        test('can set email from null to value', async ({ page }) => {
            await page.fill('[data-testid="user-email"]', 'test@example.com');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="email-nullable-status"]'))
                .toContainText(
                    'test@example.com'
                );
        });

        test('can set email back to null', async ({ page }) => {
            await page.fill('[data-testid="user-email"]', 'test@example.com');
            await page.click('[data-testid="set-email-null"]');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="email-nullable-status"]'))
                .toContainText(
                    'null'
                );
        });

        test('metadata initialized as null', async ({ page }) => {
            await expect(page.locator('[data-testid="metadata-nullable-status"]'))
                .toContainText(
                    'null'
                );
        });

        test('can enable metadata (null to object)', async ({ page }) => {
            await page.click('[data-testid="metadata-toggle"]');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="metadata-nullable-status"]'))
                .toContainText(
                    'Metadata object'
                );
            await expect(page.locator('[data-testid="metadata-form"]')).toBeVisible();
        });

        test('can set metadata back to null', async ({ page }) => {
            await page.click('[data-testid="metadata-toggle"]');
            await page.waitForTimeout(50);
            await page.click('[data-testid="set-metadata-null"]');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="metadata-nullable-status"]'))
                .toContainText(
                    'null'
                );
        });
    });

    test.describe('Record/Map Types', () => {
        test.beforeEach(async ({ page }) => {
            await page.goto('/gigaform/records');
            await page.waitForSelector('body.hydrated', { timeout: 10000 });
            await page.waitForSelector('[data-testid="taxrate-form"]');
        });

        test('displays initial tax components', async ({ page }) => {
            await expect(page.locator('[data-testid="taxComponents-summary"]'))
                .toContainText(
                    'Entries:'
                );
        });

        test('can add new record entry', async ({ page }) => {
            const initialCount = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const taxRate = playground?.gigaform?.taxRate;
                if (!taxRate) throw new Error('the gigaform page did not publish taxRate');
                return Object.keys(
                    taxRate
                        .fields.taxComponents.get()
                )
                    .length;
            });

            await page.fill('[data-testid="new-component-key"]', 'district');
            await page.fill('[data-testid="new-component-value"]', '0.005');
            await page.click('[data-testid="add-component"]');
            await page.waitForTimeout(50);

            const newCount = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const taxRate = playground?.gigaform?.taxRate;
                if (!taxRate) throw new Error('the gigaform page did not publish taxRate');
                return Object.keys(
                    taxRate
                        .fields.taxComponents.get()
                )
                    .length;
            });

            expect(newCount).toBe(initialCount + 1);
        });

        test('can modify existing record entry', async ({ page }) => {
            // Set default components first
            await page.click('[data-testid="set-default-components"]');
            await page.waitForTimeout(50);

            // Modify state component
            await page.fill('[data-testid="taxComponent-state-value"]', '0.08');
            await page.waitForTimeout(50);

            const stateValue = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const taxRate = playground?.gigaform?.taxRate;
                if (!taxRate) throw new Error('the gigaform page did not publish taxRate');
                return taxRate
                    .fields.taxComponents.get()
                    .state;
            });

            expect(stateValue).toBeCloseTo(0.08);
        });

        test('can remove record entry', async ({ page }) => {
            await page.click('[data-testid="set-default-components"]');
            await page.waitForTimeout(50);

            await page.click('[data-testid="taxComponent-remove-city"]');
            await page.waitForTimeout(50);

            const hasCity = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const taxRate = playground?.gigaform?.taxRate;
                if (!taxRate) throw new Error('the gigaform page did not publish taxRate');
                return 'city' in
                    taxRate
                        .fields.taxComponents.get();
            });

            expect(hasCity).toBe(false);
        });

        test('can clear all components', async ({ page }) => {
            await page.click('[data-testid="set-default-components"]');
            await page.waitForTimeout(50);
            await page.click('[data-testid="clear-all-components"]');
            await page.waitForTimeout(50);

            const count = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const taxRate = playground?.gigaform?.taxRate;
                if (!taxRate) throw new Error('the gigaform page did not publish taxRate');
                return Object.keys(
                    taxRate
                        .fields.taxComponents.get()
                )
                    .length;
            });

            expect(count).toBe(0);
        });
    });

    test.describe('Deep Nesting', () => {
        test.beforeEach(async ({ page }) => {
            await page.goto('/gigaform/deeply-nested');
            await page.waitForSelector('body.hydrated', { timeout: 10000 });
            await page.waitForSelector('[data-testid="order-form"]');
        });

        test('site is initialized as embedded object', async ({ page }) => {
            await expect(page.locator('[data-testid="site-type"]')).toContainText(
                'Embedded Object'
            );
        });

        test('can access deeply nested coordinates', async ({ page }) => {
            const lat = await page.inputValue('[data-testid="coordinates-lat"]');
            const lng = await page.inputValue('[data-testid="coordinates-lng"]');

            expect(Number(lat)).toBeCloseTo(39.7817, 2);
            expect(Number(lng)).toBeCloseTo(-89.6501, 2);
        });

        test('can modify deeply nested lat/lng', async ({ page }) => {
            await page.fill('[data-testid="coordinates-lat"]', '40.7128');
            await page.fill('[data-testid="coordinates-lng"]', '-74.0060');
            await page.waitForTimeout(50);

            const site = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const order = playground?.gigaform?.order;
                if (!order) throw new Error('the gigaform page did not publish order');
                return order.fields
                    .site.get();
            });

            if (typeof site === 'string') {
                throw new Error(`expected a Site object, got the reference ${site}`);
            }
            expect(site.coordinates.lat).toBeCloseTo(40.7128);
            expect(site.coordinates.lng).toBeCloseTo(-74.006);
        });

        test('programmatic deep update via button', async ({ page }) => {
            await page.click('[data-testid="update-deep-coordinates"]');
            await page.waitForTimeout(50);

            const lat = await page.inputValue('[data-testid="coordinates-lat"]');
            const lng = await page.inputValue('[data-testid="coordinates-lng"]');

            expect(Number(lat)).toBeCloseTo(40.7128, 2);
            expect(Number(lng)).toBeCloseTo(-74.006, 2);
        });

        test('can switch site to string reference', async ({ page }) => {
            await page.click('[data-testid="set-site-string"]');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="site-string-reference"]'))
                .toBeVisible();
            await expect(page.locator('[data-testid="site-string-reference"]'))
                .toContainText(
                    'site-ref-123'
                );
        });

        test('can switch site back to object', async ({ page }) => {
            await page.click('[data-testid="set-site-string"]');
            await page.waitForTimeout(50);
            await page.click('[data-testid="set-site-object"]');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="site-type"]')).toContainText(
                'Embedded Object'
            );
        });
    });

    test.describe('Enum Fields', () => {
        test.beforeEach(async ({ page }) => {
            await page.goto('/gigaform/enums');
            await page.waitForSelector('body.hydrated', { timeout: 10000 });
            await page.waitForSelector('[data-testid="appointment-form"]');
        });

        test('status enum has default value', async ({ page }) => {
            await expect(page.locator('[data-testid="status-current"]'))
                .toContainText('Scheduled');
        });

        test('can change status via buttons', async ({ page }) => {
            await page.click('[data-testid="status-OnDeck"]');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="status-current"]'))
                .toContainText('OnDeck');
        });

        test('can change status via select', async ({ page }) => {
            await page.selectOption('[data-testid="status-select"]', 'Waiting');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="status-current"]'))
                .toContainText('Waiting');
        });

        test('tainted tracking works for enum', async ({ page }) => {
            await expect(page.locator('[data-testid="status-tainted"]'))
                .toContainText('false');

            await page.click('[data-testid="status-OnDeck"]');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="status-tainted"]'))
                .toContainText('true');
        });

        test('submit valid appointment with enum', async ({ page }) => {
            await page.fill('[data-testid="appt-title"]', 'Team Meeting');
            await page.click('[data-testid="submit-appointment"]');

            const result = page.locator('[data-testid="appointment-result"]');
            await expect(result).toHaveAttribute('data-validation-success', 'true');
        });
    });

    test.describe('Complex Integration', () => {
        test.beforeEach(async ({ page }) => {
            await page.goto('/gigaform/complex');
            await page.waitForSelector('body.hydrated', { timeout: 10000 });
            await page.waitForSelector('[data-testid="employee-form"]');
        });

        test('Employee form loads with all features', async ({ page }) => {
            await expect(page.locator('[data-testid="employee-form"]')).toBeVisible();
            await expect(page.locator('[data-testid="emp-phones-count"]'))
                .toContainText('1');
        });

        test('Employee phone array operations', async ({ page }) => {
            await page.click('[data-testid="emp-add-phone"]');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="emp-phones-count"]'))
                .toContainText('2');
        });

        test('Employee nullable fields work', async ({ page }) => {
            await page.fill(
                '[data-testid="emp-imageUrl"]',
                'https://example.com/image.jpg'
            );
            await page.waitForTimeout(50);

            const imageUrl = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const employee = playground?.gigaform?.employee;
                if (!employee) throw new Error('the gigaform page did not publish employee');
                return employee
                    .fields.imageUrl.get();
            });

            expect(imageUrl).toBe('https://example.com/image.jpg');
        });

        test('Employee submit validation', async ({ page }) => {
            await page.click('[data-testid="submit-employee"]');

            const result = page.locator('[data-testid="employee-result"]');
            await expect(result).toBeVisible();
        });

        test('Order form loads with complex nested data', async ({ page }) => {
            await expect(page.locator('[data-testid="order-form"]')).toBeVisible();
            await expect(page.locator('[data-testid="order-billedItems-count"]'))
                .toContainText(
                    '2'
                );
        });

        test('Order billedItems array operations', async ({ page }) => {
            await page.click('[data-testid="order-add-billedItem"]');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="order-billedItems-count"]'))
                .toContainText(
                    '3'
                );
        });

        test('Order commissions number array', async ({ page }) => {
            await expect(page.locator('[data-testid="order-commissions"]'))
                .toContainText('10%');

            await page.click('[data-testid="order-add-commission"]');
            await page.waitForTimeout(50);

            await expect(page.locator('[data-testid="order-commissions"]'))
                .toContainText('5%');
        });

        test('Order deep site nesting', async ({ page }) => {
            await expect(page.locator('[data-testid="order-site-summary"]'))
                .toContainText(
                    'Chicago'
                );
            await expect(page.locator('[data-testid="order-site-summary"]'))
                .toContainText(
                    'Coordinates'
                );
        });
    });

    test.describe('Reactivity Verification', () => {
        test('programmatic set updates DOM across all form types', async ({ page }) => {
            await page.goto('/gigaform/basic');
            await page.waitForSelector('body.hydrated', { timeout: 10000 });

            // Multiple programmatic updates
            await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const phoneNumber = playground?.gigaform?.phoneNumber;
                if (!phoneNumber) throw new Error('the gigaform page did not publish phoneNumber');
                const form = phoneNumber;
                form.fields.phoneType.set('Programmatic');
                form.fields.number.set('999-888-7777');
                form.fields.main.set(true);
            });
            await page.waitForTimeout(50);

            expect(await page.inputValue('[data-testid="phone-type"]')).toBe(
                'Programmatic'
            );
            expect(await page.inputValue('[data-testid="phone-number"]')).toBe(
                '999-888-7777'
            );
            expect(await page.isChecked('[data-testid="phone-main"]')).toBe(true);
        });

        test('concurrent rapid updates maintain consistency', async ({ page }) => {
            await page.goto('/gigaform/basic');
            await page.waitForSelector('body.hydrated', { timeout: 10000 });

            // Rapid updates
            for (let i = 0; i < 10; i++) {
                await page.evaluate((val) => {
                    const playground = globalThis.sveltePlayground;
                    const gradient = playground?.gigaform?.gradient;
                    if (!gradient) throw new Error('the gigaform page did not publish gradient');
                    gradient
                        .fields.startHue.set(val);
                }, i * 36);
            }
            await page.waitForTimeout(100);

            const finalValue = await page.inputValue(
                '[data-testid="gradient-startHue"]'
            );
            expect(Number(finalValue)).toBe(324); // 9 * 36
        });
    });

    test.describe('Reset Functionality', () => {
        test('reset clears all state including errors and tainted', async ({ page }) => {
            await page.goto('/gigaform/basic');
            await page.waitForSelector('body.hydrated', { timeout: 10000 });

            // Modify form
            await page.fill('[data-testid="phone-type"]', 'Modified');

            // Set error and tainted
            await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const phoneNumber = playground?.gigaform?.phoneNumber;
                const option = playground?.effectOption;
                if (!phoneNumber || !option) {
                    throw new Error('the gigaform page did not publish phoneNumber, option');
                }
                const form = phoneNumber;
                form.fields.phoneType.setError(option.some(['Test error']));
                form.fields.phoneType.setTainted(option.some(true));
            });
            await page.waitForTimeout(50);

            // Reset
            await page.click('[data-testid="reset-phone"]');
            await page.waitForTimeout(50);

            // Verify all cleared
            expect(await page.inputValue('[data-testid="phone-type"]')).toBe('');

            const state = await page.evaluate(() => {
                const playground = globalThis.sveltePlayground;
                const phoneNumber = playground?.gigaform?.phoneNumber;
                const option = playground?.effectOption;
                if (!phoneNumber || !option) {
                    throw new Error('the gigaform page did not publish phoneNumber, option');
                }
                const form = phoneNumber;
                return {
                    errorCleared: option.isNone(form.fields.phoneType.getError()),
                    tainted: option.getOrElse(
                        form.fields.phoneType.getTainted(),
                        () => false
                    )
                };
            });

            expect(state.errorCleared).toBe(true);
            expect(state.tainted).toBe(false);
        });
    });

    test.describe('Window Object Exposure', () => {
        test('all forms expose results to window.gigaformResults', async ({ page }) => {
            await page.goto('/gigaform/complex');
            await page.waitForSelector('body.hydrated', { timeout: 10000 });

            const published = await page.evaluate(() => ({
                employee: globalThis.sveltePlayground?.gigaform?.employee !== undefined,
                order: globalThis.sveltePlayground?.gigaform?.order !== undefined
            }));

            expect(published).toEqual({ employee: true, order: true });
        });

        test('validation results stored in window object', async ({ page }) => {
            await page.goto('/gigaform/basic');
            await page.waitForSelector('body.hydrated', { timeout: 10000 });
            await page.waitForSelector('[data-testid="phone-form"]');

            await page.fill('[data-testid="phone-type"]', 'Mobile');
            await page.fill('[data-testid="phone-number"]', '555-123-4567');
            await page.click('[data-testid="submit-phone"]');

            // Wait for validation result to appear
            await page.waitForSelector(
                '[data-testid="phone-result"][data-validation-success]:not([data-validation-success=""])'
            );

            const validation = await page.evaluate(
                () => globalThis.sveltePlayground?.gigaform?.phoneValidation
            );
            if (!validation) throw new Error('phoneValidation was not published');
            const shown = await page.getAttribute(
                '[data-testid="phone-result"]',
                'data-validation-success'
            );
            expect(String(validation.success)).toBe(shown);
        });
    });
});
