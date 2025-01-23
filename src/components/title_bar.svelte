<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";
</script>

<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,300,0,0" />

<div data-tauri-drag-region class="titlebar">
    <button class="titlebar-button" id="titlebar-minimize" onclick={getCurrentWindow().minimize}>
        <span class="material-symbols-outlined">
            horizontal_rule
        </span>
    </button>
    <button class="titlebar-button" id="titlebar-maximize" onclick={getCurrentWindow().toggleMaximize}>
        <span class="material-symbols-outlined">
            check_box_outline_blank
        </span>
    </button>
    <button class="titlebar-button" id="titlebar-close" onclick={getCurrentWindow().close}>
        <span class="material-symbols-outlined"  style="font-size: 1.6em;">
            close
        </span>
    </button>
</div>

<style>
    .titlebar {
        height: var(--titlebar-height);
        background: transparent;
        user-select: none;
        display: flex;
        justify-content: flex-end;
        position: fixed;
        top: 0;
        left: 0;
        right: 5px;
    }

    .titlebar-button {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        border: none;
        width: 0px;
        height: 0px;
        margin: auto 8px;
        
        background-color: transparent;
        user-select: none;
        -webkit-user-select: none;
        transition: 0.3s;
        color: var(--gray);
    }

    span {
        font-size: 1.3em;
        vertical-align: middle;
    }

    .titlebar-button:hover {
        cursor: pointer;
        color: var(--foreground);
    }
</style>
