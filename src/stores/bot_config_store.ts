import { invoke } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event";
import { writable } from "svelte/store";

type BotConfig = {
    usernameTimerEnabled: boolean,
    usernameTimerInterval: number,
    usernameTimerDate: number
}

export let botConfig = writable<BotConfig>(await get_bot_config());

async function get_bot_config() {
    return await invoke("get_bot_config") as BotConfig;
}

let enableSubscription = true;

await listen<BotConfig>("bot_config_update", (config) => {
    enableSubscription = false;
    botConfig.set(config.payload);
    enableSubscription = true;
})

botConfig.subscribe(async (newConfig) => {
    if(!enableSubscription) return;

    enableSubscription = false;
    await invoke("set_bot_config", {config: newConfig});
    enableSubscription = true;
})