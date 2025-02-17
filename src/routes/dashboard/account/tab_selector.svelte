<script lang="ts">
    let {
        text,
        active,
        onclick,
    } = $props();

    let hover = $state(false);
    let pressed = $state(false);
</script>

<button 
    class={`w-25`}
    onmouseenter={() => { hover = true }}
    onmouseleave={() => { hover = false }}
    {onclick}
>
    <p class={`
        duration-200 ease-in-out font-medium text-lg
        ${active ? "text-primary-400" : hover ? "text-primary-300 cursor-pointer" : "text-gray-50"}
    `}>
        {text}
    </p>
</button>