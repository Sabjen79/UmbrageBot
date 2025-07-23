<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Button from "./button.svelte";

    let {
        timer_name
    } = $props();

    let time = $state(0);
    let controllable = $state(true);

    $effect(() => {
        readTime();
        
        const interval = setInterval(async () => {
            await readTime();
        }, 1000);

        return () => {
            clearInterval(interval);
        };
    });

    async function readTime() {
        time = await invoke("timer_get_time_left", { name: timer_name })
    }

    function parseTime() {
        let secs = Math.floor(time / 1000)

        const hours = Math.floor(secs / 3600);
        const minutes = Math.floor((secs % 3600) / 60);
        const seconds = secs % 60;

        let result = "";

        if (hours > 0) {
            result += hours.toString().padStart(2, "0") + ":";
        }

        result += minutes.toString().padStart(2, "0") + ":";
        result += seconds.toString().padStart(2, "0");

        return result;
    }
</script>

<div
    class={`
        h-12 select-none flex-1
        flex items-center
        bg-gray-950 rounded-4xl
        outline-1 outline-gray-800
    `}
>
    <div class="font-icons text-4xl text-gray-50 mx-2">
        timer
    </div>
    <div class="flex flex-col items-start mt-1">
        <div class="-mb-2 text-xs text-gray-200">{"Time left:"}</div>
        
        <div class="text-xl font-black">{parseTime()}</div>
    </div>

    <div class="flex-1"></div>

    {#each [["Run Early", "timer_run_early"], ["Reset", "timer_reset"]] as tuple}
        <button
            class={`
                mr-4 duration-200 px-2 py-0.5 rounded-xl
                ${controllable 
                    ? "hover:bg-gray-800 hover:cursor-pointer" 
                    : "bg-transparent text-gray-700"}
            `}
            onclick={async () => {
                if(!controllable) return;

                controllable = false;
                await invoke(tuple[1], { name: timer_name });
                await readTime();
                await new Promise(resolve => setTimeout(resolve, 2000));
                controllable = true;
            }}
        >
            {tuple[0]}
        </button>
    {/each}
</div>