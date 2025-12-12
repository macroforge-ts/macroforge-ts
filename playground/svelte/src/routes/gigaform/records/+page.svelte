<script lang="ts">
  import { TaxRate } from "$lib/demo/types.svelte";

  // Create TaxRate form to test Record/Map types
  const taxRateForm = TaxRate.createForm({
    id: "tax-001",
    name: "California Sales Tax",
    taxAgency: "CA State",
    zip: 90210,
    city: "Beverly Hills",
    county: "Los Angeles",
    state: "CA",
    isActive: true,
    description: "Standard California sales tax",
    taxComponents: {
      state: 0.0725,
      county: 0.01,
      city: 0.0125,
    },
  });

  // Track validation results
  let taxRateResult: { success: boolean; data?: TaxRate; errors?: Array<{ field: string; message: string }> } | null = $state(null);

  // New component form
  let newComponentKey = $state("");
  let newComponentValue = $state(0);

  // Expose to Playwright
  if (typeof window !== "undefined") {
    (window as any).gigaformResults = {
      taxRate: taxRateForm,
    };
  }

  function submitTaxRate() {
    const result = taxRateForm.validate();
    if (result.isOk()) {
      taxRateResult = { success: true, data: result.unwrap() };
    } else {
      taxRateResult = { success: false, errors: result.unwrapErr() };
    }
    if (typeof window !== "undefined") {
      (window as any).gigaformResults.taxRateValidation = taxRateResult;
    }
  }

  function resetTaxRate() {
    taxRateForm.reset({
      id: "tax-001",
      name: "California Sales Tax",
      taxAgency: "CA State",
      zip: 90210,
      city: "Beverly Hills",
      county: "Los Angeles",
      state: "CA",
      isActive: true,
      description: "Standard California sales tax",
      taxComponents: {
        state: 0.0725,
        county: 0.01,
        city: 0.0125,
      },
    });
    taxRateResult = null;
  }

  function addComponent() {
    if (!newComponentKey) return;
    const current = taxRateForm.fields.taxComponents.get();
    taxRateForm.fields.taxComponents.set({
      ...current,
      [newComponentKey]: newComponentValue,
    });
    newComponentKey = "";
    newComponentValue = 0;
  }

  function removeComponent(key: string) {
    const current = { ...taxRateForm.fields.taxComponents.get() };
    delete current[key];
    taxRateForm.fields.taxComponents.set(current);
  }

  function updateComponent(key: string, value: number) {
    const current = taxRateForm.fields.taxComponents.get();
    taxRateForm.fields.taxComponents.set({
      ...current,
      [key]: value,
    });
  }

  // Get components as entries for iteration
  function getComponentEntries(): [string, number][] {
    return Object.entries(taxRateForm.fields.taxComponents.get());
  }

  // Calculate total rate
  function getTotalRate(): number {
    const components = taxRateForm.fields.taxComponents.get();
    return Object.values(components).reduce((sum, val) => sum + val, 0);
  }
</script>

<svelte:head>
  <title>Record/Map Types - Gigaform Tests</title>
</svelte:head>

<h1>Record/Map Type Handling</h1>
<p>Testing TaxRate form with taxComponents: {'{ [key: string]: number }'}.</p>

<a href="/gigaform" class="back-link">&larr; Back to Gigaform Tests</a>

