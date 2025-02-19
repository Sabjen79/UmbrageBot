<script lang="ts">
  let {
    svgClass = 'stroke-primary-500'
  } = $props();
</script>

<div class={`
  relative flex justify-center items-center
  w-full h-full
`}>
  {#each {length: 2} as _, index}
    <svg
      class={`
        ${svgClass}
        m-auto absolute overflow-visible
        animate-loading [stroke-dasharray:100]
        [stroke-dashoffset:0] drop-shadow-[0px_3px_2px_rgba(0,0,0,1)]
        ${index == 0
          ? "w-full h-full stroke-[15px]"
          : "w-[50%] h-[50%] stroke-[30px] [animation-direction:reverse]"
        }
      `}
      viewBox="0 0 200 173.20508075688772"
      ><path
        fill="transparent"
        d="M0 86.60254037844386L50 0L150 0L200 86.60254037844386L150 173.20508075688772L50 173.20508075688772Z"
      ></path></svg
    >
  {/each}
  
</div>