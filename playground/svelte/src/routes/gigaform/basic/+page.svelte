<script lang="ts">
  import { PhoneNumber, Gradient, Coordinates } from "$lib/demo/types.svelte";

  // Create form instances
  const phoneForm = PhoneNumber.createForm();
  const gradientForm = Gradient.createForm();
  const coordinatesForm = Coordinates.createForm();

  // Track validation results
  let phoneResult: { success: boolean; data?: PhoneNumber; errors?: Array<{ field: string; message: string }> } | null = $state(null);
  let gradientResult: { success: boolean; data?: Gradient; errors?: Array<{ field: string; message: string }> } | null = $state(null);
  let coordinatesResult: { success: boolean; data?: Coordinates; errors?: Array<{ field: string; message: string }> } | null = $state(null);

  // Expose forms to Playwright for programmatic testing
  if (typeof window !== "undefined") {
    (window as any).gigaformResults = {
      phoneNumber: phoneForm,
      gradient: gradientForm,
      coordinates: coordinatesForm,
    };
  }

  function submitPhone() {
    const result = phoneForm.validate();
    if (result.isOk()) {
      phoneResult = { success: true, data: result.unwrap() };
    } else {
      phoneResult = { success: false, errors: result.unwrapErr() };
    }
    if (typeof window !== "undefined") {
      (window as any).gigaformResults.phoneValidation = phoneResult;
    }
  }

  function submitGradient() {
    const result = gradientForm.validate();
    if (result.isOk()) {
      gradientResult = { success: true, data: result.unwrap() };
    } else {
      gradientResult = { success: false, errors: result.unwrapErr() };
    }
    if (typeof window !== "undefined") {
      (window as any).gigaformResults.gradientValidation = gradientResult;
    }
  }

  function submitCoordinates() {
    const result = coordinatesForm.validate();
    if (result.isOk()) {
      coordinatesResult = { success: true, data: result.unwrap() };
    } else {
      coordinatesResult = { success: false, errors: result.unwrapErr() };
    }
    if (typeof window !== "undefined") {
      (window as any).gigaformResults.coordinatesValidation = coordinatesResult;
    }
  }

  function resetPhone() {
    phoneForm.reset();
    phoneResult = null;
  }

  function resetGradient() {
    gradientForm.reset();
    gradientResult = null;
  }

  function resetCoordinates() {
    coordinatesForm.reset();
    coordinatesResult = null;
  }
</script>

<svelte:head>
  <title>Basic Fields - Gigaform Tests</title>
</svelte:head>

<h1>Basic Field Operations</h1>
<p>Testing PhoneNumber, Gradient, and Coordinates forms with basic field types.</p>

<a href="/gigaform" class="back-link">&larr; Back to Gigaform Tests</a>

