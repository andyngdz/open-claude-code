export enum TModelField {
  Launch = "modelId",
  Fable = "fable",
  Opus = "opus",
  Sonnet = "sonnet",
  Haiku = "haiku",
}

export enum TDashboardAlertStatus {
  Danger = "danger",
}

export enum TExtendedField {
  ModelId = "extendedModelId",
  Fable = "extendedFable",
  Opus = "extendedOpus",
  Sonnet = "extendedSonnet",
  Haiku = "extendedHaiku",
}

export type TAliasField = Exclude<TModelField, TModelField.Launch>

export const MODEL_FAMILIES: TAliasField[] = [
  TModelField.Fable,
  TModelField.Opus,
  TModelField.Sonnet,
  TModelField.Haiku,
]

export const EXTENDED_FIELDS = {
  [TModelField.Fable]: TExtendedField.Fable,
  [TModelField.Opus]: TExtendedField.Opus,
  [TModelField.Sonnet]: TExtendedField.Sonnet,
  [TModelField.Haiku]: TExtendedField.Haiku,
} satisfies Record<TAliasField, TExtendedField>

export const LAUNCH_COMMAND = "open-claude-code launch"

export const DEFAULT_MODEL_LABEL = "Default model"

export const MODEL_SEARCH_PLACEHOLDER = "Search models"

export const MODEL_SEARCH_EMPTY = "No matching models"

export const ALIAS_LABELS = {
  [TModelField.Fable]: "Fable",
  [TModelField.Opus]: "Opus",
  [TModelField.Sonnet]: "Sonnet",
  [TModelField.Haiku]: "Haiku",
} satisfies Record<TAliasField, string>

export const AUTOSAVE_SAVED_TOAST = "Saved"

export const AUTOSAVE_FAILED_TOAST = "Save failed"

export const EXTENDED_CONTEXT_LABEL = "1M context"

export const EXTENDED_CONTEXT_HINT =
  "Only for models that really serve 1M tokens. Past 200K the request fails instead of compacting."

export const STARTUP_TITLE = "Startup"

export const STARTUP_LABEL = "Start Open Claude Code when you log in"

export const STARTUP_HINT =
  "Signs you in at login through your operating system, so the local gateway is already up. Leave it off to start the app yourself."

export const STARTUP_FAILED_TOAST = "Startup setting failed"
