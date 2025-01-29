<script lang="ts">
    let { 
        children,
        width = "auto",
        height = "auto",
        title = ""
    } = $props();

    export function open() {
        opened = true;

        new Promise((resolve) => {
            setTimeout(resolve, 1);
        }).then(() => {
            visible = true;
        });
    }

    export function close() {
        if(blocked) return;
        visible = false;

        new Promise((resolve) => {
            setTimeout(resolve, 200);
        }).then(() => {
            opened = false;
        });
    }

    export function block() {
        blocked = true;
    }

    export function unblock() {
        blocked = false;
    }

    let opened = $state(false);
    let visible = $state(false);
    let blocked = $state(false);
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
{#if opened}
    <div id="container">
        <div id="background" class={{visible}} onclick={close}></div>
        <div id="dialog" 
            class={{visible}}
            style="width: {width}; height: {height}"
            >
            <button class="titlebar-button" id="titlebar-close" onclick={close}>
                <span class="material-symbols-outlined"  style="font-size: 1.7em;">
                    close
                </span>
            </button>
            <div id="title">
                <p>{title}</p>
            </div>
            <div id="content">
                {@render children()}
            </div>
        </div>
    </div>
{/if}


<style>
    :root {
        --titlecolor: color-mix(in srgb, var(--secondary) 50%, var(--background));
    }

    #container {
        position: fixed;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;

        width: 100vw;
        height: 100vh;

        display: flex;
        align-items: center;
        justify-content: center;
    }

    #background {
        position: absolute;
        width: 100%;
        height: 100%;
        background-color: #0000008d;

        transition: 0.25s ease-out;
        opacity: 0;
    }

    #background.visible {
        opacity: 1;
    }

    #dialog {
        display: flex;
        flex-direction: column;
        justify-content: center;
        position: absolute;

        margin: 0;
        padding: 0;

        background-color: var(--background);
        border: 1px solid var(--titlecolor);
        border-radius: 5px;

        transition: 0.25s ease-out;
        opacity: 0;
        transform: translateY(-10px);
    }

    #dialog.visible {
        opacity: 1;
        transform: translateY(0);
    }

    #dialog > button {
        position: absolute;
        top: 10px;
        right: 10px;

        background-color: transparent;
        border: none;

        transition: 0.2s ease-out;
        opacity: 0.6;
    }

    #dialog > button:hover {
        cursor: pointer;
        opacity: 1;
    }

    #title {
        display: flex;
        align-items: center;

        width: 100%;
        height: 30px;

        margin: 10px 15px;

        background-color: transparent;
        border-radius: 2px 2px 0 0;
    }

    #title > p {
        font-size: 1.5em;
        font-weight: 500;
    }

    #content {
        display: flex;
        justify-content: center;

        padding: 10px 15px;
        
    }
</style>