import type { IApiKeyForm } from "@/features/dashboard/schemas/dashboard.schema"

export const API_KEY_FORM_DEFAULTS = { apiKey: "" } satisfies IApiKeyForm

export const apiKeyFormDefaults = (apiKey: string) => {
  return { apiKey } satisfies IApiKeyForm
}
