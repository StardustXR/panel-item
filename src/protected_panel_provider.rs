use std::{
    env,
    sync::atomic::{AtomicBool, Ordering},
};

use gluon_ipc::Ref;

use crate::panel_item_acceptor::PanelItemAcceptor;
pub use crate::protocol::protected_panel_provider::*;
static ALREADY_TRIED_ENV: AtomicBool = AtomicBool::new(false);

pub async fn get_env_var_panel(acceptor: PanelItemAcceptor) -> bool {
    if ALREADY_TRIED_ENV.swap(true, Ordering::Relaxed) {
        return false;
    };
    let Ok(env_var) = env::var("SDXR_PROTECTED_PANEL") else {
        return false;
    };
    let mut parts = env_var.split(':');
    if let Some(part) = parts.next() {
        if part != "path" {
            tracing::warn!("unknown protected panel getting protocol: {part}");
            return false;
        }
    } else {
        return false;
    }
    let Some(path_name) = parts.next() else {
        tracing::warn!("got protected panel without name");
        return false;
    };
    let Some(secret) = parts.next() else {
        tracing::warn!("got protected panel without secret");
        return false;
    };
    let Some(ref_path) = xdg::BaseDirectories::new()
        .runtime_dir
        .map(|v| v.join("stardust-protected-panel-provider").join(path_name))
    else {
        tracing::warn!("failed to get runtime dir?");
        return false;
    };

    let Ok(v) = Ref::connect(ref_path).await else {
        return false;
    };

    let Ok(res) = ProtectedPanelProvider::from_ref(v)
        .get_panel(secret, acceptor)
        .await
    else {
        return false;
    };

    match res {
        ProtectedPanelGetResult::InvalidSecret => false,
        ProtectedPanelGetResult::InvalidAcceptor => false,
        ProtectedPanelGetResult::AlreadyConsumed => false,
        ProtectedPanelGetResult::Success => true,
    }
}
