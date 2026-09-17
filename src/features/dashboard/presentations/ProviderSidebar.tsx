import { Label, ListBox } from "@heroui/react"
import { isString, map } from "es-toolkit/compat"
import { Box, CodeXml, Hexagon, Server, Sparkles } from "lucide-react"
import type { FC } from "react"

import { PROVIDER_LABELS, PROVIDERS, TProviderId } from "@/features/dashboard/constants/dashboardProviders"
import { readProviderId } from "@/features/dashboard/services/dashboardFormatters"
import type { ValueChanged } from "@/types"

interface IProviderSidebarProps {
  selected: TProviderId
  onSelect: ValueChanged<TProviderId>
}

export const ProviderSidebar: FC<IProviderSidebarProps> = ({ selected, onSelect }) => {
  return (
    <aside className="app-sidebar">
      <div className="brand-lockup">
        <p className="brand-title">Open Claude Code</p>
        <p className="brand-copy">Your local gateway for Claude Code.</p>
      </div>
      <p className="provider-nav-label">Providers</p>
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
          <ListBox.Item
            className="provider-nav-item"
            id={provider}
            key={provider}
            textValue={PROVIDER_LABELS[provider]}
          >
            <span className="provider-nav-row">
              {provider === TProviderId.OpenCode && <Server size={16} />}
              {provider === TProviderId.Codex && <CodeXml size={16} />}
              {provider === TProviderId.Grok && <Hexagon size={16} />}
              {provider === TProviderId.Cursor && <Box size={16} />}
              {provider === TProviderId.Gemini && <Sparkles size={16} />}
              <Label>{PROVIDER_LABELS[provider]}</Label>
            </span>
          </ListBox.Item>
        ))}
      </ListBox>
    </aside>
  )
}
