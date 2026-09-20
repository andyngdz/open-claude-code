#!/bin/sh
set -eu

temporary_directory="$(mktemp -d)"
trap 'rm -rf "$temporary_directory"' EXIT INT TERM

# Source the published installer rather than restating the helper, so the test
# cannot pass against a copy that drifted from the script users run.
# INSTALL_SH_LIB_ONLY stops the source from running an install.
installer_path="$(dirname "$0")/../docs/install.sh"
INSTALL_SH_LIB_ONLY=1
. "$installer_path"
unset INSTALL_SH_LIB_ONLY

if ! command -v start_installed_app >/dev/null 2>&1; then
  printf '%s\n' "start_installed_app is missing from ${installer_path}" >&2
  exit 1
fi

report_failure() {
  printf '%s\n' "$1" >&2
  exit 1
}

# Writes a stub at $1 that appends one line to $2 and then runs $3, standing in
# for the installed app so the test can see whether it was opened.
write_stub() {
  local stub_path="$1" log_path="$2" afterwards="$3"
  {
    printf '%s\n' '#!/bin/sh'
    printf 'printf "started\\n" >> "%s"\n' "$log_path"
    printf '%s\n' "$afterwards"
  } > "$stub_path"
  chmod +x "$stub_path"
}

# Waits up to a second for $1 to appear.
wait_for_file() {
  local attempt=0
  while [ ! -e "$1" ] && [ "$attempt" -lt 20 ]; do
    sleep 0.05
    attempt=$((attempt + 1))
  done
  [ -e "$1" ]
}

stub="${temporary_directory}/open-claude-code"
log_path="${temporary_directory}/starts.log"
write_stub "$stub" "$log_path" ':'

# A headless session has no window to open, so the installed app stays closed.
unset DISPLAY WAYLAND_DISPLAY
start_installed_app "$stub"
sleep 0.2
if [ -e "$log_path" ]; then
  report_failure "the app opened without a display"
fi

DISPLAY=':0'
export DISPLAY
start_installed_app "$stub"
if ! wait_for_file "$log_path"; then
  report_failure "the app did not open with a display available"
fi

# A copy that is already up must not be opened a second time.
running_stub="${temporary_directory}/running-open-claude-code"
running_log_path="${temporary_directory}/running.log"
write_stub "$running_stub" "$running_log_path" 'sleep 30'
start_installed_app "$running_stub"
if ! wait_for_file "$running_log_path"; then
  report_failure "the stub the test runs in the background did not start"
fi

start_installed_app "$running_stub"
sleep 0.3
started_count="$(wc -l < "$running_log_path")"
pkill -f "$running_stub" >/dev/null 2>&1 || true
if [ "$started_count" -ne 1 ]; then
  report_failure "a running copy was opened ${started_count} times"
fi

printf '%s\n' "ok"
