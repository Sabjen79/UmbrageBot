<script lang="ts">
    import ErrorPopup from "../components/error_popup.svelte";
    import TitleBar from "../components/title_bar.svelte";
    import icon from "../assets/logobare.svg?raw";

    let { children } = $props();

    document.documentElement.classList.add("dark");
</script>

<div class="absolute inset-0 stroke-gray-500/5 scale-200">
    {@html icon}
</div>


<main class="w-[100vw] h-[100vh] overflow-hidden flex items-center justify-center">
    {@render children()}
</main>

<ErrorPopup />
<TitleBar />