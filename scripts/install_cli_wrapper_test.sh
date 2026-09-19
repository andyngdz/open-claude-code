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

if ! command -v write_exec_wrapper >/dev/null 2>&1; then
  printf '%s\n' "write_exec_wrapper is missing from ${installer_path}" >&2
  exit 1
fi

check_wrapper() {
  local install_directory="$1"
  local wrapper="${temporary_directory}/open-claude-code"
  local output expected='3:launch:--model:x'

  write_exec_wrapper "${install_directory}/bin" "$wrapper"
  output="$("$wrapper" launch --model x)"
  if [ "$output" != "$expected" ]; then
    printf '%s\n' "expected ${expected}, got ${output} for ${install_directory}" >&2
    exit 1
  fi
}

# "${temporary_directory}/o'brien" covers the apostrophe the wrapper has to escape.
for install_directory in "${temporary_directory}/Open Claude" "${temporary_directory}/o'brien"; do
  mkdir -p "$install_directory"
  printf '%s\n' '#!/bin/sh' 'printf "%s:%s:%s:%s\n" "$#" "$1" "$2" "$3"' \
    > "${install_directory}/bin"
  chmod +x "${install_directory}/bin"
done

check_wrapper "${temporary_directory}/Open Claude"
check_wrapper "${temporary_directory}/o'brien"

printf '%s\n' "ok"
