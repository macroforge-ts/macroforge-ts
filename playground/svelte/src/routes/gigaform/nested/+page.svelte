<script lang="ts">
  import { User, Settings, type UserRole } from "$lib/demo/types.svelte";

  // Create User form with nested Settings, Metadata, AppPermissions
  const userForm = User.createForm();

  // Track validation results
  let userResult: { success: boolean; data?: User; errors?: Array<{ field: string; message: string }> } | null = $state(null);

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
    userForm.reset();
    userResult = null;
  }

  // Helper to update nested settings
  function updateScheduleDaysPerWeek(value: number) {
    const currentSettings = userForm.fields.settings.get();
    userForm.fields.settings.set({
      ...currentSettings,
      scheduleSettings: {
        ...currentSettings.scheduleSettings,
        daysPerWeek: value,
      },
    });
  }

  function updateScheduleRowHeight(value: "ExtraSmall" | "Small" | "Medium" | "Large") {
    const currentSettings = userForm.fields.settings.get();
    userForm.fields.settings.set({
      ...currentSettings,
      scheduleSettings: {
        ...currentSettings.scheduleSettings,
        rowHeight: value,
      },
    });
  }

  const roleOptions: UserRole[] = ["Administrator", "SalesRepresentative", "Technician", "HumanResources", "InformationTechnology"];
  const rowHeightOptions = ["ExtraSmall", "Small", "Medium", "Large"] as const;
</script>

<svelte:head>
  <title>Nested Objects - Gigaform Tests</title>
</svelte:head>

<h1>Nested Object Handling</h1>
<p>Testing User form with deeply nested Settings, Metadata, and AppPermissions.</p>

<a href="/gigaform" class="back-link">&larr; Back to Gigaform Tests</a>

