<script lang="ts">
    import SidebarButton from "./sidebar_button.svelte";
</script>

<div
    class={`
        flex flex-col items-start z-1
        w-12 xl:w-30 h-full pb-6 ml-2
        duration-200 ease-in-out select-none
        text-gray-100 border-r-1 border-gray-800
        shadow-[4px_0_5px_rgba(0,0,0,0.25)]
    `}
>
    <SidebarButton icon="account_circle" text="Account" href="/dashboard/account"/>
    <SidebarButton icon="forum" text="Lexicon" href="/dashboard/lexicon"/>
    <SidebarButton icon="queue_music" text="Music" href="/dashboard/music"/>
    <div class="flex-1 flex justify-end items-end">
        <SidebarButton icon="settings" text="Settings" href="/dashboard/settings"/>
    </div>
    
</div>