/**
 * Validator form page for E2E testing.
 */

import { formText, requireElement, requireForm } from './dom.util.ts';
import { playgroundResults, type ValidatorFormResults } from './playground-globals.ts';
import {
    validateEvent,
    validateProduct,
    validateUserRegistration,
    type ValidationResult
} from './validator-form.ts';

const formResults: ValidatorFormResults = {};
playgroundResults().validatorForm = formResults;

function renderErrors(errors: Array<{ field: string; message: string }>): HTMLElement {
    const list = document.createElement('ul');
    list.className = 'error-list';
    for (const error of errors) {
        const item = document.createElement('li');
        const field = document.createElement('strong');
        field.textContent = `${error.field}:`;
        item.append(field, ` ${error.message}`);
        list.append(item);
    }
    return list;
}

function renderSuccess(data: unknown): HTMLElement {
    const output = document.createElement('pre');
    output.className = 'success-output';
    output.textContent = JSON.stringify(data, null, 2);
    return output;
}

/**
 * Validates a form on submit, publishes the result for the specs and renders
 * it into the form's result container.
 */
function wireForm<T>(
    formId: string,
    resultId: string,
    validate: (form: FormData) => ValidationResult<T>,
    publish: (result: ValidationResult<T>) => void
) {
    const form = requireForm(formId);
    form.addEventListener('submit', (event) => {
        event.preventDefault();
        const result = validate(new FormData(form));
        publish(result);

        const resultElement = requireElement(resultId);
        if (result.success) {
            resultElement.className = 'result-container success';
            resultElement.replaceChildren(renderSuccess(result.value));
            resultElement.setAttribute('data-validation-success', 'true');
        } else {
            resultElement.className = 'result-container error';
            resultElement.replaceChildren(renderErrors(result.errors));
            resultElement.setAttribute('data-validation-success', 'false');
        }
    });
}

