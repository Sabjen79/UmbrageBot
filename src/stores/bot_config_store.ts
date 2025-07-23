import { invoke } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event";
import { writable } from "svelte/store";

type BotConfigUpdateEvent = {
    source: string,
    oldConfig: BotConfig,
    newConfig: BotConfig
}

type BotConfig = {
    activityTimerEnabled: boolean,
    activityTimerMin: number,
    activityTimerMax: number,
}

export let botConfig = writable<BotConfig>(await get_bot_config());

async function get_bot_config() {
    return await invoke("get_bot_config") as BotConfig;
}

await listen<BotConfigUpdateEvent>("BOT_CONFIG_UPDATE_EVENT", (event) => {
    if(event.payload.source == "Frontend") return;

    botConfig.set(event.payload.newConfig);
})

botConfig.subscribe(async (config) => {
    await invoke("set_bot_config", {config: config}).catch(async (e) => {
        botConfig.set(await get_bot_config())
    });
})