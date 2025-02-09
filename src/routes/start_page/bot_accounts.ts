import { invoke } from '@tauri-apps/api/core';
import { writable } from 'svelte/store';

export type BotAccount = {
    id: string,
    token: string,
    name: string,
    avatarUrl: string
};

export let allBots = writable<BotAccount[]>([]);

export async function refreshBots() {
    await invoke('get_all_accounts').then((result) => {
        allBots.set(result as BotAccount[])
    });
}