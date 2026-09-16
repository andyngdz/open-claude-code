import { FieldError, Input, Label, TextField } from "@heroui/react"
import { isString } from "es-toolkit/compat"
import { useController } from "react-hook-form"
import type { FC } from "react"

export const ApiKeyField: FC = () => {
  const { field, fieldState } = useController({ name: "apiKey" })
  const message = fieldState.error?.message

  return (
    <TextField
      fullWidth
      isInvalid={isString(message)}
      name={field.name}
      type="password"
      value={field.value}
      onBlur={field.onBlur}
      onChange={field.onChange}
    >
      <Label>API key</Label>
      <Input autoComplete="off" ref={field.ref} />
      <FieldError>{message}</FieldError>
    </TextField>
  )
}
