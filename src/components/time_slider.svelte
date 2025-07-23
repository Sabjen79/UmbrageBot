<script lang="ts">
    let {
        value_min = $bindable(),
        value_max = $bindable(),
        value_list
    } : {
        value_min: number,
        value_max: number,
        value_list: number[]
    } = $props();

    let indexes = $derived([value_list.indexOf(value_min), value_list.indexOf(value_max)]);
    let selected = $state(0);
    let isFixed = $derived(value_min == value_max);
    let hovers = $state([false, false]);
    let drag = $state(false);

    let sliderEl: HTMLDivElement;
    let tooltip: HTMLDivElement[] = [];

    let tooltipPos = $derived.by(() => {
        if (!sliderEl) return { left: [0, 0], top: 0 };
        const sliderRect = sliderEl.getBoundingClientRect();
        const handleLeft1 = sliderRect.left + (indexes[0] * sliderRect.width / (value_list.length - 1));
        const handleLeft2 = sliderRect.left + (indexes[1] * sliderRect.width / (value_list.length - 1));
        return {
            left: [handleLeft1, handleLeft2],
            top: sliderRect.top - 35
        };
    });

    function updateValueFromMouseEvent(e: MouseEvent) {
        if (!sliderEl) return;
        const rect = sliderEl.getBoundingClientRect();
        const clickX = e.clientX;
        const relativeX = clickX - rect.left;
        const percentage = Math.max(0, Math.min(1, relativeX / rect.width));
        const newIndex = Math.round(percentage * (value_list.length - 1));

        if(selected == 0) {
            if(Math.abs(indexes[0] - newIndex) < Math.abs(indexes[1] - newIndex)) {
                selected = 1;
            } else {
                selected = 2;
            }
        }

        if(isFixed) {
            value_min = value_max = value_list[newIndex];

            return;
        }

        if(selected == 1) {
            value_min = value_list[Math.min(newIndex, indexes[1] - 1)];
        } else if(selected == 2) {
            value_max = value_list[Math.max(newIndex, indexes[0] + 1)];
        }
    }

    function parseTime(secs: number) {
        const hours = Math.floor(secs / 3600);
        const minutes = Math.floor((secs % 3600) / 60);
        const seconds = secs % 60;

        let result = "";

        if (hours > 0) {
            result += hours.toString().padStart(2, "0") + ":";
        }

        result += minutes.toString().padStart(2, "0") + ":";
        result += seconds.toString().padStart(2, "0");

        return result;
    }
</script>

<svelte:window
    onmousemove={(e) => {
        if (drag) {
            updateValueFromMouseEvent(e);
        }
    }}
    onmouseup={() => {
        drag = false;
        selected = 0;
    }}
/>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
    class="flex flex-col justify-center w-60"
>
    <div class="m-auto w-fit flex justify-center items-center -mt-2 outline-1 outline-gray-500 rounded-2xl overflow-hidden">
        <button
            class="text-sm px-2 duration-200 {isFixed ? "bg-gray-800" : "bg-gray-900"}"
            onclick={() => {
                if(!isFixed) {
                    value_max = value_min;
                }
            }}>
            Fixed
        </button>

        <button
            class="text-sm px-2 duration-200 {!isFixed ? "bg-gray-800" : "bg-gray-900"}"
            onclick={() => {
                if(isFixed) {
                    if(indexes[0] == 0) {
                        value_max = value_list[indexes[0] + 1];
                    } else {
                        value_min = value_list[indexes[1] - 1];
                    }
                }
            }}>
            Range
        </button>
    </div>
    
    <div class="mt-4">
        <div
            bind:this={sliderEl}
            class={`
                bg-gray-700 h-1.25 rounded-xs relative cursor-pointer
            `}
            onmousedown={(e) => {
                drag = true;
                updateValueFromMouseEvent(e);
            }}>
            
            <div
                class={`
                    bg-primary-500 h-1.25 rounded-xs absolute duration-200
                `}
                style={`
                    left: ${isFixed 
                        ? "0" 
                        : indexes[0] * 100 / (value_list.length - 1)
                    }%;
                    right: calc(${isFixed 
                        ? (value_list.length - indexes[1] - 1) * 100 / (value_list.length - 1)
                        : (value_list.length - indexes[1] - 1) * 100 / (value_list.length - 1)
                    }%)
                `}
            ></div>

            {#each { length: value_list.length }, i}
                {#if i != 0 && i != value_list.length - 1}
                    <div 
                        class="absolute top-2 bg-gray-700 w-0.25 h-1"
                        style="left: calc({i * 100 / (value_list.length - 1)}%);"
                        
                    ></div>
                    <div 
                        class="absolute bottom-2 bg-gray-700 w-0.25 h-1"
                        style="left: calc({i * 100 / (value_list.length - 1)}%);"
                    ></div>
                {/if}
            {/each}

            {#each { length: 2 } as _, i}
                <div
                    class={`
                        absolute bg-gray-200 w-4 h-4 rounded-full
                        -top-1.25 duration-100
                    `}
                    style="left: calc({indexes[i] * 100 / (value_list.length - 1)}% - 7.5px);"
                    onmouseenter={() => hovers[i] = true}
                    onmouseleave={() => hovers[i] = false}
                    bind:this={tooltip[i]}
                >
                </div>
            {/each}
            
        </div>
    </div>
</div>

{#each { length: 2 } as _, i}
    {#if (hovers[i] || selected == i + 1) && tooltip[i]}
        <div
            class="fixed bg-primary-500 px-2 duration-100 pointer-events-none rounded-md z-50"
            style="left: {tooltipPos.left[i]}px; top: {tooltipPos.top}px; transform: translateX(-50%);"
        >
            {parseTime(value_list[indexes[i]])}
        </div>
    {/if}
{/each}