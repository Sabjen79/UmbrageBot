<script lang="ts">
  
  import Button from "../../components/button.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import LoadingSpinner from "../../components/loading_spinner.svelte";
  import AddBotDialog from "./add_bot_dialog.svelte";
  import { selectedBot, allBots, refreshBots, type BotAccount } from "../../stores/bot_accounts_store";
  import BotCard from "./bot_card.svelte";
  import { goto } from "$app/navigation";
  import { onDestroy } from "svelte";

  // svelte-ignore non_reactive_update
  let addBotDialog: AddBotDialog;

  let unsubscribe = selectedBot.subscribe(async (value) => {
    if(value == null) return;

    try {
      await invoke("start_bot", { token: value.token });
    } catch(err) {
      console.log(err);
      
      await goto("/start_page", { replaceState: true });
      selectedBot.set(null);
      
      return;
    }

    botLoaded = true;
    await new Promise(resolve => setTimeout(resolve, 300));

    await goto("/dashboard/account", { replaceState: true });
  })

  let botLoaded = $state(false);

  onDestroy(() => {
    unsubscribe();
  })
</script>

<div
  class={`
    absolute select-none
    border-1 rounded-2xl flex items-center
    border-gray-950 bg-gray-900
    duration-300 ease-in-out shadow-container
    ${botLoaded 
    ? "left-0 right-0 inset-y-0 top-6" 
    : "inset-x-[calc(50%-120px)] inset-y-[calc(50%-40px)]"}

    ${$selectedBot != null ? "opacity-100 translate-y-0" : "opacity-0 translate-y-2.5"}
  `}
>
  <div class="w-20 h-20 p-5 duration-200 {botLoaded ? "opacity-0" : "opacity-100"}">
    <LoadingSpinner />
  </div>
  <div class="flex-1 text-center pr-5 duration-200 {botLoaded ? "opacity-0" : "opacity-100"}">
    <b>{$selectedBot?.name}</b><br> is waking up...
  </div>
</div>

<div 
class={`maindiv shadow-container
  relative w-185 h-135 select-none
  border-1 rounded-2xl flex
  border-gray-950 bg-gray-900
  duration-300 ease-in-out
  ${$selectedBot != null ? "opacity-0 pointer-events-none -translate-y-2.5" : ""}
`}>
  {#await refreshBots()}

    <div class="w-full h-full flex">
      <div class="w-10 h-10">
        <LoadingSpinner />
      </div>
    </div>

  {:then _}

    <div class="absolute mx-4.5 my-4.5 right-0 font-oversteer text-primary-500 text-2xl [text-shadow:#000_0_0_5px]">
      UMBRAGEbot
    </div>

    <div class="absolute mx-3 my-3">
      <Button text="New Bot" icon="add" onclick={() => {
        addBotDialog.open();
      }} />
      <AddBotDialog bind:this={addBotDialog}/>
    </div>

    <div class={`
      relative mt-18 mb-2 pb-20 pt-4 pl-1 overflow-y-scroll
      flex-1 flex justify-center items-center gap-4 flex-wrap
      inset-shadow-[0_10px_10px_var(--color-gray-900)]
      ${$allBots.length == 0 ? "flex-col" : ""}
    `}>
      {#if $allBots.length == 0}
          <p class="text-4xl text-gray-50">
            It's quite empty in here
          </p>
          <p class="text-lg text-gray-300 mb-2">
            Time to enslave some bots!
          </p>
        
      {:else}
        {#each $allBots as account}
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