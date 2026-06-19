use crate::panel_shell::PanelShellHandler;
use gluon::{Handler, Object, ToObjectOrRef};
use stardust_xr_asteroids::{Component, ComponentCreateInfo, Context, FnWrapper, ValidState};
use stardust_xr_fusion::{
    Error,
    query::{QueryableExt, QueryableInterfaceGuard, QueryableObject},
    spatial::{CreatedSpatial, SpatialInterface, SpatialRef, Transform},
};
use stardust_xr_panel_item::panel_item_acceptor::PanelItemAcceptorHandler as _;
use stardust_xr_panel_item::{
    panel_item::{PanelItem, PanelShell},
    panel_item_acceptor,
};
use std::sync::Mutex;
use tokio::sync::mpsc;

#[derive_where::derive_where(Debug)]
pub struct PanelItemAcceptor<State: ValidState> {
    on_create_item: FnWrapper<dyn Fn(&mut State, Object<PanelShellHandler>) + Send + Sync>,
}
impl<State: ValidState> PanelItemAcceptor<State> {
    pub fn new(
        on_accept: impl Fn(&mut State, Object<PanelShellHandler>) + Send + Sync + 'static,
    ) -> Self {
        Self {
            on_create_item: FnWrapper(Box::new(on_accept)),
        }
    }
}

pub struct PanelItemAcceptorInner {
    handler: Object<PanelItemAcceptorHandler>,
    _queryable: QueryableObject,
    _interface_guard: QueryableInterfaceGuard,
}

impl<State: ValidState> Component<State> for PanelItemAcceptor<State> {
    type Inner = PanelItemAcceptorInner;
    type Error = Error;

    async fn create_inner(
        &self,
        ctx: &Context,
        info: ComponentCreateInfo<'_>,
    ) -> Result<Self::Inner, Self::Error> {
        let client = &ctx.stardust_client;
        let (tx, rx) = mpsc::unbounded_channel();
        let handler = client
            .pion_device()
            .register_object(PanelItemAcceptorHandler {
                tx,
                rx: Mutex::new(rx),
                spatial_ref: info.spatial.spatial_ref().await?,
                spatial_interface: ctx.stardust_client.spatial_interface().clone(),
            });
        let _queryable =
            QueryableObject::new(client, info.spatial.clone(), info.field.clone()).await?;
        let _interface_guard = _queryable
            .add_interface(
                &handler,
                panel_item_acceptor::EXTERNAL_PROTOCOL.protocol_name,
            )
            .await?;

        Ok(PanelItemAcceptorInner {
            handler,
            _queryable,
            _interface_guard,
        })
    }

    fn diff(
        &self,
        _old: &Self,
        _context: &Context,
        _create_info: ComponentCreateInfo<'_>,
        _inner: &mut Self::Inner,
    ) {
    }
    fn frame(
        &self,
        _context: &Context,
        _info: &stardust_xr_fusion::client::FrameInfo,
        state: &mut State,
        inner: &mut Self::Inner,
    ) {
        while let Ok(shell) = inner.handler.rx.lock().unwrap().try_recv() {
            self.on_create_item.0(state, shell)
        }
    }
}

#[derive(Debug, Handler)]
pub struct PanelItemAcceptorHandler {
    spatial_ref: SpatialRef,
    spatial_interface: SpatialInterface,
    tx: mpsc::UnboundedSender<Object<PanelShellHandler>>,
    rx: Mutex<mpsc::UnboundedReceiver<Object<PanelShellHandler>>>,
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

        let panel_shell = PanelShellHandler::new(
            item.to_binder_object_or_ref().device(),
            item.clone(),
            spatial,
        );
        let proxy = PanelShell::from_handler(&panel_shell);
        self.tx.send(panel_shell).unwrap();
        (proxy, spatial_ref)
    }
}
