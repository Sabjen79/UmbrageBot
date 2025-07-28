<script lang="ts">
    import ContextMenuItem from "./context_menu/context_menu_item.svelte";

    let {
        values,
        value = $bindable(undefined),
        placeholder = "",
        onchange = () => {}
    }: {
        values: dropdownValues[],
        value?: string | number | undefined,
        placeholder?: string,
        onchange?: Function
    } = $props();

    type dropdownValues = {
        label: string,
        value: any
    }

    let expanded = $state(false);

    let expand_up_state = true;
    
    let expand_up = $derived.by(() => {
        if(!expanded) return expand_up_state;
        
        let pos = container?.getBoundingClientRect();

        if (!pos) {
            expand_up_state = false;
            return false;
        }

        const result = pos.bottom + values.length * 40 > window.innerHeight;
        expand_up_state = result;
        return result;
    });

    let container: HTMLElement;
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class={`
        fixed w-[100vw] h-[100vh] z-100 top-0 left-0
        ${expanded ? "" : "pointer-events-none"}
    `}
    onclick={() => {expanded = false}}
    oncontextmenu={() => {expanded = false}}
>

</div>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<button
    class={`
        relative inline-flex items-center justify-between
        bg-gray-950 rounded-md 
        w-full h-8 p-2 m-0 
        duration-200 ease-out
        ring-gray-600 ring-1
        hover:ring-gray-500 hover:cursor-pointer
    `}
    onclick={() => {
        if(!expanded)
            expanded = true
    }}>
    
    <div class="text-gray-50">
        {values.find((a) => a.value == value)?.label ?? placeholder}
    </div>

    <div class="font-icons text-2xl">
        arrow_drop_down
    </div>

    <div
        bind:this={container} 
        class={`
            absolute inset-0 z-200 h-fit ring-gray-500 ring-1
            bg-gray-950 shadow-container rounded-md
            duration-150 transition-[opacity,scale] ease-in-out 
            ${expanded ? "opacity-100 scale-100" : "opacity-0 scale-0 pointer-events-none"}
            flex flex-col p-2 gap-1 
            ${expand_up ? "-translate-y-full origin-bottom" : "translate-y-8 origin-top"}
        `}>

        {#each values as v, index}
            <ContextMenuItem text={values[index].label} onclick={async () => {
                await new Promise(resolve => setTimeout(resolve, 10));

                expanded = false;

                value = values[index].value;

                onchange();
            }}/>
        {/each}
    </div>
</button>