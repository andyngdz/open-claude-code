#!/bin/sh
set -eu

repository="andyngdz/open-claude-code"
release_api="https://api.github.com/repos/${repository}/releases/latest"
release_page="https://github.com/${repository}/releases/latest"
temporary_directory="$(mktemp -d)"

cleanup() {
  rm -rf "$temporary_directory"
}

trap cleanup EXIT INT TERM

fetch_release_metadata() {
  if command -v curl >/dev/null 2>&1; then
    curl --fail --location --silent --show-error "$release_api"
    return
  fi

  if command -v wget >/dev/null 2>&1; then
    wget --quiet --output-document=- "$release_api"
    return
  fi

  printf '%s\n' "Install curl or wget, then run this script again." >&2
  exit 1
}

download_asset() {
  asset_url="$1"
  destination="$2"

  if command -v curl >/dev/null 2>&1; then
    curl --fail --location --retry 3 --output "$destination" "$asset_url"
    return
  fi

  wget --output-document="$destination" "$asset_url"
}

select_asset_url() {
  asset_pattern="$1"
  printf '%s' "$release_metadata" | grep -o "https://[^\"]*${asset_pattern}" | head -n 1
}

release_metadata="$(fetch_release_metadata)"
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

    asset_url="$(select_asset_url "$asset_pattern")"
    if [ -z "$asset_url" ]; then
      printf '%s\n' "No macOS installer was found. Download manually: $release_page" >&2
      exit 1
    fi

    disk_image="$temporary_directory/Open.Claude.Code.dmg"
    download_asset "$asset_url" "$disk_image"
    volume_path="$(hdiutil attach -nobrowse "$disk_image" | awk '/\/Volumes\// { print substr($0, index($0, "/Volumes/")); exit }')"
    app_path="$(find "$volume_path" -maxdepth 1 -name '*.app' -print -quit)"
    if [ -z "$app_path" ]; then
      hdiutil detach "$volume_path" >/dev/null
      printf '%s\n' "The macOS installer did not contain an app. Download manually: $release_page" >&2
      exit 1
    fi

    sudo ditto "$app_path" "/Applications/$(basename "$app_path")"
    hdiutil detach "$volume_path" >/dev/null
    printf '%s\n' "Installed Open Claude Code in /Applications."
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
      asset_url="$(select_asset_url '_amd64\.deb')"
      package_path="$temporary_directory/open-claude-code.deb"
      install_command='sudo apt-get install --yes'
    elif command -v dnf >/dev/null 2>&1; then
      asset_url="$(select_asset_url '\.x86_64\.rpm')"
      package_path="$temporary_directory/open-claude-code.rpm"
      install_command='sudo dnf install --assumeyes'
    elif command -v yum >/dev/null 2>&1; then
      asset_url="$(select_asset_url '\.x86_64\.rpm')"
      package_path="$temporary_directory/open-claude-code.rpm"
      install_command='sudo yum install --assumeyes'
    else
      asset_url="$(select_asset_url '_amd64\.AppImage')"
      package_path="${HOME}/.local/bin/open-claude-code.AppImage"
      install_command='chmod +x'
      mkdir -p "${HOME}/.local/bin"
    fi

    if [ -z "$asset_url" ]; then
      printf '%s\n' "No Linux installer was found. Download manually: $release_page" >&2
      exit 1
    fi

    download_asset "$asset_url" "$package_path"
    $install_command "$package_path"
    printf '%s\n' "Installed Open Claude Code."
    ;;
  *)
    printf '%s\n' "This installer supports macOS and Linux. For Windows, download the MSI from: $release_page" >&2
    exit 1
    ;;
esac
