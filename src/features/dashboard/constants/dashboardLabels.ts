export enum TModelField {
  Launch = "modelId",
  Fable = "fable",
  Opus = "opus",
  Sonnet = "sonnet",
  Haiku = "haiku",
}

export enum TDashboardAlertStatus {
  Danger = "danger",
  Success = "success",
  Accent = "accent",
}

export type TAliasField = Exclude<TModelField, TModelField.Launch>

export const MODEL_FAMILIES: TAliasField[] = [
  TModelField.Fable,
  TModelField.Opus,
  TModelField.Sonnet,
  TModelField.Haiku,
]

export const LAUNCH_COMMAND = "open-claude-code launch"

export const ALIAS_LABELS = {
  [TModelField.Fable]: "Fable",
  [TModelField.Opus]: "Opus",
  [TModelField.Sonnet]: "Sonnet",
  [TModelField.Haiku]: "Haiku",
} satisfies Record<TAliasField, string>
