<script lang="ts">
  import { open } from "@tauri-apps/plugin-shell";
  import Button from "../../components/button.svelte";
  import Dialog from "../../components/dialog.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import LoadingSpinner from "../../components/loading_spinner.svelte";
  import TextButton from "../../components/text_button.svelte";
  import TextInput from "../../components/text_input.svelte";

  async function loadBots() {
    await invoke('get_all_accounts').then((result) => {
      botAccounts = result;
    });
  }

  let botAccounts = null;
  let addDialog: Dialog;
</script>

<div class={`maindiv shadow-container
  absolute w-185 h-135 inset-0 select-none
  border-1 rounded-2xl m-auto 
  border-gray-950 bg-gray-900 
`}>
  {#await loadBots()}
    <div class="w-full h-full flex items-center justify-center">
      <div class="w-10 h-10">
        <LoadingSpinner />
      </div>
    </div>
  {:then _}
    <div class="absolute mx-3 my-3">
      <Button text="New Bot" icon="add" onclick={() => {
        addDialog.open();
      }} />
      <Dialog bind:this={addDialog} title="Add new bot">
        <div class="w-90 flex flex-col justify-center">
          <p class="text-center mb-3 mt-0">
            Go to <TextButton text="Discord Developer Portal" onclick={() => {
              open("https://discord.com/developers/applications");
            }}/> and create a new application. In the 'Bot' section, take the generated token and paste it down below to register your bot.
          </p>

          <TextInput placeholder="Token"/>

          <div class="w-full flex justify-end mt-3">
            <Button text="Add Bot" />
          </div>
        </div>
      </Dialog>
    </div>

    {#if botAccounts.length == 0}
      <div class="w-full h-full flex flex-col items-center justify-center">
        <p class="text-4xl text-gray-50">
          It's quite empty in here
        </p>
        <p class="text-lg text-gray-300 mb-2">
          Time to enslave some bots!
        </p>
      </div>
    {:else}
      <div>
        
      </div>
    {/if}
  {/await}
</div>



<style>
  .maindiv {
    animation: fade 0.5s ease-in-out;
  } 

  @keyframes fade {
    from {
      transform: translateY(10px);
      opacity: 0;
    }

    to {
      transform: translateY(0px);
      opacity: 1;
    }
  }
</style>