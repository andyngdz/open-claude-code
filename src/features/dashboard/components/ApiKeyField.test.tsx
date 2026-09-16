// @vitest-environment jsdom

import { FormProvider, useForm } from "react-hook-form"
import { render, screen } from "@testing-library/react"
import userEvent from "@testing-library/user-event"
import type { FC } from "react"
import { describe, expect, it } from "vitest"

import { ApiKeyField } from "@/features/dashboard/components/ApiKeyField"
import type { IApiKeyForm } from "@/features/dashboard/schemas/dashboard.schema"

const ApiKeyFieldForm: FC = () => {
  const methods = useForm<IApiKeyForm>({ defaultValues: { apiKey: "saved-api-key" } })

  return (
    <FormProvider {...methods}>
      <ApiKeyField />
    </FormProvider>
  )
}

describe("ApiKeyField", () => {
  it("reveals and hides the saved API key", async () => {
    const user = userEvent.setup()
    render(<ApiKeyFieldForm />)

    const apiKeyInput = screen.getByLabelText("API key")
    expect(apiKeyInput.getAttribute("type")).toBe("password")

    await user.click(screen.getByRole("button", { name: "Show API key" }))
    expect(apiKeyInput.getAttribute("type")).toBe("text")

    await user.click(screen.getByRole("button", { name: "Hide API key" }))
    expect(apiKeyInput.getAttribute("type")).toBe("password")
  })
})
