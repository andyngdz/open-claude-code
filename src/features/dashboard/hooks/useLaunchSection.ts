import { zodResolver } from "@hookform/resolvers/zod"
import { useEffect } from "react"
import { useForm } from "react-hook-form"
import type { SubmitHandler, UseFormReturn } from "react-hook-form"

import {
  launchFormSchema,
  TPendingAction,
  type IDashboardSnapshot,
  type ILaunchForm,
  type IModelOption,
} from "@/features/dashboard/schemas/dashboard.schema"
import { launchFormDefaults, modelOptions } from "@/features/dashboard/services/dashboardFormatters"
import type { ValueChanged } from "@/types"

interface IUseLaunchSectionReturn extends UseFormReturn<ILaunchForm> {
  isBusy: boolean
  options: IModelOption[]
  onSaveSettings: SubmitHandler<ILaunchForm>
}

export const useLaunchSection = (
  snapshot: IDashboardSnapshot,
  pending: TPendingAction,
  onSaveSettings: ValueChanged<ILaunchForm, Promise<void>>,
) => {
  const methods = useForm<ILaunchForm>({
    resolver: zodResolver(launchFormSchema),
    defaultValues: launchFormDefaults(snapshot),
  })
  const isBusy = pending !== TPendingAction.None

  useEffect(() => {
    methods.reset(launchFormDefaults(snapshot))
  }, [methods, snapshot])

  const onSave: SubmitHandler<ILaunchForm> = async (values) => {
    await onSaveSettings(values)
  }

  return {
    ...methods,
    isBusy,
    options: modelOptions(snapshot),
    onSaveSettings: onSave,
  } satisfies IUseLaunchSectionReturn
}
