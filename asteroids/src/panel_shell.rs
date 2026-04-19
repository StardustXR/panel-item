use std::sync::{
    Arc, Mutex, OnceLock,
    atomic::{AtomicBool, Ordering},
};

use binderbinder::{BinderDevice, binder_object::BinderObject};
use derive_where::derive_where;
use gluon_wire::{GluonCtx, impl_transaction_handler};
use mint::Vector2;
use rustc_hash::FxHashMap;
use stardust_xr_asteroids::{CustomElement, FnWrapper, Transformable, ValidState};
use stardust_xr_fusion::{
    node::NodeError,
    spatial::{Spatial, SpatialAspect, SpatialRef, Transform},
};
use tokio::{
    sync::{
        RwLock,
        mpsc::{self, unbounded_channel},
    },
    task::JoinHandle,
};

use stardust_xr_panel_item::protocol::{
    ChildState, Geometry, PanelItem, PanelShellHandler as _, SurfaceUpdateTarget, UVec2,
};

#[derive_where(Debug)]
pub struct PanelShell<State: ValidState> {
    handler: Arc<BinderObject<PanelShellHandler>>,
    on_toplevel_resolution_changed:
        FnWrapper<dyn Fn(&mut State, &PanelItem, Vector2<u32>) + Send + Sync>,
    on_toplevel_fullscreen_changed: FnWrapper<dyn Fn(&mut State, &PanelItem, bool) + Send + Sync>,
    on_toplevel_title_changed: FnWrapper<dyn Fn(&mut State, &PanelItem, String) + Send + Sync>,
    on_toplevel_app_id_changed: FnWrapper<dyn Fn(&mut State, &PanelItem, String) + Send + Sync>,
    cursor_visuals_changed:
        FnWrapper<dyn Fn(&mut State, &PanelItem, Option<Geometry>) + Send + Sync>,
    new_child: FnWrapper<dyn Fn(&mut State, &PanelItem, ChildState) + Send + Sync>,
    child_moved: FnWrapper<dyn Fn(&mut State, &PanelItem, u64, Geometry) + Send + Sync>,
    child_removed: FnWrapper<dyn Fn(&mut State, &PanelItem, u64) + Send + Sync>,
    item_disconnected: FnWrapper<dyn Fn(&mut State) + Send + Sync>,
    transform: Transform,
}
impl<State: ValidState> PanelShell<State> {
    pub fn new(
        handler: &Arc<BinderObject<PanelShellHandler>>,
        item_disconnected: impl Fn(&mut State) + Send + Sync + 'static,
    ) -> Self {
        Self {
            handler: handler.clone(),
            on_toplevel_resolution_changed: FnWrapper(Box::new(|_, _, _| {})),
            on_toplevel_fullscreen_changed: FnWrapper(Box::new(|_, _, _| {})),
            on_toplevel_title_changed: FnWrapper(Box::new(|_, _, _| {})),
            on_toplevel_app_id_changed: FnWrapper(Box::new(|_, _, _| {})),
            cursor_visuals_changed: FnWrapper(Box::new(|_, _, _| {})),
            new_child: FnWrapper(Box::new(|_, _, _| {})),
            child_moved: FnWrapper(Box::new(|_, _, _, _| {})),
            child_removed: FnWrapper(Box::new(|_, _, _| {})),
            item_disconnected: FnWrapper(Box::new(item_disconnected)),
            transform: Transform::identity(),
        }
    }
}

impl<State: ValidState> CustomElement<State> for PanelShell<State> {
    type Inner = SpatialRef;

    type Resource = ();

    type Error = NodeError;

    fn create_inner(
        &self,
        _asteroids_context: &stardust_xr_asteroids::Context,
        info: stardust_xr_asteroids::CreateInnerInfo,
        _resource: &mut Self::Resource,
    ) -> Result<Self::Inner, Self::Error> {
        Ok(info.parent_space.clone())
    }

    fn diff(&self, _old_self: &Self, inner: &mut Self::Inner, _resource: &mut Self::Resource) {
        // can't properly diff this since we don't know if this is the same spatial as last diff
        _ = self.handler.item_output_spatial.set_spatial_parent(inner);
        _ = self
            .handler
            .item_output_spatial
            .set_local_transform(self.transform.clone());
    }

