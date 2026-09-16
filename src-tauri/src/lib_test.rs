use super::startup_failure;

#[test]
fn startup_failure_keeps_the_session_message() {
    let error = startup_failure("Settings could not be saved. Try again.".to_owned());
    assert_eq!(error.to_string(), "Settings could not be saved. Try again.");
}
