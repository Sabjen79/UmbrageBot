<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import ErrorItem from "./error_item.svelte";
    import { page } from "$app/state";

    type Error = {
        id: number;
        text: string;
    };

    let errorList: Error[] = $state([]);

    onMount(async () => {
        await listen<string>("error", async (event) => {
            errorList.push({ id: Date.now(), text: event.payload });
        })
    })
</script>

<div
    class={`
        absolute bottom-4 right-0 px-10
        flex flex-col items-center z-100 duration-200
        ${page.url.pathname == "/start_page" ? "left-0" : "left-12"}
    `}
>
    {#each errorList as error (error.id)}
        <ErrorItem message={error.text} callback={async () => {
            // Wait for animation to finish
            await new Promise((resolve) => setTimeout(resolve, 200));

            let index = errorList.findIndex(x => x.id == error.id);
            errorList.splice(index, 1);
            errorList = errorList;
        }}/>
    {/each}
</div>