<script lang="ts">
    import { goto } from "$app/navigation";
    import type { BotAccount } from "../start_page/bot_accounts";
    import { page } from '$app/state';  
  

    let {
        icon,
        text,
        href,
    } = $props();

    let hover = $state(false);
    let pressed = $state(false);
    let active = $derived(page.url.pathname == href);
    
</script>

<button
    onmouseenter={() => { hover = true }}
    onmouseleave={() => { hover = false; pressed = false }}
    onmousedown={() => { pressed = true }}
    onmouseup={() => { pressed = false }}
    onclick={async () => {
        await goto(href);
        console.log(page.url.pathname);
    }}
    class={`
        w-40 flex items-center my-5
        ${hover ? "cursor-pointer" : ""}
        ${active || pressed ? "text-primary-600" : hover ? "text-primary-400" : "text-gray-100"}
    `}
>

    <div class={`
        font-icons ![font-variation-settings:'FILL'_1]
        text-3xl w-10 text-center duration-150
        ${hover && !pressed || active ? "scale-120" : "scale-100"}
    `}>
        {icon}
    </div>
    
    <div class={`
        text-lg hidden xl:inline
        duration-150 font-semibold
        ${hover && !pressed || active ? "ml-1" : "ml-0"}
        ${active || pressed ? "text-primary-500" : hover ? "text-primary-400" : "text-gray-100"}
    `}>
        {text}
    </div>
</button>