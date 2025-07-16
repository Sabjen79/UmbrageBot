<script lang="ts">
    import { goto } from "$app/navigation";
    import type { BotAccount } from "../../stores/bot_accounts_store";
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
        flex items-center my-3
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

    <div
        class={`
            fixed left-13 px-2 rounded-md 
            text-md text-gray-100 translate-y-1
            bg-gray-800 shadow-container xl:invisible
            origin-left duration-200 pointer-events-none
            ${hover ? "opacity-100 scale-100" : "opacity-0 scale-50"}
        `}
    >
        {text}
    </div>
    
    <div class={`
        text-lg hidden xl:inline
        duration-150 font-semibold
        ${hover && !pressed || active ? "ml-0.5" : "ml-0"}
        ${active || pressed ? "text-primary-500" : hover ? "text-primary-400" : "text-gray-100"}
    `}>
        {text}
    </div>
</button>