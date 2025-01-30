<script lang="ts">
    import LoadingSpinner from "./loading_spinner.svelte";

    let {
        text,
        icon = "",
        onclick = async () => {
            await new Promise((resolve) => setTimeout(resolve, 3000));
        },
    } = $props();

    let hover = $state(false);
    let pressed = $state(false);
    let waiting = $state(false);
</script>

<button
    id="container"
    class={`
        relative flex items-center justify-center
        overflow-visible m-1 p-0
        ${waiting ? "bg-(--primary-500)" : "bg-(--primary-500)"} text-(--primary-100) border-1 
        
        rounded-sm
        float-left duration-200
        ${hover && !waiting ? "cursor-pointer" : "cursor-auto"}
        ${pressed && !waiting ? "inset-shadow-[0_1px_5px_var(--primary-950)]" : waiting ? "inset-shadow-[0_40px_5px_var(--primary-950)]" : ""}
        ${waiting ? "border-(--primary-500)" : "border-(--primary-600)"}
    `}
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
    
    <div
        id="spinner"
        class={`
            absolute flex items-center justify-center row-span-full col-span-full
            inset-2 duration-500 ease-in-out
            ${waiting ? "opacity-100" : "opacity-0"}
        `}
    >
        <LoadingSpinner color="var(--primary)" />
    </div>

    <div
        id="content"
        class={`
            relative flex items-center justify-center
            px-3 py-1.5 text-base
            bg-transparent duration-100
            opacity-${waiting ? "0" : "100"}
        `}
    >

        {#if icon != ""}
            <span class="material-symbols-outlined mx-1 -ml-1" style="font-size: 22px;">
                {icon}
            </span>
        {/if}
        {text}
    </div>
</button>

<style lang="postcss">
    @reference "tailwindcss";

    .material-symbols-outlined {
        font-variation-settings:
            "FILL" 1,
            "wght" 400,
            "GRAD" 0,
            "opsz" 24;
    }
</style>
