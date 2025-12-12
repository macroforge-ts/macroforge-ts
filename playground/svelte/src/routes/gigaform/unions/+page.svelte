<script lang="ts">
  import { Lead, CompanyName, PersonName, type AccountName, type Priority, type LeadStage, type Sector } from "$lib/demo/types.svelte";

  // Create Lead form with union type AccountName
  const leadForm = Lead.createForm({
    id: "lead-001",
    leadName: { companyName: "Acme Corp" }, // Default to CompanyName variant
    sector: "Commercial",
    status: "Active",
    accountType: "Prospect",
    subtype: "Standard",
    paymentTerms: "Net30",
    memo: "Initial contact",
  });

  // Track current variant type
  let currentVariant: "company" | "person" = $state("company");

  // Track validation results
  let leadResult: { success: boolean; data?: Lead; errors?: Array<{ field: string; message: string }> } | null = $state(null);

  // Form fields for company name variant
  let companyNameValue = $state("Acme Corp");

  // Form fields for person name variant
  let firstNameValue = $state("");
  let lastNameValue = $state("");

  // Expose to Playwright
  if (typeof window !== "undefined") {
    (window as any).gigaformResults = {
      lead: leadForm,
    };
  }

  function submitLead() {
    const result = leadForm.validate();
    if (result.isOk()) {
      leadResult = { success: true, data: result.unwrap() };
    } else {
      leadResult = { success: false, errors: result.unwrapErr() };
    }
    if (typeof window !== "undefined") {
      (window as any).gigaformResults.leadValidation = leadResult;
    }
  }

  function resetLead() {
    leadForm.reset({
      id: "lead-001",
      leadName: { companyName: "Acme Corp" },
      sector: "Commercial",
      status: "Active",
      accountType: "Prospect",
      subtype: "Standard",
      paymentTerms: "Net30",
      memo: "Initial contact",
    });
    leadResult = null;
    currentVariant = "company";
    companyNameValue = "Acme Corp";
    firstNameValue = "";
    lastNameValue = "";
  }

  function switchToCompany() {
    currentVariant = "company";
    leadForm.fields.leadName.set({ companyName: companyNameValue || "New Company" });
  }

  function switchToPerson() {
    currentVariant = "person";
    leadForm.fields.leadName.set({
      firstName: firstNameValue || "John",
      lastName: lastNameValue || "Doe"
    });
  }

  function updateCompanyName(value: string) {
    companyNameValue = value;
    if (currentVariant === "company") {
      leadForm.fields.leadName.set({ companyName: value });
    }
  }

  function updatePersonName(first: string, last: string) {
    firstNameValue = first;
    lastNameValue = last;
    if (currentVariant === "person") {
      leadForm.fields.leadName.set({ firstName: first, lastName: last });
    }
  }

  // Helper to detect current variant
  function detectVariant(name: AccountName): "company" | "person" {
    if ("companyName" in name) return "company";
    return "person";
  }

  // Sync local state with form data
  $effect(() => {
    const currentName = leadForm.fields.leadName.get();
    const detectedVariant = detectVariant(currentName);
    if (detectedVariant !== currentVariant) {
      currentVariant = detectedVariant;
    }
  });

  const priorityOptions: Priority[] = ["High", "Medium", "Low"];
  const stageOptions: LeadStage[] = ["Open", "InitialContact", "Qualified", "Estimate", "Negotiation"];
  const sectorOptions: Sector[] = ["Residential", "Commercial"];
</script>

<svelte:head>
  <title>Union Types - Gigaform Tests</title>
</svelte:head>

<h1>Union Type Handling</h1>
<p>Testing Lead form with AccountName union type (CompanyName | PersonName).</p>

<a href="/gigaform" class="back-link">&larr; Back to Gigaform Tests</a>

