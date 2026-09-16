import { invoke } from "@tauri-apps/api/core"

import {
  dashboardSnapshotSchema,
  parseCustomModels,
  type IDashboardSnapshot,
  type ILaunchForm,
} from "@/features/dashboard/constants/dashboardSchema"

class DashboardService {
  loadSnapshot = async (): Promise<IDashboardSnapshot> => {
    return dashboardSnapshotSchema.parse(await invoke("dashboard_snapshot"))
  }

  saveApiKey = async (apiKey: string): Promise<IDashboardSnapshot> => {
    return dashboardSnapshotSchema.parse(await invoke("save_api_key", { apiKey }))
  }

  removeCredential = async (): Promise<IDashboardSnapshot> => {
    return dashboardSnapshotSchema.parse(await invoke("remove_credential"))
  }

  refreshCatalog = async (): Promise<IDashboardSnapshot> => {
    return dashboardSnapshotSchema.parse(await invoke("refresh_catalog"))
  }

  saveSettings = async (values: ILaunchForm): Promise<IDashboardSnapshot> => {
    return dashboardSnapshotSchema.parse(
      await invoke("save_dashboard_settings", { input: settingsInput(values) }),
    )
  }

  launch = async (values: ILaunchForm): Promise<IDashboardSnapshot> => {
    return dashboardSnapshotSchema.parse(
      await invoke("launch_claude_session", {
        input: {
          workspace: values.workspace,
          modelId: values.modelId,
        },
      }),
    )
  }
}

const settingsInput = (values: ILaunchForm) => {
  return {
    terminal: values.terminal,
    aliases: {
      fable: values.fable,
      opus: values.opus,
      sonnet: values.sonnet,
      haiku: values.haiku,
    },
    customModels: parseCustomModels(values.customModels),
  }
}

export const dashboardService = new DashboardService()
