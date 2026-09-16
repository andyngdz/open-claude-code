import { Label, ListBox } from "@heroui/react"
import { isString, map } from "es-toolkit/compat"
import type { FC } from "react"

import { PROVIDER_LABELS, PROVIDERS, type TProviderId } from "@/features/dashboard/constants/dashboardProviders"
import { readProviderId } from "@/features/dashboard/services/dashboardFormatters"
import type { ValueChanged } from "@/types"

interface IProviderSidebarProps {
  selected: TProviderId
  onSelect: ValueChanged<TProviderId>
}

export const ProviderSidebar: FC<IProviderSidebarProps> = ({ selected, onSelect }) => {
  return (
    <aside className="app-sidebar">
      <p className="px-3 py-2 text-sm font-medium text-muted">Providers</p>
      <ListBox
        aria-label="Providers"
        className="w-full"
        selectedKeys={new Set([selected])}
        selectionMode="single"
        onSelectionChange={(keys) => {
          if (keys === "all") return
          const [key] = keys
          if (!isString(key)) return
          const provider = readProviderId(key)
          if (!provider) return
          onSelect(provider)
        }}
      >
        {map(PROVIDERS, (provider) => (
          <ListBox.Item id={provider} key={provider} textValue={PROVIDER_LABELS[provider]}>
            <Label>{PROVIDER_LABELS[provider]}</Label>
          </ListBox.Item>
        ))}
      </ListBox>
    </aside>
  )
}
