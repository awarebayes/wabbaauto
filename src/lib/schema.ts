import { z } from "zod";
export const ModlistSchema = z.object({
  name: z.string(),
  machineURL: z.string(),
});

export type Modlist = z.infer<typeof ModlistSchema>;

export const LoginStatusSchema = z.object({
  loverslab: z.boolean(),
  nexusmods: z.boolean(),
  error: z.string(),
});

const ArchiveSchema = z.object({
  Status: z.string(),
  Original: z.object({
    Hash: z.string(),
    Meta: z.string(),
    Name: z.string(),
    Size: z.number().int(),
    State: z.object({
      $type: z.string(),
      Author: z.string().nullish(),
      Name: z.string().nullish(),
      FileID: z.number().nullish(),
      ModID: z.number().nullish(),
    }),
  }),
});

export const ModlistLongSchema = z.object({
  MachineURL: z.string(),
  Name: z.string(),
  Version: z.string(),
  ModListHash: z.string(),
  LargeImage: z.string().nullish(),
  Archives: z.array(ArchiveSchema),
});

export type ModlistLong = z.infer<typeof ModlistLongSchema>;


export const DownloadResponseSchema = z.object({
  success: z.boolean(),
  error: z.string(),
});

export type DownloadResponse = z.infer<typeof DownloadResponseSchema>


const DownloadState = z.discriminatedUnion("kind", [
  z.object({ kind: z.literal("Init") }),
  z.object({ kind: z.literal("Started") }),
  z.object({
    kind: z.literal("InProgress"),
    state: z.object({
      percent: z.number(),
      speed_kbps: z.number(),
      downloaded: z.number(),
      total_size: z.number(),
    }),
  }),
  z.object({ kind: z.literal("Ended") }),
]);

const GettingLinkState = z.discriminatedUnion("kind", [
  z.object({ kind: z.literal("Init") }),
  z.object({ kind: z.literal("ConnectedToBrowser") }),
  z.object({ kind: z.literal("WaitingForLink") }),
  z.object({ kind: z.literal("Ended") }),
]);

const HashState = z.discriminatedUnion("kind", [
  z.object({ kind: z.literal("Init") }),
  z.object({ kind: z.literal("Started") }),
  z.object({
    kind: z.literal("InProgress"),
    state: z.object({
      percent: z.number(),
    })
  }),
  z.object({ kind: z.literal("Ended") }),
]);

const FailedState = z.discriminatedUnion("kind", [
  z.object({ kind: z.literal("Downloading") }),
  z.object({ kind: z.literal("Hashing") }),
  z.object({ 
    kind: z.literal("Unknown"), 
    message: z.string() 
  }),
]);

const ModGetState = z.discriminatedUnion("kind", [
  z.object({ kind: z.literal("Init") }),
  z.object({ 
    kind: z.literal("GettingLink"), 
    state: GettingLinkState 
  }),
  z.object({ 
    kind: z.literal("Downloading"), 
    state: DownloadState 
  }),
  z.object({ 
    kind: z.literal("Hashing"), 
    state: HashState 
  }),
  z.object({ kind: z.literal("Ended") }),
  z.object({ 
    kind: z.literal("Failed"), 
    state: FailedState 
  }),
]);

const AppState = z.object({
  getting_archives: z.array(z.tuple([z.string(), ModGetState])),
  recent_fails: z.array(z.string()),
  total: z.number(),
  failed: z.number(),
  successes: z.number(),
});

type DownloadStateType = z.infer<typeof DownloadState>;
type GettingLinkStateType = z.infer<typeof GettingLinkState>;
type HashStateType = z.infer<typeof HashState>;
type FailedStateType = z.infer<typeof FailedState>;
type ModGetStateType = z.infer<typeof ModGetState>;
type AppStateType = z.infer<typeof AppState>;

// Example usage:
const validateAppState = (data: unknown) => {
  return AppState.parse(data);
};

export {
  DownloadState,
  GettingLinkState,
  HashState,
  FailedState,
  ModGetState,
  AppState,
  type DownloadStateType,
  type GettingLinkStateType,
  type HashStateType,
  type FailedStateType,
  type ModGetStateType,
  type AppStateType,
  validateAppState,
};