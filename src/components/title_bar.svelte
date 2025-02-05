<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";

    let buttonStyle = `
        inline-flex justify-center items-center 
        w-0 h-0 my-auto mx-3
        border-0 bg-transparent text-gray-300 select-none
        ease-linear duration-200
        hover:cursor-pointer hover:text-gray-50
    `;
</script>

<div 
    data-tauri-drag-region 
    class="titlebar h-8 fixed flex justify-end top-0 left-0 right-1.5 bg-transparent z-100"
>
    <button
        class="titlebar-button {buttonStyle}"
        id="titlebar-minimize"
        onclick={getCurrentWindow().minimize}
    >
        <span class="font-icons text-xl"> horizontal_rule </span>
    </button>
    <button
        class="titlebar-button {buttonStyle}"
        id="titlebar-maximize"
        onclick={getCurrentWindow().toggleMaximize}
    >
        <span class="font-icons text-xl"> check_box_outline_blank </span>
    </button>
    <button
        class="titlebar-button {buttonStyle}"
        id="titlebar-close"
        onclick={getCurrentWindow().close}
    >
        <span class="font-icons text-2xl"> close </span>
    </button>
</div>
