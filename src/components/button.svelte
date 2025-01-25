<script lang="ts">
    import LoadingSpinner from "./loading_spinner.svelte";

    let { 
        label = 'Button Test',
        onclick = async () => {
            await new Promise(resolve => setTimeout(resolve, 3000));
        }
    } = $props();

    let hover = $state(false);
    let pressed = $state(false);
    let waiting = $state(false);

    let classList = $derived(
        (hover ? "hover " : " ") + 
        (pressed ? "pressed " : " ") + 
        (waiting ? "waiting " : " ")
    );
</script>

<button id="container"
    class={classList}
    onmouseenter={() => {
        hover = true;
    }}
    onmouseleave={() => {
        hover = false;
        pressed = false;
    }}
    onmousedown={() => {
        pressed = true;
    }}
    onmouseup={() => {
        pressed = false;
    }}
    onclick={async () => {
        if(waiting) return;
        

        if(onclick instanceof (async () => {}).constructor) {
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
    <div id="content" class={classList}>{label}</div>
    <div id="spinner" class={classList}><LoadingSpinner color="var(--primary)"/></div>
    
    
</button>

<style>
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
    }

    #container.hover:not(.waiting) {
        cursor: pointer;
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
        box-shadow: 0px 7px 10px black ;
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
        float: left;
        padding: 2px 8px;
        margin: 5px 4px 4px 4px;
        
        font-size: 1.2em;

        background-color: transparent;
        opacity: 1;
        transition: 0.1s;
    }

    #content.waiting {
        opacity: 0;
    }

    #spinner {
        position: absolute;
        top: 0;
        bottom: 0;
        left: 34%;
        right: 34%;

        opacity: 0;

        transition: 0.3s ease-in-out;

        scale: 1.5;
    }

    #spinner.waiting {
        opacity: 1;

        scale: 1;
    }
</style>