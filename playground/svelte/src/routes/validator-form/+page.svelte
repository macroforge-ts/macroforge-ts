<script lang="ts">
  import {
    validateUserRegistration,
    validateProduct,
    validateEvent,
    type ValidationResult,
  } from "$lib/demo/validator-form";

  // Form data
  let userForm = $state({
    email: "",
    password: "",
    username: "",
    age: 0,
    website: "",
  });

  let productForm = $state({
    name: "",
    sku: "",
    price: 0,
    quantity: 0,
    tags: "",
  });

  let eventForm = $state({
    title: "",
    startDate: "",
    endDate: "",
    maxAttendees: 0,
  });

  // Results
  let userResult: ValidationResult<any> | null = $state(null);
  let productResult: ValidationResult<any> | null = $state(null);
  let eventResult: ValidationResult<any> | null = $state(null);

  // Global results for Playwright
  if (typeof window !== "undefined") {
    (window as any).validatorFormResults = {};
  }

  function submitUserRegistration() {
    console.log("submitUserRegistration called");
    try {
      const data = {
        email: userForm.email,
        password: userForm.password,
        username: userForm.username,
        age: userForm.age,
        website: userForm.website,
      };
      console.log("Validating:", data);
      const result = validateUserRegistration(data);
      console.log("Validation result:", result);
      userResult = result;
      if (typeof window !== "undefined") {
        (window as any).validatorFormResults.userRegistration = result;
      }
    } catch (err) {
      console.error("Validation error:", err);
      userResult = { success: false, errors: [String(err)] };
    }
  }

  function submitProduct() {
    const tags = productForm.tags
      ? productForm.tags.split(",").map((t) => t.trim()).filter(Boolean)
      : [];
    const result = validateProduct({
      name: productForm.name,
      sku: productForm.sku,
      price: productForm.price,
      quantity: productForm.quantity,
      tags,
    });
    productResult = result;
    if (typeof window !== "undefined") {
      (window as any).validatorFormResults.product = result;
    }
  }

  function submitEvent() {
    try {
      // Convert date strings to Date objects for validation
      const result = validateEvent({
        title: eventForm.title,
        startDate: new Date(eventForm.startDate),
        endDate: new Date(eventForm.endDate),
        maxAttendees: eventForm.maxAttendees,
      });
      eventResult = result;
      if (typeof window !== "undefined") {
        (window as any).validatorFormResults.event = result;
      }
    } catch (err) {
      console.error("Event validation error:", err);
      eventResult = { success: false, errors: [String(err)] };
    }
  }
</script>

<svelte:head>
  <title>Validator Form - Svelte Playground</title>
</svelte:head>

<h1>Validator Form Testing</h1>
<p>Test deserializer validators with real form data.</p>