<div class="form-section">
  <h2>Lead Form - Union Type (AccountName)</h2>
  <p class="description">Tests: switching between union variants, validation of both variants</p>

  <form
    data-testid="lead-form"
    onsubmit={(e) => { e.preventDefault(); submitLead(); }}
  >
    <!-- Union Type Selector -->
    <fieldset>
      <legend>Lead Name (AccountName Union)</legend>
      <p class="hint">AccountName = CompanyName | PersonName</p>

      <div class="variant-selector">
        <button
          type="button"
          class="variant-btn"
          class:active={currentVariant === "company"}
          data-testid="select-company-variant"
          onclick={switchToCompany}
        >
          Company Name
        </button>
        <button
          type="button"
          class="variant-btn"
          class:active={currentVariant === "person"}
          data-testid="select-person-variant"
          onclick={switchToPerson}
        >
          Person Name
        </button>
      </div>

      <div class="current-variant" data-testid="current-variant">
        Current Variant: <strong>{currentVariant}</strong>
      </div>

      {#if currentVariant === "company"}
        <div class="variant-form" data-testid="company-form">
          <div class="form-group">
            <label for="lead-companyName">Company Name</label>
            <input
              type="text"
              id="lead-companyName"
              data-testid="lead-companyName"
              value={companyNameValue}
              oninput={(e) => updateCompanyName(e.currentTarget.value)}
            />
            {#if leadForm.fields.leadName.getError()}
              <span class="error" data-testid="lead-name-error">
                {leadForm.fields.leadName.getError()?.join(", ")}
              </span>
            {/if}
          </div>
        </div>
      {:else}
        <div class="variant-form" data-testid="person-form">
          <div class="form-row">
            <div class="form-group">
              <label for="lead-firstName">First Name</label>
              <input
                type="text"
                id="lead-firstName"
                data-testid="lead-firstName"
                value={firstNameValue}
                oninput={(e) => updatePersonName(e.currentTarget.value, lastNameValue)}
              />
            </div>
            <div class="form-group">
              <label for="lead-lastName">Last Name</label>
              <input
                type="text"
                id="lead-lastName"
                data-testid="lead-lastName"
                value={lastNameValue}
                oninput={(e) => updatePersonName(firstNameValue, e.currentTarget.value)}
              />
            </div>
          </div>
          {#if leadForm.fields.leadName.getError()}
            <span class="error" data-testid="lead-name-error">
              {leadForm.fields.leadName.getError()?.join(", ")}
            </span>
          {/if}
        </div>
      {/if}

      <div class="raw-value" data-testid="leadName-raw">
        <strong>Raw leadName value:</strong>
        <pre>{JSON.stringify(leadForm.fields.leadName.get(), null, 2)}</pre>
      </div>
    </fieldset>

    <!-- Other Lead Fields -->
    <fieldset>
      <legend>Lead Details</legend>

      <div class="form-row">
        <div class="form-group">
          <label for="lead-priority">Priority</label>
          <select
            id="lead-priority"
            data-testid="lead-priority"
            value={leadForm.fields.priority.get()}
            onchange={(e) => leadForm.fields.priority.set(e.currentTarget.value as Priority)}
          >
            {#each priorityOptions as priority}
              <option value={priority}>{priority}</option>
            {/each}
          </select>
        </div>
        <div class="form-group">
          <label for="lead-stage">Stage</label>
          <select
            id="lead-stage"
            data-testid="lead-stage"
            value={leadForm.fields.stage.get()}
            onchange={(e) => leadForm.fields.stage.set(e.currentTarget.value as LeadStage)}
          >
            {#each stageOptions as stage}
              <option value={stage}>{stage}</option>
            {/each}
          </select>
        </div>
      </div>

      <div class="form-row">
        <div class="form-group">
          <label for="lead-sector">Sector</label>
          <select
            id="lead-sector"
            data-testid="lead-sector"
            value={leadForm.fields.sector.get()}
            onchange={(e) => leadForm.fields.sector.set(e.currentTarget.value as Sector)}
          >
            {#each sectorOptions as sector}
              <option value={sector}>{sector}</option>
            {/each}
          </select>
        </div>
        <div class="form-group">
          <label for="lead-value">Value ($)</label>
          <input
            type="number"
            id="lead-value"
            data-testid="lead-value"
            value={leadForm.fields.value.get()}
            oninput={(e) => leadForm.fields.value.set(Number(e.currentTarget.value))}
          />
        </div>
      </div>

      <div class="form-group">
        <label for="lead-probability">Probability (%)</label>
        <input
          type="number"
          id="lead-probability"
          data-testid="lead-probability"
          min="0"
          max="100"
          value={leadForm.fields.probability.get()}
          oninput={(e) => leadForm.fields.probability.set(Number(e.currentTarget.value))}
        />
      </div>

      <div class="form-group checkbox">
        <label>
          <input
            type="checkbox"
            data-testid="lead-favorite"
            checked={leadForm.fields.favorite.get()}
            onchange={(e) => leadForm.fields.favorite.set(e.currentTarget.checked)}
          />
          Favorite
        </label>
      </div>
    </fieldset>

    <div class="button-row">
      <button type="submit" data-testid="submit-lead">Submit</button>
      <button type="button" data-testid="reset-lead" onclick={resetLead}>Reset</button>
      <button
        type="button"
        data-testid="set-invalid-variant"
        onclick={() => {
          // Intentionally set an invalid union variant structure
          leadForm.fields.leadName.set({ invalidField: "test" } as any);
        }}
      >
        Set Invalid Variant
      </button>
    </div>
  </form>

  <div
    class="result-container"
    class:success={leadResult?.success}
    class:error={leadResult && !leadResult.success}
    data-testid="lead-result"
    data-validation-success={leadResult === null ? "" : leadResult.success ? "true" : "false"}
  >
    {#if leadResult?.success}
      <pre class="success-output">{JSON.stringify(leadResult.data, null, 2)}</pre>
    {:else if leadResult?.errors}
      <ul class="error-list">
        {#each leadResult.errors as error}
          <li data-testid="error-{error.field}">{error.field}: {error.message}</li>
        {/each}
      </ul>
    {/if}
  </div>

  <div class="data-display" data-testid="lead-data-display">
    <strong>Full Lead Data:</strong>
    <pre>{JSON.stringify(leadForm.data, null, 2)}</pre>
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
  .variant-selector {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }
  .variant-btn {
    flex: 1;
    padding: 0.75rem 1rem;
    background: #e5e7eb;
    border: 2px solid transparent;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 500;
    color: #4b5563;
    transition: all 0.2s;
  }
  .variant-btn:hover {
    background: #d1d5db;
  }
  .variant-btn.active {
    background: #4f46e5;
    color: white;
    border-color: #3730a3;
  }
  .current-variant {
    margin-bottom: 1rem;
    padding: 0.5rem;
    background: #e5e7eb;
    border-radius: 4px;
    font-size: 0.875rem;
  }
  .variant-form {
    padding: 1rem;
    background: white;
    border: 1px solid #dee2e6;
    border-radius: 6px;
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
  .form-group:last-child {
    margin-bottom: 0;
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
  .checkbox label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
  }
  .checkbox input[type="checkbox"] {
    width: auto;
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
  .result-container {
    margin-top: 1rem;
    padding: 1rem;
    border-radius: 4px;
    min-height: 50px;
    background: #e5e7eb;
    max-height: 250px;
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
  .error {
    display: block;
    color: #dc2626;
    font-size: 0.75rem;
    margin-top: 0.25rem;
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