<div class="form-section">
  <h2>User Form</h2>
  <p class="description">Tests: nested objects (Settings, Metadata), nullable nested (metadata), enum (role)</p>

  <form
    data-testid="user-form"
    onsubmit={(e) => { e.preventDefault(); submitUser(); }}
  >
    <!-- Basic User Fields -->
    <fieldset>
      <legend>Basic Info</legend>
      <div class="form-row">
        <div class="form-group">
          <label for="user-id">ID</label>
          <input
            type="text"
            id="user-id"
            data-testid="user-id"
            value={userForm.fields.id.get()}
            oninput={(e) => userForm.fields.id.set(e.currentTarget.value)}
          />
        </div>
        <div class="form-group">
          <label for="user-email">Email</label>
          <input
            type="text"
            id="user-email"
            data-testid="user-email"
            value={userForm.fields.email.get() ?? ""}
            oninput={(e) => userForm.fields.email.set(e.currentTarget.value || null)}
          />
        </div>
      </div>

      <div class="form-row">
        <div class="form-group">
          <label for="user-firstName">First Name</label>
          <input
            type="text"
            id="user-firstName"
            data-testid="user-firstName"
            value={userForm.fields.firstName.get()}
            oninput={(e) => userForm.fields.firstName.set(e.currentTarget.value)}
          />
          {#if userForm.fields.firstName.getError()}
            <span class="error" data-testid="user-firstName-error">
              {userForm.fields.firstName.getError()?.join(", ")}
            </span>
          {/if}
        </div>
        <div class="form-group">
          <label for="user-lastName">Last Name</label>
          <input
            type="text"
            id="user-lastName"
            data-testid="user-lastName"
            value={userForm.fields.lastName.get()}
            oninput={(e) => userForm.fields.lastName.set(e.currentTarget.value)}
          />
          {#if userForm.fields.lastName.getError()}
            <span class="error" data-testid="user-lastName-error">
              {userForm.fields.lastName.getError()?.join(", ")}
            </span>
          {/if}
        </div>
      </div>

      <div class="form-row">
        <div class="form-group">
          <label for="user-role">Role</label>
          <select
            id="user-role"
            data-testid="user-role"
            value={userForm.fields.role.get()}
            onchange={(e) => userForm.fields.role.set(e.currentTarget.value as UserRole)}
          >
            {#each roleOptions as role}
              <option value={role}>{role}</option>
            {/each}
          </select>
        </div>
        <div class="form-group checkbox">
          <label>
            <input
              type="checkbox"
              data-testid="user-emailVerified"
              checked={userForm.fields.emailVerified.get()}
              onchange={(e) => userForm.fields.emailVerified.set(e.currentTarget.checked)}
            />
            Email Verified
          </label>
        </div>
      </div>
    </fieldset>

    <!-- Nested Settings -->
    <fieldset>
      <legend>Settings (Nested Object)</legend>
      <p class="hint">Modify scheduleSettings which is nested inside Settings</p>

      <div class="form-row">
        <div class="form-group">
          <label for="settings-daysPerWeek">Days Per Week</label>
          <input
            type="number"
            id="settings-daysPerWeek"
            data-testid="settings-daysPerWeek"
            min="1"
            max="7"
            value={userForm.fields.settings.get().scheduleSettings.daysPerWeek}
            oninput={(e) => updateScheduleDaysPerWeek(Number(e.currentTarget.value))}
          />
        </div>
        <div class="form-group">
          <label for="settings-rowHeight">Row Height</label>
          <select
            id="settings-rowHeight"
            data-testid="settings-rowHeight"
            value={userForm.fields.settings.get().scheduleSettings.rowHeight}
            onchange={(e) => updateScheduleRowHeight(e.currentTarget.value as typeof rowHeightOptions[number])}
          >
            {#each rowHeightOptions as height}
              <option value={height}>{height}</option>
            {/each}
          </select>
        </div>
      </div>

      <div class="form-group checkbox">
        <label>
          <input
            type="checkbox"
            data-testid="settings-detailedCards"
            checked={userForm.fields.settings.get().scheduleSettings.detailedCards}
            onchange={(e) => {
              const currentSettings = userForm.fields.settings.get();
              userForm.fields.settings.set({
                ...currentSettings,
                scheduleSettings: {
                  ...currentSettings.scheduleSettings,
                  detailedCards: e.currentTarget.checked,
                },
              });
            }}
          />
          Detailed Cards
        </label>
      </div>
    </fieldset>

    <!-- Nested Permissions -->
    <fieldset>
      <legend>Permissions (Nested Object)</legend>

      <div class="nested-display" data-testid="permissions-display">
        <strong>Current Permissions Data:</strong>
        <pre>{JSON.stringify(userForm.fields.permissions.get(), null, 2)}</pre>
      </div>

      <button
        type="button"
        data-testid="add-permission-page"
        onclick={() => {
          const currentPerms = userForm.fields.permissions.get();
          userForm.fields.permissions.set({
            ...currentPerms,
            pages: [...currentPerms.pages, "UserHome"],
          });
        }}
      >
        Add UserHome Page Permission
      </button>
    </fieldset>

    <div class="button-row">
      <button type="submit" data-testid="submit-user">Submit</button>
      <button type="button" data-testid="reset-user" onclick={resetUser}>Reset</button>
      <button type="button" data-testid="set-nested-wholesale" onclick={() => {
        // Test wholesale replacement of nested object
        userForm.fields.settings.set(Settings.defaultValue());
      }}>Reset Settings to Default</button>
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
          <li data-testid="error-{error.field}">{error.field}: {error.message}</li>
        {/each}
      </ul>
    {/if}
  </div>

  <div class="data-display" data-testid="user-data-display">
    <strong>Full User Data:</strong>
    <pre>{JSON.stringify(userForm.data, null, 2)}</pre>
  </div>

  <div class="data-display" data-testid="user-errors-display">
    <strong>Errors State:</strong>
    <pre>{JSON.stringify(userForm.errors, null, 2)}</pre>
  </div>

  <div class="data-display" data-testid="user-tainted-display">
    <strong>Tainted State:</strong>
    <pre>{JSON.stringify(userForm.tainted, null, 2)}</pre>
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
    max-height: 300px;
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
    font-size: 0.75rem;
  }
  .error {
    display: block;
    color: #dc2626;
    font-size: 0.75rem;
    margin-top: 0.25rem;
  }
  .hint {
    font-size: 0.75rem;
    color: #6b7280;
    margin: 0 0 1rem 0;
  }
  .data-display, .nested-display {
    margin-top: 1rem;
    padding: 0.75rem;
    background: #e5e7eb;
    border-radius: 4px;
    font-size: 0.875rem;
    max-height: 200px;
    overflow: auto;
  }
  .data-display pre, .nested-display pre {
    margin: 0.5rem 0 0 0;
    font-size: 0.7rem;
    white-space: pre-wrap;
  }
</style>
