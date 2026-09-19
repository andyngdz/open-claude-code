use std::path::Path;

use super::resolve_macos_command;
use crate::features::launcher::service::proxy::claude_proxy_env;
use crate::features::launcher::TerminalKind;
use crate::features::settings::ModelAliasMapping;

#[test]
fn system_default_resolves_to_terminal_osascript() {
    let aliases = ModelAliasMapping::default();
    let proxy = claude_proxy_env("http://127.0.0.1:9", "token", &aliases);
    let command = resolve_macos_command(
        TerminalKind::SystemDefault,
        Path::new("/tmp/project"),
        Path::new("/usr/bin/claude"),
        "qwen3.8-max",
        &proxy,
    )
    .expect("system default should resolve");

    assert_eq!(command.program.as_os_str(), "/usr/bin/osascript");
    assert!(command.arguments[1].contains("do script"));
    assert!(command.arguments[1].contains("Terminal"));
}
