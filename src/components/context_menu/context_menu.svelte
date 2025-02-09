<script lang="ts">
    let {
        children
    } = $props();

    // TODO: Fix out of bounds menu
    let opened = $state(false);
    let visible = $state(false);

    let posx = $state(0);
    let posy = $state(0);

    export function open(e: MouseEvent) {
        opened = true;

        posx = e.clientX;
        posy = e.clientY;

        new Promise((resolve) => {
            setTimeout(resolve, 10);
        }).then(() => {
            visible = true;
        });
    }

    export function close() {
        visible = false;

        new Promise((resolve) => {
            setTimeout(resolve, 150);
        }).then(() => {
            opened = false;
        });
    }
</script>

{#if opened}
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class={`
        fixed w-[100vw] h-[100vh] z-100
        ${visible ? "" : "pointer-events-none"}
    `}
    onclick={close}
    oncontextmenu={close}
>
    <div
        style="left: {posx}px; top: {posy}px;"
        class={`
            relative bg-gray-800 shadow-container rounded-md
            duration-150 ease-in-out origin-top-left
            ${visible ? "opacity-100 scale-100" : "opacity-0 scale-0"}
            flex flex-col w-40 p-1 gap-1
        `}
    >
        {@render children()}
    </div>
</div>

{/if}


