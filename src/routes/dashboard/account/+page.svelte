<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Button from "../../../components/button.svelte";
    import Divider from "../../../components/divider.svelte";
import SettingsRow from "../../../components/settings_row.svelte";
    import { selectedBot } from "../../../stores/bot_accounts_store";
    import TabSelector from "./tab_selector.svelte";
    import UsernameTab from "./username_tab.svelte";
    import { goto } from "$app/navigation";
    import { botProfile } from "../../../stores/bot_profile_store";
    import TopBanner from "./top_banner.svelte";

    let tabIndex = 0;
</script>

<TopBanner/>

<div
    class={`
        w-full px-2 mt-2
    `}
>
    {#each ["Username", "Status", "Avatar", "Banner"] as selector, index}
        <TabSelector text={selector} active={tabIndex == index} onclick={() => {
            tabIndex = index
        }}/>
    {/each}
    
    <Divider/>

    <div
        class={`-mt-0.5 -mb-0.25 w-25 border-b-3 border-b-primary-400 duration-250 ease-out`}
        style="transform: translateX({tabIndex * 100}%)"
    ></div>
    
    <div class="w-full overflow-x-clip">
        <div
            class={`flex w-[400%] duration-250 ease-out`}
            style="transform: translateX(-{tabIndex * 100/4}%)"
        >
            <UsernameTab />
            <UsernameTab />
            <UsernameTab />
            <UsernameTab />
        </div>
    </div>
    
</div>
