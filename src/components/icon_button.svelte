<script lang="ts">
    let {
        icon = "close",

        // TODO: Add icon variation settings
        // size = "10",
        // fill = "1",
        // weight = "800",
        // grade = "0",
        // opticalSize = "24",

        disabled = $bindable(false),
        onclick = async (e: MouseEvent) => {},
    } = $props();

    let hover = $state(false);
    let pressed = $state(false);

</script>

<button
    onmouseenter={() => { hover = true }}
    onmouseleave={() => { hover = false; pressed = false }}
    onmousedown={() => { pressed = true }}
    onmouseup={(e) => { 
        pressed = false;
        onclick(e);
    }}
>
    <span class={`
        font-icons text-2xl -mr-0.5 duration-200
        ${hover ? "text-gray-100 scale-115 cursor-pointer" : "text-gray-300"}
    `}>
        {icon}
    </span>
</button>