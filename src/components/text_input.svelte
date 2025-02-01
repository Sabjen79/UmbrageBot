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
    class={`
        relative inline-flex items-center
        bg-gray-950 border-none rounded-md
        w-full h-9.5 p-2 m-0
        ring-1 
        duration-200 ease-out
        ${error ? "ring-red-700" 
        : focus ? "ring-primary-500" 
        : hover ? "ring-gray-400" 
        : "ring-gray-700"}
    `}
    onmouseenter={() => { hover = true }}
    onmouseleave={() => { hover = false }}
>
    <input type="text"
        bind:value
        placeholder={placeholder}
        onfocusin={() => { focus = true }}
        onfocusout={() => { focus = false; }}
        onmouseenter={() => { hover = true }}
        onmouseleave={() => { hover = false }}
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
    input {
        background-color: transparent;
        border: none;
        outline: none;

        width: 100%;
        height: 24px;
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