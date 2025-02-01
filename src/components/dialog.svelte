<script lang="ts">
    let { children, title = "" } = $props();

    export function open() {
        opened = true;

        new Promise((resolve) => {
            setTimeout(resolve, 1);
        }).then(() => {
            visible = true;
        });
    }

    export function close() {
        if (blocked) return;
        visible = false;

        new Promise((resolve) => {
            setTimeout(resolve, 200);
        }).then(() => {
            opened = false;
        });
    }

    export function block() {
        blocked = true;
    }

    export function unblock() {
        blocked = false;
    }

    let opened = $state(false);
    let visible = $state(false);
    let blocked = $state(false);
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
{#if opened}
    <div
        id="container"
        class={`
            fixed inset-0 w-[100vw] h-[100vh]
            flex justify-center items-center
        `}
    >
        <div 
            id="background"
            class={`
                absolute w-full h-full bg-[#0000008d]
                duration-250 ease-out 
                ${visible ? "opacity-100" : "opacity-0"}
            `}
            onclick={close}
        ></div>

        <div 
            id="dialog" 
            class={`
                absolute flex flex-col justify-center
                m-0 p-0 bg-gray-900 shadow-container
                border-1 border-gray-950 rounded-sm
                duration-250 ease-out w-auto h-auto
                ${visible ? "opacity-100 translate-y-0" : "opacity-0 -translate-y-3"}
            `}
        >
            <button 
                class={`
                    absolute top-3 right-3
                    bg-transparent border-0
                    duration-200 ease-out
                    opacity-60 hover:opacity-100 
                    hover:cursor-pointer
                `}
                onclick={close}
            >
                <span class="material-symbols-outlined text-2xl">
                    close
                </span>
            </button>

            <div 
                class={`
                    flex items-center w-full h-6
                    mx-4 my-3 text-xl font-semibold
                `}
            >
                <p>{title}</p>
            </div>

            <div class="px-3 py-3 -mt-3 overflow-hidden">
                {@render children()}
            </div>
        </div>
    </div>
{/if}
