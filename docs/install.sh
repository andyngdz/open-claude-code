#!/bin/sh
set -eu

repository="andyngdz/open-claude-code"
release_api="https://api.github.com/repos/${repository}/releases/latest"
release_page="https://github.com/${repository}/releases/latest"

# Downloads $1 into $2, or prints it when $2 is -.
#
# A stdout destination is the release metadata: quiet, so the transfer table
# does not run over the installer's own output. A file destination is a release
# asset the user waits on, so it keeps its meter.
download() {
  local url="$1" destination="$2"

  if command -v curl >/dev/null 2>&1; then
    if [ "$destination" = '-' ]; then
      curl --fail --location --retry 3 --silent --show-error "$url"
      return
    fi

    curl --fail --location --retry 3 --output "$destination" "$url"
    return
  fi

  if command -v wget >/dev/null 2>&1; then
    if [ "$destination" = '-' ]; then
      wget --quiet --output-document=- "$url"
      return
    fi

    wget --output-document="$destination" "$url"
    return
  fi

  printf '%s\n' "Install curl or wget, then run this script again." >&2
  exit 1
}

# Prints the first release asset download URL in $2 whose file name ends with
# $1. Only browser_download_url values are scanned, so a link pasted into the
# release notes cannot win over the real asset.
select_asset_url() {
  local asset_pattern="$1" release_metadata="$2"
  printf '%s' "$release_metadata" |
    sed -n 's/.*"browser_download_url": *"\([^"]*\)".*/\1/p' |
    grep "${asset_pattern}" |
    head -n 1
}

# Copies the launcher entry and icon out of the AppImage in $1, staging the
# extraction under $2.
install_appimage_launcher() {
  local appimage_path="$1" staging_directory="$2"
  local data_directory="${XDG_DATA_HOME:-${HOME}/.local/share}"
  local launcher_directory="${data_directory}/applications"
  local desktop_entry_path="${launcher_directory}/open-claude-code.desktop"
  local icon_directory="${data_directory}/icons/hicolor/128x128/apps"
  local icon_path="${icon_directory}/open-claude-code.png"
  local extraction_directory="${staging_directory}/appimage-assets"
  local extracted_desktop_entry_path="${extraction_directory}/squashfs-root/usr/share/applications/Open Claude Code.desktop"
  local extracted_icon_path="${extraction_directory}/squashfs-root/usr/share/icons/hicolor/128x128/apps/open-claude-code.png"

  mkdir -p "$launcher_directory" "$icon_directory" "$extraction_directory"
  if (
    cd "$extraction_directory" &&
    "$appimage_path" --appimage-extract 'usr/share/applications' >/dev/null 2>&1 &&
    "$appimage_path" --appimage-extract 'usr/share/icons/hicolor/128x128/apps/open-claude-code.png' >/dev/null 2>&1
  ) && [ -f "$extracted_desktop_entry_path" ] && [ -f "$extracted_icon_path" ]; then
    sed 's|^Exec=.*|Exec=/bin/sh -c "\\\\$HOME/.local/bin/open-claude-code.AppImage"|' \
      "$extracted_desktop_entry_path" > "$desktop_entry_path"
    cp "$extracted_icon_path" "$icon_path"
    return
  fi

  printf '%s\n' "Could not extract the AppImage launcher files. Download manually: $release_page" >&2
  return 1
}

# Writes a launcher for the executable in $1, which replaces this process so the
# command keeps its exit status.
write_exec_wrapper() {
  local exec_path="$1" wrapper_path="$2" escaped_path
  # Paths we write have spaces (macOS .app), so the exec target is single-quoted.
  # An apostrophe would close that quote early; escape it first.
  escaped_path="$(printf '%s' "$exec_path" | sed "s/'/'\\\\''/g")"
  printf '#!/bin/sh\nexec %s "$@"\n' "'$escaped_path'" > "$wrapper_path"
  chmod +x "$wrapper_path"
}

# Reports success, naming the directory when it still has to be added to PATH.
print_install_ok() {
  local install_directory="$1"
  case ":${PATH}:" in
    *":${install_directory}:"*)
      printf '%s\n' "Installed Open Claude Code. Run: open-claude-code launch"
      ;;
    *)
      printf '%s\n' "Installed Open Claude Code. Add ${install_directory} to PATH, then run: open-claude-code launch"
      ;;
  esac
}

# Opens the freshly installed app in $1 so the local gateway is up right away.
#
# Nothing here may fail the install: a headless session, a copy already running,
# or a launcher that refuses are each left alone rather than reported as an
# install error.
start_installed_app() {
  local app_path="$1"

  if [ "$(uname -s)" = 'Darwin' ]; then
    # The bundle path can hold spaces, so -a takes the whole path as one argument.
    open -a "$app_path" >/dev/null 2>&1 || true
    return
  fi

  if [ -z "${DISPLAY:-}" ] && [ -z "${WAYLAND_DISPLAY:-}" ]; then
    return
  fi

  # pgrep matches the whole command line, so a running copy is left alone
  # instead of opening a second window.
  if command -v pgrep >/dev/null 2>&1 && pgrep -f "$app_path" >/dev/null 2>&1; then
    return
  fi

  "$app_path" >/dev/null 2>&1 &
}

