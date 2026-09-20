use std::{env, path::PathBuf};

use super::{
    appimage_host_env, apply_env_overrides, recorded_working_directory, ENV_APPDIR, ENV_APPIMAGE,
    ENV_ARGV0, ENV_OWD,
};

const APPDIR: &str = "/tmp/.mount_open-xxxx";

fn joined_paths(paths: &[&str]) -> String {
    env::join_paths(paths.iter().map(PathBuf::from))
        .expect("test paths should join")
        .to_string_lossy()
        .into_owned()
}

#[test]
fn appimage_library_path_is_removed_and_host_path_is_kept() {
    let overrides = appimage_host_env(
        APPDIR,
        [
            (
                "LD_LIBRARY_PATH".to_owned(),
                joined_paths(&[
                    &format!("{APPDIR}/usr/lib/"),
                    &format!("{APPDIR}/usr/lib/x86_64-linux-gnu/"),
                ]),
            ),
            (
                "PATH".to_owned(),
                joined_paths(&[&format!("{APPDIR}/usr/bin/"), "/usr/bin", "/bin"]),
            ),
            ("HOME".to_owned(), "/home/user".to_owned()),
        ],
    );

    assert_eq!(
        overrides,
        vec![
            ("LD_LIBRARY_PATH".to_owned(), None),
            ("PATH".to_owned(), Some(joined_paths(&["/usr/bin", "/bin"]))),
        ]
    );
}

#[test]
fn appimage_marker_variables_are_removed_even_when_values_are_outside_the_mount() {
    let overrides = appimage_host_env(
        APPDIR,
        [
            (ENV_APPDIR.to_owned(), APPDIR.to_owned()),
            (
                ENV_APPIMAGE.to_owned(),
                "/home/user/OpenClaude.AppImage".to_owned(),
            ),
            (
                ENV_ARGV0.to_owned(),
                "/home/user/OpenClaude.AppImage".to_owned(),
            ),
            (ENV_OWD.to_owned(), "/home/user/Projects/demo".to_owned()),
            ("HOME".to_owned(), "/home/user".to_owned()),
        ],
    );

    assert_eq!(
        overrides,
        vec![
            (ENV_APPDIR.to_owned(), None),
            (ENV_APPIMAGE.to_owned(), None),
            (ENV_ARGV0.to_owned(), None),
            (ENV_OWD.to_owned(), None),
        ]
    );
}

#[test]
fn appimage_single_path_variables_are_removed() {
    let overrides = appimage_host_env(
        APPDIR,
        [(
            "GIO_MODULE_DIR".to_owned(),
            format!("{APPDIR}/usr/lib/gio/modules"),
        )],
    );

    assert_eq!(overrides, vec![("GIO_MODULE_DIR".to_owned(), None)]);
}

#[test]
fn recorded_invoking_directory_becomes_the_working_directory() {
    assert_eq!(
        recorded_working_directory(Some("/home/user/Projects/demo".to_owned())),
        Some(PathBuf::from("/home/user/Projects/demo"))
    );
}

#[test]
fn absent_or_blank_recorded_directory_is_ignored() {
    assert_eq!(recorded_working_directory(None), None);
    assert_eq!(recorded_working_directory(Some(String::new())), None);
    assert_eq!(recorded_working_directory(Some("  ".to_owned())), None);
}

#[test]
#[cfg(unix)]
fn spawned_host_process_does_not_inherit_appimage_library_path() {
    let library_path = format!("{APPDIR}/usr/lib/");
    let mut command = std::process::Command::new("/usr/bin/sh");
    command
        .args(["-c", "test -z \"${LD_LIBRARY_PATH:-}\""])
        .env("LD_LIBRARY_PATH", &library_path);
    apply_env_overrides(
        &mut command,
        appimage_host_env(APPDIR, [("LD_LIBRARY_PATH".to_owned(), library_path)]),
    );

    let status = command.status().expect("host process should spawn");
    assert!(status.success());
}
