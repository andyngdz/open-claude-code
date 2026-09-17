---
name: Open Claude Code
description: A local desktop gateway for launching Claude Code through a selected AI provider.
colors:
  canvas: "#07101d"
  sidebar: "#0b1727"
  panel: "#101d2d"
  field: "#0c1827"
  border: "#29415d"
  text: "#f3f7ff"
  text-muted: "#a9bdd8"
  primary: "#1478ff"
  primary-hover: "#2a88ff"
  success: "#48e6a4"
  danger: "#ff5f75"
typography:
  display:
    fontFamily: "system-ui, sans-serif"
    fontSize: "2.25rem"
    fontWeight: 700
    lineHeight: 1.1
  title:
    fontFamily: "system-ui, sans-serif"
    fontSize: "1.625rem"
    fontWeight: 700
    lineHeight: 1.2
  body:
    fontFamily: "system-ui, sans-serif"
    fontSize: "1rem"
    fontWeight: 400
    lineHeight: 1.5
  label:
    fontFamily: "system-ui, sans-serif"
    fontSize: "0.875rem"
    fontWeight: 600
    lineHeight: 1.25
rounded:
  field: "6px"
  panel: "8px"
  selected-nav: "8px"
spacing:
  compact: "8px"
  control: "12px"
  section: "16px"
  panel: "20px"
  page: "32px"
components:
  button-primary:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.text}"
    rounded: "{rounded.field}"
    padding: "12px 28px"
  button-secondary:
    backgroundColor: "{colors.panel}"
    textColor: "{colors.text}"
    rounded: "{rounded.field}"
    padding: "12px 28px"
  button-danger:
    backgroundColor: "transparent"
    textColor: "{colors.danger}"
    rounded: "{rounded.field}"
    padding: "12px 28px"
  panel-card:
    backgroundColor: "{colors.panel}"
    rounded: "{rounded.panel}"
    padding: "{spacing.panel}"
  form-field:
    backgroundColor: "{colors.field}"
    textColor: "{colors.text}"
    rounded: "{rounded.field}"
    padding: "12px 16px"
---

# Design System: Open Claude Code

## Overview

**Creative North Star: "Open Claude Code"**

Open Claude Code is a dark developer workbench for configuring a local gateway, not a dashboard that competes for attention. The interface gives structure to a short operational flow: connect a provider, choose runtime settings, then launch Claude Code.

The visual language is calm, direct, and technical. Dense information is grouped into bordered panels with strong headings, familiar form controls, and explicit state color. The system rejects gradients, ornamental illustration, visual noise, and decorative effects that dilute a tool's purpose.

**Key Characteristics:**

- dark navy workspace with slate panel layers
- cobalt blue for selected navigation and primary actions
- readable forms with quiet borders and high-contrast text
- green for ready and connected states, red only for destructive actions

## Colors

The palette uses cool dark layers to keep configuration work calm, while cobalt blue identifies the path forward.

### Primary

- **Launch Blue** (`#1478ff`): Used for the selected provider, primary actions, and the full-width launch action.
- **Lifted Blue** (`#2a88ff`): Hover and active refinement for primary controls.

### Neutral

- **Night Canvas** (`#07101d`): The outer application workspace.
- **Sidebar Slate** (`#0b1727`): The persistent provider navigation surface.
- **Panel Slate** (`#101d2d`): Cards that group a completed unit of setup.
- **Field Inset** (`#0c1827`): Text inputs and selects within a panel.
- **Structure Line** (`#29415d`): Borders and separators that define form structure without visual clutter.
- **Paper White** (`#f3f7ff`): Headings, labels, and action text.
- **Muted Steel** (`#a9bdd8`): Supporting descriptions and secondary metadata.

### Named Rules

**The One Blue Rule.** Blue marks selection and progress, not decoration. A screen should retain quiet dark surfaces around every primary action.

**The Semantic State Rule.** Green communicates connected or ready state. Red is reserved for removal and other destructive actions.

## Typography

**Display Font:** system-ui, sans-serif

**Body Font:** system-ui, sans-serif

