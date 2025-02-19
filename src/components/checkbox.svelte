<script lang="ts">
    let {
        checked = $bindable(),
        text = "",
    } = $props();

    let pressed = $state();

</script>

<div class="inline-flex justify-between h-6 select-none">
    <button
        class={`
            inline-flex items-center justify-center
            w-6 h-6 mx-0 
            bg-transparent border-none cursor-pointer
            duration-150 ease-in-out
            ${pressed ? "scale-90" : "scale-100"}
        `}
        onclick={() => {
            checked = !checked;
        }}
        onmouseleave={() => { pressed = false }}
        onmousedown={() => { pressed = true }}
        onmouseup={() => { pressed = false }}
    >
        {#each {length: 2} as _, index}
            <svg
            class={`
                absolute w-6 h-6
                ${index == 1 
                ? "duration-150 ease-out " + (checked ? "[clip-path:circle(100%)]" : "[clip-path:circle(0)]") 
                : ""}
            `}
            viewBox="-5 0 210 173.20508075688772"
            stroke={index == 0 ? "var(--color-gray-400)" : "var(--color-gray-200)"}
            stroke-width=11
            ><path
                fill={index == 0 ? "var(--color-gray-800)" : "var(--color-primary-500)"}
                d="M0 86.60254037844386L50 0L150 0L200 86.60254037844386L150 173.20508075688772L50 173.20508075688772Z"
            ></path></svg
            >
        {/each}
        <span class={`
            absolute font-icons text-gray-200 text-xl
            ![font-variation-settings:'FILL'_0,'wght'_500,'GRAD'_0,'opsz'_20]
            ${checked ? "[clip-path:circle(100%)]" : "[clip-path:circle(0)]"}
        `}>
            check
        </span>
    </button>

    {#if text != ""}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
        <p 
            class="ml-2 flex-1 duration-100 {checked ? "text-gray-100" : "text-gray-300"}"
            onclick={() => {
                checked = !checked;
            }}
        >
            Delete bot data
        </p>
    {/if}
    
</div>