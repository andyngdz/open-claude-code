// @vitest-environment jsdom

import { render, screen } from "@testing-library/react"
import userEvent from "@testing-library/user-event"
import { FormProvider, useForm } from "react-hook-form"
import { describe, expect, it } from "vitest"
import type { FC } from "react"

import { ModelSelect } from "@/features/dashboard/components/ModelSelect"
import { TModelField } from "@/features/dashboard/constants/dashboardLabels"
import type { IModelOption } from "@/features/dashboard/schemas/dashboard.schema"

const options = [
  { id: "qwen3.8-max", displayName: "Qwen 3.8 Max", isCustom: false },
  { id: "qwen3.8-flash", displayName: "Qwen 3.8 Flash", isCustom: false },
] satisfies IModelOption[]

const ModelSelectHarness: FC = () => {
  const methods = useForm({ defaultValues: { modelId: "qwen3.8-max" } })
  return (
    <FormProvider {...methods}>
      <ModelSelect label="Default model" name={TModelField.Launch} options={options} />
    </FormProvider>
  )
}

describe("ModelSelect", () => {
  it("filters catalog options as the user types in search", async () => {
    const user = userEvent.setup()
    render(<ModelSelectHarness />)

    await user.click(screen.getByRole("button", { name: /default model/i }))
    await user.type(screen.getByPlaceholderText("Search models"), "flash")

    expect(screen.getByRole("option", { name: "Qwen 3.8 Flash" })).toBeTruthy()
    expect(screen.queryByRole("option", { name: "Qwen 3.8 Max" })).toBeNull()
  })
})
