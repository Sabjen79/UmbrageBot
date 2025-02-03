<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { StateFlags } from "@tauri-apps/plugin-window-state";

    let errorList: String[] = $state([]);

    // TODO: Implement error alerts
</script>

<div
    class={`
        absolute w-full bottom-0
    `}
>
</div>