<script>
    import Sidebar from './sidebar.svelte';

	let { children } = $props();
</script>

<main class="absolute inset-2 top-8 overflow-hidden z-1">

    <div class={`
        absolute inset-0 left-12 xl:left-35
        flex items-center justify-center overflow-clip
        bg-gray-900 rounded-lg shadow-container
    `}>
        <div class={`
            fade absolute flex flex-col
            inset-0 overflow-y-scroll py-4 px-3 max-w-300 m-auto
        `}>
            {@render children()}
        </div>
    </div>
    
    <div class="fade">
        <Sidebar />
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