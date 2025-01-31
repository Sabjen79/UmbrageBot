<script lang="ts">
    import TitleBar from "../components/title_bar.svelte";

    let { children } = $props();
</script>

<TitleBar />
<main class="w-[100vw] h-[100vh] overflow-hidden">
    {@render children()}
</main>
