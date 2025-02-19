<script lang="ts">
    import ContextMenu from "./context_menu/context_menu.svelte";
    import ContextMenuItem from "./context_menu/context_menu_item.svelte";
    import { open as openExternal } from "@tauri-apps/plugin-shell";

    let {
        text = "Text Button",
        link = null,
        onclick = () => {
            openExternal(link);
        }
    } = $props();

    let hover = $state(false);
    let pressed = $state(false);

    let contextMenu: ContextMenu;
</script>

<button 
    class={`
        relative inline-flex p-0
        bg-transparent border-0
        font-semibold text-primary-400
        hover:cursor-pointer
    `}
    onmouseenter={() => { hover = true; }}
    onmouseleave={() => { hover = false; pressed = false; }}
    onmousedown={() => { pressed = true; }}
    onmouseup={() => { pressed = false; }}
    oncontextmenu={(e) => {contextMenu.open(e)}}
    onclick={onclick()}
>
    <div id="content">{text}</div>
    <div 
        class={`
            absolute inset-0 
            text-gray-900 
            select-none  ease-in-out
            ${pressed ? "duration-50 bg-primary-500" : "duration-100 bg-primary-400"}

            ${pressed ? "[clip-path:polygon(0%_90%,100%_90%,100%_10%,0%_10%)]"
            : hover ? "[clip-path:polygon(0%_90%,100%_90%,100%_85%,0%_85%)]"
            : "[clip-path:polygon(50%_90%,50%_90%,50%_85%,50%_85%)]" 
            }
        `}
    >{text}</div>
</button>

<ContextMenu bind:this={contextMenu}>
    <ContextMenuItem icon="open_in_new" text="Open Link" {onclick}/>
    {#if link != null}
        <ContextMenuItem icon="content_copy" text="Copy Link" onclick={async () => {
            await navigator.clipboard.writeText(link);
        }}/>
    {/if}
    
</ContextMenu>