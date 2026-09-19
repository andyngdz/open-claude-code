import { zodResolver } from "@hookform/resolvers/zod"
import { toast } from "@heroui/react"
import { useEffect, useRef } from "react"
import { useForm, useWatch } from "react-hook-form"
import type { UseFormReturn } from "react-hook-form"

import {
  AUTOSAVE_FAILED_TOAST,
  AUTOSAVE_SAVED_TOAST,
} from "@/features/dashboard/constants/dashboardLabels"
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
  persistableLaunchForm,
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
    const snapshotFingerprint = launchSettingsFingerprint(defaults)
    savedSettings.current = snapshotFingerprint
    const parsed = launchFormSchema.safeParse(methods.getValues())
    if (parsed.success && launchSettingsFingerprint(parsed.data) === snapshotFingerprint) return
    methods.reset(defaults)
  }, [methods, snapshot])

  useEffect(() => {
    if (import.meta.env.SSR) return
    const parsed = launchFormSchema.safeParse(watchedValues)
    if (!parsed.success) return
    const persistable = persistableLaunchForm(parsed.data)
    const fingerprint = launchSettingsFingerprint(persistable)
    if (fingerprint === savedSettings.current) return

    let cancelled = false
    const timer = window.setTimeout(() => {
      void onSaveSettings(persistable).then((saved) => {
        if (cancelled) return
        if (saved) {
          savedSettings.current = fingerprint
          toast.success(AUTOSAVE_SAVED_TOAST)
          return
        }
        toast.danger(AUTOSAVE_FAILED_TOAST)
      })
    }, AUTO_SAVE_DELAY_MS)
    return () => {
      cancelled = true
      window.clearTimeout(timer)
    }
  }, [onSaveSettings, watchedValues])

  return {
    ...methods,
    isBusy,
    options: modelOptions(snapshot),
  } satisfies IUseLaunchSectionReturn
}
