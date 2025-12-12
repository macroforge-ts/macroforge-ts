<script lang="ts">
  import { User, Metadata } from "$lib/demo/types.svelte";

  // Create User form to test nullable fields
  const userForm = User.createForm({
    id: "user-001",
    firstName: "John",
    lastName: "Doe",
    email: null, // nullable string
    metadata: null, // nullable nested object
    password: null,
    verificationToken: null,
    verificationExpires: null,
    passwordResetToken: null,
    passwordResetExpires: null,
  });

  // Track validation results
  let userResult: { success: boolean; data?: User; errors?: Array<{ field: string; message: string }> } | null = $state(null);

  // Track metadata toggle
  let hasMetadata = $state(false);

  // Metadata form fields
  let metadataCreatedAt = $state(new Date().toISOString());
  let metadataLastLogin = $state<string | null>(null);
  let metadataIsActive = $state(true);
  let metadataRoles = $state<string[]>(["user"]);

  // Expose to Playwright
  if (typeof window !== "undefined") {
    (window as any).gigaformResults = {
      user: userForm,
    };
  }

  function submitUser() {
    const result = userForm.validate();
    if (result.isOk()) {
      userResult = { success: true, data: result.unwrap() };
    } else {
      userResult = { success: false, errors: result.unwrapErr() };
    }
    if (typeof window !== "undefined") {
      (window as any).gigaformResults.userValidation = userResult;
    }
  }

  function resetUser() {
    userForm.reset({
      id: "user-001",
      firstName: "John",
      lastName: "Doe",
      email: null,
      metadata: null,
      password: null,
    });
    userResult = null;
    hasMetadata = false;
  }

  function toggleMetadata() {
    hasMetadata = !hasMetadata;
    if (hasMetadata) {
      userForm.fields.metadata.set({
        createdAt: metadataCreatedAt,
        lastLogin: metadataLastLogin,
        isActive: metadataIsActive,
        roles: metadataRoles,
      });
    } else {
      userForm.fields.metadata.set(null);
    }
  }

  function updateMetadata() {
    if (hasMetadata) {
      userForm.fields.metadata.set({
        createdAt: metadataCreatedAt,
        lastLogin: metadataLastLogin,
        isActive: metadataIsActive,
        roles: metadataRoles,
      });
    }
  }

  // Sync hasMetadata with form state
  $effect(() => {
    const currentMetadata = userForm.fields.metadata.get();
    hasMetadata = currentMetadata !== null;
    if (currentMetadata) {
      metadataCreatedAt = currentMetadata.createdAt;
      metadataLastLogin = currentMetadata.lastLogin;
      metadataIsActive = currentMetadata.isActive;
      metadataRoles = currentMetadata.roles;
    }
  });
</script>

<svelte:head>
  <title>Nullable Fields - Gigaform Tests</title>
</svelte:head>

<h1>Nullable Field Handling</h1>
<p>Testing User form with nullable fields (email: string | null, metadata: Metadata | null).</p>

<a href="/gigaform" class="back-link">&larr; Back to Gigaform Tests</a>