<div class="form-section">
  <h2>TaxRate Form - Record Type</h2>
  <p class="description">Tests: dynamic key operations, adding/removing entries, modifying values</p>

  <form
    data-testid="taxrate-form"
    onsubmit={(e) => { e.preventDefault(); submitTaxRate(); }}
  >
    <!-- Basic TaxRate Fields -->
    <fieldset>
      <legend>Tax Rate Info</legend>

      <div class="form-row">
        <div class="form-group">
          <label for="taxrate-name">Name</label>
          <input
            type="text"
            id="taxrate-name"
            data-testid="taxrate-name"
            value={taxRateForm.fields.name.get()}
            oninput={(e) => taxRateForm.fields.name.set(e.currentTarget.value)}
          />
        </div>
        <div class="form-group">
          <label for="taxrate-taxAgency">Tax Agency</label>
          <input
            type="text"
            id="taxrate-taxAgency"
            data-testid="taxrate-taxAgency"
            value={taxRateForm.fields.taxAgency.get()}
            oninput={(e) => taxRateForm.fields.taxAgency.set(e.currentTarget.value)}
          />
        </div>
      </div>

      <div class="form-row">
        <div class="form-group">
          <label for="taxrate-city">City</label>
          <input
            type="text"
            id="taxrate-city"
            data-testid="taxrate-city"
            value={taxRateForm.fields.city.get()}
            oninput={(e) => taxRateForm.fields.city.set(e.currentTarget.value)}
          />
        </div>
        <div class="form-group">
          <label for="taxrate-state">State</label>
          <input
            type="text"
            id="taxrate-state"
            data-testid="taxrate-state"
            value={taxRateForm.fields.state.get()}
            oninput={(e) => taxRateForm.fields.state.set(e.currentTarget.value)}
          />
        </div>
      </div>

      <div class="form-group checkbox">
        <label>
          <input
            type="checkbox"
            data-testid="taxrate-isActive"
            checked={taxRateForm.fields.isActive.get()}
            onchange={(e) => taxRateForm.fields.isActive.set(e.currentTarget.checked)}
          />
          Is Active
        </label>
      </div>
    </fieldset>

    <!-- Tax Components (Record Type) -->
    <fieldset>
      <legend>Tax Components ({'{ [key: string]: number }'})</legend>
      <p class="hint">Dynamic key-value pairs for tax breakdown</p>

      <div class="record-summary" data-testid="taxComponents-summary">
        <span>Entries: {Object.keys(taxRateForm.fields.taxComponents.get()).length}</span>
        <span>Total Rate: {(getTotalRate() * 100).toFixed(2)}%</span>
      </div>

      <div class="record-entries">
        {#each getComponentEntries() as [key, value], index}
          <div class="record-entry" data-testid="taxComponent-{key}">
            <span class="entry-key">{key}</span>
            <input
              type="number"
              step="0.0001"
              class="entry-value"
              data-testid="taxComponent-{key}-value"
              value={value}
              oninput={(e) => updateComponent(key, Number(e.currentTarget.value))}
            />
            <span class="entry-percent">{(value * 100).toFixed(2)}%</span>
            <button
              type="button"
              class="danger small"
              data-testid="taxComponent-remove-{key}"
              onclick={() => removeComponent(key)}
            >
              Remove
            </button>
          </div>
        {/each}

        {#if Object.keys(taxRateForm.fields.taxComponents.get()).length === 0}
          <div class="empty-record" data-testid="taxComponents-empty">
            No tax components defined. Add one below.
          </div>
        {/if}
      </div>

      <div class="add-entry-form">
        <h4>Add Component</h4>
        <div class="form-row">
          <input
            type="text"
            placeholder="Component name (e.g., district)"
            data-testid="new-component-key"
            bind:value={newComponentKey}
          />
          <input
            type="number"
            step="0.0001"
            placeholder="Rate (e.g., 0.01)"
            data-testid="new-component-value"
            bind:value={newComponentValue}
          />
          <button type="button" data-testid="add-component" onclick={addComponent}>
            Add
          </button>
        </div>
      </div>

      <div class="button-group">
        <button
          type="button"
          data-testid="clear-all-components"
          onclick={() => taxRateForm.fields.taxComponents.set({})}
        >
          Clear All
        </button>
        <button
          type="button"
          data-testid="set-default-components"
          onclick={() => taxRateForm.fields.taxComponents.set({
            state: 0.06,
            county: 0.01,
            city: 0.01,
          })}
        >
          Set Default Components
        </button>
      </div>

      <div class="raw-value" data-testid="taxComponents-raw">
        <strong>Raw taxComponents value:</strong>
        <pre>{JSON.stringify(taxRateForm.fields.taxComponents.get(), null, 2)}</pre>
      </div>
    </fieldset>

    <div class="button-row">
      <button type="submit" data-testid="submit-taxrate">Submit</button>
      <button type="button" data-testid="reset-taxrate" onclick={resetTaxRate}>Reset</button>
    </div>
  </form>

  <div
    class="result-container"
    class:success={taxRateResult?.success}
    class:error={taxRateResult && !taxRateResult.success}
    data-testid="taxrate-result"
    data-validation-success={taxRateResult === null ? "" : taxRateResult.success ? "true" : "false"}
  >
    {#if taxRateResult?.success}
      <pre class="success-output">{JSON.stringify(taxRateResult.data, null, 2)}</pre>
    {:else if taxRateResult?.errors}
      <ul class="error-list">
        {#each taxRateResult.errors as error}
          <li>{error.field}: {error.message}</li>
        {/each}
      </ul>
    {/if}
  </div>

  <div class="data-display" data-testid="taxrate-data-display">
    <strong>Full TaxRate Data:</strong>
    <pre>{JSON.stringify(taxRateForm.data, null, 2)}</pre>
  </div>
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
  .hint {
    font-size: 0.75rem;
    color: #6b7280;
    margin: 0 0 1rem 0;
  }
  .record-summary {
    display: flex;
    justify-content: space-between;
    padding: 0.75rem;
    background: #e0e7ff;
    border-radius: 4px;
    margin-bottom: 1rem;
    font-weight: 500;
    color: #3730a3;
  }
  .record-entries {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }
  .record-entry {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    background: white;
    border: 1px solid #dee2e6;
    border-radius: 4px;
  }
  .entry-key {
    font-weight: 600;
    color: #4f46e5;
    min-width: 80px;
  }
  .entry-value {
    width: 120px;
    padding: 0.25rem 0.5rem;
    border: 1px solid #ced4da;
    border-radius: 4px;
  }
  .entry-percent {
    color: #6b7280;
    font-size: 0.875rem;
    min-width: 60px;
  }
  .empty-record {
    padding: 2rem;
    background: #f3f4f6;
    border: 2px dashed #d1d5db;
    border-radius: 6px;
    text-align: center;
    color: #6b7280;
  }
  .add-entry-form {
    padding: 1rem;
    background: #e5e7eb;
    border-radius: 4px;
    margin-bottom: 1rem;
  }
  .add-entry-form h4 {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    color: #4b5563;
  }
  .button-group {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }
  .raw-value {
    padding: 0.5rem;
    background: #f3f4f6;
    border-radius: 4px;
    font-size: 0.75rem;
  }
  .raw-value pre {
    margin: 0.25rem 0 0 0;
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
  .form-group input[type="number"] {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #ced4da;
    border-radius: 4px;
    font-size: 1rem;
    box-sizing: border-box;
  }
  .form-group input:focus {
    outline: none;
    border-color: #4f46e5;
    box-shadow: 0 0 0 2px rgba(79, 70, 229, 0.2);
  }
  .form-row {
    display: flex;
    gap: 0.5rem;
  }
  .form-row input {
    flex: 1;
  }
  .checkbox label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
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
  .data-display {
    margin-top: 1rem;
    padding: 0.75rem;
    background: #e5e7eb;
    border-radius: 4px;
    font-size: 0.875rem;
    max-height: 200px;
    overflow: auto;
  }
  .data-display pre {
    margin: 0.5rem 0 0 0;
    font-size: 0.7rem;
    white-space: pre-wrap;
  }
</style>
