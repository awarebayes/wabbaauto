import { persisted } from "svelte-persisted-store";
import { writable, type Writable } from "svelte/store";
import { type Modlist } from "./schema";
export const modlists: Writable<Modlist[]> = persisted("modlists", []);

export const chromeRunning = writable(false);
export const nexusLoggedIn = writable(false);
export const loversLoggedIn = writable(false);
export const errorMsg = writable("");
