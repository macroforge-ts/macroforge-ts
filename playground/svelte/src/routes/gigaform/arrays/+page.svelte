<script lang="ts">
  import { Account, PhoneNumber, type Sector } from "$lib/demo/types.svelte";

  // Create Account form with arrays
  const accountForm = Account.createForm({
    id: "acc-001",
    accountName: { companyName: "Test Company" },
    sector: "Commercial",
    leadSource: "Web",
    accountType: "Customer",
    subtype: "Regular",
    paymentTerms: "Net30",
    dateAdded: new Date().toISOString(),
  });

  // Track validation results
  let accountResult: { success: boolean; data?: Account; errors?: Array<{ field: string; message: string }> } | null = $state(null);

  // New phone form fields
  let newPhoneType = $state("");
  let newPhoneNumber = $state("");
  let newPhoneMain = $state(false);
  let newPhoneCanText = $state(true);
  let newPhoneCanCall = $state(true);

  // New tag field
  let newTag = $state("");

  // New custom field
  let newCustomKey = $state("");
  let newCustomValue = $state("");

  // Expose to Playwright
  if (typeof window !== "undefined") {
    (window as any).gigaformResults = {
      account: accountForm,
    };
  }

  function submitAccount() {
    const result = accountForm.validate();
    if (result.isOk()) {
      accountResult = { success: true, data: result.unwrap() };
    } else {
      accountResult = { success: false, errors: result.unwrapErr() };
    }
    if (typeof window !== "undefined") {
      (window as any).gigaformResults.accountValidation = accountResult;
    }
  }

  function resetAccount() {
    accountForm.reset({
      id: "acc-001",
      accountName: { companyName: "Test Company" },
      sector: "Commercial",
      leadSource: "Web",
      accountType: "Customer",
      subtype: "Regular",
      paymentTerms: "Net30",
      dateAdded: new Date().toISOString(),
    });
    accountResult = null;
  }

  function addPhone() {
    if (!newPhoneType || !newPhoneNumber) return;
    accountForm.fields.phones.push({
      phoneType: newPhoneType,
      number: newPhoneNumber,
      main: newPhoneMain,
      canText: newPhoneCanText,
      canCall: newPhoneCanCall,
    });
    // Reset form
    newPhoneType = "";
    newPhoneNumber = "";
    newPhoneMain = false;
    newPhoneCanText = true;
    newPhoneCanCall = true;
  }

  function removePhone(index: number) {
    accountForm.fields.phones.remove(index);
  }

  function swapPhones(a: number, b: number) {
    accountForm.fields.phones.swap(a, b);
  }

  function addTag() {
    if (!newTag) return;
    const currentTags = accountForm.fields.tags.get();
    accountForm.fields.tags.set([...currentTags, newTag]);
    newTag = "";
  }

  function removeTag(index: number) {
    const currentTags = accountForm.fields.tags.get();
    accountForm.fields.tags.set(currentTags.filter((_, i) => i !== index));
  }

  function addCustomField() {
    if (!newCustomKey || !newCustomValue) return;
    const currentFields = accountForm.fields.customFields.get();
    accountForm.fields.customFields.set([...currentFields, [newCustomKey, newCustomValue]]);
    newCustomKey = "";
    newCustomValue = "";
  }

  function removeCustomField(index: number) {
    const currentFields = accountForm.fields.customFields.get();
    accountForm.fields.customFields.set(currentFields.filter((_, i) => i !== index));
  }
</script>

<svelte:head>
  <title>Array Operations - Gigaform Tests</title>
</svelte:head>

<h1>Array Operations</h1>
<p>Testing Account form with phones[], tags[], and customFields[] arrays.</p>

<a href="/gigaform" class="back-link">&larr; Back to Gigaform Tests</a>

