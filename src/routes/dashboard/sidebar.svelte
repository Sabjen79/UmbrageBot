<script lang="ts">
    import { activeBot } from "../start_page/bot_accounts";
    import SidebarButton from "./sidebar_button.svelte";
</script>

<div
    class={`
        absolute overflow-clip
        left-0 top-0 h-full
        duration-200 ease-in-out select-none
        text-gray-100 rounded-lg
    `}
>
    <SidebarButton icon="account_circle" text="Account" href="/dashboard/account"/>
    <SidebarButton icon="forum" text="Lexicon" href="/dashboard/lexicon"/>
    <SidebarButton icon="queue_music" text="Music" href="/dashboard/music"/>
    <SidebarButton icon="settings" text="Settings" href="/dashboard/settings"/>
</div>