; NSIS expands $VAR at runtime. $$ writes a PowerShell $ into the helper script.

; PowerShell rather than NSIS ReadRegStr/WriteRegExpandStr: NSIS strings stop at
; NSIS_MAX_STRLEN, 1024 by default and not raised by the Tauri template, so
; rewriting a long user PATH through $0 would truncate it. Reading without
; expanding %VAR% and writing back as ExpandString also keeps the value in
; REG_EXPAND_SZ, which SetEnvironmentVariable downcasts to REG_SZ.
!macro RunUserPathPowerShell line
  Push $0
  FileOpen $0 "$TEMP\occ-path.ps1" w
  FileWrite $0 "$$dir = '$INSTDIR'$\r$\n"
  FileWrite $0 "${line}$\r$\n"
  FileClose $0
  nsExec::ExecToLog 'powershell.exe -NoProfile -ExecutionPolicy Bypass -File "$TEMP\occ-path.ps1"'
  Pop $0
  Delete "$TEMP\occ-path.ps1"
  StrCmp $0 "0" occ_path_written
  DetailPrint "Could not update PATH (PowerShell exit $0). Add $INSTDIR to PATH manually."
occ_path_written:
  SendMessage 0xFFFF 0x001A 0 "STR:Environment" /TIMEOUT=5000
  Pop $0
!macroend

; Scope is the account running the installer, matching the default installMode
; "currentUser". A perMachine installer would have to write HKLM\Environment
; instead, which needs elevation this hook does not have.
!macro NSIS_HOOK_POSTINSTALL
  !insertmacro RunUserPathPowerShell "$$key = [Microsoft.Win32.Registry]::CurrentUser.OpenSubKey('Environment', $$true); $$path = [string]$$key.GetValue('Path', '', 'DoNotExpandEnvironmentNames'); $$parts = @($$path -split ';' | Where-Object { $$_ -ne '' }); if ($$parts -notcontains $$dir) { $$key.SetValue('Path', (($$parts + $$dir) -join ';'), [Microsoft.Win32.RegistryValueKind]::ExpandString) }"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  !insertmacro RunUserPathPowerShell "$$key = [Microsoft.Win32.Registry]::CurrentUser.OpenSubKey('Environment', $$true); $$path = [string]$$key.GetValue('Path', '', 'DoNotExpandEnvironmentNames'); $$parts = @($$path -split ';' | Where-Object { $$_ -ne '' -and $$_ -ne $$dir }); $$key.SetValue('Path', ($$parts -join ';'), [Microsoft.Win32.RegistryValueKind]::ExpandString)"
!macroend
