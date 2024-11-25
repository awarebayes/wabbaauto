import { z } from "zod";
import {
  LoginStatusSchema,
  ModlistLongSchema,
  ModlistSchema,
  type Modlist,
  type ModlistLong,
} from "./schema";
import { fetch } from "@tauri-apps/plugin-http";
import { invoke } from "@tauri-apps/api/core";
import { chromeRunning, nexusLoggedIn, errorMsg } from "./stores";

export async function getModlists(): Promise<Modlist[]> {
  let modlistsReq = await fetch(
    "https://raw.githubusercontent.com/wabbajack-tools/mod-lists/refs/heads/master/reports/modListSummary.json",
  );
  let modlistsJson = await modlistsReq.json();
  let result = z.array(ModlistSchema).safeParse(modlistsJson);
  if (!result.success) {
    errorMsg.set(
      "Zod schema error (possible GitHub api change by Wabbajack): " +
        result.error.toString(),
    );
    return [];
  }
  return result.data;
}

export async function checkBrowser() {
  let login = (await invoke("check_nexus_login")) as string;
  let loginJson = await JSON.parse(login);
  let result = LoginStatusSchema.safeParse(loginJson);
  if (!result.success) {
    errorMsg.set("Zod schema error (WTF?): " + result.error.toString());
    return;
  }
  let loginStatus = result.data;
  console.log(loginStatus);
  chromeRunning.set(true);
  nexusLoggedIn.set(loginStatus.website);
  errorMsg.set(loginStatus.error);
}

export async function getModistLong(
  machineURL: string,
): Promise<ModlistLong | null> {
  let modlistQuery = await fetch(
    `https://raw.githubusercontent.com/wabbajack-tools/mod-lists/refs/heads/master/reports/${machineURL}/status.json`,
  );
  let modlistJson = await modlistQuery.json();
  let result = ModlistLongSchema.safeParse(modlistJson);
  if (!result.success) {
    errorMsg.set("Zod schema error (WTF?): " + result.error.toString());
    return null;
  }
  errorMsg.set("");
  return result.data;
}
