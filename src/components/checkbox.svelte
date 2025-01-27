<script lang="ts">
    let {
        checked = $bindable(),
    } = $props();

    let pressed = $state();

</script>

<div id="container" >
    <button id="checkbox" 
        onclick={() => {
            checked = !checked;
            pressed = false;
        }}
        onmouseleave={() => { pressed = false; }}
        onmousedown={() => { pressed = true; }}
        class={{checked, pressed}}
    >
        {#each {length: 2} as _, index}
            <svg
            viewBox="-5 0 210 173.20508075688772"
            stroke={index == 0 ? "var(--gray)" : "var(--foreground)"}
            stroke-width=10
            ><path
                fill={index == 0 ? "var(--secondary)" : "var(--primary)"}
                d="M0 86.60254037844386L50 0L150 0L200 86.60254037844386L150 173.20508075688772L50 173.20508075688772Z"
            ></path></svg
            >
        {/each}
        <span class="material-symbols-outlined">
            check
        </span>
    </button>
</div>

<style>
    .material-symbols-outlined {
        font-variation-settings:
        'FILL' 0,
        'wght' 500,
        'GRAD' 0,
        'opsz' 20
    }

    #container {
        display: inline-flex;
        justify-content: space-between;

        height: 24px;
        clip-path: none;
        user-select: none;
    }

    #checkbox {
        background-color: transparent;
        border: none;

        display: inline-flex;
        align-items: center;
        justify-content: center;

        cursor: pointer;
        width: 24px;
        height: 24px;
        margin: 0 5px;

        scale: 1;
        transition: 0.15s ease-in-out;
    }

    #checkbox.pressed {
        scale: 0.9;
    }

    #checkbox > * {
        position: absolute;
        
    }

    svg {
        width: 24px;
        height: 24px;
    }

    svg:last-of-type, span {
        clip-path: circle(0);

        transition: 0.15s ease-out;
    }

    .checked > svg:last-of-type, .checked > span {
        clip-path: circle(100%);
    }

    span {
        color: var(--foreground);
        font-size: 1.2em;
    }
</style>