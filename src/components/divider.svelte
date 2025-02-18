<script lang="ts">
    let {
        css = "",
        visible = true
    } = $props();
</script>
<div class="w-full h-0 duration-200 {visible ? "opacity-100" : "opacity-0"}">
    <div class={`w-full max-h-0 m-0 p-0 border-b-1 border-b-gray-600 ${css}`}></div>
</div>
