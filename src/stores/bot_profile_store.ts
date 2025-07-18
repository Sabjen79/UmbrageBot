import { listen } from "@tauri-apps/api/event";
import { writable } from "svelte/store";

export type BotProfile = {
    username: string,
    status: string
}

type BotProfileUpdateEvent = {
    data: BotProfile
}

export let botProfile = writable<BotProfile>({
    username: "",
    status: "invisible"
});

await listen<BotProfileUpdateEvent>("BOT_PROFILE_UPDATE_EVENT", (event) => {
    botProfile.set(event.payload.data);
    console.log(event);
})