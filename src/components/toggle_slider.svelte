<script lang="ts">
    let {
        label = "",
        alignRight = false,
        toggled = $bindable(),
    } = $props();

    let pressed = $state();
</script>

<button 
    id="container" 
    aria-label="Toggle" 
    class={{toggled, pressed}}
    onmousedown={() => { pressed = true; }}
    onmouseup={() => { pressed = false; }}
    onmouseleave={() => { pressed = false; }}
    onclick={() => {toggled = !toggled;}}
>
    <svg
        id="background"
        viewBox="50 0 250 173.20508075688772"
        stroke={toggled ? "var(--foreground)" : "var(--gray)"}
        stroke-width=10
    ><path
        style="transition: 0.2s;"
        fill={toggled ? "var(--primary)" : "var(--secondary)"}
        d="M0 86.60254037844386L50 0L300 0L350 86.60254037844386L300 173.20508075688772L50 173.20508075688772Z"
    ></path></svg
    >

    <svg
        id="switch"
        viewBox="-5 0 210 173.20508075688772"
        stroke="var(--gray)"
        stroke-width=5
    ><path
        fill="var(--foreground)"
        d="M0 86.60254037844386L50 0L150 0L200 86.60254037844386L150 173.20508075688772L50 173.20508075688772Z"
    ></path></svg
    >
</button>

<style>
    #container {
        width: 50px;
        height: 24px;

        display: inline-flex;
        align-items: center;

        background-color: transparent;

        border: 0;

        transition: 0.15s ease-out;
    }

    #background {
        width: 100%;
        height: 100%;

        overflow: visible;
        
    }

    #container:hover {
        cursor: pointer;
    }

    #container > #switch {
        position: absolute;
        left: 16px;

        width: 20px;
        scale: 1;

        transform: scaleX(1);

        transition: 0.15s ease-out;
    }

    #container.pressed > #switch {
        transform: scaleX(120%);
        left: 18px;
    }

    #container.toggled.pressed > #switch {
        transform: scaleX(120%);
        left: 32px;
    }
    
    #container.toggled > #switch {
        left: 34px;
    }
</style>