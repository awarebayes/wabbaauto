import type { ModGetState } from "$lib/schema";
import type { z } from "zod";

function formatBytes(bytes: number): string {
    const sizes = ['B', 'KB', 'MB', 'GB'];
    if (bytes === 0) return '0 B';
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(2)} ${sizes[i]}`;
};
  
export function stateToString(state: z.infer<typeof ModGetState>): string {
switch (state.kind) {
    case "Init":
    return "Initializing";
    
    case "GettingLink": {
    switch (state.state.kind) {
        case "Init":
        return "Preparing to get link";
        case "ConnectedToBrowser":
        return "Connected to browser";
        case "WaitingForLink":
        return "Waiting for link";
        case "Ended":
        return "Link retrieval completed";
    }
    }
    
    case "Downloading": {
    switch (state.state.kind) {
        case "Init":
        return "Preparing download";
        case "Started":
        return "Starting download";
        case "InProgress":
        return `Downloading: ${state.state.state.percent.toFixed(1)}% - ` +
                `${state.state.state.speed_kbps.toFixed(1)} KB/s - ` +
                `${formatBytes(state.state.state.downloaded)} / ${formatBytes(state.state.state.total_size)}`;
        case "Ended":
        return "Download completed";
    }
    }
    
    case "Hashing": {
    switch (state.state.kind) {
        case "Init":
        return "Preparing to verify hash";
        case "Started":
        return "Starting hash verification";
        case "InProgress":
        return `Verifying hash: ${state.state.state.percent.toFixed(1)}%`;
        case "Ended":
        return "Hash verification completed";
    }
    }
    
    case "Ended":
    return "Process completed";
    
    case "Failed": {
    switch (state.state.kind) {
        case "Downloading":
        return "Failed during download";
        case "Hashing":
        return "Failed during hash verification";
        case "Unknown":
        return `Failed: ${state.state.message}`;
    }
    }
}
};
  