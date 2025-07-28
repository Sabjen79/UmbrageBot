import { listen } from "@tauri-apps/api/event";
import { writable } from "svelte/store";

// TODO: UNLISTEN FOR STORES
export type BotProfile = {
    username: string,
    avatar_url: string,
    banner_url: string,
    status: string,
    activity?: ActivityWrapper,
}

type ActivityWrapper = {
    Playing: string,
    Streaming: string,
    Listening: string,
    Watching: string,
    Competing: string,
    Custom: string,
}

type BotProfileUpdateEvent = {
    data: BotProfile
}

export let botProfile = writable<BotProfile>({
    username: "",
    status: "invisible",
    avatar_url: "",
    banner_url: "",
});

await listen<BotProfileUpdateEvent>("BOT_PROFILE_UPDATE_EVENT", (event) => {
    botProfile.set(event.payload.data);
})