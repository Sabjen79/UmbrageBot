<script lang="ts">
    import { blur } from "svelte/transition";
    import LoadingSpinner from "./loading_spinner.svelte";
    import { invoke } from "@tauri-apps/api/core";

    let {
        value = $bindable(""),
        placeholder = "Input Text",
        validationType = "",
    } = $props();

    let hover = $state(false);
    let focus = $state(false);
    let waiting = $state(false);
    let error = $state(false);

    let validationId = 0;
    async function validationFunc() {
        waiting = true;

        validationId = (validationId + 1) % 127;

        await invoke('validate_input', {
            message: value,
            validationType: validationType,
            validationId: validationId
        }).then((value) => {
            let v: number = value as number;
            if(Math.abs(v) == validationId) {
                waiting = false;
                error = (v < 0);
            }
        });
    }

    if(value == "" && validationType != "") {
        error = true;
    }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- Input is the main focusable object, Accesibility is useless for this div -->
<div
    id="container"
    aria-label="input"
    class={{hover, focus, waiting, error}}
    onmouseenter={() => { hover = true }}
    onmouseleave={() => { hover = false }}
>
    <div id="border"></div>
    <input type="text"
        bind:value
        placeholder={placeholder}
        onfocusin={() => { focus = true }}
        onfocusout={() => { focus = false; }}
        onkeypress={(event) => {
            if(event.key == 'Enter') document.querySelector("input")?.blur();
        }}
        oninput={validationFunc}
    >
    <div id="spinner">
        <LoadingSpinner />
    </div>
</div>

<style>
    #container {
        position: relative;
        display: inline-flex;
        align-items: center;
        background-color: color-mix(in hsl, var(--foreground) 2%, transparent);
        border: none;

        width: 100%;
        height: 100%;

        padding: 7px 7px;
        margin: 0;
    }

    #border {
        position: absolute;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;

        border-radius: 5px;
        pointer-events: none;

        border: solid 1px var(--gray);
        transition: 0.2s ease-in-out;
        outline: solid 1px transparent;
    }
    
    .hover:not(.focus, .error) > #border {
        border: solid 1px var(--foreground);
    }

    .focus > #border {
        border: solid 1px var(--primary);
        outline: solid 1px var(--primary);
    }

    .error > #border {
        border: solid 1px var(--error);
        outline: solid 1px var(--error);
    }

    input {
        background-color: transparent;
        border: none;
        outline: none;

        width: 100%;
    }

    #spinner {
        position: absolute;
        width: 24px;
        height: 24px;
        right: 8px;

        pointer-events: none;
        
        opacity: 0;
        transition: 0.15s ease-in-out;
    }

    .waiting > #spinner {
        opacity: 1;
    }
</style>