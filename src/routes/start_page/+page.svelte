<script lang="ts">
  import TitleBar from "../../components/title_bar.svelte";
  import TextButton from "../../components/text_button.svelte";
    import Button from "../../components/button.svelte";
</script>
  
<TitleBar />
<main class="container" style="font-size: 1em;">
  <div><Button /></div>
  <p>Find more info by pressing this <TextButton />.</p>
</main>

<style>
  .container {
    display: flex;
  }
  div {
    display: block;
  }
</style>