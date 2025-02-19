<script lang="ts">
    let {
        children
    } = $props();

    let visible = $state(false);

    let posx = $state(0);
    let posy = $state(0);

    let openLeft = $state(false);
    let openTop = $state(true);
    
    // svelte-ignore non_reactive_update
    let container : HTMLElement;

    // svelte-ignore non_reactive_update
    let menu : HTMLElement;

    export function open(e: MouseEvent) {
        let pos = container.getBoundingClientRect();
        
        posx = e.clientX - pos.left;
        posy = e.clientY - pos.top;

        if(posx + menu.offsetWidth > innerWidth) {
            posx = posx - menu.offsetWidth;
            openLeft = true;
        } else {
            openLeft = false;
        }

        if(posy + menu.offsetHeight > innerHeight) {
            posy = posy - menu.offsetHeight;
            openTop = false;
        } else {
            openTop = true;
        }

        visible = true;
    }

    export function close() {
        visible = false;
    }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    bind:this={container}
    class={`
        fixed w-[100vw] h-[100vh] z-100 top-0 left-0
        ${visible ? "" : "pointer-events-none"}
    `}
    onclick={close}
    oncontextmenu={close}
>
    <div
        bind:this={menu}
        style="left: {posx}px; top: {posy}px;"
        class={`
            relative bg-gray-800 shadow-container rounded-md
            duration-150 transition-[opacity,scale] ease-in-out 
            ${visible ? "opacity-100 scale-100" : "opacity-0 scale-0 pointer-events-none"}
            flex flex-col w-40 p-1 gap-1
            ${openTop
                ? openLeft ? "origin-top-right" : "origin-top-left"
                : openLeft ? "origin-bottom-right" : "origin-bottom-left"
            }
        `}
    >
        {@render children()}
    </div>
</div>


