import { Button, Input, Label, TextField } from "@heroui/react"
import { Check, Copy } from "lucide-react"
import { useMemo } from "react"
import type { FC } from "react"

import { LAUNCH_COMMAND } from "@/features/dashboard/constants/dashboardLabels"

interface ILaunchCommandFieldProps {
  copied: boolean
  onCopy: () => Promise<void>
}

export const LaunchCommandField: FC<ILaunchCommandFieldProps> = ({ copied, onCopy }) => {
  const copyIcon = useMemo(() => {
    if (copied) return <Check />
    return <Copy />
  }, [copied])

  return (
    <div className="flex items-end gap-2">
      <div className="min-w-0 flex-1">
        <TextField isReadOnly fullWidth value={LAUNCH_COMMAND}>
          <Label>Launch command</Label>
          <Input readOnly />
        </TextField>
      </div>
      <Button
        aria-label={copied ? "Copied" : "Copy launch command"}
        isIconOnly
        type="button"
        variant="secondary"
        onPress={() => {
          void onCopy()
        }}
      >
        {copyIcon}
      </Button>
    </div>
  )
}
