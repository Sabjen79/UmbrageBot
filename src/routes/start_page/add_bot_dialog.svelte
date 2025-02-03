<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Button from "../../components/button.svelte";
    import Dialog from "../../components/dialog.svelte";
    import TextButton from "../../components/text_button.svelte";
    import TextInput from "../../components/text_input.svelte";
    import { open as openExternal } from "@tauri-apps/plugin-shell";

    let {
        loadBots = async () => {}
    } = $props();

    let dialog: Dialog;
    let token = $state("");
    let tokenValidated = $state(false);

    export function open() {
        token = "";
        dialog.open();
    }
</script>

<Dialog bind:this={dialog} title="Add new bot">
    <div class="w-90 flex flex-col justify-center">
        <p class="text-center mb-3 mt-0">
            Go to <TextButton text="Discord Developer Portal" onclick={() => {
                openExternal("https://discord.com/developers/applications");
            }}/> and create a new application. In the 'Bot' section, take the generated token and paste it down below to register your bot.
        </p>

        <TextInput bind:value={token} bind:validated={tokenValidated} placeholder="Token" validationType="token"/>

        <div class="w-full flex justify-end mt-3">
            <Button text="Add Bot" disabled={!tokenValidated} onclick={async () => {
                await invoke("insert_account", {token: token}).then(async () => {
                    await loadBots();
                    dialog.close();
                })
            }}/>
        </div>
    </div>
</Dialog>