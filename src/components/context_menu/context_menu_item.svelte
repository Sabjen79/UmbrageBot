<script lang="ts">
    import ContextMenuCustomItem from "./context_menu_custom_item.svelte";

    let {
        text = "",
        icon = "",
        isRed = false,
        onclick = () => {}
    } = $props();
</script>

<ContextMenuCustomItem {onclick}>
    <div class={`font-icons text-xl mx-1 ${isRed ? "text-red-500" : ""}`}>
        {icon}
    </div>
    <div class={`text-base ${isRed ? "text-red-500" : ""}`}>
        {text}
    </div>
</ContextMenuCustomItem>