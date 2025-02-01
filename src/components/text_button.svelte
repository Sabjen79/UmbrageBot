<script lang="ts">
    let {
        text = "Text Button",
        onclick
    } = $props();

    let hover = $state(false);
    let pressed = $state(false);
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
    onclick={onclick}
>
    <div id="content">{text}</div>
    <div 
        id="border"
        class={{hover, pressed}}
    >{text}</div>
</button>

<style>
    /* Tailwind doesn't have clip-path :( */
    #border {
        position: absolute;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;

        color: var(--color-gray-900);
        background-color: var(--color-primary-400);
        user-select: none;

        clip-path: polygon(50% 90%, 50% 90%, 50% 85%, 50% 85%);

        transition: 0.1s ease-in-out;
    }

    #border.hover:not(.pressed) {
        clip-path: polygon(0% 90%, 100% 90%, 100% 85%, 0% 85%);
    }

    #border.pressed {
        clip-path: polygon(0% 90%, 100% 90%, 100% 10%, 0% 10%);
        background-color: var(--color-primary-400);
        transition: 0.05s ease-in-out;
    }
</style>