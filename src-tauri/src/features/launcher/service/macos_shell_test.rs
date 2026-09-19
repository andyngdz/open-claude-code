use super::applescript_escape;

#[test]
fn applescript_escape_doubles_backslashes_and_quotes() {
    assert_eq!(applescript_escape("say \"hi\""), "say \\\"hi\\\"");
}
