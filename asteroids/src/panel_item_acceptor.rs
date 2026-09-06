use crate::panel_shell::PanelShellHandler;
use gluon::{Handler, Interface, Node, RefExt};
use stardust_xr_asteroids::{
    Component, ComponentCreateInfo, Context, FnWrapper, Inners, ValidState,
};
use stardust_xr_fusion::{
    Error,
    query::QueryableInterface,
    spatial::{CreatedSpatial, SpatialInterface, SpatialRef, Transform},
};
use stardust_xr_panel_item::{
    panel_item::PanelItem,
    panel_item_acceptor::{self, PanelItemAcceptorHandler as _},
};
use std::sync::Mutex;
use tokio::sync::mpsc;

#[derive_where::derive_where(Debug)]
pub struct PanelItemAcceptor<State: ValidState> {
    on_create_item: FnWrapper<dyn Fn(&mut State, Node<PanelShellHandler>) + Send + Sync>,
}
impl<State: ValidState> PanelItemAcceptor<State> {
    pub fn new(
        on_accept: impl Fn(&mut State, Node<PanelShellHandler>) + Send + Sync + 'static,
    ) -> Self {
        Self {
            on_create_item: FnWrapper(Box::new(on_accept)),
        }
    }
}

pub struct PanelItemAcceptorInner {
    node: Node<PanelItemAcceptorHandler>,
    // the entity owns the shared queryable; we just hold our interface guard on it
    _interface: QueryableInterface,
}

impl<State: ValidState> Component<State> for PanelItemAcceptor<State> {
    type Inner = PanelItemAcceptorInner;
    type Error = Error;

    async fn create_inner(
        &self,
        ctx: &Context,
        info: ComponentCreateInfo<'_>,
    ) -> Result<Self::Inner, Self::Error> {
        let (tx, rx) = mpsc::unbounded_channel();
        let (node, panel_ref) =
            panel_item_acceptor::PanelItemAcceptor::new_node(PanelItemAcceptorHandler {
                tx,
                rx: Mutex::new(rx),
                spatial_ref: info.spatial.spatial_ref().await?,
                spatial_interface: ctx.stardust_client.spatial_interface().clone(),
            })?;
        let _interface = info
            .queryable
            .add_interface(&panel_ref, panel_item_acceptor::PanelItemAcceptor::ID)
            .await??;

        Ok(PanelItemAcceptorInner { node, _interface })
    }

    fn diff(
        &self,
        _old: &Self,
        _context: &Context,
        _create_info: ComponentCreateInfo<'_>,
        _inners: &mut Inners<'_, State, Self>,
    ) {
    }
    fn frame(
        &self,
        _context: &Context,
        _info: &stardust_xr_fusion::client::FrameInfo,
        state: &mut State,
        inners: &mut Inners<'_, State, Self>,
    ) {
        let inner = inners.self_inner();
        while let Ok(shell) = inner.node.rx.lock().unwrap().try_recv() {
            self.on_create_item.0(state, shell)
        }
    }
}

#[derive(Debug, Handler)]
pub struct PanelItemAcceptorHandler {
    spatial_ref: SpatialRef,
    spatial_interface: SpatialInterface,
    tx: mpsc::UnboundedSender<Node<PanelShellHandler>>,
    rx: Mutex<mpsc::UnboundedReceiver<Node<PanelShellHandler>>>,
}
impl stardust_xr_panel_item::panel_item_acceptor::PanelItemAcceptorHandler
    for PanelItemAcceptorHandler
{
    async fn accept(
        &self,
        _ctx: gluon::Context,
        item: PanelItem,
    ) -> (stardust_xr_panel_item::panel_item::PanelShell, SpatialRef) {
        let CreatedSpatial {
            spatial,
            spatial_ref,
        } = self
            .spatial_interface
            .create_spatial(self.spatial_ref.clone(), Transform::IDENTITY)
            .await
            .unwrap()
            .unwrap();

        let (panel_shell, panel_shell_ref) = PanelShellHandler::new(item.clone(), spatial).unwrap();
        self.tx.send(panel_shell).unwrap();
        (panel_shell_ref.into_proxy(), spatial_ref)
    }
}
