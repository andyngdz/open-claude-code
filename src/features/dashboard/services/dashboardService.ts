import { invoke } from "@tauri-apps/api/core"
import { open } from "@tauri-apps/plugin-dialog"
import { isNull, isString, isUndefined } from "es-toolkit/compat"

import { customModelIds } from "@/features/dashboard/services/dashboardFormatters"
import {
  dashboardSnapshotSchema,
  type IDashboardSnapshot,
  type ILaunchForm,
} from "@/features/dashboard/schemas/dashboard.schema"

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

  chooseWorkspace = async (lastWorkspace?: string): Promise<string | undefined> => {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Choose folder",
      ...(isString(lastWorkspace) ? { defaultPath: lastWorkspace } : {}),
    })
    if (isCancelledWorkspace(selected)) return
    if (!isString(selected)) throw new Error("Choose a workspace directory.")
    return selected
  }

  launch = async (values: ILaunchForm, workspace: string): Promise<IDashboardSnapshot> => {
    return dashboardSnapshotSchema.parse(
      await invoke("launch_claude_session", {
        input: {
          workspace,
          modelId: values.modelId,
        },
      }),
    )
  }
}


const isCancelledWorkspace = (selected: unknown) => {
  return isNull(selected) || isUndefined(selected)
}

const settingsInput = (values: ILaunchForm) => {
  return {
    terminal: values.terminal,
    modelId: values.modelId,
    aliases: {
      fable: values.fable,
      opus: values.opus,
      sonnet: values.sonnet,
      haiku: values.haiku,
    },
    customModels: customModelIds(values.customModels),
  }
}

export const dashboardService = new DashboardService()
