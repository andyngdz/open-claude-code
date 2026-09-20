use std::{env, path::PathBuf, process::Command};

const ENV_APPDIR: &str = "APPDIR";
const ENV_APPIMAGE: &str = "APPIMAGE";
const ENV_ARGV0: &str = "ARGV0";
const ENV_OWD: &str = "OWD";

/// Strips AppImage runtime paths from a spawned host process.
///
/// The desktop process needs the bundled GLib for WebKitGTK. Host terminals
/// must not inherit that `LD_LIBRARY_PATH`, or GTK binaries load the Ubuntu
/// runtime and fail on newer system libraries. Marker vars (`APPDIR`,
/// `APPIMAGE`, `ARGV0`, `OWD`) are dropped so the child does not look like
/// the AppImage.
pub(crate) fn apply_appimage_host_env(command: &mut Command) {
    let Ok(appdir) = env::var(ENV_APPDIR) else {
        return;
    };
    if appdir.is_empty() {
        return;
    }
    apply_env_overrides(command, appimage_host_env(&appdir, env::vars()));
}

fn apply_env_overrides(
    command: &mut Command,
    overrides: impl IntoIterator<Item = (String, Option<String>)>,
) {
    for (key, value) in overrides {
        match value {
            Some(cleaned) => {
                command.env(key, cleaned);
            }
            None => {
                command.env_remove(key);
            }
        }
    }
}

/// Returns env overrides that drop AppImage mount paths from inherited values.
///
/// `None` removes the variable. `Some` replaces it with the host-only remainder.
fn appimage_host_env(
    appdir: &str,
    env_vars: impl IntoIterator<Item = (String, String)>,
) -> Vec<(String, Option<String>)> {
    env_vars
        .into_iter()
        .filter_map(|(key, value)| {
            if is_appimage_marker(&key) {
                return Some((key, None));
            }
            if !value.contains(appdir) {
                return None;
            }
            Some((key, without_appimage_paths(appdir, &value)))
        })
        .collect()
}

fn is_appimage_marker(key: &str) -> bool {
    key == ENV_APPDIR || key == ENV_APPIMAGE || key == ENV_ARGV0 || key == ENV_OWD
}

fn without_appimage_paths(appdir: &str, value: &str) -> Option<String> {
    let kept: Vec<_> = env::split_paths(value)
        .filter(|segment| !segment.to_string_lossy().contains(appdir))
        .collect();
    if kept.is_empty() {
        return None;
    }
    env::join_paths(&kept)
        .ok()
        .map(|joined| joined.to_string_lossy().into_owned())
}

/// Returns the directory a launched process should start in.
///
/// The AppImage runtime moves the process into `$APPDIR/usr` and records the
/// directory the user ran it from in `OWD`. A CLI launch execs Claude Code in
/// place, so the child inherits the mount unless the launch puts this directory
/// back. A package install has no runtime rewriting the directory, so it has no
/// `OWD` and keeps the inherited one.
pub(crate) fn host_working_directory() -> Option<PathBuf> {
    let directory = recorded_working_directory(env::var(ENV_OWD).ok())?;
    // The recorded directory can be gone by the time the launch runs.
    directory.is_dir().then_some(directory)
}

/// Returns the directory the AppImage runtime recorded, treating a blank value
/// as absent.
fn recorded_working_directory(recorded: Option<String>) -> Option<PathBuf> {
    let recorded = recorded?;
    let trimmed = recorded.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(PathBuf::from(trimmed))
}

#[cfg(test)]
#[path = "appimage_env_test.rs"]
mod appimage_env_test;
