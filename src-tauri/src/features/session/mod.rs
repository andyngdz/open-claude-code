//! Desktop session that owns settings, the OpenCode Go backend, and launched processes.

mod domain;
mod errors;
mod service;

pub(crate) use domain::DashboardSnapshot;
pub(crate) use errors::SessionError;
pub(crate) use service::AppSession;
