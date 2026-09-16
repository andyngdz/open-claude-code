import { zodResolver } from "@hookform/resolvers/zod"
import { useEffect } from "react"
import { useForm } from "react-hook-form"
import type { SubmitHandler, UseFormReturn } from "react-hook-form"

import {
  API_KEY_FORM_DEFAULTS,
  apiKeyFormDefaults,
} from "@/features/dashboard/constants/dashboardDefaults"
import {
  apiKeySchema,
  TConnectionStatus,
  TPendingAction,
  type IApiKeyForm,
  type IDashboardSnapshot,
} from "@/features/dashboard/schemas/dashboard.schema"
import type { ValueChanged } from "@/types"

interface IUseConnectionSectionReturn extends UseFormReturn<IApiKeyForm> {
  isBusy: boolean
  isConnected: boolean
  onSubmit: SubmitHandler<IApiKeyForm>
}

export const useConnectionSection = (
  snapshot: IDashboardSnapshot,
  apiKey: string,
  pending: TPendingAction,
  onSaveApiKey: ValueChanged<string, Promise<boolean>>,
) => {
  const methods = useForm<IApiKeyForm>({
    resolver: zodResolver(apiKeySchema),
    defaultValues: API_KEY_FORM_DEFAULTS,
  })

  useEffect(() => {
    methods.reset(apiKeyFormDefaults(apiKey))
  }, [apiKey, methods])

  const onSubmit: SubmitHandler<IApiKeyForm> = async (values) => {
    await onSaveApiKey(values.apiKey)
  }

  return {
    ...methods,
    isBusy: pending !== TPendingAction.None,
    isConnected: snapshot.connection.status === TConnectionStatus.Connected,
    onSubmit,
  } satisfies IUseConnectionSectionReturn
}
