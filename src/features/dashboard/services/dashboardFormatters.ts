import { compact, filter, isError, isString, map, trim } from "es-toolkit/compat"
import { TProviderId } from "@/features/dashboard/constants/dashboardProviders"
import type {
  ICustomModelField,
  IDashboardSnapshot,
  ILaunchForm,
  IModelOption,
} from "@/features/dashboard/schemas/dashboard.schema"
import { TConnectionStatus } from "@/features/dashboard/schemas/dashboard.schema"

export const readProviderId = (value: string) => {
  switch (value) {
    case TProviderId.OpenCode:
      return TProviderId.OpenCode
    case TProviderId.Codex:
      return TProviderId.Codex
    case TProviderId.Grok:
      return TProviderId.Grok
    case TProviderId.Cursor:
      return TProviderId.Cursor
    case TProviderId.Gemini:
      return TProviderId.Gemini
    default:
      return
  }
}

export const customModelIds = (entries: readonly ICustomModelField[]) => {
  return compact(map(entries, (entry) => trim(entry.modelId)))
}

/// Drops blank custom-model drafts so fingerprints match what persistence stores.
export const persistableLaunchForm = (values: ILaunchForm) => {
  return {
    ...values,
    customModels: map(customModelIds(values.customModels), (modelId) => {
      return { modelId } satisfies ICustomModelField
    }),
  } satisfies ILaunchForm
}

export const launchSettingsFingerprint = (values: ILaunchForm) => {
  return JSON.stringify(persistableLaunchForm(values))
}

/// Short connection fact for the status strip. Failures stay in the alert, not this label.
export const connectionStatusLabel = (snapshot: IDashboardSnapshot) => {
  if (snapshot.connection.status === TConnectionStatus.Connected) return "Connected"
  if (snapshot.connection.status === TConnectionStatus.Failed) return "Connection failed"
  return "Not connected"
}

export const launchFormDefaults = (snapshot: IDashboardSnapshot) => {
  return {
    terminal: snapshot.terminal,
    modelId: snapshot.launchModelId,
    fable: snapshot.aliases.fable,
    opus: snapshot.aliases.opus,
    sonnet: snapshot.aliases.sonnet,
    haiku: snapshot.aliases.haiku,
    extendedFable: snapshot.aliases.extended.fable,
    extendedOpus: snapshot.aliases.extended.opus,
    extendedSonnet: snapshot.aliases.extended.sonnet,
    extendedHaiku: snapshot.aliases.extended.haiku,
    customModels: map(snapshot.customModels, (modelId) => {
      return { modelId } satisfies ICustomModelField
    }),
  } satisfies ILaunchForm
}

export const modelOptions = (snapshot: IDashboardSnapshot) => {
  const catalogIds = new Set(map(snapshot.models, (model) => model.id))
  const customOptions = map(
    filter(snapshot.customModels, (modelId) => !catalogIds.has(modelId)),
    (modelId) => {
      return { id: modelId, displayName: modelId, isCustom: true } satisfies IModelOption
    },
  )

  return [...snapshot.models, ...customOptions]
}

export const modelAvailabilityLabel = (snapshot: IDashboardSnapshot) => {
  const availableCount = modelOptions(snapshot).length
  if (availableCount === 1) return "1 available"
  return `${availableCount} available`
}

export const readCommandError = (error: unknown) => {
  if (isString(error)) return error
  if (isError(error)) return error.message
  return "The request failed. Try again."
}
