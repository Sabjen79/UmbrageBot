<script lang="ts">
    let {
        text = "",
        icon = "",
        isRed = false,
        onclick = () => {}
    } = $props();
</script>

<button
    class={`
        w-full h-8 flex items-center
        bg-transparent hover:bg-gray-700
        duration-100 hover:cursor-pointer
        rounded-md ${isRed ? "text-red-500" : ""}
    `}
    onclick={() => {onclick()}}
>
    <div class="font-icons text-xl mx-1">
        {icon}
    </div>
    <div class="text-base ">
        {text}
    </div>
    
</button>