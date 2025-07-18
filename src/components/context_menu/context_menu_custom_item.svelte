<script lang="ts">
    let {
        onclick = () => {},
        children
    } = $props();
</script>

<button
    class={`
        w-full h-8 flex items-center
        bg-transparent hover:bg-gray-900
        duration-100 hover:cursor-pointer
        rounded-md
    `}
    onclick={() => {onclick()}}
>
    {@render children()}
</button>