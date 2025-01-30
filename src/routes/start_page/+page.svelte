<script lang="ts">
  import TitleBar from "../../components/title_bar.svelte";
  import Button from "../../components/button.svelte";
  import Dialog from "../../components/dialog.svelte";
</script>
  
<TitleBar />
<main class="container" style="font-size: 1em;">
  <div>
    <Button text="New Bot" icon="add"/>
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
</style>