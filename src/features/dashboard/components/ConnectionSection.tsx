import { Button, Card, Form } from "@heroui/react"
import { FormProvider } from "react-hook-form"
import type { FC } from "react"

import { ApiKeyField } from "@/features/dashboard/components/ApiKeyField"
import { useConnectionSection } from "@/features/dashboard/hooks/useConnectionSection"
import { connectionLabel } from "@/features/dashboard/services/dashboardFormatters"
import {
  TPendingAction,
  type IDashboardSnapshot,
} from "@/features/dashboard/schemas/dashboard.schema"
import type { ValueChanged } from "@/types"

interface IConnectionSectionProps {
  snapshot: IDashboardSnapshot
  pending: TPendingAction
  onSaveApiKey: ValueChanged<string, Promise<void>>
  onDisconnect: () => Promise<void>
  onRefresh: () => Promise<void>
}

export const ConnectionSection: FC<IConnectionSectionProps> = ({
  snapshot,
  pending,
  onSaveApiKey,
  onDisconnect,
  onRefresh,
}) => {
  const { isBusy, isConnected, onSubmit, ...methods } = useConnectionSection(
    snapshot,
    pending,
    onSaveApiKey,
  )

  return (
    <Card className="w-full">
      <Card.Header>
        <Card.Title>Connection</Card.Title>
        <Card.Description>{connectionLabel(snapshot)}</Card.Description>
      </Card.Header>
      <Card.Content>
        <FormProvider {...methods}>
          <Form onSubmit={methods.handleSubmit(onSubmit)}>
            <div className="flex flex-col gap-4">
              <ApiKeyField />
              <div className="flex flex-wrap gap-2">
                <Button isDisabled={isBusy} isPending={pending === TPendingAction.SavingKey} type="submit">
                  {pending === TPendingAction.SavingKey ? "Saving" : "Save API key"}
                </Button>
<Button
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
<Button
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