<!-- PhoneNumber Form -->
<div class="form-section">
  <h2>PhoneNumber Form</h2>
  <p class="description">Tests: string fields, boolean fields, required constraints, validation</p>

  <form
    data-testid="phone-form"
    onsubmit={(e) => { e.preventDefault(); submitPhone(); }}
  >
    <div class="form-row">
      <div class="form-group">
        <label for="phone-type">Phone Type</label>
        <input
          type="text"
          id="phone-type"
          data-testid="phone-type"
          placeholder="Mobile, Home, Work..."
          value={phoneForm.fields.phoneType.get()}
          oninput={(e) => {
            phoneForm.fields.phoneType.set(e.currentTarget.value);
            phoneForm.fields.phoneType.setTainted(true);
          }}
        />
        {#if phoneForm.fields.phoneType.getError()}
          <span class="error" data-testid="phone-type-error">
            {phoneForm.fields.phoneType.getError()?.join(", ")}
          </span>
        {/if}
        <span class="tainted-indicator" data-testid="phone-type-tainted">
          Tainted: {phoneForm.fields.phoneType.getTainted()}
        </span>
      </div>

      <div class="form-group">
        <label for="phone-number">Number</label>
        <input
          type="text"
          id="phone-number"
          data-testid="phone-number"
          placeholder="555-123-4567"
          value={phoneForm.fields.number.get()}
          oninput={(e) => {
            phoneForm.fields.number.set(e.currentTarget.value);
            phoneForm.fields.number.setTainted(true);
          }}
        />
        {#if phoneForm.fields.number.getError()}
          <span class="error" data-testid="phone-number-error">
            {phoneForm.fields.number.getError()?.join(", ")}
          </span>
        {/if}
      </div>
    </div>

    <div class="form-row checkboxes">
      <div class="form-group checkbox">
        <label>
          <input
            type="checkbox"
            data-testid="phone-main"
            checked={phoneForm.fields.main.get()}
            onchange={(e) => {
              phoneForm.fields.main.set(e.currentTarget.checked);
              phoneForm.fields.main.setTainted(true);
            }}
          />
          Main Phone
        </label>
      </div>

      <div class="form-group checkbox">
        <label>
          <input
            type="checkbox"
            data-testid="phone-canText"
            checked={phoneForm.fields.canText.get()}
            onchange={(e) => {
              phoneForm.fields.canText.set(e.currentTarget.checked);
              phoneForm.fields.canText.setTainted(true);
            }}
          />
          Can Text
        </label>
      </div>

      <div class="form-group checkbox">
        <label>
          <input
            type="checkbox"
            data-testid="phone-canCall"
            checked={phoneForm.fields.canCall.get()}
            onchange={(e) => {
              phoneForm.fields.canCall.set(e.currentTarget.checked);
              phoneForm.fields.canCall.setTainted(true);
            }}
          />
          Can Call
        </label>
      </div>
    </div>

    <div class="button-row">
      <button type="button" data-testid="submit-phone" onclick={submitPhone}>Submit</button>
      <button type="button" data-testid="reset-phone" onclick={resetPhone}>Reset</button>
      <button type="button" data-testid="validate-phone-type" onclick={() => {
        const errors = phoneForm.fields.phoneType.validate();
        phoneForm.fields.phoneType.setError(errors.length > 0 ? errors : undefined);
      }}>Validate Phone Type</button>
    </div>
  </form>

  <div
    class="result-container"
    class:success={phoneResult?.success}
    class:error={phoneResult && !phoneResult.success}
    data-testid="phone-result"
    data-validation-success={phoneResult === null ? "" : phoneResult.success ? "true" : "false"}
  >
    {#if phoneResult?.success}
      <pre class="success-output">{JSON.stringify(phoneResult.data, null, 2)}</pre>
    {:else if phoneResult?.errors}
      <ul class="error-list">
        {#each phoneResult.errors as error}
          <li>{error.field}: {error.message}</li>
        {/each}
      </ul>
    {/if}
  </div>

  <div class="data-display" data-testid="phone-data-display">
    <strong>Current Data:</strong>
    <pre>{JSON.stringify(phoneForm.data, null, 2)}</pre>
  </div>
</div>

<!-- Gradient Form -->
<div class="form-section">
  <h2>Gradient Form</h2>
  <p class="description">Tests: single number field</p>

  <form
    data-testid="gradient-form"
    onsubmit={(e) => { e.preventDefault(); submitGradient(); }}
  >
    <div class="form-group">
      <label for="gradient-startHue">Start Hue (0-360)</label>
      <input
        type="number"
        id="gradient-startHue"
        data-testid="gradient-startHue"
        min="0"
        max="360"
        value={gradientForm.fields.startHue.get()}
        oninput={(e) => {
          gradientForm.fields.startHue.set(Number(e.currentTarget.value));
          gradientForm.fields.startHue.setTainted(true);
        }}
      />
      {#if gradientForm.fields.startHue.getError()}
        <span class="error" data-testid="gradient-startHue-error">
          {gradientForm.fields.startHue.getError()?.join(", ")}
        </span>
      {/if}
    </div>

    <div class="button-row">
      <button type="button" data-testid="submit-gradient" onclick={submitGradient}>Submit</button>
      <button type="button" data-testid="reset-gradient" onclick={resetGradient}>Reset</button>
    </div>
  </form>

  <div
    class="result-container"
    class:success={gradientResult?.success}
    class:error={gradientResult && !gradientResult.success}
    data-testid="gradient-result"
    data-validation-success={gradientResult === null ? "" : gradientResult.success ? "true" : "false"}
  >
    {#if gradientResult?.success}
      <pre class="success-output">{JSON.stringify(gradientResult.data, null, 2)}</pre>
    {:else if gradientResult?.errors}
      <ul class="error-list">
        {#each gradientResult.errors as error}
          <li>{error.field}: {error.message}</li>
        {/each}
      </ul>
    {/if}
  </div>

  <div class="data-display" data-testid="gradient-data-display">
    <strong>Current Data:</strong>
    <pre>{JSON.stringify(gradientForm.data, null, 2)}</pre>
  </div>
</div>

<!-- Coordinates Form -->
<div class="form-section">
  <h2>Coordinates Form</h2>
  <p class="description">Tests: two number fields (lat, lng)</p>

  <form
    data-testid="coordinates-form"
    onsubmit={(e) => { e.preventDefault(); submitCoordinates(); }}
  >
    <div class="form-row">
      <div class="form-group">
        <label for="coordinates-lat">Latitude</label>
        <input
          type="number"
          id="coordinates-lat"
          data-testid="coordinates-lat"
          step="0.000001"
          value={coordinatesForm.fields.lat.get()}
          oninput={(e) => {
            coordinatesForm.fields.lat.set(Number(e.currentTarget.value));
            coordinatesForm.fields.lat.setTainted(true);
          }}
        />
        {#if coordinatesForm.fields.lat.getError()}
          <span class="error" data-testid="coordinates-lat-error">
            {coordinatesForm.fields.lat.getError()?.join(", ")}
          </span>
        {/if}
      </div>

      <div class="form-group">
        <label for="coordinates-lng">Longitude</label>
        <input
          type="number"
          id="coordinates-lng"
          data-testid="coordinates-lng"
          step="0.000001"
          value={coordinatesForm.fields.lng.get()}
          oninput={(e) => {
            coordinatesForm.fields.lng.set(Number(e.currentTarget.value));
            coordinatesForm.fields.lng.setTainted(true);
          }}
        />
        {#if coordinatesForm.fields.lng.getError()}
          <span class="error" data-testid="coordinates-lng-error">
            {coordinatesForm.fields.lng.getError()?.join(", ")}
          </span>
        {/if}
      </div>
    </div>

    <div class="button-row">
      <button type="button" data-testid="submit-coordinates" onclick={submitCoordinates}>Submit</button>
      <button type="button" data-testid="reset-coordinates" onclick={resetCoordinates}>Reset</button>
    </div>
  </form>

  <div
    class="result-container"
    class:success={coordinatesResult?.success}
    class:error={coordinatesResult && !coordinatesResult.success}
    data-testid="coordinates-result"
    data-validation-success={coordinatesResult === null ? "" : coordinatesResult.success ? "true" : "false"}
  >
    {#if coordinatesResult?.success}
      <pre class="success-output">{JSON.stringify(coordinatesResult.data, null, 2)}</pre>
    {:else if coordinatesResult?.errors}
      <ul class="error-list">
        {#each coordinatesResult.errors as error}
          <li>{error.field}: {error.message}</li>
        {/each}
      </ul>
    {/if}
  </div>

  <div class="data-display" data-testid="coordinates-data-display">
    <strong>Current Data:</strong>
    <pre>{JSON.stringify(coordinatesForm.data, null, 2)}</pre>
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
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }
  .form-row.checkboxes {
    grid-template-columns: repeat(3, 1fr);
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
    font-size: 0.875rem;
  }
  .error {
    display: block;
    color: #dc2626;
    font-size: 0.75rem;
    margin-top: 0.25rem;
  }
  .tainted-indicator {
    display: block;
    font-size: 0.75rem;
    color: #6b7280;
    margin-top: 0.25rem;
  }
  .data-display {
    margin-top: 1rem;
    padding: 0.75rem;
    background: #e5e7eb;
    border-radius: 4px;
    font-size: 0.875rem;
  }
  .data-display pre {
    margin: 0.5rem 0 0 0;
    font-size: 0.75rem;
  }
</style>
