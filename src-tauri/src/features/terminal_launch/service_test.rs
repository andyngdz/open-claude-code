use super::{launch, CliError};

#[test]
fn launch_without_a_running_app_reports_not_running() {
    let error = launch(Some("missing".to_owned())).unwrap_err();

    assert!(matches!(
        error,
        CliError::NotRunning | CliError::UnknownModel
    ));
}
