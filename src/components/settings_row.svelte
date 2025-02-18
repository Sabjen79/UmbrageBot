<script lang="ts">
    let {
        title,
        description,
        visible = true,
        children
    } = $props();


</script>

<div
    class={`
        flex justify-between items-center
        mx-4 select-none duration-200 ease-out
        ${visible ? "h-14 my-2 opacity-100" : "h-0 opacity-0 my-0 pointer-events-none"}
    `}
>
    <div class="flex flex-col justify-center">
        <p class="font-semibold">
            {title}
        </p>
        <p class="text-sm">
            {description}
        </p>
    </div>
    <div class="flex items-center">
        {@render children()}
    </div>
</div>