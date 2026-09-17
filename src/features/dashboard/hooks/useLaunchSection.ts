import { zodResolver } from "@hookform/resolvers/zod"
import { useEffect, useRef } from "react"
import { useForm, useWatch } from "react-hook-form"
import type { UseFormReturn } from "react-hook-form"

import {
  launchFormSchema,
  TPendingAction,
  type IDashboardSnapshot,
  type ILaunchForm,
  type IModelOption,
} from "@/features/dashboard/schemas/dashboard.schema"
import {
  launchFormDefaults,
  launchSettingsFingerprint,
  modelOptions,
} from "@/features/dashboard/services/dashboardFormatters"
import type { ValueChanged } from "@/types"

interface IUseLaunchSectionReturn extends UseFormReturn<ILaunchForm> {
  isBusy: boolean
  options: IModelOption[]
}

const AUTO_SAVE_DELAY_MS = 350

export const useLaunchSection = (
  snapshot: IDashboardSnapshot,
  pending: TPendingAction,
  onSaveSettings: ValueChanged<ILaunchForm, Promise<boolean>>,
) => {
  const methods = useForm<ILaunchForm>({
    resolver: zodResolver(launchFormSchema),
    defaultValues: launchFormDefaults(snapshot),
  })
  const isBusy = pending !== TPendingAction.None
  const savedSettings = useRef(launchSettingsFingerprint(launchFormDefaults(snapshot)))
  const watchedValues = useWatch({ control: methods.control })

  useEffect(() => {
    const defaults = launchFormDefaults(snapshot)
    savedSettings.current = launchSettingsFingerprint(defaults)
    methods.reset(defaults)
  }, [methods, snapshot])

  useEffect(() => {
    const parsed = launchFormSchema.safeParse(watchedValues)
    if (!parsed.success) return
    const fingerprint = launchSettingsFingerprint(parsed.data)
    if (fingerprint === savedSettings.current) return

    const timer = window.setTimeout(() => {
      void onSaveSettings(parsed.data).then((saved) => {
        if (saved) savedSettings.current = fingerprint
      })
    }, AUTO_SAVE_DELAY_MS)
    return () => {
      window.clearTimeout(timer)
    }
  }, [onSaveSettings, watchedValues])

  return {
    ...methods,
    isBusy,
    options: modelOptions(snapshot),
  } satisfies IUseLaunchSectionReturn
}
