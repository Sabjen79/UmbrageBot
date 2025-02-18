<script lang="ts">
    let {
        toggled = $bindable(),
    } : {
        toggled: boolean
    } = $props();

    let pressed = $state();
</script>

<button 
    aria-label="Toggle" 
    class={`
        w-12 h-6 inline-flex items-center
        bg-transparent border-0
        duration-150 ease-out
        hover:cursor-pointer
    `}
    onmousedown={() => { pressed = true; }}
    onmouseup={() => { pressed = false; }}
    onmouseleave={() => { pressed = false; }}
    onclick={() => {toggled = !toggled;}}
>
    <svg
        class={`
            w-full h-full overflow-visible
        `}
        viewBox="50 0 250 173.20508075688772"
        stroke={toggled ? "var(--color-gray-400)" : "var(--color-gray-600)"}
        stroke-width=10
    ><path
        style="transition: 0.2s;"
        fill={toggled ? "var(--color-primary-500)" : "var(--color-gray-800)"}
        d="M0 86.60254037844386L50 0L300 0L350 86.60254037844386L300 173.20508075688772L50 173.20508075688772Z"
    ></path></svg
    >

    <svg
        class={`
            absolute w-5
            duration-150 ease-out
            ${toggled 
                ? pressed ? "translate-x-5.5" : "translate-x-6" 
                : pressed ? "translate-x-1.5" : "translate-x-1"
            }
        `}
        viewBox="0 0 200 173.20508075688772"
    ><path
        fill="var(--color-gray-100)"
        d="M0 86.60254037844386L50 0L150 0L200 86.60254037844386L150 173.20508075688772L50 173.20508075688772Z"
    ></path></svg
    >
</button>