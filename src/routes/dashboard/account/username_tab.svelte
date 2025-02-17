<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import SettingsRow from "../../../components/settings_row.svelte";
    import ToggleSlider from "../../../components/toggle_slider.svelte";
    import { botConfig } from "../bot_config";

    let a = $state(false);

</script>

<div class="w-full">
    <SettingsRow title="Enable Timer" description="Description" >
        <ToggleSlider bind:toggled={$botConfig.usernameTimerEnabled}/>
    </SettingsRow>
</div>