export function initValidatorFormPage() {
    const app = document.getElementById('app');
    if (!app) return;

    app.innerHTML = `
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
      .form-group input, .form-group textarea {
        width: 100%;
        padding: 0.5rem;
        border: 1px solid #ced4da;
        border-radius: 4px;
        font-size: 1rem;
        box-sizing: border-box;
      }
      .form-group input:focus, .form-group textarea:focus {
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

    <h1>Validator Form Testing</h1>
    <p>Test deserializer validators with real form data.</p>

    <!-- User Registration Form -->
    <div class="form-section">
      <h3>User Registration</h3>
      <form id="user-registration-form" data-testid="user-registration-form">
        <div class="form-row">
          <div class="form-group">
            <label for="user-email">Email</label>
            <input type="text" id="user-email" name="email" data-testid="user-email" placeholder="user@example.com">
            <div class="hint">Must be a valid email address</div>
          </div>
          <div class="form-group">
            <label for="user-username">Username</label>
            <input type="text" id="user-username" name="username" data-testid="user-username" placeholder="johndoe">
            <div class="hint">3-20 chars, lowercase, start with letter, only letters/numbers/underscore</div>
          </div>
        </div>
        <div class="form-row">
          <div class="form-group">
            <label for="user-password">Password</label>
            <input type="text" id="user-password" name="password" data-testid="user-password" placeholder="********">
            <div class="hint">8-50 characters</div>
          </div>
          <div class="form-group">
            <label for="user-age">Age</label>
            <input type="number" id="user-age" name="age" data-testid="user-age" placeholder="25">
            <div class="hint">Integer between 18 and 120</div>
          </div>
        </div>
        <div class="form-group">
          <label for="user-website">Website</label>
          <input type="text" id="user-website" name="website" data-testid="user-website" placeholder="https://example.com">
          <div class="hint">Must be a valid URL</div>
        </div>
        <button type="submit" data-testid="submit-user-registration">Submit Registration</button>
      </form>
      <div id="user-registration-result" class="result-container" data-testid="user-registration-result"></div>
    </div>

    <!-- Product Form -->
    <div class="form-section">
      <h3>Product Entry</h3>
      <form id="product-form" data-testid="product-form">
        <div class="form-row">
          <div class="form-group">
            <label for="product-name">Product Name</label>
            <input type="text" id="product-name" name="name" data-testid="product-name" placeholder="Widget">
            <div class="hint">1-100 characters, non-empty</div>
          </div>
          <div class="form-group">
            <label for="product-sku">SKU (UUID)</label>
            <input type="text" id="product-sku" name="sku" data-testid="product-sku" placeholder="123e4567-e89b-12d3-a456-426614174000">
            <div class="hint">Must be a valid UUID</div>
          </div>
        </div>
        <div class="form-row">
          <div class="form-group">
            <label for="product-price">Price</label>
            <input type="number" id="product-price" name="price" data-testid="product-price" step="0.01" placeholder="29.99">
            <div class="hint">Positive number, less than 1,000,000</div>
          </div>
          <div class="form-group">
            <label for="product-quantity">Quantity</label>
            <input type="number" id="product-quantity" name="quantity" data-testid="product-quantity" placeholder="100">
            <div class="hint">Non-negative integer</div>
          </div>
        </div>
        <div class="form-group">
          <label for="product-tags">Tags (comma-separated)</label>
          <input type="text" id="product-tags" name="tags" data-testid="product-tags" placeholder="electronics, gadget, new">
          <div class="hint">1-5 tags</div>
        </div>
        <button type="submit" data-testid="submit-product">Submit Product</button>
      </form>
      <div id="product-result" class="result-container" data-testid="product-result"></div>
    </div>

    <!-- Event Form -->
    <div class="form-section">
      <h3>Event Creation</h3>
      <form id="event-form" data-testid="event-form">
        <div class="form-group">
          <label for="event-title">Event Title</label>
          <input type="text" id="event-title" name="title" data-testid="event-title" placeholder="Annual Conference">
          <div class="hint">Non-empty, no leading/trailing whitespace</div>
        </div>
        <div class="form-row">
          <div class="form-group">
            <label for="event-start">Start Date</label>
            <input type="text" id="event-start" name="startDate" data-testid="event-start" placeholder="2025-06-15">
            <div class="hint">Valid date after 2020-01-01</div>
          </div>
          <div class="form-group">
            <label for="event-end">End Date</label>
            <input type="text" id="event-end" name="endDate" data-testid="event-end" placeholder="2025-06-17">
            <div class="hint">Valid date</div>
          </div>
        </div>
        <div class="form-group">
          <label for="event-attendees">Max Attendees</label>
          <input type="number" id="event-attendees" name="maxAttendees" data-testid="event-attendees" placeholder="200">
          <div class="hint">Integer between 1 and 1000</div>
        </div>
        <button type="submit" data-testid="submit-event">Submit Event</button>
      </form>
      <div id="event-result" class="result-container" data-testid="event-result"></div>
    </div>
  `;

    wireForm(
        'user-registration-form',
        'user-registration-result',
        (form) =>
            validateUserRegistration({
                email: formText(form, 'email'),
                password: formText(form, 'password'),
                username: formText(form, 'username'),
                age: parseInt(formText(form, 'age'), 10) || 0,
                website: formText(form, 'website')
            }),
        (result) => {
            formResults.userRegistration = result;
        }
    );

    wireForm(
        'product-form',
        'product-result',
        (form) =>
            validateProduct({
                name: formText(form, 'name'),
                sku: formText(form, 'sku'),
                price: parseFloat(formText(form, 'price')) || 0,
                quantity: parseInt(formText(form, 'quantity'), 10) || 0,
                tags: formText(form, 'tags')
                    .split(',')
                    .map((tag) => tag.trim())
                    .filter(Boolean)
            }),
        (result) => {
            formResults.product = result;
        }
    );

    wireForm(
        'event-form',
        'event-result',
        // The validator checks real dates, so the text fields become `Date`s.
        (form) =>
            validateEvent({
                title: formText(form, 'title'),
                startDate: new Date(formText(form, 'startDate')),
                endDate: new Date(formText(form, 'endDate')),
                maxAttendees: parseInt(formText(form, 'maxAttendees'), 10) || 0
            }),
        (result) => {
            formResults.event = result;
        }
    );
}
