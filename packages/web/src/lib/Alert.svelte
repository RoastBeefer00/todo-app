<script>
  import { helloWorld } from '$lib/pkg/vite_wasm_functions'
</script>

<button class="py-1 px-2 rounded-3xl bg-[#ff3e001a] active:bg-[#ff3e0033] border-solid focus:border-[#ff3e00] text-[#ff3e00] border-2 border-[#ff3e0000] outline-none w-[200px] cursor-pointer" on:click={() => helloWorld()}>
  Alert from Rust!
</button>

