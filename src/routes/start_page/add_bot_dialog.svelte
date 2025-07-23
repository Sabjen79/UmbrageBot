<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Button from "../../components/button.svelte";
    import Dialog from "../../components/dialog.svelte";
    import TextButton from "../../components/text_button.svelte";
    import TextInput from "../../components/text_input.svelte";
    import { open as openExternal } from "@tauri-apps/plugin-shell";
    import { refreshBots, type BotAccount } from "../../stores/bot_accounts_store";

    let { 
        botAccount = null // Used for update mode
    }: {
        botAccount?: BotAccount | null
    } = $props();

    let dialog: Dialog;
    let token = $state("");
    let tokenValidated = $state(false);
    let textInput: TextInput;

    export function open() {
        token = "";
        dialog.open();
    }
</script>

<Dialog bind:this={dialog} title={botAccount == null ? "Add New Bot" : "Update token"}>
    <div class="w-90 flex flex-col justify-center">
        <p class="text-center mb-3 mt-0">
            {#if botAccount == null}
                Go to <TextButton text="Discord Developer Portal" link="https://discord.com/developers/applications"/>
                and select your bot. In the 'Bot' section, take the generated token and paste it down below to register your bot.
            {:else}
                Go to <TextButton text="Discord Developer Portal" link="https://discord.com/developers/applications"/> 
                and select <b>{botAccount?.name}</b>. Refresh its token and insert it below.
            {/if}
        </p>

        <TextInput 
            bind:this={textInput}
            bind:value={token} 
            placeholder="Token"
            fastValidation={true}
            validation={async () =>  {
                tokenValidated = false;

                return await invoke('db_validate_token', {token: token}).then(() => {
                    tokenValidated = true;
                    return null;
                }).catch((error) => {
                    tokenValidated = false;
                    return error;
                });
            }} 
        />

        <div class="w-full flex justify-end mt-3">
            {#if botAccount == null}
                <Button text="Add Bot" disabled={!tokenValidated} onclick={async () => {
                    await invoke("db_insert_account", {token: token}).then(async () => {
                        await refreshBots();
                        dialog.close();
                    }).catch((e) => {
                        textInput.setError(e);
                    });
                }}/>
            {:else}
                <Button text="Update Token" disabled={!tokenValidated} onclick={async () => {
                    await invoke("db_update_account", {id: botAccount.id, newToken: token}).then(async () => {
                        await refreshBots();
                        dialog.close();
                    }).catch((e) => {
                        textInput.setError(e);
                    });
                }}/>
            {/if}
            
        </div>
    </div>
</Dialog>