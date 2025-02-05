<script lang="ts">
    let {
        icon = "close",

        size = "10",
        fill = "1",
        weight = "800",
        grade = "0",
        opticalSize = "24",

        disabled = $bindable(false),
        onclick = async () => {
            await new Promise((resolve) => setTimeout(resolve, 3000));
        },
    } = $props();

    let hover = $state(false);
    let pressed = $state(false);

</script>

<button
    onmouseenter={() => { hover = true }}
    onmouseleave={() => { hover = false; pressed = false }}
    onmousedown={() => { pressed = true }}
    onmouseup={() => { pressed = false }}
>
    <span class={`
        font-icons text-2xl -mr-0.5 duration-200
        ${hover ? "text-gray-100 scale-120 cursor-pointer" : "text-gray-300"}
    `}>
        {icon}
    </span>
</button>

<style lang="postcss">

</style>