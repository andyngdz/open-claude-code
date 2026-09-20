import { Card, Form } from "@heroui/react"
import { map } from "es-toolkit/compat"
import { FormProvider } from "react-hook-form"
import type { FC } from "react"

import { CustomModelsField } from "@/features/dashboard/components/CustomModelsField"
import { LaunchOptionsSection } from "@/features/dashboard/components/LaunchOptionsSection"
import { ModelSelect } from "@/features/dashboard/components/ModelSelect"
import { OneMillionCheckbox } from "@/features/dashboard/components/OneMillionCheckbox"
import { TerminalSelect } from "@/features/dashboard/components/TerminalSelect"
import { WORKBENCH_CARD_CLASS } from "@/features/dashboard/constants/dashboardLayout"
import {
  ALIAS_LABELS,
  DEFAULT_MODEL_LABEL,
  EXTENDED_CONTEXT_HINT,
  EXTENDED_FIELDS,
  MODEL_FAMILIES,
  TExtendedField,
  TModelField,
} from "@/features/dashboard/constants/dashboardLabels"
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
                  <div className="flex flex-col gap-2">
                    <ModelSelect
                      label={DEFAULT_MODEL_LABEL}
                      name={TModelField.Launch}
                      options={options}
                    />
                    <OneMillionCheckbox
                      rowLabel={DEFAULT_MODEL_LABEL}
                      name={TExtendedField.ModelId}
                    />
                  </div>
                </div>
                <div className="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-4">
                  {map(MODEL_FAMILIES, (family) => (
                    <div className="flex flex-col gap-2" key={family}>
                      <ModelSelect label={ALIAS_LABELS[family]} name={family} options={options} />
                      <OneMillionCheckbox
                        rowLabel={ALIAS_LABELS[family]}
                        name={EXTENDED_FIELDS[family]}
                      />
                    </div>
                  ))}
                </div>
                <p className="text-sm text-muted">{EXTENDED_CONTEXT_HINT}</p>
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