    fn frame(
        &self,
        _context: &stardust_xr_asteroids::Context,
        _info: &stardust_xr_fusion::root::FrameInfo,
        state: &mut State,
        _inner: &mut Self::Inner,
    ) {
        if self
            .handler
            .death_task
            .get()
            .is_some_and(|v| v.is_finished())
            && !self.handler.death_handled.load(Ordering::Relaxed)
        {
            self.item_disconnected.0(state);
            self.handler.death_handled.store(true, Ordering::Relaxed);
        }
        while let Ok(event) = self.handler.rx.lock().unwrap().try_recv() {
            match event {
                PanelShellEvent::ToplevelFullscreen { fullscreen_active } => {
                    self.on_toplevel_fullscreen_changed.0(
                        state,
                        &self.handler.item,
                        fullscreen_active,
                    )
                }
                PanelShellEvent::ToplevelTitle { title } => {
                    self.on_toplevel_title_changed.0(state, &self.handler.item, title)
                }
                PanelShellEvent::ToplevelAppId { app_id } => {
                    self.on_toplevel_app_id_changed.0(state, &self.handler.item, app_id)
                }
                PanelShellEvent::SetCursorVisuals { geometry } => {
                    self.cursor_visuals_changed.0(state, &self.handler.item, geometry)
                }
                PanelShellEvent::CreateChild { child } => {
                    self.new_child.0(state, &self.handler.item, child)
                }
                PanelShellEvent::MoveChild { child_id, geometry } => {
                    self.child_moved.0(state, &self.handler.item, child_id, geometry)
                }
                PanelShellEvent::DestroyChild { child_id } => {
                    self.child_removed.0(state, &self.handler.item, child_id)
                }
                PanelShellEvent::ToplevelResized { new_size } => {
                    self.on_toplevel_resolution_changed.0(state, &self.handler.item, new_size)
                }
            }
        }
    }

    fn spatial_aspect(&self, inner: &Self::Inner) -> stardust_xr_fusion::spatial::SpatialRef {
        inner.clone()
    }
}
impl<State: ValidState> Transformable for PanelShell<State> {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

#[derive(Debug)]
pub(super) struct SurfaceUpdate {
    pub(super) dmatex_uid: u64,
    pub(super) acquire_point: u64,
    pub(super) release_point: u64,
    pub(super) opaque: bool,
}

#[derive(Debug)]
pub struct PanelShellHandler {
    pub(super) surface_rx: Arc<
        RwLock<FxHashMap<SurfaceUpdateTarget, Arc<RwLock<mpsc::UnboundedReceiver<SurfaceUpdate>>>>>,
    >,
    surface_tx: Arc<RwLock<FxHashMap<SurfaceUpdateTarget, mpsc::UnboundedSender<SurfaceUpdate>>>>,
    tx: mpsc::UnboundedSender<PanelShellEvent>,
    rx: Mutex<mpsc::UnboundedReceiver<PanelShellEvent>>,
    item_output_spatial: Spatial,
    item: PanelItem,
    death_task: OnceLock<JoinHandle<()>>,
    death_handled: AtomicBool,
}
impl PanelShellHandler {
    pub fn new(
        device: &Arc<BinderDevice>,
        item: PanelItem,
        item_output_spatial: Spatial,
    ) -> BinderObject<Self> {
        let (toplevel_tx, toplevel_rx) = mpsc::unbounded_channel();
        let (cursor_tx, cursor_rx) = mpsc::unbounded_channel();
        let mut surface_tx = FxHashMap::default();
        let mut surface_rx = FxHashMap::default();
        surface_tx.insert(SurfaceUpdateTarget::Toplevel, toplevel_tx);
        surface_rx.insert(
            SurfaceUpdateTarget::Toplevel,
            Arc::new(RwLock::new(toplevel_rx)),
        );
        surface_tx.insert(SurfaceUpdateTarget::Cursor, cursor_tx);
        surface_rx.insert(
            SurfaceUpdateTarget::Cursor,
            Arc::new(RwLock::new(cursor_rx)),
        );
        let (tx, rx) = mpsc::unbounded_channel();
        let v = device.register_object(Self {
            tx,
            rx: Mutex::new(rx),
            item_output_spatial,
            item,
            surface_rx: Arc::new(surface_rx.into()),
            surface_tx: Arc::new(surface_tx.into()),
            death_task: OnceLock::new(),
            death_handled: AtomicBool::new(false),
        });
        let death_future = v.strong_refs_hit_zero();
        let death_task = tokio::spawn(death_future);
        _ = v.death_task.set(death_task);
        v
    }
    pub fn item(&self) -> &PanelItem {
        &self.item
    }
}

impl Drop for PanelShellHandler {
    fn drop(&mut self) {
        if let Some(t) = self.death_task.get() {
            t.abort()
        }
    }
}

enum PanelShellEvent {
    ToplevelResized { new_size: Vector2<u32> },
    ToplevelFullscreen { fullscreen_active: bool },
    ToplevelTitle { title: String },
    ToplevelAppId { app_id: String },
    SetCursorVisuals { geometry: Option<Geometry> },
    CreateChild { child: ChildState },
    MoveChild { child_id: u64, geometry: Geometry },
    DestroyChild { child_id: u64 },
}

impl stardust_xr_panel_item::protocol::PanelShellHandler for PanelShellHandler {
    async fn update_surface_dmatex(
        &self,
        _ctx: GluonCtx,
        surface: SurfaceUpdateTarget,
        dmatex_uid: u64,
        acquire_point: u64,
        release_point: u64,
        opaque: bool,
    ) {
        let surface_tx = self.surface_tx.clone();
        tokio::spawn(async move {
            if let Some(tx) = surface_tx.read().await.get(&surface) {
                tx.send(SurfaceUpdate {
                    dmatex_uid,
                    acquire_point,
                    release_point,
                    opaque,
                })
                .unwrap();
            }
        });
    }

