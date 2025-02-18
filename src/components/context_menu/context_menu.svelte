<script lang="ts">
    let {
        children
    } = $props();

    // TODO: Position menu to not go outside viewport
    let opened = $state(false);
    let visible = $state(false);

    let posx = $state(0);
    let posy = $state(0);

    
    // svelte-ignore non_reactive_update
    let container : HTMLElement;

    export function open(e: MouseEvent) {
        visible = true;

        let dim = container.getBoundingClientRect();

        posx = e.clientX - dim.left;
        posy = e.clientY - dim.top;
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
        style="left: {posx}px; top: {posy}px;"
        class={`
            relative bg-gray-800 shadow-container rounded-md
            duration-150 transition-[opacity,scale] ease-in-out origin-top-left
            ${visible ? "opacity-100 scale-100" : "opacity-0 scale-0 pointer-events-none"}
            flex flex-col w-40 p-1 gap-1
        `}
    >
        {@render children()}
    </div>
</div>


