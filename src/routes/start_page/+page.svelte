<script lang="ts">
  
  import Button from "../../components/button.svelte";
  import Dialog from "../../components/dialog.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import LoadingSpinner from "../../components/loading_spinner.svelte";
  import TextButton from "../../components/text_button.svelte";
  import TextInput from "../../components/text_input.svelte";
  import AddBotDialog from "./add_bot_dialog.svelte";

  type BotAccount = {
    id: string,
    token: string,
    name: string,
    avatarUrl: string
  };

  async function loadBots() {
    await invoke('get_all_accounts').then((result) => {
      botAccounts = result as BotAccount[];
    });
  }

  let botAccounts = $state([] as BotAccount[]);

  // svelte-ignore non_reactive_update
  let addBotDialog: AddBotDialog;

  
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
      <Button  text="New Bot" icon="add" onclick={() => {
        addBotDialog.open();
      }} />
      <AddBotDialog bind:this={addBotDialog} {loadBots}/>
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