<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";

    let buttonStyle = `
        inline-flex justify-center items-center 
        w-0 h-0 my-auto mx-3
        border-0 bg-transparent text-(--gray-300) select-none
        ease-linear duration-200
        hover:cursor-pointer hover:text-(--gray-50)
    `;
</script>

<link
    rel="stylesheet"
    href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,300,0,0"
/>
<link
    rel="stylesheet"
    href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200"
/>

<div 
    data-tauri-drag-region 
    class="titlebar h-(--titlebar-height) fixed flex justify-end top-0 left-0 right-1 bg-transparent select-none z-100"
>
    <button
        class="titlebar-button {buttonStyle}"
        id="titlebar-minimize"
        onclick={getCurrentWindow().minimize}
    >
        <span class="material-symbols-outlined !text-base"> horizontal_rule </span>
    </button>
    <button
        class="titlebar-button {buttonStyle}"
        id="titlebar-maximize"
        onclick={getCurrentWindow().toggleMaximize}
    >
        <span class="material-symbols-outlined !text-lg"> check_box_outline_blank </span>
    </button>
    <button
        class="titlebar-button {buttonStyle}"
        id="titlebar-close"
        onclick={getCurrentWindow().close}
    >
        <span class="material-symbols-outlined !text-xl"> close </span>
    </button>
</div>

<style>
    .material-symbols-outlined {
        font-variation-settings:
            "FILL" 0,
            "wght" 300,
            "GRAD" 0,
            "opsz" 24;
    }
</style>
