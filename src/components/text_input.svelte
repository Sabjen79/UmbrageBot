<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let {
        value = $bindable(""),
        validated = $bindable(false),
        placeholder = "Input Text",
        validationType = "",
        fastValidate = false
    } = $props();

    let hover = $state(false);
    let focus = $state(false);
    let waiting = $state(false);

    let firstValidation = $state(true);
    let validationId = 0;
    let lastError = $state("");
    let errorVisible = $state(false);

    type InputValidationError = {
        validationId: number;
        message: string;
    };

    async function validationFunc() {
        if(validationType == "") return;

        firstValidation = false;
        waiting = true;
        validated = false;
        validationId = (validationId + 1) % 127;

        await invoke('validate_input', {
            message: value,
            validationType: validationType,
            validationId: validationId
        }).then((value) => {
            if(value == validationId) {
                waiting = false;
                validated = true;
            }
        }).catch((error: InputValidationError) => {
            if(error.validationId == validationId) {
                waiting = false;
                validated = false;
                lastError = error.message;
            }
        });
    }

    if(value == "" && validationType != "") {
        validated = false;
    }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- Input is the main focusable object, Accesibility is useless for this div -->
<div class="relative z-1">
    <div
        id="container"
        aria-label="input"
        class={`
            relative inline-flex items-center
            bg-gray-950 border-none rounded-md ring-1
            w-full h-9.5 p-2 m-0 
            duration-200 ease-out
            ${!firstValidation && !validated ? "ring-red-700"
            : waiting ? "ring-gray-400"
            : focus ? "ring-primary-500" 
            : hover ? "ring-gray-400" 
            : "ring-gray-700"}
        `}
        onmouseenter={() => { hover = true }}
        onmouseleave={() => { hover = false }}>

        <input type="text"
            class={`
                bg-transparent border-0 outline-0
                flex-1 h-6
            `}
            bind:value
            placeholder={placeholder}
            onfocusin={() => { focus = true }}
            onfocusout={() => { 
                focus = false;
                
                if(!fastValidate) {
                    validationFunc()
                }
            }}
            onmouseenter={() => { hover = true }}
            onmouseleave={() => { hover = false }}
            onkeypress={(event) => {
                if(event.key == 'Enter') document.querySelector("input")?.blur();
            }}
            oninput={fastValidate ? validationFunc : null}
        />

        <div
            class={`
                absolute h-full w-7.5 text-red-700 right-2
                flex justify-end items-center pointer-events-none
                duration-200 ease-in-out overflow-visible
                ${!firstValidation && !validated ? "opacity-100" : "opacity-0"}
        `}>

            <button
                onmouseenter={() => {
                    if(!firstValidation && !validated)
                        errorVisible = true
                }}
                onmouseleave={() => { errorVisible = false }}
                class={`
                    font-icons text-2xl ![font-variation-settings:'FILL'_1,'wght'_400,'GRAD'_0,'opsz'_24]
                    pb-0.5 ${!firstValidation && !validated ? "pointer-events-auto" : "pointer-events-none"}
                    bg-gray-950
                `}
            >
                warning
            </button>
        </div>

        <div class={`
                absolute bg-red-700 text-gray-950
                right-4 top-0 -z-1 !text-ellipsis translate-x-2
                px-2 py-0.5 text-base duration-250 ease-out
                rounded-t-md pointer-events-none overflow-clip
                ${errorVisible 
                ? "-translate-y-full [clip-path:polygon(0_0,_100%_0%,100%_100%,0_100%)]"
                : "translate-y-0 [clip-path:polygon(0_0,_100%_0%,100%_50%,0_50%)]"}
            `}>

            {lastError}
        </div>

    </div>
</div>