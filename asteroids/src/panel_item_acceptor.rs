use std::{
    env,
    fs::OpenOptions,
    sync::{Mutex, OnceLock},
};

use binderbinder::binder_object::{BinderObject, ToBinderObjectOrRef};
use gluon_wire::{GluonCtx, impl_transaction_handler};
use pion_binder::PionBinderDevice;
use stardust_xr_asteroids::{CustomElement, FnWrapper, Transformable, ValidState};
use stardust_xr_fusion::{
    fields::{Field, FieldAspect as _, Shape},
    node::NodeError,
    spatial::{Spatial, SpatialAspect, Transform},
};
use stardust_xr_panel_item::protocol::{
    FieldRefId, PanelItem, PanelItemAcceptor, PanelItemAcceptorHandler as _, PanelItemProvider,
    PanelShell, SpatialRefId,
};
use tokio::sync::mpsc;

use crate::panel_shell::PanelShellHandler;

#[derive_where::derive_where(Debug)]
pub struct PanelItemAcceptorElement<State: ValidState> {
    binder_dev: PionBinderDevice,
    transform: Transform,
    shape: Shape,
    on_create_item: FnWrapper<dyn Fn(&mut State, BinderObject<PanelShellHandler>) + Send + Sync>,
}
impl<State: ValidState> PanelItemAcceptorElement<State> {
    pub fn new(
        binder_dev: &PionBinderDevice,
        shape: Shape,
        on_accept: impl Fn(&mut State, BinderObject<PanelShellHandler>) + Send + Sync + 'static,
    ) -> Self {
        Self {
            binder_dev: binder_dev.clone(),
            transform: Transform::none(),
            shape,
            on_create_item: FnWrapper(Box::new(on_accept)),
        }
    }
}

impl<State: ValidState> CustomElement<State> for PanelItemAcceptorElement<State> {
    type Inner = BinderObject<PanelItemAcceptorHandler>;

    type Resource = ();

    type Error = NodeError;

    fn create_inner(
        &self,
        _ctx: &stardust_xr_asteroids::Context,
        info: stardust_xr_asteroids::CreateInnerInfo,
        _resource: &mut Self::Resource,
    ) -> Result<Self::Inner, Self::Error> {
        let field = Field::create(
            info.parent_space,
            self.transform.clone(),
            self.shape.clone(),
        )?;
        let (tx, rx) = mpsc::unbounded_channel();
        let handler = self.binder_dev.register_object(PanelItemAcceptorHandler {
            field,
            field_id: OnceLock::new(),
            tx,
            rx: Mutex::new(rx),
        });
        tokio::spawn({
            let dev = self.binder_dev.clone();
            let acceptor = PanelItemAcceptor::from_handler(&handler);
            async move {
                let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(
                        directories::BaseDirs::new()
                            .unwrap()
                            .runtime_dir()
                            .unwrap()
                            .join(format!("{}.lock", env::var("WAYLAND_DISPLAY").unwrap())),
                    )
                    .unwrap();
                let binder_ref = dev.get_binder_ref_from_file(file).await.unwrap();
                let v = PanelItemProvider::from_object_or_ref(binder_ref);
                v.register_acceptor(acceptor).unwrap();
                // TODO: figure out how to call drop_acceptor on drop
            }
        });

        Ok(handler)
    }

    fn diff(&self, old: &Self, inner: &mut Self::Inner, _resource: &mut Self::Resource) {
        self.apply_transform(old, &inner.field);
        if self.shape != old.shape {
            let _ = inner.field.set_shape(self.shape.clone());
        }
    }
    fn frame(
        &self,
        _context: &stardust_xr_asteroids::Context,
        _info: &stardust_xr_fusion::root::FrameInfo,
        state: &mut State,
        inner: &mut Self::Inner,
    ) {
        while let Ok(shell) = inner.rx.lock().unwrap().try_recv() {
            self.on_create_item.0(state, shell)
        }
    }

    fn spatial_aspect(&self, inner: &Self::Inner) -> stardust_xr_fusion::spatial::SpatialRef {
        inner.field.clone().as_spatial_ref()
    }
}
impl<State: ValidState> Transformable for PanelItemAcceptorElement<State> {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

#[derive(Debug)]
pub struct PanelItemAcceptorHandler {
    field: Field,
    field_id: OnceLock<FieldRefId>,
    tx: mpsc::UnboundedSender<BinderObject<PanelShellHandler>>,
    rx: Mutex<mpsc::UnboundedReceiver<BinderObject<PanelShellHandler>>>,
}
impl stardust_xr_panel_item::protocol::PanelItemAcceptorHandler for PanelItemAcceptorHandler {
    async fn accept(
        &self,
        _ctx: GluonCtx,
        item: PanelItem,
    ) -> (
        stardust_xr_panel_item::protocol::PanelShell,
        stardust_xr_panel_item::protocol::SpatialRefId,
    ) {
        let output_spatial = Spatial::create(&self.field, Transform::none()).unwrap();
        let id = output_spatial.export_spatial().await.unwrap();

        let panel_shell = PanelShellHandler::new(
            item.to_binder_object_or_ref().device(),
            item.clone(),
            output_spatial,
        );
        let proxy = PanelShell::from_handler(&panel_shell);
        self.tx.send(panel_shell).unwrap();
        (proxy, SpatialRefId { id })
    }

    async fn get_field(&self, _ctx: GluonCtx) -> stardust_xr_panel_item::protocol::FieldRefId {
        if let Some(id) = self.field_id.get() {
            id.clone()
        } else {
            let id = self.field.export_field().await.unwrap();
            let id = FieldRefId { id };
            _ = self.field_id.set(id.clone());
            id
        }
    }
}
impl_transaction_handler!(PanelItemAcceptorHandler);
