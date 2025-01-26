<script lang="ts">
  import TitleBar from "../../components/title_bar.svelte";
  import TextButton from "../../components/text_button.svelte";
  import Button from "../../components/button.svelte";
  import Checkbox from "../../components/checkbox.svelte";
  import { open } from '@tauri-apps/plugin-shell';

  let checked = $state(false);
  let checked2 = $state(false);

</script>
  
<TitleBar />
<main class="container" style="font-size: 1em;">
  <div><Button text="Hello!" icon="play_arrow"/></div>
  <div>Find more info by pressing this <TextButton onclick={() => {
    open("https://discord.com/developers/applications");
  }}/>.</div>
  <div><Checkbox bind:checked label="Woooow!"/></div>
  <div><Checkbox bind:checked={checked2} label="Option2" alignRight={true}/></div>
</main>

<style>
  .container {
    display: flex;
    margin-left: 10px;
    flex-direction: column;
  }

  div {
    margin-top: 10px;
  }
</style>