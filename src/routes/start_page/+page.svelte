<script lang="ts">
  
  import Button from "../../components/button.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import LoadingSpinner from "../../components/loading_spinner.svelte";
  import AddBotDialog from "./add_bot_dialog.svelte";
  import { type BotAccount } from "./bot_account";
  import BotCard from "./bot_card.svelte";

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
  relative w-185 h-135 select-none
  border-1 rounded-2xl flex
  border-gray-950 bg-gray-900 
`}>
  {#await loadBots()}
    <div class="w-full h-full flex">
      <div class="w-10 h-10">
        <LoadingSpinner />
      </div>
    </div>
  {:then _}
    <div class="absolute mx-4.5 my-4.5 right-0 !font-oversteer text-primary-500 text-2xl">
      UMBRAGEbot
    </div>
    <div class="absolute mx-3 my-3">
      <Button text="New Bot" icon="add" onclick={() => {
        addBotDialog.open();
      }} />
      <AddBotDialog bind:this={addBotDialog} {loadBots}/>
    </div>

    <div class={`
      relative mt-18 mb-2 pb-20 pt-4 pl-1 overflow-y-scroll
      flex-1 flex justify-center items-center gap-4 flex-wrap
      inset-shadow-[0_10px_10px_var(--color-gray-900)]
      [mask-image:linear-gradient(transparent_0%,#000_3%,#000_97%,transparent_100%)]
    `}>
      {#if botAccounts.length == 0}
          <p class="text-4xl text-gray-50">
            It's quite empty in here
          </p>
          <p class="text-lg text-gray-300 mb-2">
            Time to enslave some bots!
          </p>
        
      {:else}
        {#each botAccounts as account}
          <BotCard {account}/>
        {/each}
      {/if}
    </div>
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