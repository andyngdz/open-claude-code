use axum::http::StatusCode;

use super::ResponsePresenter;

#[test]
fn authentication_errors_use_unauthorized_status() {
    let response = ResponsePresenter::authentication_error("invalid");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
