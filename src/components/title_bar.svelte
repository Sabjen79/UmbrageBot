<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";
</script>

<div data-tauri-drag-region class="titlebar">
    <button class="titlebar-button" id="titlebar-minimize" on:click={getCurrentWindow().minimize}>
        <img
            src="https://api.iconify.design/mdi:window-minimize.svg"
            alt="minimize"
        />
    </button>
    <button class="titlebar-button" id="titlebar-maximize" on:click={getCurrentWindow().toggleMaximize}>
        <img
            src="https://api.iconify.design/mdi:window-maximize.svg"
            alt="maximize"
        />
    </button>
    <button class="titlebar-button" id="titlebar-close" on:click={getCurrentWindow().close}>
        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
    </button>
</div>

<style>
    .titlebar {
        height: 30px;
        background: #329ea3;
        user-select: none;
        display: flex;
        justify-content: flex-end;
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
    }

    .titlebar-button {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: 30px;
        height: 30px;
        user-select: none;
        -webkit-user-select: none;
    }

    .titlebar-button:hover {
        background: #5bbec3;
    }
</style>
