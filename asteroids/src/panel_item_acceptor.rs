use std::sync::Mutex;

use binderbinder::binder_object::{BinderObject, ToBinderObjectOrRef};
use gluon::Handler;
use stardust_xr_asteroids::{CustomElement, FnWrapper, Transformable, ValidState};
use stardust_xr_fusion::{
    Error,
    fields::{Field, FieldExt as _, Shape},
    query::{QueryableExt, QueryableInterfaceGuard, QueryableObject},
    spatial::{CreatedSpatial, Spatial, SpatialExt, SpatialInterface, SpatialRef, Transform},
};
use stardust_xr_panel_item::panel_item_acceptor::PanelItemAcceptorHandler as _;
use stardust_xr_panel_item::{
    panel_item::{PanelItem, PanelShell},
    panel_item_acceptor,
};
use tokio::sync::mpsc;

use crate::panel_shell::PanelShellHandler;

#[derive_where::derive_where(Debug)]
pub struct PanelItemAcceptor<State: ValidState> {
    transform: Transform,
    shape: Shape,
    on_create_item: FnWrapper<dyn Fn(&mut State, BinderObject<PanelShellHandler>) + Send + Sync>,
}
impl<State: ValidState> PanelItemAcceptor<State> {
    pub fn new(
        shape: Shape,
        on_accept: impl Fn(&mut State, BinderObject<PanelShellHandler>) + Send + Sync + 'static,
    ) -> Self {
        Self {
            transform: Transform::IDENTITY,
            shape,
            on_create_item: FnWrapper(Box::new(on_accept)),
        }
    }
}

pub struct PanelItemAcceptorInner {
    handler: BinderObject<PanelItemAcceptorHandler>,
    _queryable: QueryableObject,
    _interface_guard: QueryableInterfaceGuard,
    spatial: Spatial,
    field: Field,
}

impl<State: ValidState> CustomElement<State> for PanelItemAcceptor<State> {
    type Inner = PanelItemAcceptorInner;

    type Error = Error;

    async fn create_inner(
        &self,
        ctx: &stardust_xr_asteroids::Context,
        info: stardust_xr_asteroids::CreateInnerInfo,
    ) -> Result<Self::Inner, Self::Error> {
        let client = &ctx.stardust_client;
        let (spatial, spatial_ref) =
            Spatial::new(client, &info.parent_space, self.transform).await?;
        let (field, _) = Field::new(client, &spatial, self.shape.clone()).await?;
        let (tx, rx) = mpsc::unbounded_channel();
        let handler = client
            .pion_device()
            .register_object(PanelItemAcceptorHandler {
                tx,
                rx: Mutex::new(rx),
                spatial_ref,
                spatial_interface: ctx.stardust_client.spatial_interface().clone(),
            });
        let _queryable = QueryableObject::new(client, spatial.clone(), field.clone()).await?;
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
            spatial,
            field,
        })
    }

    fn diff(&self, old: &Self, _context: &stardust_xr_asteroids::Context, inner: &mut Self::Inner) {
        self.apply_transform(old, &inner.spatial);
        if self.shape != old.shape {
            let _ = inner.field.set_shape(self.shape.clone());
        }
    }
    fn frame(
        &self,
        _context: &stardust_xr_asteroids::Context,
        _info: &stardust_xr_fusion::client::FrameInfo,
        state: &mut State,
        inner: &mut Self::Inner,
    ) {
        while let Ok(shell) = inner.handler.rx.lock().unwrap().try_recv() {
            self.on_create_item.0(state, shell)
        }
    }
}
impl<State: ValidState> Transformable for PanelItemAcceptor<State> {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

#[derive(Debug, Handler)]
pub struct PanelItemAcceptorHandler {
    spatial_ref: SpatialRef,
    spatial_interface: SpatialInterface,
    tx: mpsc::UnboundedSender<BinderObject<PanelShellHandler>>,
    rx: Mutex<mpsc::UnboundedReceiver<BinderObject<PanelShellHandler>>>,
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
