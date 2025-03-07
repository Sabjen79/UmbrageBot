import { listen } from "@tauri-apps/api/event";
import { writable } from "svelte/store";

export type BotProfile = {
    username: string
}

export let botProfile = writable<BotProfile>({
    username: ""
});

await listen<BotProfile>("bot_user_update", (event) => {
    botProfile.set(event.payload);
})