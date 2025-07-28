<script lang="ts">
    import { tick } from 'svelte';
    import { botConfig } from '../../stores/bot_config_store';
    import Sidebar from './sidebar.svelte';

	let { children } = $props();
</script>

<main class={`
    w-full h-full mt-12 overflow-hidden
    flex bg-gray-900 shadow-container z-1
`}>
    <div class="fade z-10">
        <Sidebar />
    </div>

    <div class={`
        flex-1 flex items-center justify-center
    `}>
        <div class={`
            h-full w-0 flex-grow min-w-0
            fade overflow-y-scroll
            max-w-250 mx-4 mt-4
        `}>
            {@render children()}
        </div>
    </div>

</main>

<style>
    .fade {
        animation: fade 0.2s ease-in-out;
    } 

    @keyframes fade {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }
</style>