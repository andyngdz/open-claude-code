import { Button, Card, Form } from "@heroui/react"
import { map } from "es-toolkit/compat"
import { FormProvider } from "react-hook-form"
import type { FC } from "react"

import { CustomModelsField } from "@/features/dashboard/components/CustomModelsField"
import { ModelSelect } from "@/features/dashboard/components/ModelSelect"
import { TerminalSelect } from "@/features/dashboard/components/TerminalSelect"
import { ALIAS_LABELS, MODEL_FAMILIES, TModelField } from "@/features/dashboard/constants/dashboardLabels"
import { useLaunchSection } from "@/features/dashboard/hooks/useLaunchSection"
import {
  TPendingAction,
  type IDashboardSnapshot,
  type ILaunchForm,
} from "@/features/dashboard/schemas/dashboard.schema"
import type { ValueChanged } from "@/types"

interface ILaunchSectionProps {
  snapshot: IDashboardSnapshot
  pending: TPendingAction
  onSaveSettings: ValueChanged<ILaunchForm, Promise<void>>
}

export const LaunchSection: FC<ILaunchSectionProps> = ({
  snapshot,
  pending,
  onSaveSettings,
}) => {
  const {
    isBusy,
    options,
    onSaveSettings: saveSettings,
    ...methods
  } = useLaunchSection(snapshot, pending, onSaveSettings)

  return (
    <Card className="w-full">
      <Card.Header>
        <Card.Title>Models</Card.Title>
      </Card.Header>
      <Card.Content>
        <FormProvider {...methods}>
          <Form>
            <div className="flex flex-col gap-4">
              <TerminalSelect terminals={snapshot.terminals} />
              <ModelSelect label="Model" name={TModelField.Launch} options={options} />
              <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                {map(MODEL_FAMILIES, (family) => (
                  <ModelSelect key={family} label={ALIAS_LABELS[family]} name={family} options={options} />
                ))}
              </div>
              <CustomModelsField isDisabled={isBusy} />
              <div className="flex flex-wrap gap-2">
                <Button
                  isDisabled={isBusy}
                  isPending={pending === TPendingAction.SavingSettings}
                  type="button"
                  variant="secondary"
                  onPress={() => {
                    void methods.handleSubmit(saveSettings)()
                  }}
                >
                  {pending === TPendingAction.SavingSettings ? "Saving" : "Save settings"}
                </Button>
              </div>
            </div>
          </Form>
        </FormProvider>
      </Card.Content>
    </Card>
  )
}
