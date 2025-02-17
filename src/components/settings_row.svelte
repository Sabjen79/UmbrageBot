<script lang="ts">
    let {
        title,
        description,
        children
    } = $props();


</script>

<div
    class={`
        flex justify-between items-center
        mx-4 my-2
    `}
>
    <div class="flex flex-col justify-center">
        <p>
            {title}
        </p>
        <p>
            {description}
        </p>
    </div>
    {@render children()}
</div>