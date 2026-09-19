use std::{fs, os::unix::fs::PermissionsExt, path::Path, process::Command};

use uuid::Uuid;

use super::{applescript_escape, write_launch_script};
use crate::features::launcher::service::proxy::claude_proxy_env;
use crate::features::settings::ModelAliasMapping;

#[test]
fn applescript_escape_doubles_backslashes_and_quotes() {
    assert_eq!(applescript_escape("say \"hi\""), "say \\\"hi\\\"");
}

#[test]
fn write_launch_script_cds_into_the_selected_workspace() {
    let aliases = ModelAliasMapping::default();
    let proxy = claude_proxy_env("http://127.0.0.1:9", "token", &aliases);
    let script_path = write_launch_script(
        Path::new("/Users/me/src/app"),
        Path::new("/usr/bin/claude"),
        "qwen3.8-max",
        &proxy,
    )
    .expect("wrapper should write");
    let body = fs::read_to_string(&script_path).expect("wrapper should be readable");
    fs::remove_file(&script_path).ok();

    assert!(body.contains("cd '/Users/me/src/app' || exit 1"));
    assert!(!body.contains("cd '/var/folders"));
}

#[test]
fn launch_script_stops_when_the_workspace_is_unavailable() {
    let test_directory =
        std::env::temp_dir().join(format!("open-claude-code-test-{}", Uuid::new_v4()));
    let missing_workspace = test_directory.join("missing-workspace");
    let fake_claude = test_directory.join("claude");
    let result_path = test_directory.join("result");
    fs::create_dir_all(&test_directory).expect("test directory should be created");
    fs::write(
        &fake_claude,
        format!("#!/bin/zsh\nprintf ran > '{}'\n", result_path.display()),
    )
    .expect("fake Claude should be written");
    let mut permissions = fs::metadata(&fake_claude)
        .expect("fake Claude metadata should be readable")
        .permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(&fake_claude, permissions).expect("fake Claude should be executable");
    let aliases = ModelAliasMapping::default();
    let proxy = claude_proxy_env("http://127.0.0.1:9", "token", &aliases);
    let script_path = write_launch_script(&missing_workspace, &fake_claude, "model", &proxy)
        .expect("wrapper should write");

    let status = Command::new(&script_path)
        .status()
        .expect("wrapper should execute");

    assert!(!status.success());
    assert!(!result_path.exists());
    assert!(!script_path.exists());
    fs::remove_dir_all(test_directory).expect("test directory should be removable");
}

#[test]
fn launch_script_enters_the_workspace_passes_proxy_credentials_and_removes_itself() {
    let test_directory =
        std::env::temp_dir().join(format!("open-claude-code-test-{}", Uuid::new_v4()));
    let workspace = test_directory.join("workspace");
    let fake_claude = test_directory.join("claude");
    let result_path = test_directory.join("result");
    fs::create_dir_all(&workspace).expect("workspace should be created");
    fs::write(
        &fake_claude,
        format!(
            "#!/bin/zsh\npwd > '{}'\nprintf '%s' \"$ANTHROPIC_AUTH_TOKEN\" >> '{}'\n",
            result_path.display(),
            result_path.display()
        ),
    )
    .expect("fake Claude should be written");
    let mut permissions = fs::metadata(&fake_claude)
        .expect("fake Claude metadata should be readable")
        .permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(&fake_claude, permissions).expect("fake Claude should be executable");

    let aliases = ModelAliasMapping::default();
    let proxy = claude_proxy_env("http://127.0.0.1:9", "token", &aliases);
    let script_path = write_launch_script(&workspace, &fake_claude, "model", &proxy)
        .expect("wrapper should write");
    assert_eq!(
        fs::metadata(&script_path)
            .expect("wrapper metadata should be readable")
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
    let status = Command::new(&script_path)
        .status()
        .expect("wrapper should execute");

    assert!(status.success());
    assert!(!script_path.exists());
    assert_eq!(
        fs::read_to_string(&result_path).expect("fake Claude result should be readable"),
        format!("{}\ntoken", workspace.display())
    );
    fs::remove_dir_all(test_directory).expect("test directory should be removable");
}
