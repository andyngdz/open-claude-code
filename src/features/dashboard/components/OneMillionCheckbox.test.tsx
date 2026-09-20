// @vitest-environment jsdom

import { cleanup, render, screen } from "@testing-library/react"
import userEvent from "@testing-library/user-event"
import { FormProvider, useForm, useWatch } from "react-hook-form"
import { afterEach, describe, expect, it } from "vitest"
import type { FC } from "react"

import { OneMillionCheckbox } from "@/features/dashboard/components/OneMillionCheckbox"
import { TExtendedField } from "@/features/dashboard/constants/dashboardLabels"

// Vitest runs without globals, so testing-library never registers its own cleanup.
afterEach(cleanup)

const OneMillionCheckboxHarness: FC = () => {
  const methods = useForm({ defaultValues: { extendedOpus: false } })
  const value = useWatch({ control: methods.control, name: TExtendedField.Opus })

  return (
    <FormProvider {...methods}>
      <OneMillionCheckbox aliasLabel="Opus" name={TExtendedField.Opus} />
      <p>{`tick:${String(value)}`}</p>
    </FormProvider>
  )
}

describe("OneMillionCheckbox", () => {
  it("names its row so four ticks on screen read apart", () => {
    render(<OneMillionCheckboxHarness />)

    expect(screen.getByRole("checkbox", { name: "Opus 1M context" })).toBeTruthy()
  })

  it("writes the tick into the form value", async () => {
    const user = userEvent.setup()
    render(<OneMillionCheckboxHarness />)

    await user.click(screen.getByRole("checkbox", { name: "Opus 1M context" }))

    expect(screen.getByText("tick:true")).toBeTruthy()
  })
})
