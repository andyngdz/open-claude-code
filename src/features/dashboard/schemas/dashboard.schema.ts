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

export const terminalKindSchema = z.enum(TTerminalKind)

const modelSchema = z.object({
  id: z.string(),
  displayName: z.string(),
  isCustom: z.boolean(),
})

export const customModelFieldSchema = z.object({
  modelId: z.string(),
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
  kind: terminalKindSchema,
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
  launchModelId: z.string(),
  terminal: terminalKindSchema,
  terminals: z.array(terminalOptionSchema),
  lastWorkspace: z.string().nullable(),
  catalogRefreshedAtEpochSeconds: z.number().nullable(),
  gatewayBaseUrl: z.string().min(1),
})

export const apiKeySchema = z.object({
  apiKey: z.string().trim().min(1, "Enter an API key."),
})

export const launchFormSchema = z.object({
  terminal: terminalKindSchema,
  modelId: z.string().trim().min(1, "Choose a model."),
  fable: z.string().trim().min(1, "Choose a Fable model."),
  opus: z.string().trim().min(1, "Choose an Opus model."),
  sonnet: z.string().trim().min(1, "Choose a Sonnet model."),
  haiku: z.string().trim().min(1, "Choose a Haiku model."),
  customModels: z.array(customModelFieldSchema),
})

export type IDashboardSnapshot = z.infer<typeof dashboardSnapshotSchema>
export type IApiKeyForm = z.infer<typeof apiKeySchema>
export type ILaunchForm = z.infer<typeof launchFormSchema>
export type IModelOption = z.infer<typeof modelSchema>
export type ICustomModelField = z.infer<typeof customModelFieldSchema>
