import { zodResolver } from "@hookform/resolvers/zod"
import { useEffect, useRef, useState } from "react"
import { useForm, useWatch } from "react-hook-form"
import type { UseFormReturn } from "react-hook-form"

import { TAutosaveStatus } from "@/features/dashboard/constants/dashboardLabels"
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
  autosaveStatus: TAutosaveStatus
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
  const [autosaveStatus, setAutosaveStatus] = useState(TAutosaveStatus.Idle)

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

    setAutosaveStatus(TAutosaveStatus.Idle)
    let cancelled = false
    const timer = window.setTimeout(() => {
      setAutosaveStatus(TAutosaveStatus.Saving)
      void onSaveSettings(persistable).then((saved) => {
        if (cancelled) return
        if (saved) {
          savedSettings.current = fingerprint
          setAutosaveStatus(TAutosaveStatus.Saved)
          return
        }
        setAutosaveStatus(TAutosaveStatus.Failed)
      })
    }, AUTO_SAVE_DELAY_MS)
    return () => {
      cancelled = true
      window.clearTimeout(timer)
    }
  }, [onSaveSettings, watchedValues])

  return {
    ...methods,
    autosaveStatus,
    isBusy,
    options: modelOptions(snapshot),
  } satisfies IUseLaunchSectionReturn
}
