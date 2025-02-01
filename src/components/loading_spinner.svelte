<script lang="ts">
  let {
    svgCLass = 'stroke-primary-500'
  } = $props();
</script>

<div class="loader">
  {#each {length: 2} as _, index}
    <svg
      class="{svgCLass}"
      viewBox="0 0 200 173.20508075688772"
      ><path
        fill="transparent"
        d="M0 86.60254037844386L50 0L150 0L200 86.60254037844386L150 173.20508075688772L50 173.20508075688772Z"
      ></path></svg
    >
  {/each}
  
</div>

<style>
  .loader {
    display: flex;
    justify-content: center;
    align-items: center;
    position: relative;
    
    width: 100%;
    height: 100%;
  }

  svg {
    margin: auto;
    width: 100%;
    height: 100%;
    position: absolute;
    overflow: visible;

    animation: a 0.4s infinite linear;
    stroke-width: 15px;
    stroke-dasharray: 100;
    stroke-dashoffset: 0;

    filter: drop-shadow( 0px 3px 2px rgba(0, 0, 0, 1));
  }

  svg:nth-of-type(2) {
    width: 50%;
    height: 50%;
    stroke-width: 30px;
    stroke-dasharray: 100;
    animation-direction: reverse;
  }

  @keyframes a {
    0% {
      stroke-dashoffset: 0;
    }
    100% {
      stroke-dashoffset: 200;
    }
  }
</style>