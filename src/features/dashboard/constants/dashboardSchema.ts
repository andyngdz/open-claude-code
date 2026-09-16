import { z } from "zod"

export enum TConnectionStatus {
  Disconnected = "disconnected",
  Connected = "connected",
  Failed = "failed",
}

export enum TTerminalKind {
  SystemDefault = "system_default",
  Ghostty = "ghostty",
  GnomeTerminal = "gnome_terminal",
  Konsole = "konsole",
  Kitty = "kitty",
  Alacritty = "alacritty",
}

export enum TDashboardStatus {
  Loading = "loading",
  Ready = "ready",
  Failed = "failed",
}

export enum TPendingAction {
  None = "none",
  SavingKey = "saving-key",
  Refreshing = "refreshing",
  Disconnecting = "disconnecting",
  SavingSettings = "saving-settings",
  Launching = "launching",
}

const modelSchema = z.object({
  id: z.string(),
  displayName: z.string(),
  isCustom: z.boolean(),
})

const connectionSchema = z.discriminatedUnion("status", [
  z.object({ status: z.literal(TConnectionStatus.Disconnected) }),
  z.object({ status: z.literal(TConnectionStatus.Connected) }),
  z.object({
    status: z.literal(TConnectionStatus.Failed),
    message: z.string(),
  }),
])

const terminalOptionSchema = z.object({
  kind: z.nativeEnum(TTerminalKind),
  label: z.string(),
  isAvailable: z.boolean(),
})

export const dashboardSnapshotSchema = z.object({
  connection: connectionSchema,
  models: z.array(modelSchema),
  customModels: z.array(z.string()),
  aliases: z.object({
    fable: z.string(),
    opus: z.string(),
    sonnet: z.string(),
    haiku: z.string(),
  }),
  terminal: z.nativeEnum(TTerminalKind),
  terminals: z.array(terminalOptionSchema),
  lastWorkspace: z.string().nullable(),
  catalogRefreshedAtEpochSeconds: z.number().nullable(),
})

export const apiKeySchema = z.object({
  apiKey: z.string().trim().min(1, "Enter an API key."),
})

export const launchFormSchema = z.object({
  terminal: z.nativeEnum(TTerminalKind),
  workspace: z.string(),
  modelId: z.string().trim().min(1, "Choose a model."),
  fable: z.string().trim().min(1, "Choose a Fable model."),
  opus: z.string().trim().min(1, "Choose an Opus model."),
  sonnet: z.string().trim().min(1, "Choose a Sonnet model."),
  haiku: z.string().trim().min(1, "Choose a Haiku model."),
  customModels: z.string(),
})

export type IDashboardSnapshot = z.infer<typeof dashboardSnapshotSchema>
export type IApiKeyForm = z.infer<typeof apiKeySchema>
export type ILaunchForm = z.infer<typeof launchFormSchema>
export type IModelOption = z.infer<typeof modelSchema>

export const parseCustomModels = (value: string) => {
  return value
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line)
}

export const ALIAS_LABELS = {
  fable: "Fable",
  opus: "Opus",
  sonnet: "Sonnet",
  haiku: "Haiku",
} as const

export const connectionLabel = (snapshot: IDashboardSnapshot) => {
  if (snapshot.connection.status === TConnectionStatus.Connected) return "Connected"
  if (snapshot.connection.status === TConnectionStatus.Failed) return snapshot.connection.message
  return "Not connected"
}
