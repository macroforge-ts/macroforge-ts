<script lang="ts">
  const routes = [
    { path: "/gigaform/basic", name: "Basic Fields", description: "PhoneNumber, Gradient, Coordinates - simple field types and validators" },
    { path: "/gigaform/nested", name: "Nested Objects", description: "User with Settings, Metadata, AppPermissions - deep object nesting" },
    { path: "/gigaform/arrays", name: "Array Operations", description: "Account with phones[], tags[], customFields - array methods (push, remove, swap, at)" },
    { path: "/gigaform/unions", name: "Union Types", description: "Lead with AccountName (CompanyName | PersonName) - polymorphic fields" },
    { path: "/gigaform/nullable", name: "Nullable Fields", description: "User.metadata, nullable primitives - null handling" },
    { path: "/gigaform/records", name: "Record/Map Types", description: "TaxRate.taxComponents - dynamic key-value structures" },
    { path: "/gigaform/deeply-nested", name: "Deep Nesting", description: "Order -> Site -> Coordinates - multi-level navigation" },
    { path: "/gigaform/enums", name: "Enum Fields", description: "UserRole, Status, Sector, Priority - enum validation" },
    { path: "/gigaform/complex", name: "Complex Integration", description: "Employee, Order - full integration with all features" },
  ];
</script>

<svelte:head>
  <title>Gigaform E2E Tests - Svelte Playground</title>
</svelte:head>

<h1>Gigaform E2E Test Suite</h1>
<p data-testid="gigaform-description">
  Comprehensive tests for the Gigaform macro via demo types. Each route tests specific features of the generated form system.
</p>

<nav class="route-grid" data-testid="gigaform-nav">
  {#each routes as route}
    <a href={route.path} class="route-card" data-testid="route-{route.path.split('/').pop()}">
      <h3>{route.name}</h3>
      <p>{route.description}</p>
    </a>
  {/each}
</nav>

<style>
  h1 {
    margin-bottom: 0.5rem;
  }
  p {
    color: #6b7280;
    margin-bottom: 2rem;
  }
  .route-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1rem;
  }
  .route-card {
    display: block;
    padding: 1.5rem;
    background: #f8f9fa;
    border: 1px solid #dee2e6;
    border-radius: 8px;
    text-decoration: none;
    color: inherit;
    transition: border-color 0.2s, box-shadow 0.2s;
  }
  .route-card:hover {
    border-color: #4f46e5;
    box-shadow: 0 4px 12px rgba(79, 70, 229, 0.15);
  }
  .route-card h3 {
    margin: 0 0 0.5rem 0;
    color: #4f46e5;
    font-size: 1.1rem;
  }
  .route-card p {
    margin: 0;
    font-size: 0.875rem;
    color: #6b7280;
  }
</style>