<div class="form-section">
  <h2>User Form - Nullable Fields</h2>
  <p class="description">Tests: null initialization, null to object transition, object to null transition, tainted tracking</p>

  <form
    data-testid="user-form"
    onsubmit={(e) => { e.preventDefault(); submitUser(); }}
  >
    <!-- Nullable String Field -->
    <fieldset>
      <legend>Email (string | null)</legend>
      <p class="hint">Simple nullable primitive field</p>

      <div class="nullable-status" data-testid="email-nullable-status">
        Current: {userForm.fields.email.get() === null ? "null" : `"${userForm.fields.email.get()}"`}
      </div>

      <div class="form-group">
        <label for="user-email">Email</label>
        <input
          type="text"
          id="user-email"
          data-testid="user-email"
          placeholder="Enter email or leave empty for null"
          value={userForm.fields.email.get() ?? ""}
          oninput={(e) => {
            const value = e.currentTarget.value;
            userForm.fields.email.set(value === "" ? null : value);
            userForm.fields.email.setTainted(true);
          }}
        />
      </div>

      <div class="button-group">
        <button
          type="button"
          data-testid="set-email-null"
          onclick={() => userForm.fields.email.set(null)}
        >
          Set to null
        </button>
        <button
          type="button"
          data-testid="set-email-value"
          onclick={() => userForm.fields.email.set("test@example.com")}
        >
          Set to "test@example.com"
        </button>
      </div>

      <div class="tainted-status" data-testid="email-tainted">
        Tainted: {userForm.fields.email.getTainted()}
      </div>
    </fieldset>

    <!-- Nullable Nested Object Field -->
    <fieldset>
      <legend>Metadata (Metadata | null)</legend>
      <p class="hint">Nullable nested object - can be null or full Metadata object</p>

      <div class="nullable-status" data-testid="metadata-nullable-status">
        Current: {userForm.fields.metadata.get() === null ? "null" : "Metadata object"}
      </div>

      <div class="toggle-container">
        <label class="toggle-label">
          <input
            type="checkbox"
            data-testid="metadata-toggle"
            checked={hasMetadata}
            onchange={toggleMetadata}
          />
          Has Metadata
        </label>
      </div>

      {#if hasMetadata}
        <div class="nested-form" data-testid="metadata-form">
          <div class="form-row">
            <div class="form-group">
              <label for="metadata-createdAt">Created At</label>
              <input
                type="text"
                id="metadata-createdAt"
                data-testid="metadata-createdAt"
                bind:value={metadataCreatedAt}
                oninput={updateMetadata}
              />
            </div>
            <div class="form-group">
              <label for="metadata-lastLogin">Last Login (nullable)</label>
              <input
                type="text"
                id="metadata-lastLogin"
                data-testid="metadata-lastLogin"
                placeholder="null if empty"
                value={metadataLastLogin ?? ""}
                oninput={(e) => {
                  metadataLastLogin = e.currentTarget.value === "" ? null : e.currentTarget.value;
                  updateMetadata();
                }}
              />
            </div>
          </div>

          <div class="form-group checkbox">
            <label>
              <input
                type="checkbox"
                data-testid="metadata-isActive"
                bind:checked={metadataIsActive}
                onchange={updateMetadata}
              />
              Is Active
            </label>
          </div>

          <div class="form-group">
            <label>Roles (comma-separated)</label>
            <input
              type="text"
              data-testid="metadata-roles"
              value={metadataRoles.join(", ")}
              oninput={(e) => {
                metadataRoles = e.currentTarget.value.split(",").map(r => r.trim()).filter(Boolean);
                updateMetadata();
              }}
            />
          </div>
        </div>
      {:else}
        <div class="null-placeholder" data-testid="metadata-null-placeholder">
          Metadata is currently <code>null</code>
        </div>
      {/if}

      <div class="button-group">
        <button
          type="button"
          data-testid="set-metadata-null"
          onclick={() => {
            userForm.fields.metadata.set(null);
            hasMetadata = false;
          }}
        >
          Set to null
        </button>
        <button
          type="button"
          data-testid="set-metadata-default"
          onclick={() => {
            userForm.fields.metadata.set(Metadata.defaultValue());
            hasMetadata = true;
          }}
        >
          Set to Default Metadata
        </button>
      </div>

      <div class="raw-value" data-testid="metadata-raw">
        <strong>Raw metadata value:</strong>
        <pre>{JSON.stringify(userForm.fields.metadata.get(), null, 2)}</pre>
      </div>
    </fieldset>

    <!-- Other nullable string fields -->
    <fieldset>
      <legend>Other Nullable Strings</legend>

      <div class="form-row">
        <div class="form-group">
          <label for="user-password">Password (nullable)</label>
          <input
            type="text"
            id="user-password"
            data-testid="user-password"
            placeholder="null if empty"
            value={userForm.fields.password.get() ?? ""}
            oninput={(e) => {
              const value = e.currentTarget.value;
              userForm.fields.password.set(value === "" ? null : value);
            }}
          />
          <span class="nullable-indicator">
            {userForm.fields.password.get() === null ? "null" : "has value"}
          </span>
        </div>

        <div class="form-group">
          <label for="user-verificationToken">Verification Token (nullable)</label>
          <input
            type="text"
            id="user-verificationToken"
            data-testid="user-verificationToken"
            placeholder="null if empty"
            value={userForm.fields.verificationToken.get() ?? ""}
            oninput={(e) => {
              const value = e.currentTarget.value;
              userForm.fields.verificationToken.set(value === "" ? null : value);
            }}
          />
          <span class="nullable-indicator">
            {userForm.fields.verificationToken.get() === null ? "null" : "has value"}
          </span>
        </div>
      </div>
    </fieldset>

    <div class="button-row">
      <button type="submit" data-testid="submit-user">Submit</button>
      <button type="button" data-testid="reset-user" onclick={resetUser}>Reset</button>
    </div>
  </form>

  <div
    class="result-container"
    class:success={userResult?.success}
    class:error={userResult && !userResult.success}
    data-testid="user-result"
    data-validation-success={userResult === null ? "" : userResult.success ? "true" : "false"}
  >
    {#if userResult?.success}
      <pre class="success-output">{JSON.stringify(userResult.data, null, 2)}</pre>
    {:else if userResult?.errors}
      <ul class="error-list">
        {#each userResult.errors as error}
          <li>{error.field}: {error.message}</li>
        {/each}
      </ul>
    {/if}
  </div>

  <div class="data-display" data-testid="user-data-display">
    <strong>Full User Data (showing nulls):</strong>
    <pre>{JSON.stringify(userForm.data, null, 2)}</pre>
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
  .nullable-status {
    padding: 0.5rem;
    background: #fef3c7;
    border: 1px solid #f59e0b;
    border-radius: 4px;
    margin-bottom: 1rem;
    font-family: monospace;
    font-size: 0.875rem;
  }
  .toggle-container {
    margin-bottom: 1rem;
  }
  .toggle-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    font-weight: 500;
  }
  .nested-form {
    padding: 1rem;
    background: white;
    border: 1px solid #dee2e6;
    border-radius: 6px;
    margin-bottom: 1rem;
  }
  .null-placeholder {
    padding: 2rem;
    background: #f3f4f6;
    border: 2px dashed #d1d5db;
    border-radius: 6px;
    text-align: center;
    color: #6b7280;
    margin-bottom: 1rem;
  }
  .null-placeholder code {
    background: #e5e7eb;
    padding: 0.1rem 0.3rem;
    border-radius: 3px;
  }
  .button-group {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }
  .tainted-status, .nullable-indicator {
    font-size: 0.75rem;
    color: #6b7280;
    margin-top: 0.25rem;
  }
  .raw-value {
    padding: 0.5rem;
    background: #f3f4f6;
    border-radius: 4px;
    font-size: 0.75rem;
  }
  .raw-value pre {
    margin: 0.25rem 0 0 0;
    max-height: 150px;
    overflow: auto;
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
  .form-group input[type="text"] {
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
