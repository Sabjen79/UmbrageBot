<script lang="ts">
    import { onMount } from "svelte";

    let {
        message,
        callback
    } = $props();

    let visible = $state(false);
    
    onMount(() => {
        new Promise((resolve) => setTimeout(resolve, 10)).then(async () => {
            visible = true;

            await new Promise((resolve) => setTimeout(resolve, 5000 - 200));

            visible = false;

            callback();
        })
    })
</script>

<div
    id="container"
    class={`
        w-full max-w-200 rounded-lg overflow-clip
        flex items-center px-2 select-none
        bg-red-900/30 border-1 border-red-900/50
        duration-200 ease-out
        ${visible ? "h-10 my-1 opacity-100" : "h-0 my-0 opacity-0"}
    `}
>
    <p class={"text-red-500 font-bold mr-1"} >
        {"ERROR: "}
    </p>

    <p class={"text-red-500 text-ellipsis truncate"} >
        {message}
    </p>
</div>