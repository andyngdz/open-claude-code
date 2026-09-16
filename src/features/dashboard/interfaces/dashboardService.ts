import type { ILaunchForm } from "@/features/dashboard/schemas/dashboard.schema"

export interface IClaudeLaunch {
  settings: ILaunchForm
  workspace: string
}
