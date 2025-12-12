<script lang="ts">
  import {
    Appointment,
    type Status,
    type UserRole,
    type Sector,
    type Priority,
    type LeadStage,
    type RowHeight,
    type OrderStage,
    type Weekday,
  } from "$lib/demo/types.svelte";

  // Create Appointment form to test enum fields
  const appointmentForm = Appointment.createForm({
    id: "appt-001",
    title: "Client Meeting",
    status: "Scheduled",
    begins: new Date().toISOString(),
    duration: 60,
    timeZone: "America/New_York",
    offsetMs: -18000000,
    allDay: false,
    multiDay: false,
    employees: [],
    location: "site-001",
    description: "Initial consultation",
    colors: { main: "#4f46e5", hover: "#4338ca", active: "#3730a3" },
    recurrenceRule: null,
  });

  // Track validation results
  let appointmentResult: { success: boolean; data?: Appointment; errors?: Array<{ field: string; message: string }> } | null = $state(null);

  // Expose to Playwright
  if (typeof window !== "undefined") {
    (window as any).gigaformResults = {
      appointment: appointmentForm,
    };
  }

  function submitAppointment() {
    const result = appointmentForm.validate();
    if (result.isOk()) {
      appointmentResult = { success: true, data: result.unwrap() };
    } else {
      appointmentResult = { success: false, errors: result.unwrapErr() };
    }
    if (typeof window !== "undefined") {
      (window as any).gigaformResults.appointmentValidation = appointmentResult;
    }
  }

  function resetAppointment() {
    appointmentForm.reset({
      id: "appt-001",
      title: "Client Meeting",
      status: "Scheduled",
      begins: new Date().toISOString(),
      duration: 60,
      timeZone: "America/New_York",
    });
    appointmentResult = null;
  }

  // All enum options for testing
  const statusOptions: Status[] = ["Scheduled", "OnDeck", "Waiting"];
  const userRoleOptions: UserRole[] = ["Administrator", "SalesRepresentative", "Technician", "HumanResources", "InformationTechnology"];
  const sectorOptions: Sector[] = ["Residential", "Commercial"];
  const priorityOptions: Priority[] = ["High", "Medium", "Low"];
  const leadStageOptions: LeadStage[] = ["Open", "InitialContact", "Qualified", "Estimate", "Negotiation"];
  const rowHeightOptions: RowHeight[] = ["ExtraSmall", "Small", "Medium", "Large"];
  const orderStageOptions: OrderStage[] = ["Estimate", "Active", "Invoice"];
  const weekdayOptions: Weekday[] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

  // Test enum for demonstration
  let selectedUserRole: UserRole = $state("Administrator");
  let selectedSector: Sector = $state("Residential");
  let selectedPriority: Priority = $state("Medium");
  let selectedRowHeight: RowHeight = $state("Medium");
  let selectedWeekdays: Weekday[] = $state(["Monday", "Wednesday", "Friday"]);
</script>

<svelte:head>
  <title>Enum Fields - Gigaform Tests</title>
</svelte:head>

<h1>Enum Field Handling</h1>
<p>Testing various enum types: Status, UserRole, Sector, Priority, LeadStage, RowHeight, OrderStage, Weekday.</p>

<a href="/gigaform" class="back-link">&larr; Back to Gigaform Tests</a>