    async fn toplevel_resized(&self, _ctx: GluonCtx, new_size: UVec2) {
        self.tx
            .send(PanelShellEvent::ToplevelResized {
                new_size: new_size.into(),
            })
            .unwrap();
    }

    async fn toplevel_fullscreen(&self, _ctx: GluonCtx, fullscreen_active: bool) {
        self.tx
            .send(PanelShellEvent::ToplevelFullscreen { fullscreen_active })
            .unwrap();
    }

    async fn toplevel_title(&self, _ctx: GluonCtx, title: String) {
        self.tx
            .send(PanelShellEvent::ToplevelTitle { title })
            .unwrap();
    }

    async fn toplevel_app_id(&self, _ctx: GluonCtx, app_id: String) {
        self.tx
            .send(PanelShellEvent::ToplevelAppId { app_id })
            .unwrap();
    }

    async fn set_cursor_visuals(&self, _ctx: GluonCtx, geometry: Option<Geometry>) {
        self.tx
            .send(PanelShellEvent::SetCursorVisuals { geometry })
            .unwrap();
    }

    async fn create_child(&self, _ctx: GluonCtx, child: ChildState) {
        let surface_target = SurfaceUpdateTarget::Child { id: child.id };
        self.tx
            .send(PanelShellEvent::CreateChild { child })
            .unwrap();
        let surface_tx = self.surface_tx.clone();
        let surface_rx = self.surface_rx.clone();
        tokio::spawn(async move {
            let (tx, rx) = unbounded_channel();
            surface_tx.write().await.insert(surface_target, tx);
            surface_rx
                .write()
                .await
                .insert(surface_target, Arc::new(RwLock::new(rx)));
        });
    }

    async fn move_child(&self, _ctx: GluonCtx, child_id: u64, geometry: Geometry) {
        self.tx
            .send(PanelShellEvent::MoveChild { child_id, geometry })
            .unwrap();
    }

    async fn destroy_child(&self, _ctx: GluonCtx, child_id: u64) {
        self.tx
            .send(PanelShellEvent::DestroyChild { child_id })
            .unwrap();
        let surface_target = SurfaceUpdateTarget::Child { id: child_id };
        let surface_tx = self.surface_tx.clone();
        let surface_rx = self.surface_rx.clone();
        tokio::spawn(async move {
            surface_tx.write().await.remove(&surface_target);
            surface_rx.write().await.remove(&surface_target);
        });
    }
}
impl_transaction_handler!(PanelShellHandler);
impl<State: ValidState> PanelShell<State> {
    pub fn on_toplevel_resolution_changed(
        mut self,
        func: impl Fn(&mut State, &PanelItem, Vector2<u32>) + Send + Sync + 'static,
    ) -> Self {
        self.on_toplevel_resolution_changed = FnWrapper(Box::new(func));
        self
    }
    pub fn on_toplevel_fullscreen_changed(
        mut self,
        func: impl Fn(&mut State, &PanelItem, bool) + Send + Sync + 'static,
    ) -> Self {
        self.on_toplevel_fullscreen_changed = FnWrapper(Box::new(func));
        self
    }
    pub fn on_toplevel_title_changed(
        mut self,
        func: impl Fn(&mut State, &PanelItem, String) + Send + Sync + 'static,
    ) -> Self {
        self.on_toplevel_title_changed = FnWrapper(Box::new(func));
        self
    }
    pub fn on_toplevel_app_id_changed(
        mut self,
        func: impl Fn(&mut State, &PanelItem, String) + Send + Sync + 'static,
    ) -> Self {
        self.on_toplevel_app_id_changed = FnWrapper(Box::new(func));
        self
    }
    pub fn cursor_visuals_changed(
        mut self,
        func: impl Fn(&mut State, &PanelItem, Option<Geometry>) + Send + Sync + 'static,
    ) -> Self {
        self.cursor_visuals_changed = FnWrapper(Box::new(func));
        self
    }
    pub fn new_child(
        mut self,
        func: impl Fn(&mut State, &PanelItem, ChildState) + Send + Sync + 'static,
    ) -> Self {
        self.new_child = FnWrapper(Box::new(func));
        self
    }
    pub fn child_moved(
        mut self,
        func: impl Fn(&mut State, &PanelItem, u64, Geometry) + Send + Sync + 'static,
    ) -> Self {
        self.child_moved = FnWrapper(Box::new(func));
        self
    }
    pub fn child_removed(
        mut self,
        func: impl Fn(&mut State, &PanelItem, u64) + Send + Sync + 'static,
    ) -> Self {
        self.child_removed = FnWrapper(Box::new(func));
        self
    }
}
