<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Button from "../../components/button.svelte";
    import ContextMenu from "../../components/context_menu/context_menu.svelte";
    import ContextMenuItem from "../../components/context_menu/context_menu_item.svelte";
    import Dialog from "../../components/dialog.svelte";
    import IconButton from "../../components/icon_button.svelte";
    import { activeBot, refreshBots, type BotAccount } from "./bot_accounts";
    import AddBotDialog from "./add_bot_dialog.svelte";

    let {
        account,
    }: {
        account: BotAccount;
    } = $props();

    let hover = $state();
    let pressed = $state();

    let contextMenu: ContextMenu;

    // svelte-ignore non_reactive_update
    let deleteDialog: Dialog;

    // svelte-ignore non_reactive_update
    let updateDialog: AddBotDialog;
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div 
    class={`
        relative w-35 h-48 duration-200 ease-in-out rounded-xl 
        
        ${pressed ? "bg-gray-700 shadow-container -translate-y-0.25" 
        : hover ? "bg-gray-700/60 shadow-container-xl -translate-y-0.5" 
        : "bg-gray-800 shadow-container translate-y-0"}
    `}
    onmouseenter={() => { hover = true }}
    onmouseleave={() => { hover = false; pressed = false }}
    onmouseup={() => {pressed = false}}
>
    <button
        oncontextmenu={(event) => {
            contextMenu.open(event);
        }}
        onmousedown={() => { pressed = true }}
        onmouseup={(event) => { 
            if(event.button != 0) return;
            
            $activeBot = account;
        }}
        class={`
            flex flex-col items-center
            z-0 w-full h-full
            ${hover ? "cursor-pointer" : ""}
        `}
    >   
        <div class={`relative
            rounded-full w-[80%] mt-[10%] inset-shadow-[0_0_10px_rgba(0,0,0,0.5)]
            overflow-hidden
        `}>
            <img class="-z-2 relative pointer-events-none" 
                alt={account.name} 
                src={account.avatarUrl}
            />
        </div>
        <p class="flex-1 h-full align-middle flex items-center">
            {account.name}
        </p>
    </button>
    
    <div
        class={`
            absolute top-1.5 right-1.5
        `}
    >
        <IconButton icon="more_vert" onclick={(e: MouseEvent) => {
            contextMenu.open(e);
        }}/>
    </div>
</div>

<Dialog bind:this={deleteDialog} title="Delete Bot">
    <p class="mr-10 mb-2">
        Are you sure you want to remove <b>{account.name}</b>?<br>
        All its data will be deleted forever.
    </p>
    
    <div class="flex w-full justify-end">
        <Button isRed={true} text="Goodbye ;-;" onclick={async () => {
            deleteDialog.block();
            await invoke("delete_account", {id: account.id});
            deleteDialog.unblock();
            deleteDialog.close();
            await refreshBots();
        }}/>
    </div>
</Dialog>

<AddBotDialog bind:this={updateDialog} botAccount={account}/>

<ContextMenu bind:this={contextMenu}>
    <ContextMenuItem text="Edit Token" icon="edit" onclick={() => updateDialog.open()}/>
    <ContextMenuItem text="Delete" icon="delete" isRed={true} onclick={() => deleteDialog.open()}/>
</ContextMenu>