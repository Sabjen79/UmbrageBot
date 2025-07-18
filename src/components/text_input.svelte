<script lang="ts">
    // TODO: Rework this ASAP
    let {
        value = $bindable(""),
        validated = $bindable(false),
        placeholder = "Input Text",
        validation: validation,
        fastValidation = false
    }: {
        value: string,
        validated?: boolean,
        placeholder?: string,
        /** Return null for success, or error string */ 
        validation?: () => Promise<string | null>,
        /** If true, validation is made every change */ 
        fastValidation?: boolean
    } = $props();

    let hover = $state(false);
    let focus = $state(false);
    let waiting = $state(false);

    let firstValidation = $state(true);
    let lastError: string | null = $state(null);
    let errorVisible = $state(false);

    export async function fullValidation() {
        if(!validation) return;

        firstValidation = false;
        errorVisible = false;
        lastError = null;

        waiting = true;
        let result = await validation();
        waiting = false;

        lastError = result;
        validated = (result == null);
    }

    export async function setError(err: string | null) {
        firstValidation = false;
        errorVisible = true;
        lastError = err;
        validated = (err == null);
    }

    if(value == "" && validation != null) {
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
            ${waiting ? "ring-gray-600"
            : !validated && !firstValidation ? "ring-red-700"
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
            onfocusout={async () => { 
                focus = false;
                
                if(!fastValidation) {
                    await fullValidation();
                }
            }}
            onmouseenter={() => { hover = true }}
            onmouseleave={() => { hover = false }}
            onkeypress={(event) => {
                if(event.key == 'Enter') document.querySelector("input")?.blur();
            }}
            oninput={fastValidation ? fullValidation : null}
        />

        <div
            class={`
                absolute h-full w-7.5 text-red-700 right-2
                flex justify-end items-center pointer-events-none
                duration-200 ease-in-out overflow-visible
                ${!validated && !firstValidation && !waiting ? "opacity-100" : "opacity-0"}
        `}>

            <button
                onmouseenter={() => {
                    if(!validated && !firstValidation && !waiting)
                        errorVisible = true
                }}
                onmouseleave={() => { errorVisible = false }}
                class={`
                    font-icons text-2xl ![font-variation-settings:'FILL'_1,'wght'_400,'GRAD'_0,'opsz'_24]
                    pb-0.5 ${!validated && !firstValidation ? "pointer-events-auto" : "pointer-events-none"}
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