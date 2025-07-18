<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Button from "../../../components/button.svelte";
    import { selectedBot } from "../../../stores/bot_accounts_store";
    import { botProfile } from "../../../stores/bot_profile_store";
    import { goto } from "$app/navigation";
    import Dialog from "../../../components/dialog.svelte";
    import TextInput from "../../../components/text_input.svelte";
    import ContextMenu from "../../../components/context_menu/context_menu.svelte";
    import ContextMenuItem from "../../../components/context_menu/context_menu_item.svelte";
    import ContextMenuCustomItem from "../../../components/context_menu/context_menu_custom_item.svelte";

    let dialog: Dialog;
    let usernameTextInput: TextInput;
    let username = $state("");

    let statusContextMenu: ContextMenu;

    let statusList = new Map<string, Array<string>>([
        ["online", ["Online", "#43A25A"]],
        ["idle", ["Idle", "#CA9654"]],
        ["dnd", ["Do not Disturb", "#D83A42"]],
        ["invisible", ["Invisible", "#82838B"]]
    ]);
</script>

<div
    class={`
        w-full h-40 bg-gray-800 rounded-md shadow-container
        flex items-center relative
    `}
>
    <div
        class={`
            relative mx-5 w-30 h-30
            rounded-full inset-shadow-[0_0_10px_rgba(0,0,0,0.5)]
        `}
    >
        <img
            class="pointer-events-none"
            alt={$botProfile.username}
            src={$selectedBot?.avatarUrl}
        />

        <!-- svelte-ignore a11y_consider_explicit_label -->
        <button
            class={`
                absolute w-6 h-6 bottom-1.5 right-1.5 
                rounded-full ring-4 ring-gray-800
                hover:cursor-pointer hover:scale-120 duration-200
            `}
            style="background-color: {statusList.get($botProfile.status)![1]};"
            onclick={(e) => {
                statusContextMenu.open(e);
            }}
        ></button>
    </div>

    <div
        class={`
            flex-1 flex flex-col items-start justify-center
        `}
    >
        <button
            class={`
                font-semibold text-3xl select-none duration-200
                hover:cursor-pointer hover:text-primary-300
            `}
            onclick={() => {
                username = $botProfile.username;
                dialog.open();
            }}
        >
            {$botProfile.username}
        </button>
    </div>

    <div
        class={`
            absolute right-1 top-1
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

<ContextMenu bind:this={statusContextMenu}>
    {#each statusList.keys() as k, i}
        <ContextMenuCustomItem
            onclick={() => {
                invoke('bot_set_status', { status: i });
            }}
        >
            <div class="flex items-center ml-1">
                <div
                    class="w-3 h-3 rounded-full mr-2"
                    style="background-color: {statusList.get(k)![1]};"
                ></div>

                {statusList.get(k)![0]}
            </div>
        </ContextMenuCustomItem>
    {/each}
</ContextMenu>

<Dialog bind:this={dialog} title="Change Username">
    <div class="w-90 flex flex-col justify-center">
        <p class="text-center mb-3 mt-0">
            Enter a new username for this bot. Note that it may take a while to see the new username in Discord.
        </p>

        <TextInput 
            bind:this={usernameTextInput}
            bind:value={username} 
            placeholder="Username"
            fastValidation={false}
        />

        <div class="w-full flex justify-end mt-3">
            <Button text="Submit" onclick={async () => {
                await invoke("bot_set_username", { username: username }).then(async () => {
                    usernameTextInput.setError(null);
                    dialog.close();
                }).catch((e) => {
                    usernameTextInput.setError(e);
                })
            }}/>
        </div>
    </div>
</Dialog>