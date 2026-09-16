import type { SubmitHandler } from "react-hook-form"
import type { FC } from "react"

import {
  TConnectionStatus,
  TPendingAction,
  connectionLabel,
  type IApiKeyForm,
  type IDashboardSnapshot,
} from "@/features/dashboard/constants/dashboardSchema"
import { useApiKeyForm } from "@/features/dashboard/hooks/useApiKeyForm"

interface IConnectionSectionProps {
  snapshot: IDashboardSnapshot
  pending: TPendingAction
  onSave: (apiKey: string) => Promise<void>
  onDisconnect: () => Promise<void>
  onRefresh: () => Promise<void>
}

export const ConnectionSection: FC<IConnectionSectionProps> = ({
  snapshot,
  pending,
  onSave,
  onDisconnect,
  onRefresh,
}) => {
  const { register, handleSubmit, reset, formState } = useApiKeyForm()
  const isBusy = pending !== TPendingAction.None
  const isConnected = snapshot.connection.status === TConnectionStatus.Connected
  const onSubmit: SubmitHandler<IApiKeyForm> = async (values) => {
    await onSave(values.apiKey)
    reset()
  }

  return (
    <section className="panel" aria-labelledby="connection-heading">
      <div className="panel-heading">
        <h2 id="connection-heading">Connection</h2>
        <p>{connectionLabel(snapshot)}</p>
      </div>
      <form className="stack" onSubmit={handleSubmit(onSubmit)}>
        <label className="field" htmlFor="api-key">
          <span>API key</span>
          <input id="api-key" type="password" autoComplete="off" {...register("apiKey")} />
          {formState.errors.apiKey?.message && (
            <span role="alert">{formState.errors.apiKey.message}</span>
          )}
        </label>
        <div className="actions">
          <button type="submit" disabled={isBusy}>
            {pending === TPendingAction.SavingKey ? "Saving" : "Save API key"}
          </button>
          <button type="button" onClick={() => void onRefresh()} disabled={!isConnected || isBusy}>
            {pending === TPendingAction.Refreshing ? "Refreshing" : "Refresh models"}
          </button>
          <button type="button" onClick={() => void onDisconnect()} disabled={!isConnected || isBusy}>
            {pending === TPendingAction.Disconnecting ? "Removing" : "Disconnect"}
          </button>
        </div>
      </form>
    </section>
  )
}
