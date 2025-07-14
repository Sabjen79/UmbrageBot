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

    let tabIndex = 0;
</script>

<div
    class={`
        w-full h-40 bg-gray-800 rounded-md shadow-container
        flex items-center
    `}
>
    <div
        class={`
            rounded-full w-30 h-30 inset-shadow-[0_0_10px_rgba(0,0,0,0.5)]
            overflow-hidden mx-5
        `}
    >
        <img class="pointer-events-none" 
            alt={$botProfile.username} 
            src={$selectedBot?.avatarUrl}
        />
    </div>

    <div
        class={`
            flex-1 flex flex-col justify-center
        `}
    >
        <p class="font-semibold text-3xl">
            {$botProfile.username}
        </p>
    </div>

    <div
        class={`
            absolute right-5 top-6
        `}
    >
        <Button 
            icon="power_settings_new"
            isRed={true}
            onclick={async () => {
                await invoke("shutdown_bot");

                $selectedBot = null;
                await goto("/start_page");
            }}
        />
    </div>
</div>

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
        class={`-mt-0.5 -mb-0.25 w-25 border-b-3 border-b-primary-500 duration-250 ease-out`}
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
