<script>
  let count = 0;
</script>

<svelte:head>
  <title>Sample App - Monorepo</title>
  <meta name="description" content="Sample SvelteKit application in monorepo" />
</svelte:head>

<div class="container mx-auto px-4 py-8">
  <h1 class="text-4xl font-bold mb-8 text-center">Sample SvelteKit Application</h1>
  
  <div class="max-w-2xl mx-auto">
    <div class="bg-white rounded-lg shadow-md p-6 mb-6">
      <h2 class="text-2xl font-semibold mb-4">Welcome to the Monorepo</h2>
      <p class="mb-4">
        This is a sample SvelteKit frontend application demonstrating the monorepo structure.
      </p>
      <button
        on:click={() => count++}
        class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
      >
        Click me! ({count})
      </button>
    </div>
  </div>
</div>

