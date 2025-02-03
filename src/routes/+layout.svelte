<script lang="ts">
    import ErrorPopup from "../components/error_popup.svelte";
    import TitleBar from "../components/title_bar.svelte";

    let { children } = $props();

    document.documentElement.classList.add("dark");
</script>

<TitleBar />
<main class="w-[100vw] h-[100vh] overflow-hidden">
    {@render children()}
</main>
<ErrorPopup />
