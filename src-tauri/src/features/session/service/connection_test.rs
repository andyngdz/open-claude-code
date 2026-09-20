use std::path::PathBuf;

use open_claude_code_backend::ProviderConnectionState;

use super::published_connection;
use crate::features::gateway::GatewayHost;

/// Path no process is installed at, so the host owns no gateway.
const MISSING_EXECUTABLE: &str = "/nonexistent/open-claude-code";

#[tokio::test]
async fn a_gateway_the_app_does_not_own_is_reported_as_a_failed_connection() {
    let gateway = GatewayHost::new(PathBuf::from(MISSING_EXECUTABLE));

    let connection = published_connection(&gateway).await;

    assert!(matches!(connection, ProviderConnectionState::Failed { .. }));
}
