<script lang="ts">
  import {
    Employee,
    Order,
    Settings,
    PhoneNumber,
    type JobTitle,
  } from "$lib/demo/types.svelte";

  // Create Employee form - complex type with nested Settings, arrays, nullable fields
  const employeeForm = Employee.createForm({
    id: "emp-001",
    name: "John Smith",
    imageUrl: null,
    phones: [
      { main: true, phoneType: "Mobile", number: "555-123-4567", canText: true, canCall: true },
    ],
    role: "Developer",
    title: "Technician",
    email: { canEmail: true, emailString: "john.smith@company.com" },
    address: "123 Main St, Springfield, IL 62701",
    username: "jsmith",
    route: "route-001",
    ratePerHour: 75.0,
    active: true,
    isTechnician: true,
    isSalesRep: false,
    description: "Senior developer with 10 years experience",
    linkedinUrl: "https://linkedin.com/in/jsmith",
    attendance: ["2025-01-01", "2025-01-02", "2025-01-03"],
  });

  // Create Order form - deeply nested with multiple arrays
  const orderForm = Order.createForm({
    id: "ord-001",
    number: 1001,
    stage: "Active",
    opportunity: "Website Redesign",
    reference: "REF-2025-001",
    leadSource: "Referral",
    group: "Services",
    subgroup: "Development",
    memo: "Priority project for Q1",
    actionItem: "Schedule kickoff meeting",
    dateCreated: new Date().toISOString(),
    due: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(),
    isPosted: false,
    needsReview: true,
    balance: 5000,
    total: 15000,
    discount: 500,
    tip: 0,
    commissions: [0.1, 0.05],
    billedItems: [
      { item: "prod-001", quantity: 2, taxed: true, upsale: false },
      { item: "svc-001", quantity: 1, taxed: true, upsale: true },
    ],
    site: {
      id: "site-001",
      addressLine1: "456 Business Ave",
      addressLine2: "Suite 200",
      sublocalityLevel1: null,
      locality: "Chicago",
      administrativeAreaLevel3: null,
      administrativeAreaLevel2: "Cook",
      administrativeAreaLevel1: "IL",
      country: "USA",
      postalCode: "60601",
      postalCodeSuffix: null,
      coordinates: { lat: 41.8781, lng: -87.6298 },
    },
  });

  // Validation results
  let employeeResult: { success: boolean; data?: Employee; errors?: Array<{ field: string; message: string }> } | null = $state(null);
  let orderResult: { success: boolean; data?: Order; errors?: Array<{ field: string; message: string }> } | null = $state(null);

  // Expose to Playwright
  if (typeof window !== "undefined") {
    (window as any).gigaformResults = {
      employee: employeeForm,
      order: orderForm,
    };
  }

  function submitEmployee() {
    const result = employeeForm.validate();
    if (result.isOk()) {
      employeeResult = { success: true, data: result.unwrap() };
    } else {
      employeeResult = { success: false, errors: result.unwrapErr() };
    }
    if (typeof window !== "undefined") {
      (window as any).gigaformResults.employeeValidation = employeeResult;
    }
  }

  function submitOrder() {
    const result = orderForm.validate();
    if (result.isOk()) {
      orderResult = { success: true, data: result.unwrap() };
    } else {
      orderResult = { success: false, errors: result.unwrapErr() };
    }
    if (typeof window !== "undefined") {
      (window as any).gigaformResults.orderValidation = orderResult;
    }
  }

  function resetEmployee() {
    employeeForm.reset();
    employeeResult = null;
  }

  function resetOrder() {
    orderForm.reset();
    orderResult = null;
  }

  // Employee phone operations
  function addEmployeePhone() {
    employeeForm.fields.phones.push({
      main: false,
      phoneType: "Work",
      number: "555-000-0000",
      canText: false,
      canCall: true,
    });
  }

  // Order billed item operations
  function addBilledItem() {
    const currentItems = orderForm.fields.billedItems.get();
    orderForm.fields.billedItems.set([
      ...currentItems,
      { item: "new-item", quantity: 1, taxed: true, upsale: false },
    ]);
  }

  const jobTitleOptions: JobTitle[] = ["Technician", "SalesRepresentative", "HumanResources", "InformationTechnology"];
