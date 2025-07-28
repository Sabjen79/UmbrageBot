import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";

export type DbActivity = {
    id: string,
    type_num: number,
    content: string,
    url: string
}

export let dbActivities = writable<DbActivity[]>([]);

export async function refreshActivities() {
    await invoke('db_get_all_activities').then((result) => {
        dbActivities.set(result as DbActivity[])
    });
}