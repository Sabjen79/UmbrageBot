<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import SettingsRow from "../../../../components/settings_row.svelte";
    import ToggleSlider from "../../../../components/toggle_slider.svelte";
    import Divider from "../../../../components/divider.svelte";
    import { onDestroy } from "svelte";
    import TextInput from "../../../../components/text_input.svelte";
    import { emit } from "@tauri-apps/api/event";
    import { botConfig } from "../../../../stores/bot_config_store";
    import { botProfile } from "../../../../stores/bot_profile_store";
    import TimerInfo from "../../../../components/timer_info.svelte";
    import TimeSlider from "../../../../components/time_slider.svelte";
    import { dbActivities, refreshActivities } from "../../../../stores/bot_activity_store";
    import Button from "../../../../components/button.svelte";
    import Dropdown from "../../../../components/dropdown.svelte";

    refreshActivities();
</script>

<div class="w-full pb-10">

    <SettingsRow
        title="Activity Timer"
        description={`Change the bot's activity at a set interval of time`}
    >
        <ToggleSlider bind:toggled={$botConfig.activityTimerEnabled}/>
    </SettingsRow>

    <Divider />

    <SettingsRow
        title="Time Interval"
        description="How long should the bot wait for activity changes"
        visible={$botConfig.activityTimerEnabled}
    >
        <TimeSlider bind:value_min={$botConfig.activityTimerMin} bind:value_max={$botConfig.activityTimerMax} value_list={[5, 10, 15, 20, 25, 30, 45, 60, 90, 120, 180, 360, 720].map(m => m * 60)}/>
    </SettingsRow>

    <Divider />

    <SettingsRow visible={$botConfig.activityTimerEnabled}>
        <TimerInfo timer_name={"BOT_ACTIVITY_TIMER"} />
    </SettingsRow>
    
    <SettingsRow small={false} visible={$botConfig.activityTimerEnabled}>
        <div 
            class={`
                w-full
                bg-gray-950 rounded-xl 
                outline-1 outline-gray-800
            `}>

            <div
                class={`
                    h-10 w-full border-b-1 border-gray-800
                    flex justify-between items-center px-4
                    ${$dbActivities.length == 0 ? "border-b-transparent" : ""}
                `}>

                <div class="text-lg font-black">
                    Activity List
                </div>

                <Button
                    icon="add"
                    rounded small
                    onclick={async () => {
                        await invoke('db_insert_activity');
                        await refreshActivities();
                    }}
                />
                
            </div>

            {#each $dbActivities as activity}
                <div class="flex items-center w-full my-2">
                    <div class="ml-2 h-8 w-34">
                        <Dropdown
                            bind:value={activity.type_num}
                            values={[
                                { value: 1, label: "Playing" },
                                { value: 2, label: "Streaming" },
                                { value: 3, label: "Listening to" },
                                { value: 4, label: "Watching" },
                                { value: 5, label: "Competing in" },
                                { value: 6, label: "Custom" }
                            ]}
                            onchange={async () => {
                                await invoke("db_update_activity", { activity: activity })
                            }}
                        />
                    </div>
                    
                    <div class="flex-1 mx-2">
                        <input
                            class={`
                                rounded-md outline-0
                                w-full h-8 p-2 m-0 
                                duration-200 ease-out
                                ring-1 ring-gray-600 focus:ring-gray-400
                            `}
                            maxlength={50}
                            bind:value={activity.content}
                            placeholder="No activity"
                            onblur={async () => {
                                await invoke("db_update_activity", { activity: activity })
                            }}
                        />
                    </div>

                    <button class={`
                        font-icons text-2xl text-gray-300 hover:text-red-600
                        rounded-md h-8 w-8 mr-2 hover:cursor-pointer
                        ring-1 ring-gray-600 hover:ring-red-600
                        duration-200 ease-in-out
                    `}
                    onclick={async () => {
                        await invoke("db_delete_activity", { id: activity.id });

                        await refreshActivities();
                    }}>
                        close
                    </button>
                </div>
            {/each}
        </div>
    </SettingsRow>
    
</div>