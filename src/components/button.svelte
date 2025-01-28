<script lang="ts">
    import LoadingSpinner from "./loading_spinner.svelte";

    let {
        text,
        icon = "",
        fontSize = "1em",
        onclick = async () => {
            await new Promise((resolve) => setTimeout(resolve, 3000));
        },
    } = $props();

    let hover = $state(false);
    let pressed = $state(false);
    let waiting = $state(false);

    let classList = $derived({hover, pressed, waiting});
</script>

<button
    id="container"
    class={classList}
    style="font-size: {fontSize};"
    onmouseenter={() => { hover = true; }}
    onmouseleave={() => { hover = false; pressed = false; }}
    onmousedown={() => { pressed = true; }}
    onmouseup={() => { pressed = false; }}
    onclick={async () => {
        if (waiting) return;

        if (onclick instanceof (async () => {}).constructor) {
            waiting = true;
            await onclick();
            waiting = false;
        } else {
            onclick();
        }
    }}
>
    <div id="border1" class={classList}></div>
    <div id="border2" class={classList}></div>
    <div id="spinner" class={classList}>
        <LoadingSpinner color="var(--primary)" />
    </div>
    <div id="content" class={classList}>
        {#if icon != ""}
            <span class="material-symbols-outlined" style="font-size: calc({fontSize}*1.5);">
                {icon}
            </span>
        {/if}
        {text}
    </div>
</button>

<style>
    .material-symbols-outlined {
        font-variation-settings:
        'FILL' 1,
        'wght' 400,
        'GRAD' 0,
        'opsz' 24
    }

    #container {
        position: relative;
        display: inline-flex;
        align-items: center;
        justify-content: center;

        overflow: visible;
        margin: 5px;
        padding: 0;
        border: none;
        background-color: transparent;
        float: left;

        transition: 0.2s;
    }

    #container.hover:not(.waiting) {
        cursor: pointer;
    }

    #container.pressed,
    #container.waiting {
        transform: translateY(2px);
    }

    #border1 {
        position: absolute;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;

        border: solid 1px var(--gray);
        border-radius: 4px;
        opacity: 0;

        transition: 0.15s;
    }

    #border1.hover {
        top: 2px;
        bottom: 2px;
        left: 2px;
        right: 2px;

        opacity: 1;

        border: solid 1px var(--gray);
    }

    #border1.pressed {
        opacity: 0;
    }

    #border1.waiting {
        opacity: 0;
    }

    #border2 {
        position: absolute;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;

        background-color: color-mix(in hsl, var(--foreground) 2%, transparent);
        border: solid 1px var(--gray);
        border-radius: 4px;
        box-shadow: 0px 5px 10px black;
        opacity: 1;

        transition: 0.15s;
    }

    #border2.hover {
        top: -2px;
        bottom: -2px;
        left: -2px;
        right: -2px;

        border: solid 1px var(--gray);
    }

    #border2.pressed {
        top: 2px;
        bottom: 2px;
        left: 2px;
        right: 2px;

        border: solid 2px var(--primary);
        box-shadow: 0px 0px 10px black;
        opacity: 1;
    }

    #border2.waiting {
        top: 2px;
        bottom: 2px;
        left: 34%;
        right: 34%;

        transition: 0.3s ease-out;
        border: solid 2px var(--primary);
        opacity: 0;
    }

    #content {
        display: flex;
        align-items: center;
        justify-content: center;

        float: left;
        padding: 2px 8px;
        margin: 5px 4px 4px 4px;

        background-color: transparent;
        opacity: 1;
        transition: 0.1s;
    }

    #content > span {
        margin: 0 4px 0 -6px;
    }

    #content.waiting {
        opacity: 0;
    }

    #spinner {
        position: absolute;
        top: 0;
        bottom: 4px;
        left: 0;
        right: 0;

        opacity: 0;

        transition: 0.3s ease-in-out;

        scale: 0.5;
    }

    #spinner.waiting {
        opacity: 1;

        scale: 1;
    }
</style>
