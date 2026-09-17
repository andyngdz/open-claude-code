import { Button, Card, Form } from "@heroui/react"
import { FormProvider } from "react-hook-form"
import type { FC } from "react"

import { ApiKeyField } from "@/features/dashboard/components/ApiKeyField"
import { useConnectionSection } from "@/features/dashboard/hooks/useConnectionSection"
import {
  TPendingAction,
  type IDashboardSnapshot,
} from "@/features/dashboard/schemas/dashboard.schema"
import type { ValueChanged } from "@/types"

interface IConnectionSectionProps {
  apiKey: string
  snapshot: IDashboardSnapshot
  pending: TPendingAction
  onSaveApiKey: ValueChanged<string, Promise<boolean>>
  onDisconnect: () => Promise<void>
  onRefresh: () => Promise<void>
}

export const ConnectionSection: FC<IConnectionSectionProps> = ({
  apiKey,
  snapshot,
  pending,
  onSaveApiKey,
  onDisconnect,
  onRefresh,
}) => {
  const { isBusy, isConnected, onSubmit, ...methods } = useConnectionSection(
    snapshot,
    apiKey,
    pending,
    onSaveApiKey,
  )

  return (
    <Card className="workbench-card w-full">
      <Card.Header className="workbench-card-header">
        <Card.Title>Connection</Card.Title>
        <Card.Description>
          Save an OpenCode Go API key so Claude Code can use this provider through the local gateway.
        </Card.Description>
      </Card.Header>
      <Card.Content className="workbench-card-content">
        <FormProvider {...methods}>
          <Form onSubmit={methods.handleSubmit(onSubmit)}>
            <div className="flex flex-col gap-4">
              <ApiKeyField />
              <div className="connection-actions">
                <div className="flex flex-wrap gap-2">
                  <Button className="connection-button" isDisabled={isBusy} isPending={pending === TPendingAction.SavingKey} type="submit">
                    {pending === TPendingAction.SavingKey ? "Saving" : "Save API key"}
                  </Button>
                  <Button
                    className="connection-button connection-button-secondary"
                    isDisabled={!isConnected || isBusy}
                    isPending={pending === TPendingAction.Refreshing}
                    type="button"
                    variant="secondary"
                    onPress={() => {
                      void onRefresh()
                    }}
                  >
                    {pending === TPendingAction.Refreshing ? "Refreshing" : "Refresh models"}
                  </Button>
                </div>
                <Button
                  className="connection-button connection-button-danger"
                  isDisabled={!isConnected || isBusy}
                  isPending={pending === TPendingAction.Disconnecting}
                  type="button"
                  variant="danger"
                  onPress={() => {
                    void onDisconnect()
                  }}
                >
                  {pending === TPendingAction.Disconnecting ? "Removing" : "Disconnect"}
                </Button>
              </div>
            </div>
          </Form>
        </FormProvider>
      </Card.Content>
    </Card>
  )
}
