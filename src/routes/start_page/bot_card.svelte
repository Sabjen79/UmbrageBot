<script lang="ts">
    import { type BotAccount } from "./bot_account";

    let {
        account,
    }: {
        account: BotAccount;
    } = $props();

    let hover = $state();
    let pressed = $state();
</script>

<button
    onmouseenter={() => { hover = true }}
    onmouseleave={() => { hover = false; pressed = false }}
    onmousedown={() => { pressed = true }}
    onmouseup={() => { pressed = false }}
    class={`
        flex flex-col items-center
        w-35 h-48 rounded-xl z-0
        duration-200 ease-in-out
        ${hover ? "cursor-pointer" : ""}
        ${pressed ? "bg-gray-700 shadow-container -translate-y-0.25" 
        : hover ? "bg-gray-700/60 shadow-container-xl -translate-y-0.5" 
        : "bg-gray-800 shadow-container translate-y-0"}
    `}
>   

    
    <div class={`relative
        rounded-full w-[80%] mt-[10%] inset-shadow-[0_0_10px_rgba(0,0,0,0.5)]
        overflow-hidden
    `}>
        <img class="-z-2 relative pointer-events-none" 
            alt={account.name} 
            src={account.avatarUrl}
        />
    </div>
    <p class="flex-1 h-full align-middle flex items-center">
        {account.name}
    </p>
</button>
