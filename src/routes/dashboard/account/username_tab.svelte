<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import SettingsRow from "../../../components/settings_row.svelte";
    import ToggleSlider from "../../../components/toggle_slider.svelte";
    import Divider from "../../../components/divider.svelte";
    import { onDestroy } from "svelte";
    import TextInput from "../../../components/text_input.svelte";
    import { emit } from "@tauri-apps/api/event";
    import { botConfig } from "../../../stores/bot_config_store";
    import { botProfile } from "../../../stores/bot_profile_store";
    import TimerInfo from "../../../components/timer_info.svelte";

    let a = $state(false);

    let usernameSubscriber = botConfig.subscribe((config) => {
        if (config.usernameTimerEnabled) {
        } else {
        }
    });

    let value = $state($botProfile.username);

    onDestroy(() => {
        usernameSubscriber();
    });
</script>

<div class="w-full pb-1">

    <SettingsRow
        title="Username"
        description={`Be wary that Discord will not let you spam change your nickname.`}
        visible={!$botConfig.usernameTimerEnabled}
    >
        <div class="w-60">
            <TextInput
                bind:value={value}
                placeholder="Username" 
                validation={async () => {
                    if(value == "") {
                        return "Username cannot be empty!"
                    }

                    return await invoke('change_username', {username: value}).then(() => {
                        return null;
                    }).catch((err) => {
                        return err;
                    })
                }}
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

    <TimerInfo timer_name={"BOT_TEST_TIMER"} />
</div>