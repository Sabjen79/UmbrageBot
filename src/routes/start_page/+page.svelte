<script lang="ts">
  import TitleBar from "../../components/title_bar.svelte";
  import Button from "../../components/button.svelte";
  import Dialog from "../../components/dialog.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import LoadingSpinner from "../../components/loading_spinner.svelte";

  async function loadBots() {
    await invoke('get_all_accounts').then((result) => {
      botAccounts = result;
    });

    
  }

  let botAccounts = null;
</script>

<div class={`maindiv
  absolute w-185 h-135 inset-0
  border-1 rounded-2xl m-auto 
  border-(--gray-950) bg-(--gray-900) shadow-[0_2px_10px_rgba(0,0,0,0.5)] inset-shadow-[0_1px_1px_rgba(255,255,255,.05)]
`}>
  {#await loadBots()}
    <div class="w-full h-full flex items-center justify-center">
      <div class="w-10 h-10">
        <LoadingSpinner  color="var(--primary-500)"/>
      </div>
    </div>
  {:then _}
    <div class="absolute mx-3 my-3">
      <Button text="New Bot" icon="add"/>
    </div>

    {#if botAccounts.length == 0}
      <div class="w-full h-full flex flex-col items-center justify-center">
        <p class="text-4xl text-(--gray-50)">
          It's quite empty in here
        </p>
        <p class="text-lg text-(--gray-300) mb-2">
          Time to enslave some bots!
        </p>
      </div>
      
    {:else}
      <div>
        <Button text="New Bot" icon="add"/>
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