<script lang="ts">
    let {
        label = "Text Button",
        color = "var(--primary)",
        onclick
    } = $props();

    let hover = $state(false);
    let pressed = $state(false);
</script>

<button 
    id="container" 
    style="color: {color}"
    onmouseenter={() => { hover = true; }}
    onmouseleave={() => { hover = false; pressed = false; }}
    onmousedown={() => { pressed = true; }}
    onmouseup={() => { pressed = false; }}
    onclick={onclick}
>
    <div id="content">{label}</div>
    <div 
        id="border" 
        style="background-color: {color}"
        class={{hover, pressed}}
    >{label}</div>
</button>

<style>
    #container {
        position: relative;
        display: inline-flex;

        padding: 0;
        background-color: transparent;
        border: 0;
        font-size: inherit;
    }

    #container:hover {
        cursor: pointer;
    }

    #border {
        position: absolute;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;

        color: var(--background);
        user-select: none;

        clip-path: polygon(50% 90%, 50% 90%, 50% 85%, 50% 85%);

        transition: 0.1s ease-in-out;
    }

    #border.hover:not(.pressed) {
        clip-path: polygon(0% 90%, 100% 90%, 100% 85%, 0% 85%);
    }

    #border.pressed {
        clip-path: polygon(0% 90%, 100% 90%, 100% 10%, 0% 10%);
        transition: 0.05s ease-in-out;
    }
</style>