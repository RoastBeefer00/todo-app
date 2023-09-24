<script>
    import { onMount } from "svelte";
    import { getTodos, addTodo, deleteTodo, init } from "$lib/pkg/vite_wasm_functions";
    import { todos } from "./stores";
    import { browser } from "$app/environment";
    import SunIcon from "virtual:icons/ph/sun";
    import MoonIcon from "virtual:icons/ph/moon";
    import AddIcon from "virtual:icons/ph/plus-bold";
    import TrashIcon from "virtual:icons/ph/trash";
    import CheckIcon from "virtual:icons/ph/check-circle";
    let theme = "mocha"

    const STORAGE_KEY = "todos";
    let text;

    browser && init();
    onMount(async () => {
        await get();
    });

    function toggle_theme() {
        if (theme == "mocha") {
            theme = "latte";
        }
        else {
            theme = "mocha";
        }
    }

    async function get() {
        let storage = browser && await getTodos();
        console.log("length: " + storage.todos);
        $todos = storage.todos;
        return $todos;
    }

    async function add(task) {
        browser && addTodo(task);
        await get();
        // $todos = storage.todos;
    }

    async function del(id) {
        browser && deleteTodo(id);
        await get();
    }
    
</script>

<html class="{theme}">
    <body class="bg-base h-screen w-screen text-center transition-colors">
        <div class="fixed flex p-4 right-0 top-0">
            <MoonIcon class={theme === "mocha" ? "text-lavender text-xl mr-1" : "text-transparent text-xl mr-1"} />
            <label for="toggle" class="relative flex cursor-pointer">
                <input type="checkbox" id="toggle" class="sr-only peer" />
                <div on:click={toggle_theme}  class="h-6 bg-lavender border-2 border-lavender rounded-full w-11 after:absolute after:top-0.5 after:left-0.5 after:bg-white after:border after:border-lavender after:h-5 after:w-5 after:shadow-sm after:rounded-full peer-checked:after:translate-x-full peer-checked:after:border-white peer-checked:bg-yellow peer-checked:border-yellow after:transition-all after:duration-300"></div>
            </label>
            <SunIcon class={theme === "latte" ? "text-yellow text-xl mr-1" : "text-transparent text-xl mr-1"} />
        </div>
        <div class="pt-10 item-center justify-center">
            <h1 class="text-blue uppercase text-6xl font-bold leading-12 my-2 mx-auto">TODO</h1>
            <div class="max-w-[85%] sm:max-w-2xl m-2 mx-auto"> 
                <div class="flex bg-surface0">
                    <input bind:value={text} type="text" placeholder="What needs to be done?" class="flex-auto w-full h-full p-4 bg-surface0 focus:border-4 focus:ring-blue focus:border-blue outline-none text-text transition-all">
                    <button on:click={add(text)} class="flex-none rounded bg-green p-2 m-4"> 
                        <AddIcon class="text-base text-xl" /> 
                    </button>
                </div>
                {#each $todos || [] as todo}
                    <div class="flex w-full h-full p-4 bg-surface0 border-x-4 border-y-2 border-surface1">
                        <button class="flex-none">
                            <CheckIcon class="text-4xl text-text hover:bg-green" />
                        </button>
                        <p class="flex-auto text-text text-xl">
                            {todo.task} 
                        </p>
                        <button on:click={del(todo.id)} class="flex-none rounded bg-red p-2 text-base"> 
                            <TrashIcon class="text-xl" /> 
                        </button>
                        <br/>
                    </div>
                {/each}
            </div>
        </div>
    </body>
</html>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }
</style>
