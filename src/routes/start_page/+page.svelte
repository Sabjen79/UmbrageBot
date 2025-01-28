<script lang="ts">
  import TitleBar from "../../components/title_bar.svelte";
  import TextButton from "../../components/text_button.svelte";
  import Button from "../../components/button.svelte";
  import Checkbox from "../../components/checkbox.svelte";
  import { open } from '@tauri-apps/plugin-shell';
    import ToggleSlider from "../../components/toggle_slider.svelte";
    import TextInput from "../../components/text_input.svelte";

  let checked = $state(false);
  let toggled = $state(false);
  let text = $state("oks");
</script>
  
<TitleBar />
<main class="container" style="font-size: 1em;">
  <div><Button text="Hello!" icon="play_arrow"/></div>
  <div>Find more info by pressing this <TextButton onclick={() => {
    open("https://discord.com/developers/applications");
  }}/>.</div>
  <div class="flex"><Checkbox bind:checked /> WooooW!</div>
  <div><ToggleSlider bind:toggled/></div>
  <div style="width: 150px;">
    <TextInput bind:value={text} validation="tye"/>
  </div>
</main>

<style>
  .container {
    display: flex;
    margin-left: 10px;
    flex-direction: column;
  }

  div {
    margin: 5px 0;
  }

  .flex {
    display: flex;
    align-items: center;
  }
</style>