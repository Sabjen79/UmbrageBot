<script lang="ts">
    let {
        title = "",
        description = "",
        visible = true,
        children
    } = $props();

    let singleRow = $state(title == "" && description == "");
</script>

<div class="overflow-y-hidden">
    <div
        class={`
            flex justify-between items-center
            select-none duration-200 ease-out
            ${visible ? "h-14 my-2 opacity-100" : "h-0 opacity-0 my-0 pointer-events-none"}
            ${singleRow ? "mx-2" : "mx-4"}
        `}>

        {#if !singleRow}
            <div class="flex flex-col justify-center">
                <p class="font-semibold">
                    {title}
                </p>
                <p class="text-sm">
                    {description}
                </p>
            </div>
        {/if}

        <div class="flex items-center {singleRow ? "flex-1" : ""}">
            {@render children()}
        </div>
    </div>
</div>
