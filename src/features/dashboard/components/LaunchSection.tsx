import { Card, Form } from "@heroui/react"
import { map } from "es-toolkit/compat"
import { FormProvider } from "react-hook-form"
import type { FC } from "react"

import { CustomModelsField } from "@/features/dashboard/components/CustomModelsField"
import { LaunchOptionsSection } from "@/features/dashboard/components/LaunchOptionsSection"
import { ModelSelect } from "@/features/dashboard/components/ModelSelect"
import { TerminalSelect } from "@/features/dashboard/components/TerminalSelect"
import { WORKBENCH_CARD_CLASS } from "@/features/dashboard/constants/dashboardLayout"
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
  onSaveSettings: ValueChanged<ILaunchForm, Promise<boolean>>
  onLaunch: ValueChanged<ILaunchForm, Promise<void>>
}

export const LaunchSection: FC<ILaunchSectionProps> = ({
  snapshot,
  pending,
  onSaveSettings,
  onLaunch,
}) => {
  const { isBusy, options, ...methods } = useLaunchSection(snapshot, pending, onSaveSettings)

  return (
    <>
      <Card className={WORKBENCH_CARD_CLASS}>
        <Card.Header className="flex flex-col gap-2">
          <Card.Title className="text-2xl font-bold">Models</Card.Title>
          <Card.Description>
            Choose the terminal, default model, and family aliases Claude Code will use.
          </Card.Description>
        </Card.Header>
        <Card.Content>
          <FormProvider {...methods}>
            <Form>
              <div className="flex flex-col gap-4">
                <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                  <TerminalSelect terminals={snapshot.terminals} />
                  <ModelSelect label="Default model" name={TModelField.Launch} options={options} />
                </div>
                <div className="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-4">
                  {map(MODEL_FAMILIES, (family) => (
                    <ModelSelect key={family} label={ALIAS_LABELS[family]} name={family} options={options} />
                  ))}
                </div>
                <CustomModelsField isDisabled={isBusy} />
              </div>
            </Form>
          </FormProvider>
        </Card.Content>
      </Card>
      <LaunchOptionsSection
        canLaunch
        pending={pending}
        snapshot={snapshot}
        onLaunch={async () => methods.handleSubmit(onLaunch)()}
      />
    </>
  )
}
