<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import SettingsRow from "../../../components/settings_row.svelte";
    import ToggleSlider from "../../../components/toggle_slider.svelte";
    import { botConfig } from "../bot_config";
    import Divider from "../../../components/divider.svelte";
    import { onDestroy } from "svelte";
    import TextInput from "../../../components/text_input.svelte";

    let a = $state(false);

    let usernameSubscriber = botConfig.subscribe((config) => {
        if (config.usernameTimerEnabled) {
        } else {
        }
    });

    onDestroy(() => {
        usernameSubscriber();
    });
</script>

<div class="w-full overflow-hidden pb-1">
    <SettingsRow
        title="Multiple Usernames"
        description="Whether the bot will change his username after a period of time."
    >
        <ToggleSlider bind:toggled={$botConfig.usernameTimerEnabled} />
    </SettingsRow>

    <SettingsRow
        title="Username"
        description="Bot's display name. Won't replace nicknames."
        visible={!$botConfig.usernameTimerEnabled}
    >
        <div class="w-50">
            <TextInput 
                placeholder="Username" 
                validationType="token"
            />
        </div>
    </SettingsRow>

    <Divider />

    <SettingsRow
        title="Change Interval"
        description="How much time - in minutes - the bot will wait between username changes."
        visible={$botConfig.usernameTimerEnabled}
    >
        <div class="">
        </div>
    </SettingsRow>

    <Divider />
</div>