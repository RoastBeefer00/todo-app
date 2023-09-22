<script>
    import SunIcon from "virtual:icons/ph/sun";
    import MoonIcon from "virtual:icons/ph/moon";
    let theme = "mocha"

    function toggle_theme() {
        if (theme == "mocha") {
            theme = "latte";
        }
        else {
            theme = "mocha";
        }
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
                <div>
                    <input type="text" placeholder="What needs to be done?" class="w-full h-full p-4 bg-surface0 focus:border-4 focus:ring-blue focus:border-blue outline-none text-text transition-all">
                </div>
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