main() {
  local temporary_directory release_metadata operating_system machine_architecture
  local asset_pattern asset_url package_path install_kind install_command
  local disk_image mount_point app_path installed_app cli_wrapper

  temporary_directory="$(mktemp -d)"
  trap 'rm -rf "$temporary_directory"' EXIT INT TERM

  release_metadata="$(download "$release_api" -)"
  operating_system="$(uname -s)"
  machine_architecture="$(uname -m)"

  case "$operating_system" in
    Darwin)
      case "$machine_architecture" in
        arm64)
          asset_pattern='_aarch64\.dmg'
          ;;
        x86_64)
          asset_pattern='_x64\.dmg'
          ;;
        *)
          printf '%s\n' "macOS architecture $machine_architecture is not supported. Download manually: $release_page" >&2
          exit 1
          ;;
      esac

      asset_url="$(select_asset_url "$asset_pattern" "$release_metadata")"
      if [ -z "$asset_url" ]; then
        printf '%s\n' "No macOS installer was found. Download manually: $release_page" >&2
        exit 1
      fi

      disk_image="$temporary_directory/Open.Claude.Code.dmg"
      mount_point="$temporary_directory/mount"
      download "$asset_url" "$disk_image"
      mkdir -p "$mount_point"
      # -mountpoint picks the path here, so hdiutil's mount table never has to
      # be parsed to find out where the volume landed.
      if ! hdiutil attach -nobrowse -mountpoint "$mount_point" "$disk_image" >/dev/null; then
        printf '%s\n' "Could not open the macOS installer. Download manually: $release_page" >&2
        exit 1
      fi

      app_path="$(find "$mount_point" -maxdepth 1 -name '*.app' -print -quit)"
      if [ -z "$app_path" ]; then
        hdiutil detach "$mount_point" >/dev/null
        printf '%s\n' "The macOS installer did not contain an app. Download manually: $release_page" >&2
        exit 1
      fi

      installed_app="/Applications/$(basename "$app_path")"
      sudo ditto "$app_path" "$installed_app"
      hdiutil detach "$mount_point" >/dev/null
      cli_wrapper="$temporary_directory/open-claude-code"
      write_exec_wrapper "$installed_app/Contents/MacOS/open-claude-code" "$cli_wrapper"
      sudo mkdir -p /usr/local/bin
      sudo install -m 755 "$cli_wrapper" /usr/local/bin/open-claude-code
      print_install_ok /usr/local/bin
      start_installed_app "$installed_app"
      ;;
    Linux)
      case "$machine_architecture" in
        x86_64|amd64)
          ;;
        *)
          printf '%s\n' "Linux architecture $machine_architecture is not supported. Download manually: $release_page" >&2
          exit 1
          ;;
      esac

      if command -v apt-get >/dev/null 2>&1; then
        asset_pattern='_amd64\.deb'
        package_path="$temporary_directory/open-claude-code.deb"
        install_kind='package'
        install_command='sudo apt-get install --yes'
      elif command -v dnf >/dev/null 2>&1; then
        asset_pattern='\.x86_64\.rpm'
        package_path="$temporary_directory/open-claude-code.rpm"
        install_kind='package'
        install_command='sudo dnf install --assumeyes'
      elif command -v yum >/dev/null 2>&1; then
        asset_pattern='\.x86_64\.rpm'
        package_path="$temporary_directory/open-claude-code.rpm"
        install_kind='package'
        install_command='sudo yum install --assumeyes'
      else
        asset_pattern='_amd64\.AppImage'
        package_path="${HOME}/.local/bin/open-claude-code.AppImage"
        install_kind='appimage'
        install_command='chmod +x'
        mkdir -p "${HOME}/.local/bin"
      fi

      asset_url="$(select_asset_url "$asset_pattern" "$release_metadata")"
      if [ -z "$asset_url" ]; then
        printf '%s\n' "No Linux installer was found. Download manually: $release_page" >&2
        exit 1
      fi

      download "$asset_url" "$package_path"
      # install_command carries the command and its flags, so it is split on purpose.
      $install_command "$package_path"
      if [ "$install_kind" = 'appimage' ]; then
        install_appimage_launcher "$package_path" "$temporary_directory"
        write_exec_wrapper "$package_path" "${HOME}/.local/bin/open-claude-code"
        print_install_ok "${HOME}/.local/bin"
        start_installed_app "${HOME}/.local/bin/open-claude-code"
      else
        print_install_ok /usr/bin
        start_installed_app /usr/bin/open-claude-code
      fi
      ;;
    *)
      printf '%s\n' "This installer supports macOS and Linux. For Windows, download the MSI from: $release_page" >&2
      exit 1
      ;;
  esac
}

# scripts/install_cli_wrapper_test.sh sets this to source the helpers without
# running an install.
if [ "${INSTALL_SH_LIB_ONLY:-0}" != 1 ]; then
  main
fi
