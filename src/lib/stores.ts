import { writable, type Writable } from "svelte/store";
import { type Modlist } from "./schema";

export const chromeRunning = writable(false);
export const nexusLoggedIn = writable(false);
export const loversLoggedIn = writable(false);
export const errorMsg = writable("");