<!-- User Registration Form -->
<div class="form-section">
  <h3>User Registration</h3>
  <form
    data-testid="user-registration-form"
    onsubmit={(e) => {
      e.preventDefault();
      submitUserRegistration();
    }}
  >
    <div class="form-row">
      <div class="form-group">
        <label for="user-email">Email</label>
        <input
          type="text"
          id="user-email"
          data-testid="user-email"
          placeholder="user@example.com"
          bind:value={userForm.email}
        />
        <div class="hint">Must be a valid email address</div>
      </div>
      <div class="form-group">
        <label for="user-username">Username</label>
        <input
          type="text"
          id="user-username"
          data-testid="user-username"
          placeholder="johndoe"
          bind:value={userForm.username}
        />
        <div class="hint">3-20 chars, lowercase, start with letter</div>
      </div>
    </div>
    <div class="form-row">
      <div class="form-group">
        <label for="user-password">Password</label>
        <input
          type="text"
          id="user-password"
          data-testid="user-password"
          placeholder="********"
          bind:value={userForm.password}
        />
        <div class="hint">8-50 characters</div>
      </div>
      <div class="form-group">
        <label for="user-age">Age</label>
        <input
          type="number"
          id="user-age"
          data-testid="user-age"
          placeholder="25"
          bind:value={userForm.age}
        />
        <div class="hint">Integer between 18 and 120</div>
      </div>
    </div>
    <div class="form-group">
      <label for="user-website">Website</label>
      <input
        type="text"
        id="user-website"
        data-testid="user-website"
        placeholder="https://example.com"
        bind:value={userForm.website}
      />
      <div class="hint">Must be a valid URL</div>
    </div>
    <button type="submit" data-testid="submit-user-registration">Submit Registration</button>
  </form>
  <div
    class="result-container"
    class:success={userResult?.success}
    class:error={userResult && !userResult.success}
    data-testid="user-registration-result"
    data-validation-success={userResult === null ? "" : userResult.success ? "true" : "false"}
  >
    {#if userResult?.success}
      <pre class="success-output">{JSON.stringify(userResult.data, null, 2)}</pre>
    {:else if userResult?.errors}
      <ul class="error-list">
        {#each userResult.errors as error}
          <li>{error}</li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<!-- Product Form -->
<div class="form-section">
  <h3>Product Entry</h3>
  <form
    data-testid="product-form"
    onsubmit={(e) => {
      e.preventDefault();
      submitProduct();
    }}
  >
    <div class="form-row">
      <div class="form-group">
        <label for="product-name">Product Name</label>
        <input
          type="text"
          id="product-name"
          data-testid="product-name"
          placeholder="Widget"
          bind:value={productForm.name}
        />
        <div class="hint">1-100 characters, non-empty</div>
      </div>
      <div class="form-group">
        <label for="product-sku">SKU (UUID)</label>
        <input
          type="text"
          id="product-sku"
          data-testid="product-sku"
          placeholder="123e4567-e89b-12d3-a456-426614174000"
          bind:value={productForm.sku}
        />
        <div class="hint">Must be a valid UUID</div>
      </div>
    </div>
    <div class="form-row">
      <div class="form-group">
        <label for="product-price">Price</label>
        <input
          type="number"
          id="product-price"
          data-testid="product-price"
          step="0.01"
          placeholder="29.99"
          bind:value={productForm.price}
        />
        <div class="hint">Positive number, less than 1,000,000</div>
      </div>
      <div class="form-group">
        <label for="product-quantity">Quantity</label>
        <input
          type="number"
          id="product-quantity"
          data-testid="product-quantity"
          placeholder="100"
          bind:value={productForm.quantity}
        />
        <div class="hint">Non-negative integer</div>
      </div>
    </div>
    <div class="form-group">
      <label for="product-tags">Tags (comma-separated)</label>
      <input
        type="text"
        id="product-tags"
        data-testid="product-tags"
        placeholder="electronics, gadget, new"
        bind:value={productForm.tags}
      />
      <div class="hint">1-5 tags</div>
    </div>
    <button type="submit" data-testid="submit-product">Submit Product</button>
  </form>
  <div
    class="result-container"
    class:success={productResult?.success}
    class:error={productResult && !productResult.success}
    data-testid="product-result"
    data-validation-success={productResult?.success ?? ""}
  >
    {#if productResult?.success}
      <pre class="success-output">{JSON.stringify(productResult.data, null, 2)}</pre>
    {:else if productResult?.errors}
      <ul class="error-list">
        {#each productResult.errors as error}
          <li>{error}</li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<!-- Event Form -->
<div class="form-section">
  <h3>Event Creation</h3>
  <form
    data-testid="event-form"
    onsubmit={(e) => {
      e.preventDefault();
      submitEvent();
    }}
  >
    <div class="form-group">
      <label for="event-title">Event Title</label>
      <input
        type="text"
        id="event-title"
        data-testid="event-title"
        placeholder="Annual Conference"
        bind:value={eventForm.title}
      />
      <div class="hint">Non-empty, no leading/trailing whitespace</div>
    </div>
    <div class="form-row">
      <div class="form-group">
        <label for="event-start">Start Date</label>
        <input
          type="text"
          id="event-start"
          data-testid="event-start"
          placeholder="2025-06-15"
          bind:value={eventForm.startDate}
        />
        <div class="hint">Valid date after 2020-01-01</div>
      </div>
      <div class="form-group">
        <label for="event-end">End Date</label>
        <input
          type="text"
          id="event-end"
          data-testid="event-end"
          placeholder="2025-06-17"
          bind:value={eventForm.endDate}
        />
        <div class="hint">Valid date</div>
      </div>
    </div>
    <div class="form-group">
      <label for="event-attendees">Max Attendees</label>
      <input
        type="number"
        id="event-attendees"
        data-testid="event-attendees"
        placeholder="200"
        bind:value={eventForm.maxAttendees}
      />
      <div class="hint">Integer between 1 and 1000</div>
    </div>
    <button type="submit" data-testid="submit-event">Submit Event</button>
  </form>
  <div
    class="result-container"
    class:success={eventResult?.success}
    class:error={eventResult && !eventResult.success}
    data-testid="event-result"
    data-validation-success={eventResult?.success ?? ""}
  >
    {#if eventResult?.success}
      <pre class="success-output">{JSON.stringify(eventResult.data, null, 2)}</pre>
    {:else if eventResult?.errors}
      <ul class="error-list">
        {#each eventResult.errors as error}
          <li>{error}</li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<style>
  .form-section {
    background: #f8f9fa;
    border: 1px solid #dee2e6;
    border-radius: 8px;
    padding: 1.5rem;
    margin: 1.5rem 0;
  }
  .form-section h3 {
    margin-top: 0;
    color: #495057;
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
  .form-group input {
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
  button[type="submit"] {
    background: #4f46e5;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    font-size: 1rem;
    cursor: pointer;
    margin-top: 1rem;
  }
  button[type="submit"]:hover {
    background: #4338ca;
  }
  .result-container {
    margin-top: 1rem;
    padding: 1rem;
    border-radius: 4px;
    min-height: 50px;
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
  .hint {
    font-size: 0.75rem;
    color: #6b7280;
    margin-top: 0.25rem;
  }
</style>
