import { open } from "@tauri-apps/plugin-dialog"
import { useCallback, useEffect } from "react"
import type { SubmitHandler } from "react-hook-form"
import type { FC } from "react"

import {
  TConnectionStatus,
  TPendingAction,
  ALIAS_LABELS,
  parseCustomModels,
  type IDashboardSnapshot,
  type ILaunchForm,
  type IModelOption,
} from "@/features/dashboard/constants/dashboardSchema"
import { launchFormDefaults, useLaunchForm } from "@/features/dashboard/hooks/useLaunchForm"
import { ModelSelect } from "@/features/dashboard/presentations/ModelSelect"

interface ILaunchSectionProps {
  snapshot: IDashboardSnapshot
  pending: TPendingAction
  onSaveSettings: (values: ILaunchForm) => Promise<void>
  onLaunch: (values: ILaunchForm) => Promise<void>
}

const modelOptions = (snapshot: IDashboardSnapshot) => {
  const catalogIds = new Set(snapshot.models.map((model) => model.id))
  const customOptions = parseCustomModels(snapshot.customModels.join("\n"))
    .filter((modelId) => !catalogIds.has(modelId))
    .map((modelId) => {
      return { id: modelId, displayName: modelId, isCustom: true } satisfies IModelOption
    })
  return [...snapshot.models, ...customOptions]
}

export const LaunchSection: FC<ILaunchSectionProps> = ({
  snapshot,
  pending,
  onSaveSettings,
  onLaunch,
}) => {
  const { register, handleSubmit, reset, setValue, setError, watch, formState } = useLaunchForm(snapshot)
  const options = modelOptions(snapshot)
  const isBusy = pending !== TPendingAction.None
  const canLaunch = snapshot.connection.status === TConnectionStatus.Connected && !isBusy
  const resetFromSnapshot = useCallback(() => {
    reset(launchFormDefaults(snapshot))
  }, [reset, snapshot])

  useEffect(() => {
    resetFromSnapshot()
  }, [resetFromSnapshot])

  const onChooseWorkspace = async () => {
    const selected = await open({ directory: true, multiple: false, title: "Choose workspace" })
    if (typeof selected !== "string") return
    setValue("workspace", selected, { shouldValidate: true })
  }

  const requireWorkspace = (values: ILaunchForm) => {
    if (values.workspace) return true
    setError("workspace", { message: "Choose a workspace directory." })
    return false
  }

  const onSave: SubmitHandler<ILaunchForm> = async (values) => {
    await onSaveSettings(values)
  }

  const onLaunchSession: SubmitHandler<ILaunchForm> = async (values) => {
    if (!requireWorkspace(values)) return
    await onLaunch(values)
  }

  return (
    <section className="panel" aria-labelledby="launch-heading">
      <h2 id="launch-heading">Launch</h2>
      <form className="stack" onSubmit={handleSubmit(onLaunchSession)}>
        <label className="field" htmlFor="terminal">
          <span>Terminal</span>
          <select id="terminal" {...register("terminal")}>
            {snapshot.terminals.map((terminal) => (
              <option key={terminal.kind} value={terminal.kind} disabled={!terminal.isAvailable}>
                {terminal.label}
              </option>
            ))}
          </select>
        </label>
        <label className="field" htmlFor="workspace">
          <span>Workspace</span>
          <input id="workspace" readOnly value={watch("workspace")} />
          {formState.errors.workspace?.message && (
            <span role="alert">{formState.errors.workspace.message}</span>
          )}
        </label>
        <div className="actions">
          <button type="button" onClick={() => void onChooseWorkspace()} disabled={isBusy}>
            Choose workspace
          </button>
        </div>
        <ModelSelect
          id="launch-model"
          label="Model"
          value={watch("modelId")}
          options={options}
          errorMessage={formState.errors.modelId?.message}
          onChange={(modelId) => setValue("modelId", modelId, { shouldValidate: true })}
        />
        <div className="alias-grid">
          {(["fable", "opus", "sonnet", "haiku"] as const).map((family) => (
            <ModelSelect
              key={family}
              id={`${family}-model`}
              label={ALIAS_LABELS[family]}
              value={watch(family)}
              options={options}
              errorMessage={formState.errors[family]?.message}
              onChange={(modelId) => setValue(family, modelId, { shouldValidate: true })}
            />
          ))}
        </div>
        <label className="field" htmlFor="custom-models">
          <span>Custom models</span>
          <textarea id="custom-models" rows={4} {...register("customModels")} />
        </label>
        <div className="actions">
          <button type="button" onClick={() => void handleSubmit(onSave)()} disabled={isBusy}>
            {pending === TPendingAction.SavingSettings ? "Saving" : "Save settings"}
          </button>
          <button type="submit" disabled={!canLaunch}>
            {pending === TPendingAction.Launching ? "Opening" : "Launch Claude Code"}
          </button>
        </div>
      </form>
    </section>
  )
}