**Character:** A native-feeling sans-serif hierarchy that prioritizes immediate scanning. Weight and scale create hierarchy rather than a decorative type pairing.

### Hierarchy

- **Display** (700, 2.25rem, 1.1): The provider and workflow title at the top of a workspace.
- **Title** (700, 1.625rem, 1.2): Panel headings such as Connection and Models.
- **Body** (400, 1rem, 1.5): Explanatory text and selected control values.
- **Label** (600, 0.875rem, 1.25): Field labels, compact status text, and navigation labels.

### Named Rules

**The Plainspoken Hierarchy Rule.** Use type scale to explain the workflow. Avoid display styling, all-caps labels, and decorative font treatment.

## Layout

The desktop shell uses a persistent left provider sidebar and a flexible main workspace. The main column is centered with a comfortable maximum reading width, while cards stack in the order users complete the task: connection first, model configuration second, launch last.

Use 8px as the compact rhythm, 16px between related form groups, 20px inside cards, and 32px around the main workspace. On narrow screens, the sidebar becomes a horizontal or stacked top region and controls collapse from multi-column to one column. Every interactive control keeps a clear label and full-width touch target.

## Elevation & Depth

Depth comes from tonal layering and thin borders, not from gradients or floating decoration. The canvas is darkest, the sidebar and cards lift one tonal step, and fields recede slightly into cards. Shadows, when present, stay low and diffuse enough to separate overlapping surfaces without becoming a visual motif.

### Named Rules

**The Border Before Shadow Rule.** Use a 1px structure line first. Add a soft shadow only when an overlay or focus state needs separation.

## Shapes

Panels use restrained 8px corners. Inputs, selects, and buttons use 6px corners to remain precise and tool-like. Borders are solid and quiet. Avoid pills except for compact status indicators, and avoid oversized rounding, glass effects, gradients, or decorative containers.

## Components

### Provider Navigation

- **Shape:** Full-width rows with gently rounded selected state (8px).
- **Default:** Cool muted text and simple line icons on Sidebar Slate.
- **Selected:** Launch Blue background with Paper White text.
- **State:** A provider selection changes the workspace content, so the selected state must remain unambiguous.

### Status Strip

- **Style:** A bordered horizontal row that summarizes connection, provider, model availability, local gateway address, and launch readiness.
- **Color:** Green dot and text communicate ready state. The rest remains neutral.
- **Behavior:** Status is factual and compact. It does not duplicate action controls.

### Buttons

- **Primary:** Launch Blue background, Paper White label, 12px by 28px padding, and 6px corners.
- **Secondary:** Panel Slate surface with Structure Line border and Paper White label.
- **Danger:** Transparent surface with Danger red label and border. Use for Disconnect only.
- **Hover / Focus:** Brighten primary blue slightly. Focus uses a visible blue ring with enough contrast against the dark surface.

### Cards / Containers

- **Corner Style:** Restrained 8px corners.
- **Background:** Panel Slate on Night Canvas.
- **Border:** 1px Structure Line.
- **Internal Padding:** 20px, with 16px between form groups.

### Inputs / Fields

- **Style:** Field Inset background, Structure Line border, 6px corners, Paper White selected value.
- **Focus:** Blue border and visible focus ring.
- **Error / Disabled:** Danger red is reserved for error messaging. Disabled controls reduce contrast without concealing their label.

### Auto-save Model Settings

- **Style:** A small green confirmation next to the Models heading.
- **Behavior:** Changing a terminal or model dropdown persists immediately. Do not show a Save settings button.

## Do's and Don'ts

### Do:

- **Do** keep the workflow ordered from connection to models to launch.
- **Do** use cobalt blue only for selection and progress.
- **Do** show immediate, local feedback when model settings auto-save.
- **Do** keep connected state factual, short, and green.

### Don't:

- **Don't** use gradients, illustrations, decorative motifs, or glass effects.
- **Don't** turn every container into a bright card or raised surface.
- **Don't** use green or red as general-purpose accent colors.
- **Don't** require a manual save action for dropdown model settings.
