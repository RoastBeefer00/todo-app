<script> 
  import { addArray } from "$lib/pkg/vite_wasm_functions";

  let count = 0;
  let arr = [];
  let rustSum = 0
  const increment = () => {
    count += 1;
    arr.push(count);
    rustSum = addArray(arr)
  };
</script>

<button class="py-2 px-4 w-[300px] focus:border-[#ff3e00] active:bg-[#ff3e0033] text-[#ff3e00] outline-none cursor-pointer bg-[#ff3e001a] rounded-3xl border-2 border-solid border-[#ff3e0000]" on:click={increment}>
  Sum of 0 to {count}: {rustSum}
</button>

