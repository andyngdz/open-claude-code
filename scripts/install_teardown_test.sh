#!/bin/sh
set -eu

# Runs the published installer end to end and checks how it finishes.
#
# The install is stubbed at every edge that leaves the process: the network, the
# release metadata, and the AppImage it downloads. No package manager is on the
# stub PATH, so the run takes the AppImage branch on any host, and HOME, TMPDIR,
# and the working directory all point into a sandbox, so nothing lands in the
# real system or next to the caller.

report_failure() {
  printf '%s\n' "$1" >&2
  exit 1
}

installer_path="$(cd "$(dirname "$0")/.." && pwd)/docs/install.sh"

sandbox="$(mktemp -d)"
trap 'rm -rf "$sandbox"' EXIT INT TERM

stub_bin="${sandbox}/bin"
staging_root="${sandbox}/tmp"
install_home="${sandbox}/home"
run_directory="${sandbox}/run"
mkdir -p "$stub_bin" "$staging_root" "$install_home" "$run_directory"

# The installer picks its install kind by probing for a package manager, so the
# stub PATH deliberately carries none.
for tool in sh mktemp rm mkdir cp chmod sed grep head basename cat; do
  tool_path="$(command -v "$tool")" || report_failure "${tool} is missing from PATH"
  ln -s "$tool_path" "${stub_bin}/${tool}"
done

# Writes the executable at $1 from the script on stdin.
write_stub() {
  local stub_path="$1"
  cat > "$stub_path"
  chmod +x "$stub_path"
}

# Prints the platform the AppImage branch expects.
write_stub "${stub_bin}/uname" <<'STUB'
#!/bin/sh
case "${1:-}" in
  -m) printf '%s\n' x86_64 ;;
  *) printf '%s\n' Linux ;;
esac
STUB

# Serves the release metadata on stdout, and the fake asset into --output. The
# real asset is the whole app bundle over the network; both live in files the
# test owns.
write_stub "${stub_bin}/curl" <<'STUB'
#!/bin/sh
destination=''
while [ "$#" -gt 0 ]; do
  case "$1" in
    --output) destination="$2"; shift 2 ;;
    --*) shift ;;
    *) shift ;;
  esac
done
if [ -n "$destination" ]; then
  cp "$FAKE_APPIMAGE" "$destination"
else
  cat "$RELEASE_METADATA"
fi
STUB

release_metadata="${sandbox}/release.json"
cat > "$release_metadata" <<'JSON'
{"tag_name":"v0.0.0","assets":[{"name":"open-claude-code_0.0.0_amd64.AppImage","browser_download_url":"https://example.invalid/open-claude-code_0.0.0_amd64.AppImage"}]}
JSON

# The installer runs the asset with --appimage-extract and copies what comes
# back, so the stand-in only has to produce those two files relative to the
# directory it is run from.
fake_appimage="${sandbox}/fake.AppImage"
write_stub "$fake_appimage" <<'STUB'
#!/bin/sh
mkdir -p 'squashfs-root/usr/share/applications' 'squashfs-root/usr/share/icons/hicolor/128x128/apps'
printf '%s\n' 'Icon=open-claude-code' 'Exec=open-claude-code' > 'squashfs-root/usr/share/applications/Open Claude Code.desktop'
printf '%s\n' 'icon' > 'squashfs-root/usr/share/icons/hicolor/128x128/apps/open-claude-code.png'
STUB

stdout_path="${sandbox}/stdout"
stderr_path="${sandbox}/stderr"

status=0
(
  # Without a display the installer leaves the app closed, so the run is limited
  # to what this test checks. The run directory is a throwaway so anything the
  # installer writes relative to where it was called stays in the sandbox.
  cd "$run_directory"
  unset DISPLAY WAYLAND_DISPLAY
  TMPDIR="$staging_root" \
  HOME="$install_home" \
  XDG_DATA_HOME="${install_home}/.local/share" \
  PATH="$stub_bin" \
  FAKE_APPIMAGE="$fake_appimage" \
  RELEASE_METADATA="$release_metadata" \
    sh "$installer_path" > "$stdout_path" 2> "$stderr_path"
) || status=$?

if [ "$status" -ne 0 ]; then
  report_failure "the installer exited ${status}: $(cat "$stderr_path")"
fi

if [ -s "$stderr_path" ]; then
  report_failure "the installer reported an error: $(cat "$stderr_path")"
fi

# A run that bailed out early would exit clean and clean up nothing, so the
# install has to have reached its last step before the assertions below count.
cli_wrapper="${install_home}/.local/bin/open-claude-code"
if [ ! -x "$cli_wrapper" ]; then
  report_failure "the installer did not write ${cli_wrapper}: $(cat "$stdout_path")"
fi

# The staging directory holds the whole downloaded package, so a run that leaves
# it behind fills the disk quietly.
leftover="$(ls -A "$staging_root")"
if [ -n "$leftover" ]; then
  report_failure "the installer left its staging directory behind: ${leftover}"
fi

# Everything the installer writes belongs in HOME, XDG_DATA_HOME, or the staging
# directory. Anything landing next to the caller is a stray file in their tree.
run_leftover="$(ls -A "$run_directory")"
if [ -n "$run_leftover" ]; then
  report_failure "the installer wrote into its working directory: ${run_leftover}"
fi

printf '%s\n' "ok"
