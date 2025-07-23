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
</script>

<div class="w-full pb-1">

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
    
</div>