<div class="form-section">
  <h2>Account Form - Array Fields</h2>
  <p class="description">Tests: at(), push(), remove(), swap(), array element access</p>

  <form
    data-testid="account-form"
    onsubmit={(e) => { e.preventDefault(); submitAccount(); }}
  >
    <!-- Phones Array -->
    <fieldset>
      <legend>Phones Array (PhoneNumber[])</legend>
      <p class="hint">Tests push(), remove(), swap(), at() methods</p>

      <div class="array-count" data-testid="phones-count">
        Count: {accountForm.fields.phones.get().length}
      </div>

      <div class="array-items">
        {#each accountForm.fields.phones.get() as phone, index}
          <div class="array-item" data-testid="phone-item-{index}">
            <div class="item-content">
              <strong>{phone.phoneType}</strong>: {phone.number}
              {#if phone.main}<span class="badge">Main</span>{/if}
              {#if phone.canText}<span class="badge">Text</span>{/if}
              {#if phone.canCall}<span class="badge">Call</span>{/if}
            </div>
            <div class="item-actions">
              <button
                type="button"
                data-testid="phone-edit-{index}"
                onclick={() => {
                  const controller = accountForm.fields.phones.at(index);
                  const current = controller.get();
                  controller.set({ ...current, main: !current.main });
                }}
              >
                Toggle Main
              </button>
              {#if index > 0}
                <button
                  type="button"
                  data-testid="phone-swap-up-{index}"
                  onclick={() => swapPhones(index, index - 1)}
                >
                  Move Up
                </button>
              {/if}
              <button
                type="button"
                class="danger"
                data-testid="phone-remove-{index}"
                onclick={() => removePhone(index)}
              >
                Remove
              </button>
            </div>
          </div>
        {/each}
      </div>

      <div class="add-item-form">
        <h4>Add Phone</h4>
        <div class="form-row">
          <input
            type="text"
            placeholder="Type (Mobile, Home...)"
            data-testid="new-phone-type"
            bind:value={newPhoneType}
          />
          <input
            type="text"
            placeholder="Number"
            data-testid="new-phone-number"
            bind:value={newPhoneNumber}
          />
        </div>
        <div class="checkbox-row">
          <label>
            <input type="checkbox" data-testid="new-phone-main" bind:checked={newPhoneMain} />
            Main
          </label>
          <label>
            <input type="checkbox" data-testid="new-phone-canText" bind:checked={newPhoneCanText} />
            Can Text
          </label>
          <label>
            <input type="checkbox" data-testid="new-phone-canCall" bind:checked={newPhoneCanCall} />
            Can Call
          </label>
        </div>
        <button type="button" data-testid="add-phone" onclick={addPhone}>Add Phone</button>
      </div>
    </fieldset>

    <!-- Tags Array -->
    <fieldset>
      <legend>Tags Array (string[])</legend>
      <p class="hint">Simple string array operations</p>

      <div class="array-count" data-testid="tags-count">
        Count: {accountForm.fields.tags.get().length}
      </div>

      <div class="tags-list" data-testid="tags-list">
        {#each accountForm.fields.tags.get() as tag, index}
          <span class="tag" data-testid="tag-item-{index}">
            {tag}
            <button
              type="button"
              class="tag-remove"
              data-testid="tag-remove-{index}"
              onclick={() => removeTag(index)}
            >
              &times;
            </button>
          </span>
        {/each}
      </div>

      <div class="add-item-form inline">
        <input
          type="text"
          placeholder="New tag..."
          data-testid="new-tag"
          bind:value={newTag}
          onkeydown={(e) => { if (e.key === "Enter") { e.preventDefault(); addTag(); } }}
        />
        <button type="button" data-testid="add-tag" onclick={addTag}>Add Tag</button>
      </div>
    </fieldset>

    <!-- Custom Fields Array (Tuples) -->
    <fieldset>
      <legend>Custom Fields ([string, string][])</legend>
      <p class="hint">Tuple array - key/value pairs</p>

      <div class="array-count" data-testid="customFields-count">
        Count: {accountForm.fields.customFields.get().length}
      </div>

      <div class="custom-fields-list">
        {#each accountForm.fields.customFields.get() as field, index}
          <div class="custom-field" data-testid="customField-item-{index}">
            <span class="field-key">{field[0]}</span>
            <span class="field-value">{field[1]}</span>
            <button
              type="button"
              class="danger small"
              data-testid="customField-remove-{index}"
              onclick={() => removeCustomField(index)}
            >
              &times;
            </button>
          </div>
        {/each}
      </div>

      <div class="add-item-form">
        <div class="form-row">
          <input
            type="text"
            placeholder="Key"
            data-testid="new-customField-key"
            bind:value={newCustomKey}
          />
          <input
            type="text"
            placeholder="Value"
            data-testid="new-customField-value"
            bind:value={newCustomValue}
          />
          <button type="button" data-testid="add-customField" onclick={addCustomField}>Add</button>
        </div>
      </div>
    </fieldset>

    <div class="button-row">
      <button type="submit" data-testid="submit-account">Submit</button>
      <button type="button" data-testid="reset-account" onclick={resetAccount}>Reset</button>
    </div>
  </form>

  <div
    class="result-container"
    class:success={accountResult?.success}
    class:error={accountResult && !accountResult.success}
    data-testid="account-result"
    data-validation-success={accountResult === null ? "" : accountResult.success ? "true" : "false"}
  >
    {#if accountResult?.success}
      <pre class="success-output">{JSON.stringify(accountResult.data, null, 2)}</pre>
    {:else if accountResult?.errors}
      <ul class="error-list">
        {#each accountResult.errors as error}
          <li>{error.field}: {error.message}</li>
        {/each}
      </ul>
    {/if}
  </div>

  <div class="data-display" data-testid="account-phones-data">
    <strong>phones[] Data:</strong>
    <pre>{JSON.stringify(accountForm.fields.phones.get(), null, 2)}</pre>
  </div>

  <div class="data-display" data-testid="account-tags-data">
    <strong>tags[] Data:</strong>
    <pre>{JSON.stringify(accountForm.fields.tags.get(), null, 2)}</pre>
  </div>

  <div class="data-display" data-testid="account-customFields-data">
    <strong>customFields[] Data:</strong>
    <pre>{JSON.stringify(accountForm.fields.customFields.get(), null, 2)}</pre>
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
  .array-count {
    font-weight: 600;
    margin-bottom: 0.5rem;
    color: #4f46e5;
  }
  .array-items {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }
  .array-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 0.75rem;
    background: white;
    border: 1px solid #dee2e6;
    border-radius: 4px;
  }
  .item-content {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .item-actions {
    display: flex;
    gap: 0.25rem;
  }
  .item-actions button {
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
  }
  .badge {
    font-size: 0.65rem;
    padding: 0.1rem 0.35rem;
    background: #e5e7eb;
    border-radius: 3px;
    color: #4b5563;
  }
  .tags-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-bottom: 1rem;
    min-height: 32px;
  }
  .tag {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
    background: #4f46e5;
    color: white;
    border-radius: 4px;
    font-size: 0.875rem;
  }
  .tag-remove {
    background: none;
    border: none;
    color: white;
    cursor: pointer;
    padding: 0;
    font-size: 1rem;
    line-height: 1;
  }
  .custom-fields-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }
  .custom-field {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    background: white;
    border: 1px solid #dee2e6;
    border-radius: 4px;
  }
  .field-key {
    font-weight: 600;
    color: #4f46e5;
  }
  .field-value {
    flex: 1;
    color: #4b5563;
  }
  .add-item-form {
    padding: 1rem;
    background: #e5e7eb;
    border-radius: 4px;
  }
  .add-item-form h4 {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    color: #4b5563;
  }
  .add-item-form.inline {
    display: flex;
    gap: 0.5rem;
  }
  .add-item-form input {
    padding: 0.5rem;
    border: 1px solid #ced4da;
    border-radius: 4px;
    font-size: 0.875rem;
    flex: 1;
  }
  .form-row {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }
  .checkbox-row {
    display: flex;
    gap: 1rem;
    margin-bottom: 0.5rem;
  }
  .checkbox-row label {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.875rem;
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
    max-height: 150px;
    overflow: auto;
  }
  .data-display pre {
    margin: 0.5rem 0 0 0;
    font-size: 0.7rem;
    white-space: pre-wrap;
  }
</style>
