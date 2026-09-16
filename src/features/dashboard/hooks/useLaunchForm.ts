import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"

import {
  launchFormSchema,
  type IDashboardSnapshot,
  type ILaunchForm,
} from "@/features/dashboard/constants/dashboardSchema"

export const launchFormDefaults = (snapshot: IDashboardSnapshot) => {
  return {
    terminal: snapshot.terminal,
    workspace: snapshot.lastWorkspace ?? "",
    modelId: snapshot.aliases.sonnet,
    fable: snapshot.aliases.fable,
    opus: snapshot.aliases.opus,
    sonnet: snapshot.aliases.sonnet,
    haiku: snapshot.aliases.haiku,
    customModels: snapshot.customModels.join("\n"),
  } satisfies ILaunchForm
}

export const useLaunchForm = (snapshot: IDashboardSnapshot) => {
  return useForm<ILaunchForm>({
    resolver: zodResolver(launchFormSchema),
    defaultValues: launchFormDefaults(snapshot),
  })
}
