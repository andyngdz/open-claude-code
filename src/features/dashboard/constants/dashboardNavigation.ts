import type { TProviderId } from "@/features/dashboard/constants/dashboardProviders"

/// The sidebar carries provider rows plus one Settings entry, kept apart from
/// them.
///
/// One selection value covers both groups, so the workspace can never show a
/// provider panel and the Settings panel at once, and the sidebar needs no
/// second state kept in step with the first by hand.
export enum TSidebarView {
  Settings = "settings",
}

export type TSidebarSelection = TProviderId | TSidebarView

export const isSettingsView = (selection: TSidebarSelection) => {
  return selection === TSidebarView.Settings
}
