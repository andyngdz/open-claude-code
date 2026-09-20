use super::*;

use crate::features::settings::AppSettings;

const LAUNCHED_MODEL: &str = "qwen3.8-max";
const OTHER_MODEL: &str = "qwen3.8-flash";

fn declared(settings: &AppSettings, model_id: &str) -> ContextWindow {
    declared_window(
        &settings.aliases,
        settings.launch_model_id.as_deref(),
        settings.launch_extended_context,
        model_id,
    )
}

fn settings_with_default_row(launch_model_id: &str, declaration: ContextWindow) -> AppSettings {
    let mut settings = AppSettings::default();
    settings.launch_model_id = Some(launch_model_id.to_owned());
    settings.launch_extended_context = declaration;
    settings
}

fn settings_with_opus_row(opus_model_id: &str, declaration: ContextWindow) -> AppSettings {
    let mut settings = AppSettings::default();
    settings.aliases.opus = opus_model_id.to_owned();
    settings.aliases.extended.opus = declaration;
    settings
}

#[test]
fn unticked_rows_declare_the_standard_window() {
    let settings = AppSettings::default();

    assert_eq!(declared(&settings, LAUNCHED_MODEL), ContextWindow::Standard);
}

#[test]
fn a_ticked_alias_row_declares_one_million_for_the_model_it_points_at() {
    let settings = settings_with_opus_row(LAUNCHED_MODEL, ContextWindow::OneMillion);

    assert_eq!(
        declared(&settings, LAUNCHED_MODEL),
        ContextWindow::OneMillion
    );
    assert_eq!(declared(&settings, OTHER_MODEL), ContextWindow::Standard);
}

#[test]
fn the_default_row_declares_one_million_for_the_model_it_launches() {
    let settings = settings_with_default_row(LAUNCHED_MODEL, ContextWindow::OneMillion);

    assert_eq!(
        declared(&settings, LAUNCHED_MODEL),
        ContextWindow::OneMillion
    );
}

#[test]
fn the_default_row_does_not_declare_for_another_model() {
    let settings = settings_with_default_row(LAUNCHED_MODEL, ContextWindow::OneMillion);

    assert_eq!(declared(&settings, OTHER_MODEL), ContextWindow::Standard);
}

#[test]
fn an_unticked_default_row_leaves_a_ticked_alias_row_alone() {
    let mut settings = settings_with_opus_row(LAUNCHED_MODEL, ContextWindow::OneMillion);
    settings.launch_model_id = Some(LAUNCHED_MODEL.to_owned());

    assert_eq!(
        declared(&settings, LAUNCHED_MODEL),
        ContextWindow::OneMillion
    );
}

#[test]
fn the_default_row_declares_without_any_ticked_alias_row() {
    let mut settings = settings_with_default_row(LAUNCHED_MODEL, ContextWindow::OneMillion);
    settings.aliases.opus = LAUNCHED_MODEL.to_owned();

    assert_eq!(
        declared(&settings, LAUNCHED_MODEL),
        ContextWindow::OneMillion
    );
}

#[test]
fn a_row_without_a_launch_model_declares_nothing() {
    assert_eq!(
        default_row_window(None, ContextWindow::OneMillion, LAUNCHED_MODEL),
        ContextWindow::Standard
    );
}
