import { Button, Card, Form } from "@heroui/react"
import { FormProvider } from "react-hook-form"
import type { FC } from "react"

import { ApiKeyField } from "@/features/dashboard/components/ApiKeyField"
import { WORKBENCH_CARD_CLASS } from "@/features/dashboard/constants/dashboardLayout"
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
    <Card className={WORKBENCH_CARD_CLASS}>
      <Card.Header className="flex flex-col gap-2">
        <Card.Title className="text-2xl font-bold">Connection</Card.Title>
        <Card.Description>
          Save an OpenCode Go API key so Claude Code can use this provider through the local gateway.
        </Card.Description>
      </Card.Header>
      <Card.Content>
        <FormProvider {...methods}>
          <Form onSubmit={methods.handleSubmit(onSubmit)}>
            <div className="flex flex-col gap-4">
              <ApiKeyField />
              <div className="flex flex-wrap items-center justify-between gap-2">
                <div className="flex flex-wrap gap-2">
                  <Button className="rounded-md" isDisabled={isBusy} isPending={pending === TPendingAction.SavingKey} type="submit">
                    Save API key
                  </Button>
                  <Button
                    className="rounded-md"
                    isDisabled={!isConnected || isBusy}
                    isPending={pending === TPendingAction.Refreshing}
                    type="button"
                    variant="secondary"
                    onPress={() => {
                      void onRefresh()
                    }}
                  >
                    Refresh models
                  </Button>
                </div>
                <Button
                  className="rounded-md"
                  isDisabled={!isConnected || isBusy}
                  isPending={pending === TPendingAction.Disconnecting}
                  type="button"
                  variant="danger"
                  onPress={() => {
                    void onDisconnect()
                  }}
                >
                  Disconnect
                </Button>
              </div>
            </div>
          </Form>
        </FormProvider>
      </Card.Content>
    </Card>
  )
}
