<script lang="ts">
  import { redirect } from "@sveltejs/kit";
    import { onMount } from "svelte";

  onMount(() => {
    redirect(302, '/start_page/');
  })
</script>
