export enum TProviderId {
  OpenCode = "opencode",
  Codex = "codex",
  Grok = "grok",
  Cursor = "cursor",
  Gemini = "gemini",
}

export const PROVIDERS = [
  TProviderId.OpenCode,
  TProviderId.Codex,
  TProviderId.Grok,
  TProviderId.Cursor,
  TProviderId.Gemini,
]

export const PROVIDER_LABELS = {
  [TProviderId.OpenCode]: "OpenCode Go",
  [TProviderId.Codex]: "Codex",
  [TProviderId.Grok]: "Grok",
  [TProviderId.Cursor]: "Cursor",
  [TProviderId.Gemini]: "Gemini",
} satisfies Record<TProviderId, string>

export const providerUnavailableCopy = (label: string) => {
  return `Sign-in for ${label} is not in this build.`
}