</script>

<svelte:head>
  <title>Complex Integration - Gigaform Tests</title>
</svelte:head>

<h1>Complex Integration Tests</h1>
<p>Full integration testing with Employee and Order forms - combining all features.</p>

<a href="/gigaform" class="back-link">&larr; Back to Gigaform Tests</a>

<!-- Employee Form -->
<div class="form-section">
  <h2>Employee Form (Full Integration)</h2>
  <p class="description">Tests: nested Settings, phones[] array, nullable fields (imageUrl, description, linkedinUrl), attendance[] array, recursive structures</p>

  <form
    data-testid="employee-form"
    onsubmit={(e) => { e.preventDefault(); submitEmployee(); }}
  >
    <!-- Basic Info -->
    <fieldset>
      <legend>Basic Info</legend>
      <div class="form-row">
        <div class="form-group">
          <label for="emp-name">Name</label>
          <input
            type="text"
            id="emp-name"
            data-testid="emp-name"
            value={employeeForm.fields.name.get()}
            oninput={(e) => employeeForm.fields.name.set(e.currentTarget.value)}
          />
        </div>
        <div class="form-group">
          <label for="emp-username">Username</label>
          <input
            type="text"
            id="emp-username"
            data-testid="emp-username"
            value={employeeForm.fields.username.get()}
            oninput={(e) => employeeForm.fields.username.set(e.currentTarget.value)}
          />
        </div>
      </div>

      <div class="form-row">
        <div class="form-group">
          <label for="emp-title">Title (Enum)</label>
          <select
            id="emp-title"
            data-testid="emp-title"
            value={employeeForm.fields.title.get()}
            onchange={(e) => employeeForm.fields.title.set(e.currentTarget.value as JobTitle)}
          >
            {#each jobTitleOptions as title}
              <option value={title}>{title}</option>
            {/each}
          </select>
        </div>
        <div class="form-group">
          <label for="emp-ratePerHour">Rate/Hour ($)</label>
          <input
            type="number"
            id="emp-ratePerHour"
            data-testid="emp-ratePerHour"
            step="0.01"
            value={employeeForm.fields.ratePerHour.get()}
            oninput={(e) => employeeForm.fields.ratePerHour.set(Number(e.currentTarget.value))}
          />
        </div>
      </div>

      <div class="checkbox-row">
        <label>
          <input
            type="checkbox"
            data-testid="emp-active"
            checked={employeeForm.fields.active.get()}
            onchange={(e) => employeeForm.fields.active.set(e.currentTarget.checked)}
          />
          Active
        </label>
        <label>
          <input
            type="checkbox"
            data-testid="emp-isTechnician"
            checked={employeeForm.fields.isTechnician.get()}
            onchange={(e) => employeeForm.fields.isTechnician.set(e.currentTarget.checked)}
          />
          Technician
        </label>
        <label>
          <input
            type="checkbox"
            data-testid="emp-isSalesRep"
            checked={employeeForm.fields.isSalesRep.get()}
            onchange={(e) => employeeForm.fields.isSalesRep.set(e.currentTarget.checked)}
          />
          Sales Rep
        </label>
      </div>
    </fieldset>

    <!-- Phones Array -->
    <fieldset>
      <legend>Phones (PhoneNumber[])</legend>
      <div class="array-count" data-testid="emp-phones-count">
        Count: {employeeForm.fields.phones.get().length}
      </div>
      <div class="array-items">
        {#each employeeForm.fields.phones.get() as phone, i}
          <div class="array-item" data-testid="emp-phone-{i}">
            <span>{phone.phoneType}: {phone.number}</span>
            <button
              type="button"
              class="danger small"
              data-testid="emp-phone-remove-{i}"
              onclick={() => employeeForm.fields.phones.remove(i)}
            >
              Remove
            </button>
          </div>
        {/each}
      </div>
      <button type="button" data-testid="emp-add-phone" onclick={addEmployeePhone}>
        Add Phone
      </button>
    </fieldset>

    <!-- Nullable Fields -->
    <fieldset>
      <legend>Nullable Fields</legend>
      <div class="form-row">
        <div class="form-group">
          <label for="emp-imageUrl">Image URL (nullable)</label>
          <input
            type="text"
            id="emp-imageUrl"
            data-testid="emp-imageUrl"
            placeholder="null if empty"
            value={employeeForm.fields.imageUrl.get() ?? ""}
            oninput={(e) => employeeForm.fields.imageUrl.set(e.currentTarget.value || null)}
          />
          <span class="nullable-indicator">
            {employeeForm.fields.imageUrl.get() === null ? "null" : "has value"}
          </span>
        </div>
        <div class="form-group">
          <label for="emp-linkedinUrl">LinkedIn (nullable)</label>
          <input
            type="text"
            id="emp-linkedinUrl"
            data-testid="emp-linkedinUrl"
            placeholder="null if empty"
            value={employeeForm.fields.linkedinUrl.get() ?? ""}
            oninput={(e) => employeeForm.fields.linkedinUrl.set(e.currentTarget.value || null)}
          />
          <span class="nullable-indicator">
            {employeeForm.fields.linkedinUrl.get() === null ? "null" : "has value"}
          </span>
        </div>
      </div>
    </fieldset>

    <!-- Nested Settings -->
    <fieldset>
      <legend>Settings (Deeply Nested)</legend>
      <div class="nested-summary" data-testid="emp-settings-summary">
        Schedule: {employeeForm.fields.settings.get().scheduleSettings.daysPerWeek} days/week,
        Row Height: {employeeForm.fields.settings.get().scheduleSettings.rowHeight}
      </div>
      <button
        type="button"
        data-testid="emp-reset-settings"
        onclick={() => employeeForm.fields.settings.set(Settings.defaultValue())}
      >
        Reset Settings to Default
      </button>
    </fieldset>

    <div class="button-row">
      <button type="submit" data-testid="submit-employee">Submit</button>
      <button type="button" data-testid="reset-employee" onclick={resetEmployee}>Reset</button>
    </div>
  </form>

  <div
    class="result-container"
    class:success={employeeResult?.success}
    class:error={employeeResult && !employeeResult.success}
    data-testid="employee-result"
    data-validation-success={employeeResult === null ? "" : employeeResult.success ? "true" : "false"}
  >
    {#if employeeResult?.success}
      <pre class="success-output">{JSON.stringify(employeeResult.data, null, 2)}</pre>
    {:else if employeeResult?.errors}
      <ul class="error-list">
        {#each employeeResult.errors as error}
          <li>{error.field}: {error.message}</li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<!-- Order Form -->
<div class="form-section">
  <h2>Order Form (Full Integration)</h2>
  <p class="description">Tests: deeply nested (Site/Coordinates), billedItems[] array, commissions[] number array, nullable arrays (package, promotion), string | object references</p>

  <form
    data-testid="order-form"
    onsubmit={(e) => { e.preventDefault(); submitOrder(); }}
  >
    <!-- Basic Info -->
    <fieldset>
      <legend>Order Info</legend>
      <div class="form-row">
        <div class="form-group">
          <label for="order-number">Order #</label>
          <input
            type="number"
            id="order-number"
            data-testid="order-number"
            value={orderForm.fields.number.get()}
            oninput={(e) => orderForm.fields.number.set(Number(e.currentTarget.value))}
          />
        </div>
        <div class="form-group">
          <label for="order-reference">Reference</label>
          <input
            type="text"
            id="order-reference"
            data-testid="order-reference"
            value={orderForm.fields.reference.get()}
            oninput={(e) => orderForm.fields.reference.set(e.currentTarget.value)}
          />
        </div>
      </div>

      <div class="form-row">
        <div class="form-group">
          <label for="order-total">Total ($)</label>
          <input
            type="number"
            id="order-total"
            data-testid="order-total"
            value={orderForm.fields.total.get()}
            oninput={(e) => orderForm.fields.total.set(Number(e.currentTarget.value))}
          />
        </div>
        <div class="form-group">
          <label for="order-balance">Balance ($)</label>
          <input
            type="number"
            id="order-balance"
            data-testid="order-balance"
            value={orderForm.fields.balance.get()}
            oninput={(e) => orderForm.fields.balance.set(Number(e.currentTarget.value))}
          />
        </div>
      </div>
    </fieldset>

    <!-- Billed Items Array -->
    <fieldset>
      <legend>Billed Items (BilledItem[])</legend>
      <div class="array-count" data-testid="order-billedItems-count">
        Count: {orderForm.fields.billedItems.get().length}
      </div>
      <div class="array-items">
        {#each orderForm.fields.billedItems.get() as item, i}
          <div class="array-item" data-testid="order-billedItem-{i}">
            <span>
              Item: {typeof item.item === "string" ? item.item : "object"} |
              Qty: {item.quantity} |
              {item.taxed ? "Taxed" : "Not taxed"} |
              {item.upsale ? "Upsale" : "Regular"}
            </span>
          </div>
        {/each}
      </div>
      <button type="button" data-testid="order-add-billedItem" onclick={addBilledItem}>
        Add Billed Item
      </button>
    </fieldset>

    <!-- Commissions Array (number[]) -->
    <fieldset>
      <legend>Commissions (number[])</legend>
      <div class="array-display" data-testid="order-commissions">
        [{orderForm.fields.commissions.get().map(c => (c * 100).toFixed(0) + "%").join(", ")}]
      </div>
      <button
        type="button"
        data-testid="order-add-commission"
        onclick={() => {
          const current = orderForm.fields.commissions.get();
          orderForm.fields.commissions.set([...current, 0.05]);
        }}
      >
        Add 5% Commission
      </button>
    </fieldset>

    <!-- Deep Nested Site -->
    <fieldset>
      <legend>Site (Deeply Nested)</legend>
      <div class="nested-summary" data-testid="order-site-summary">
        {#if typeof orderForm.fields.site.get() === "string"}
          Reference: {orderForm.fields.site.get()}
        {:else}
          {@const site = orderForm.fields.site.get() as any}
          {site.addressLine1}, {site.locality}, {site.administrativeAreaLevel1}
          <br />
          Coordinates: ({site.coordinates.lat.toFixed(4)}, {site.coordinates.lng.toFixed(4)})
        {/if}
      </div>
    </fieldset>

    <div class="button-row">
      <button type="submit" data-testid="submit-order">Submit</button>
      <button type="button" data-testid="reset-order" onclick={resetOrder}>Reset</button>
    </div>
  </form>

  <div
    class="result-container"
    class:success={orderResult?.success}
    class:error={orderResult && !orderResult.success}
    data-testid="order-result"
    data-validation-success={orderResult === null ? "" : orderResult.success ? "true" : "false"}
  >
    {#if orderResult?.success}
      <pre class="success-output">{JSON.stringify(orderResult.data, null, 2)}</pre>
    {:else if orderResult?.errors}
      <ul class="error-list">
        {#each orderResult.errors as error}
          <li>{error.field}: {error.message}</li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<!-- Data Displays -->
<div class="form-section">
  <h2>Raw Form Data</h2>

  <details>
    <summary data-testid="employee-data-toggle">Employee Form Data</summary>
    <pre data-testid="employee-data-display">{JSON.stringify(employeeForm.data, null, 2)}</pre>
  </details>

  <details>
    <summary data-testid="order-data-toggle">Order Form Data</summary>
    <pre data-testid="order-data-display">{JSON.stringify(orderForm.data, null, 2)}</pre>
  </details>
</div>

<style>
  h1 { margin-bottom: 0.5rem; }
  p { color: #6b7280; }
  .back-link {
    display: inline-block;
    margin-bottom: 1.5rem;
    color: #4f46e5;
    text-decoration: none;
  }
  .back-link:hover { text-decoration: underline; }
  .form-section {
    background: #f8f9fa;
    border: 1px solid #dee2e6;
    border-radius: 8px;
    padding: 1.5rem;
    margin: 1.5rem 0;
  }
  .form-section h2 {
    margin: 0 0 0.25rem 0;
    color: #495057;
  }
  .description {
    margin: 0 0 1rem 0;
    font-size: 0.875rem;
  }
  fieldset {
    border: 1px solid #dee2e6;
    border-radius: 6px;
    padding: 1rem;
    margin-bottom: 1rem;
  }
  legend {
    font-weight: 600;
    color: #4f46e5;
    padding: 0 0.5rem;
  }
  .form-group {
    margin-bottom: 1rem;
  }
  .form-group label {
    display: block;
    margin-bottom: 0.25rem;
    font-weight: 500;
    color: #495057;
  }
  .form-group input[type="text"],
  .form-group input[type="number"],
  .form-group select {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #ced4da;
    border-radius: 4px;
    font-size: 1rem;
    box-sizing: border-box;
  }
  .form-group input:focus,
  .form-group select:focus {
    outline: none;
    border-color: #4f46e5;
    box-shadow: 0 0 0 2px rgba(79, 70, 229, 0.2);
  }
  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }
  .checkbox-row {
    display: flex;
    gap: 1.5rem;
    margin-top: 0.5rem;
  }
  .checkbox-row label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
  }
  .array-count {
    font-weight: 600;
    margin-bottom: 0.5rem;
    color: #4f46e5;
  }
  .array-items {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }
  .array-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    background: white;
    border: 1px solid #dee2e6;
    border-radius: 4px;
    font-size: 0.875rem;
  }
  .array-display {
    padding: 0.5rem;
    background: #e5e7eb;
    border-radius: 4px;
    font-family: monospace;
    margin-bottom: 0.5rem;
  }
  .nested-summary {
    padding: 0.5rem;
    background: #e0e7ff;
    border-radius: 4px;
    font-size: 0.875rem;
    color: #3730a3;
  }
  .nullable-indicator {
    display: block;
    font-size: 0.75rem;
    color: #6b7280;
    margin-top: 0.25rem;
  }
  .button-row {
    display: flex;
    gap: 0.5rem;
    margin-top: 1rem;
  }
  button {
    background: #4f46e5;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-size: 0.875rem;
    cursor: pointer;
  }
  button:hover { background: #4338ca; }
  button[type="button"] {
    background: #6b7280;
  }
  button[type="button"]:hover { background: #4b5563; }
  button.danger {
    background: #dc2626;
  }
  button.danger:hover { background: #b91c1c; }
  button.small {
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
  }
  .result-container {
    margin-top: 1rem;
    padding: 1rem;
    border-radius: 4px;
    min-height: 50px;
    background: #e5e7eb;
    max-height: 200px;
    overflow: auto;
  }
  .result-container.success {
    background: #d1fae5;
    border: 1px solid #10b981;
  }
  .result-container.error {
    background: #fee2e2;
    border: 1px solid #ef4444;
  }
  .error-list {
    margin: 0;
    padding-left: 1.5rem;
    color: #dc2626;
  }
  .success-output {
    margin: 0;
    background: transparent;
    color: #059669;
    font-size: 0.7rem;
  }
  details {
    margin-top: 1rem;
  }
  details summary {
    cursor: pointer;
    font-weight: 500;
    padding: 0.5rem;
    background: #e5e7eb;
    border-radius: 4px;
  }
  details pre {
    margin: 0.5rem 0 0 0;
    padding: 1rem;
    background: white;
    border: 1px solid #dee2e6;
    border-radius: 4px;
    font-size: 0.7rem;
    max-height: 300px;
    overflow: auto;
  }
</style>
