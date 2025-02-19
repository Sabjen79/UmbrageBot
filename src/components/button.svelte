<script lang="ts">
    import LoadingSpinner from "./loading_spinner.svelte";

    let {
        text = "",
        icon = "",
        isRed = false,
        disabled = $bindable(false),
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
        ${text == "" ? "w-10" : ""}
        relative flex items-center justify-center
        overflow-visible m-1 p-0 scheme-only-light
        text-primary-100 border-1 rounded-sm float-left duration-200
        ${hover && !waiting && !disabled ? "cursor-pointer" : "cursor-auto"}

        ${disabled ? "bg-gray-800" 
        : hover && !waiting ? isRed ? "bg-red-900" : "bg-primary-600" 
        : isRed ? "bg-red-800" : "bg-primary-500"}

        ${pressed && !waiting ? "inset-shadow-[0_1px_5px_var(--color-primary-950)]" 
        : waiting ? "inset-shadow-[0_40px_5px_var(--color-primary-950)]" : ""}
        
        ${disabled ? "border-gray-900" 
        : waiting ? isRed ? "border-red-800" : "border-primary-500" 
        : isRed ? "border-red-950" : "border-primary-600"}
    `}
    onmouseenter={() => {
        hover = true;
    }}
    onmouseleave={() => {
        hover = false;
        pressed = false;
    }}
    onmousedown={() => {
        if(disabled) return;

        pressed = true;
    }}
    onmouseup={() => {
        if(disabled) return;

        pressed = false;
    }}
    onclick={async () => {
        if (waiting || disabled) return;

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
        <LoadingSpinner svgClass={isRed ? "stroke-red-700" : "stroke-primary-500"}/>
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
            <span class="font-icons text-2xl {text == "" ? "-ml-0.25" : "mx-1 -ml-1"}">
                {icon}
            </span>
        {/if}
        {text}
    </div>
</button>
