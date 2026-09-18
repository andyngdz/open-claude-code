import { Button, FieldError, InputGroup, Label, TextField } from "@heroui/react"
import { isString } from "es-toolkit/compat"
import { Eye, EyeOff } from "lucide-react"
import { useController } from "react-hook-form"
import { useMemo, useState } from "react"
import type { FC } from "react"

export const ApiKeyField: FC = () => {
  const { field, fieldState } = useController({ name: "apiKey" })
  const [isVisible, setIsVisible] = useState(false)
  const message = fieldState.error?.message
  const visibilityIcon = useMemo(() => {
    if (isVisible) return <EyeOff />
    return <Eye />
  }, [isVisible])

  return (
    <TextField
      fullWidth
      isInvalid={isString(message)}
      name={field.name}
      type={isVisible ? "text" : "password"}
      value={field.value}
      onBlur={field.onBlur}
      onChange={field.onChange}
    >
      <Label>API key</Label>
      <InputGroup fullWidth>
        <InputGroup.Input autoComplete="off" ref={field.ref} />
        <InputGroup.Suffix>
          <Button
            aria-label={isVisible ? "Hide API key" : "Show API key"}
            isIconOnly
            size="sm"
            type="button"
            variant="ghost"
            onPress={() => {
              setIsVisible((currentVisibility) => !currentVisibility)
            }}
          >
            {visibilityIcon}
          </Button>
        </InputGroup.Suffix>
      </InputGroup>
      <FieldError>{message}</FieldError>
      <p className="text-sm text-muted">Stored in the system keyring on this machine.</p>
    </TextField>
  )
}
