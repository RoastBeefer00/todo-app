<script>
  import logo from "../assets/svelte.png";
  import Alert from "../lib/Alert.svelte";
  import Counter from "../lib/Counter.svelte";
</script>

<main class="text-center p-1 my-0 mx-auto">
  <img class="mx-auto h-64 w-64" src={logo} alt="Svelte Logo" />
  <h1 class="max-w-none min-w-[480px] max-w-[14rem] text-[#ff3e00] uppercase text-6xl font-thin leading-12 my-2 mx-auto">Hello Vite+Rust!</h1>

  <div>
    <Counter />
  </div>
  <div class="mt-3">
    <Alert />
  </div>

  <p class="max-w-md my-1 mx-auto leading-5">
    Check out <a href="https://github.com/sveltejs/kit#readme">SvelteKit</a>, powered by Vite!
  </p>
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }
</style>
