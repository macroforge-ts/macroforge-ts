<script lang="ts">
  import { Order, Site, Coordinates, type OrderStage } from "$lib/demo/types.svelte";

  // Create Order form to test deeply nested paths
  // Order -> site (Site) -> coordinates (Coordinates) -> lat/lng
  const orderForm = Order.createForm({
    id: "order-001",
    number: 1001,
    stage: "Active",
    opportunity: "New Sale",
    reference: "REF-001",
    leadSource: "Web",
    group: "Sales",
    subgroup: "Direct",
    memo: "Test order",
    actionItem: "Follow up",
    dateCreated: new Date().toISOString(),
    due: new Date().toISOString(),
    // Site as embedded object (not string reference)
    site: {
      id: "site-001",
      addressLine1: "123 Main Street",
      addressLine2: null,
      sublocalityLevel1: null,
      locality: "Springfield",
      administrativeAreaLevel3: null,
      administrativeAreaLevel2: null,
      administrativeAreaLevel1: "IL",
      country: "USA",
      postalCode: "62701",
      postalCodeSuffix: null,
      coordinates: {
        lat: 39.7817,
        lng: -89.6501,
      },
    },
  });

  // Track validation results
  let orderResult: { success: boolean; data?: Order; errors?: Array<{ field: string; message: string }> } | null = $state(null);

  // Expose to Playwright
  if (typeof window !== "undefined") {
    (window as any).gigaformResults = {
      order: orderForm,
    };
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

  function resetOrder() {
    orderForm.reset({
      id: "order-001",
      number: 1001,
      stage: "Active",
      site: {
        id: "site-001",
        addressLine1: "123 Main Street",
        addressLine2: null,
        sublocalityLevel1: null,
        locality: "Springfield",
        administrativeAreaLevel3: null,
        administrativeAreaLevel2: null,
        administrativeAreaLevel1: "IL",
        country: "USA",
        postalCode: "62701",
        postalCodeSuffix: null,
        coordinates: { lat: 39.7817, lng: -89.6501 },
      },
    });
    orderResult = null;
  }

  // Helper to get site as object (handling string | Site union)
  function getSiteAsObject(): Site | null {
    const site = orderForm.fields.site.get();
    if (typeof site === "string") return null;
    return site;
  }

  // Deep update: Order -> site -> coordinates -> lat
  function updateCoordinatesLat(lat: number) {
    const currentSite = getSiteAsObject();
    if (!currentSite) return;
    orderForm.fields.site.set({
      ...currentSite,
      coordinates: {
        ...currentSite.coordinates,
        lat,
      },
    });
  }

  // Deep update: Order -> site -> coordinates -> lng
  function updateCoordinatesLng(lng: number) {
    const currentSite = getSiteAsObject();
    if (!currentSite) return;
    orderForm.fields.site.set({
      ...currentSite,
      coordinates: {
        ...currentSite.coordinates,
        lng,
      },
    });
  }

  // Deep update: Order -> site -> addressLine1
  function updateSiteAddress(addressLine1: string) {
    const currentSite = getSiteAsObject();
    if (!currentSite) return;
    orderForm.fields.site.set({
      ...currentSite,
      addressLine1,
    });
  }

  const stageOptions: OrderStage[] = ["Estimate", "Active", "Invoice"];
</script>

<svelte:head>
  <title>Deep Nesting - Gigaform Tests</title>
</svelte:head>

<h1>Deep Nesting</h1>
<p>Testing Order form with deep nesting: Order &rarr; Site &rarr; Coordinates &rarr; lat/lng.</p>

<a href="/gigaform" class="back-link">&larr; Back to Gigaform Tests</a>

<div class="form-section">
  <h2>Order Form - Deep Nesting</h2>
  <p class="description">Tests: multi-level navigation, deep path access, deep modification, string vs object reference</p>

  <form
    data-testid="order-form"
    onsubmit={(e) => { e.preventDefault(); submitOrder(); }}
  >
    <!-- Order Basic Fields -->
    <fieldset>
      <legend>Order Info (Level 0)</legend>

      <div class="form-row">
        <div class="form-group">
          <label for="order-id">Order ID</label>
          <input
            type="text"
            id="order-id"
            data-testid="order-id"
            value={orderForm.fields.id.get()}
            oninput={(e) => orderForm.fields.id.set(e.currentTarget.value)}
          />
        </div>
        <div class="form-group">
          <label for="order-number">Order Number</label>
          <input
            type="number"
            id="order-number"
            data-testid="order-number"
            value={orderForm.fields.number.get()}
            oninput={(e) => orderForm.fields.number.set(Number(e.currentTarget.value))}
          />
        </div>
      </div>

      <div class="form-group">
        <label for="order-stage">Stage (enum)</label>
        <select
          id="order-stage"
          data-testid="order-stage"
          value={orderForm.fields.stage.get()}
          onchange={(e) => orderForm.fields.stage.set(e.currentTarget.value as OrderStage)}
        >
          {#each stageOptions as stage}
            <option value={stage}>{stage}</option>
          {/each}
        </select>
      </div>
    </fieldset>

    <!-- Site (Level 1) -->
    <fieldset>
      <legend>Site (Level 1 - Order.site)</legend>
      <p class="hint">site: string | Site - currently an embedded Site object</p>

      <div class="nesting-indicator" data-testid="site-type">
        Site type: {typeof orderForm.fields.site.get() === "string" ? "String Reference" : "Embedded Object"}
      </div>

      {#if getSiteAsObject()}
        {@const site = getSiteAsObject()!}
        <div class="nested-form">
          <div class="form-row">
            <div class="form-group">
              <label for="site-addressLine1">Address Line 1</label>
              <input
                type="text"
                id="site-addressLine1"
                data-testid="site-addressLine1"
                value={site.addressLine1}
                oninput={(e) => updateSiteAddress(e.currentTarget.value)}
              />
            </div>
            <div class="form-group">
              <label for="site-locality">Locality (City)</label>
              <input
                type="text"
                id="site-locality"
                data-testid="site-locality"
                value={site.locality}
                oninput={(e) => {
                  const currentSite = getSiteAsObject()!;
                  orderForm.fields.site.set({ ...currentSite, locality: e.currentTarget.value });
                }}
              />
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label for="site-state">State</label>
              <input
                type="text"
                id="site-state"
                data-testid="site-administrativeAreaLevel1"
                value={site.administrativeAreaLevel1}
                oninput={(e) => {
                  const currentSite = getSiteAsObject()!;
                  orderForm.fields.site.set({ ...currentSite, administrativeAreaLevel1: e.currentTarget.value });
                }}
              />
            </div>
            <div class="form-group">
              <label for="site-postalCode">Postal Code</label>
              <input
                type="text"
                id="site-postalCode"
                data-testid="site-postalCode"
                value={site.postalCode}
                oninput={(e) => {
                  const currentSite = getSiteAsObject()!;
                  orderForm.fields.site.set({ ...currentSite, postalCode: e.currentTarget.value });
                }}
              />
            </div>
          </div>

          <!-- Coordinates (Level 2) -->
          <fieldset class="inner-fieldset">
            <legend>Coordinates (Level 2 - Order.site.coordinates)</legend>

            <div class="path-display" data-testid="coordinates-path">
              Path: order &rarr; site &rarr; coordinates
            </div>

            <div class="form-row">
              <div class="form-group">
                <label for="coordinates-lat">Latitude</label>
                <input
                  type="number"
                  id="coordinates-lat"
                  data-testid="coordinates-lat"
                  step="0.0001"
                  value={site.coordinates.lat}
                  oninput={(e) => updateCoordinatesLat(Number(e.currentTarget.value))}
                />
              </div>
              <div class="form-group">
                <label for="coordinates-lng">Longitude</label>
                <input
                  type="number"
                  id="coordinates-lng"
                  data-testid="coordinates-lng"
                  step="0.0001"
                  value={site.coordinates.lng}
                  oninput={(e) => updateCoordinatesLng(Number(e.currentTarget.value))}
                />
              </div>
            </div>

            <div class="coordinates-display" data-testid="coordinates-display">
              Current: ({site.coordinates.lat.toFixed(4)}, {site.coordinates.lng.toFixed(4)})
            </div>
          </fieldset>
        </div>
      {:else}
        <div class="string-reference" data-testid="site-string-reference">
          Site is a string reference: "{orderForm.fields.site.get()}"
        </div>
      {/if}

      <div class="button-group">
        <button
          type="button"
          data-testid="set-site-string"
          onclick={() => orderForm.fields.site.set("site-ref-123")}
        >
          Set as String Reference
        </button>
        <button
          type="button"
          data-testid="set-site-object"
          onclick={() => orderForm.fields.site.set(Site.defaultValue())}
        >
          Set as Site Object
        </button>
        <button
          type="button"
          data-testid="update-deep-coordinates"
          onclick={() => {
            updateCoordinatesLat(40.7128);
            updateCoordinatesLng(-74.0060);
          }}
        >
          Set to NYC Coordinates
        </button>
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
          <li data-testid="error-{error.field}">{error.field}: {error.message}</li>
        {/each}
      </ul>
    {/if}
  </div>

  <div class="data-display" data-testid="order-site-data">
    <strong>Order.site Data:</strong>
    <pre>{JSON.stringify(orderForm.fields.site.get(), null, 2)}</pre>
  </div>

  <div class="data-display" data-testid="order-data-display">
    <strong>Full Order Data:</strong>
    <pre>{JSON.stringify(orderForm.data, null, 2)}</pre>
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
  .inner-fieldset {
    border-color: #c7d2fe;
    background: #eef2ff;
    margin-top: 1rem;
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
  .nesting-indicator {
    padding: 0.5rem;
    background: #fef3c7;
    border: 1px solid #f59e0b;
    border-radius: 4px;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }
  .nested-form {
    padding: 1rem;
    background: white;
    border: 1px solid #dee2e6;
    border-radius: 6px;
    margin-bottom: 1rem;
  }
  .path-display {
    padding: 0.5rem;
    background: #dbeafe;
    border-radius: 4px;
    margin-bottom: 1rem;
    font-family: monospace;
    font-size: 0.875rem;
    color: #1e40af;
  }
  .coordinates-display {
    padding: 0.5rem;
    background: #d1fae5;
    border-radius: 4px;
    font-family: monospace;
    color: #065f46;
  }
  .string-reference {
    padding: 2rem;
    background: #f3f4f6;
    border: 2px dashed #d1d5db;
    border-radius: 6px;
    text-align: center;
    color: #6b7280;
    margin-bottom: 1rem;
    font-family: monospace;
  }
  .button-group {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
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