<div class="form-section">
  <h2>Appointment Form - Status Enum</h2>
  <p class="description">Tests: enum selection, validation, tainted tracking</p>

  <form
    data-testid="appointment-form"
    onsubmit={(e) => { e.preventDefault(); submitAppointment(); }}
  >
    <fieldset>
      <legend>Status (Enum: Status)</legend>
      <p class="hint">Status = "Scheduled" | "OnDeck" | "Waiting"</p>

      <div class="current-value" data-testid="status-current">
        Current: <strong>{appointmentForm.fields.status.get()}</strong>
      </div>

      <div class="enum-selector">
        {#each statusOptions as status}
          <button
            type="button"
            class="enum-option"
            class:active={appointmentForm.fields.status.get() === status}
            data-testid="status-{status}"
            onclick={() => {
              appointmentForm.fields.status.set(status);
              appointmentForm.fields.status.setTainted(true);
            }}
          >
            {status}
          </button>
        {/each}
      </div>

      <div class="form-group">
        <label for="status-select">Or use select:</label>
        <select
          id="status-select"
          data-testid="status-select"
          value={appointmentForm.fields.status.get()}
          onchange={(e) => {
            appointmentForm.fields.status.set(e.currentTarget.value as Status);
            appointmentForm.fields.status.setTainted(true);
          }}
        >
          {#each statusOptions as status}
            <option value={status}>{status}</option>
          {/each}
        </select>
      </div>

      <div class="tainted-status" data-testid="status-tainted">
        Tainted: {appointmentForm.fields.status.getTainted()}
      </div>
    </fieldset>

    <fieldset>
      <legend>Appointment Details</legend>

      <div class="form-row">
        <div class="form-group">
          <label for="appt-title">Title</label>
          <input
            type="text"
            id="appt-title"
            data-testid="appt-title"
            value={appointmentForm.fields.title.get()}
            oninput={(e) => appointmentForm.fields.title.set(e.currentTarget.value)}
          />
        </div>
        <div class="form-group">
          <label for="appt-duration">Duration (minutes)</label>
          <input
            type="number"
            id="appt-duration"
            data-testid="appt-duration"
            value={appointmentForm.fields.duration.get()}
            oninput={(e) => appointmentForm.fields.duration.set(Number(e.currentTarget.value))}
          />
        </div>
      </div>

      <div class="form-row">
        <div class="form-group checkbox">
          <label>
            <input
              type="checkbox"
              data-testid="appt-allDay"
              checked={appointmentForm.fields.allDay.get()}
              onchange={(e) => appointmentForm.fields.allDay.set(e.currentTarget.checked)}
            />
            All Day
          </label>
        </div>
        <div class="form-group checkbox">
          <label>
            <input
              type="checkbox"
              data-testid="appt-multiDay"
              checked={appointmentForm.fields.multiDay.get()}
              onchange={(e) => appointmentForm.fields.multiDay.set(e.currentTarget.checked)}
            />
            Multi-Day
          </label>
        </div>
      </div>
    </fieldset>

    <div class="button-row">
      <button type="button" data-testid="submit-appointment" onclick={submitAppointment}>Submit</button>
      <button type="button" data-testid="reset-appointment" onclick={resetAppointment}>Reset</button>
      <button
        type="button"
        data-testid="set-invalid-status"
        onclick={() => {
          appointmentForm.fields.status.set("InvalidStatus" as any);
        }}
      >
        Set Invalid Status
      </button>
    </div>
  </form>

  <div
    class="result-container"
    class:success={appointmentResult?.success}
    class:error={appointmentResult && !appointmentResult.success}
    data-testid="appointment-result"
    data-validation-success={appointmentResult === null ? "" : appointmentResult.success ? "true" : "false"}
  >
    {#if appointmentResult?.success}
      <pre class="success-output">{JSON.stringify(appointmentResult.data, null, 2)}</pre>
    {:else if appointmentResult?.errors}
      <ul class="error-list">
        {#each appointmentResult.errors as error}
          <li>{error.field}: {error.message}</li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<!-- All Enum Types Reference -->
<div class="form-section">
  <h2>All Enum Types Reference</h2>
  <p class="description">Interactive showcase of all enum types in the system</p>

  <div class="enum-showcase">
    <!-- UserRole -->
    <fieldset>
      <legend>UserRole</legend>
      <div class="enum-display" data-testid="userRole-showcase">
        <select bind:value={selectedUserRole} data-testid="userRole-select">
          {#each userRoleOptions as role}
            <option value={role}>{role}</option>
          {/each}
        </select>
        <span class="enum-value">Selected: {selectedUserRole}</span>
      </div>
    </fieldset>

    <!-- Sector -->
    <fieldset>
      <legend>Sector</legend>
      <div class="enum-display" data-testid="sector-showcase">
        <div class="radio-group">
          {#each sectorOptions as sector}
            <label>
              <input
                type="radio"
                name="sector"
                value={sector}
                checked={selectedSector === sector}
                onchange={() => selectedSector = sector}
                data-testid="sector-{sector}"
              />
              {sector}
            </label>
          {/each}
        </div>
        <span class="enum-value">Selected: {selectedSector}</span>
      </div>
    </fieldset>

    <!-- Priority -->
    <fieldset>
      <legend>Priority</legend>
      <div class="enum-display" data-testid="priority-showcase">
        <div class="priority-buttons">
          {#each priorityOptions as priority}
            <button
              type="button"
              class="priority-btn priority-{priority.toLowerCase()}"
              class:active={selectedPriority === priority}
              onclick={() => selectedPriority = priority}
              data-testid="priority-{priority}"
            >
              {priority}
            </button>
          {/each}
        </div>
        <span class="enum-value">Selected: {selectedPriority}</span>
      </div>
    </fieldset>

    <!-- RowHeight -->
    <fieldset>
      <legend>RowHeight</legend>
      <div class="enum-display" data-testid="rowHeight-showcase">
        <div class="slider-container">
          {#each rowHeightOptions as height, i}
            <button
              type="button"
              class="height-btn"
              class:active={selectedRowHeight === height}
              onclick={() => selectedRowHeight = height}
              data-testid="rowHeight-{height}"
            >
              {height}
            </button>
          {/each}
        </div>
        <span class="enum-value">Selected: {selectedRowHeight}</span>
      </div>
    </fieldset>

    <!-- Weekday (multi-select example) -->
    <fieldset>
      <legend>Weekday (Multi-Select)</legend>
      <div class="enum-display" data-testid="weekday-showcase">
        <div class="weekday-grid">
          {#each weekdayOptions as day}
            <label class="weekday-checkbox">
              <input
                type="checkbox"
                checked={selectedWeekdays.includes(day)}
                onchange={(e) => {
                  if (e.currentTarget.checked) {
                    selectedWeekdays = [...selectedWeekdays, day];
                  } else {
                    selectedWeekdays = selectedWeekdays.filter(d => d !== day);
                  }
                }}
                data-testid="weekday-{day}"
              />
              {day.slice(0, 3)}
            </label>
          {/each}
        </div>
        <span class="enum-value">Selected: [{selectedWeekdays.join(", ")}]</span>
      </div>
    </fieldset>

    <!-- LeadStage and OrderStage -->
    <fieldset>
      <legend>Stage Enums (LeadStage & OrderStage)</legend>
      <div class="enum-display">
        <div class="stage-lists">
          <div>
            <strong>LeadStage:</strong>
            <ul data-testid="leadStage-list">
              {#each leadStageOptions as stage}
                <li>{stage}</li>
              {/each}
            </ul>
          </div>
          <div>
            <strong>OrderStage:</strong>
            <ul data-testid="orderStage-list">
              {#each orderStageOptions as stage}
                <li>{stage}</li>
              {/each}
            </ul>
          </div>
        </div>
      </div>
    </fieldset>
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
    font-family: monospace;
  }
  .current-value {
    padding: 0.5rem;
    background: #e0e7ff;
    border-radius: 4px;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }
  .enum-selector {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }
  .enum-option {
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
  .enum-option:hover {
    background: #d1d5db;
  }
  .enum-option.active {
    background: #4f46e5;
    color: white;
    border-color: #3730a3;
  }
  .tainted-status {
    font-size: 0.75rem;
    color: #6b7280;
    margin-top: 0.5rem;
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
  /* Enum showcase styles */
  .enum-showcase {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1rem;
  }
  .enum-display {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .enum-value {
    font-size: 0.75rem;
    color: #6b7280;
    font-family: monospace;
  }
  .radio-group {
    display: flex;
    gap: 1rem;
  }
  .radio-group label {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    cursor: pointer;
  }
  .priority-buttons {
    display: flex;
    gap: 0.5rem;
  }
  .priority-btn {
    flex: 1;
    padding: 0.5rem;
    border-radius: 4px;
    border: 2px solid transparent;
  }
  .priority-btn.priority-high { background: #fee2e2; color: #dc2626; }
  .priority-btn.priority-medium { background: #fef3c7; color: #d97706; }
  .priority-btn.priority-low { background: #d1fae5; color: #059669; }
  .priority-btn.active { border-color: currentColor; }
  .slider-container {
    display: flex;
    gap: 0.25rem;
  }
  .height-btn {
    flex: 1;
    padding: 0.5rem;
    font-size: 0.75rem;
    background: #e5e7eb;
    border: 2px solid transparent;
  }
  .height-btn.active {
    background: #4f46e5;
    color: white;
  }
  .weekday-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
  .weekday-checkbox {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
    background: #e5e7eb;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
  }
  .weekday-checkbox:has(input:checked) {
    background: #4f46e5;
    color: white;
  }
  .stage-lists {
    display: flex;
    gap: 2rem;
  }
  .stage-lists ul {
    margin: 0.5rem 0 0 1.5rem;
    padding: 0;
    font-size: 0.875rem;
  }
</style>
