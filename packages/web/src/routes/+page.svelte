<script>
    import { browser } from "$app/environment";
    import { getTodos, addTodo, deleteTodo, init, toggleComplete, markAllComplete, markAllActive, checkAllComplete } from "$lib/pkg/vite_wasm_functions";
    import { onMount } from "svelte";
    import { todos } from "./stores";

    // Icons
    import AddIcon from "virtual:icons/ph/plus-bold";
    import MoonIcon from "virtual:icons/ph/moon";
    import SunIcon from "virtual:icons/ph/sun";
    import TrashIcon from "virtual:icons/ph/trash";

    let theme = "mocha"

    let text;
    let all_tasks_complete = browser && checkAllComplete();

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
        $todos = storage.todos;
        // return $todos;
    }

    async function add(task) {
        browser && addTodo(task);
        text = ""
        await get();
    }

    async function del(id) {
        browser && deleteTodo(id);
        await get();
    }

    async function allComplete() {
        browser && markAllComplete();
        all_tasks_complete = browser && checkAllComplete();
        await get();
    }

    async function allActive() {
        browser && markAllActive();
        all_tasks_complete = browser && checkAllComplete();
        await get();
    }

    async function toggle(id) {
        browser && toggleComplete(id);
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
                    <div class="flex-none">
                        <div class="mt-2 ml-2">
                            <label class="inline-flex items-center">
                                <input type="checkbox" class="m-3 w-8 h-8 rounded-full border-0 text-green focus:ring-0" bind:checked={all_tasks_complete} on:click={all_tasks_complete ? allActive : allComplete} />
                            </label>
                        </div>
                    </div>
                    <form on:submit={add(text)} class="flex flex-auto w-full h-full">
                        <input bind:value={text} type="text" placeholder="What needs to be done?" class="m-2 flex-auto w-full h-full p-4 bg-surface0 border-none focus:border-4 focus:ring-blue focus:border-blue outline-none text-text transition-all">
                        <button class="flex-none rounded bg-green p-2 mt-4 mb-4 ml-4 mr-5"> 
                            <AddIcon class="text-base text-xl" /> 
                        </button>
                    </form>
                </div>
                {#each $todos || [] as todo}
                    <div class="flex w-full h-full p-4 bg-surface0 border-x-4 border-y-2 border-surface1">
                        <div class="block">
                            <div class="mt-2">
                                <label class="inline-flex items-center">
                                    <input type="checkbox" class="w-8 h-8 rounded-full border-0 text-green focus:ring-0" bind:checked={todo.completed} on:change={toggle(todo.id)} />
                                </label>
                            </div>
                        </div>
                        <p class="mt-2 flex-auto text-text text-xl">
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
