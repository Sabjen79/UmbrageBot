<script lang="ts">
    import { activeBot } from "../../start_page/bot_accounts";


</script>

<div
    class={`
        w-full h-40 bg-gray-800 rounded-md shadow-container
        flex items-center
    `}
>
    <div
        class={`
            rounded-full w-30 h-30 inset-shadow-[0_0_10px_rgba(0,0,0,0.5)]
            overflow-hidden mx-5
        `}
    >
        <img class="pointer-events-none" 
            alt={$activeBot?.name} 
            src={$activeBot?.avatarUrl}
        />
    </div>

    <div
        class={`
            flex-1 flex flex-col justify-center
        `}
    >
        <p class="font-semibold text-3xl">
            {$activeBot?.name}
        </p>

    </div>
    
</div>