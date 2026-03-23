#![allow(unused)]
use gluon_wire::GluonConvertable;
#[derive(Debug, Clone)]
pub struct PanelItem {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for PanelItem {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(data)
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(data)?;
        Ok(PanelItem::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl PanelItem {
    pub async fn register_xkb_keymap(
        &self,
        xkb_keymap: String,
    ) -> Result<KeymapId, gluon_wire::GluonSendError> {
        let obj = binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
            &self.obj,
        );
        tokio::task::spawn_blocking(move || {
                let mut builder = gluon_wire::GluonDataBuilder::new();
                xkb_keymap.write(&mut builder)?;
                let reader = obj
                    .device()
                    .transact_blocking(&obj, 8u32, builder.to_payload())?
                    .1;
                let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
                Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
            })
            .await
            .unwrap()
    }
    pub fn register_xkb_keymap_blocking(
        &self,
        xkb_keymap: String,
    ) -> Result<KeymapId, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        xkb_keymap.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn absolute_pointer_motion(
        &self,
        surface: SurfaceId,
        position: Vec2,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        surface.write(&mut builder)?;
        position.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, builder.to_payload())?;
        Ok(())
    }
    pub fn relative_pointer_motion(
        &self,
        surface: SurfaceId,
        delta: Vec2,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        surface.write(&mut builder)?;
        delta.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 10u32, builder.to_payload())?;
        Ok(())
    }
    pub fn pointer_button(
        &self,
        surface: SurfaceId,
        button: u32,
        pressed: bool,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        surface.write(&mut builder)?;
        button.write(&mut builder)?;
        pressed.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 11u32, builder.to_payload())?;
        Ok(())
    }
    pub fn pointer_scroll_pixels(
        &self,
        surface: SurfaceId,
        delta: Vec2,
        source: ScrollSource,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        surface.write(&mut builder)?;
        delta.write(&mut builder)?;
        source.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 12u32, builder.to_payload())?;
        Ok(())
    }
    pub fn pointer_scroll_discrete(
        &self,
        surface: SurfaceId,
        delta: Vec2,
        source: ScrollSource,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        surface.write(&mut builder)?;
        delta.write(&mut builder)?;
        source.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 13u32, builder.to_payload())?;
        Ok(())
    }
    pub fn pointer_scroll_stop(
        &self,
        surface: SurfaceId,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        surface.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 14u32, builder.to_payload())?;
        Ok(())
    }
    pub fn key(
        &self,
        surface: SurfaceId,
        keymap: KeymapId,
        key: u32,
        pressed: bool,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        surface.write(&mut builder)?;
        keymap.write(&mut builder)?;
        key.write(&mut builder)?;
        pressed.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 15u32, builder.to_payload())?;
        Ok(())
    }
    pub fn touch_down(
        &self,
        surface: SurfaceId,
        touch_id: u32,
        position: Vec2,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        surface.write(&mut builder)?;
        touch_id.write(&mut builder)?;
        position.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 16u32, builder.to_payload())?;
        Ok(())
    }
    pub fn touch_move(
        &self,
        touch_id: u32,
        position: Vec2,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        touch_id.write(&mut builder)?;
        position.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 17u32, builder.to_payload())?;
        Ok(())
    }
    pub fn touch_up(&self, touch_id: u32) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        touch_id.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 18u32, builder.to_payload())?;
        Ok(())
    }
    pub fn close_toplevel(&self) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        self.obj.device().transact_one_way(&self.obj, 19u32, builder.to_payload())?;
        Ok(())
    }
    pub fn resize_toplevel_to_app_request(
        &self,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        self.obj.device().transact_one_way(&self.obj, 20u32, builder.to_payload())?;
        Ok(())
    }
    pub fn request_toplevel_resize(
        &self,
        new_size: UVec2,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        new_size.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 21u32, builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_focused(
        &self,
        focused: bool,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        focused.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 22u32, builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: PanelItemHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> PanelItem {
        PanelItem::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> PanelItem {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        PanelItem {
            obj,
            drop_notification,
        }
    }
    pub fn death_or_drop(&self) -> impl Future<Output = ()> + Send + Sync + 'static {
        let death_notification_future = match &self.obj {
            binderbinder::binder_object::BinderObjectOrRef::Ref(r) => {
                Some(r.death_notification())
            }
            binderbinder::binder_object::BinderObjectOrRef::WeakRef(r) => {
                Some(r.death_notification())
            }
            _ => None,
        };
        let drop_notification = self.drop_notification.clone();
        async move {
            if let Some(death) = death_notification_future {
                tokio::select! {
                    _ = death => {} _ = drop_notification.wait() => {}
                }
            } else {
                drop_notification.wait().await;
            }
        }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for PanelItem {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait PanelItemHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn register_xkb_keymap(
        &self,
        _ctx: gluon_wire::GluonCtx,
        xkb_keymap: String,
    ) -> impl Future<Output = KeymapId> + Send + Sync;
    fn absolute_pointer_motion(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceId,
        position: Vec2,
    );
    fn relative_pointer_motion(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceId,
        delta: Vec2,
    );
    fn pointer_button(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceId,
        button: u32,
        pressed: bool,
    );
    fn pointer_scroll_pixels(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceId,
        delta: Vec2,
        source: ScrollSource,
    );
    fn pointer_scroll_discrete(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceId,
        delta: Vec2,
        source: ScrollSource,
    );
    fn pointer_scroll_stop(&self, _ctx: gluon_wire::GluonCtx, surface: SurfaceId);
    fn key(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceId,
        keymap: KeymapId,
        key: u32,
        pressed: bool,
    );
    fn touch_down(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceId,
        touch_id: u32,
        position: Vec2,
    );
    fn touch_move(&self, _ctx: gluon_wire::GluonCtx, touch_id: u32, position: Vec2);
    fn touch_up(&self, _ctx: gluon_wire::GluonCtx, touch_id: u32);
    fn close_toplevel(&self, _ctx: gluon_wire::GluonCtx);
    fn resize_toplevel_to_app_request(&self, _ctx: gluon_wire::GluonCtx);
    fn request_toplevel_resize(&self, _ctx: gluon_wire::GluonCtx, new_size: UVec2);
    fn toplevel_focused(&self, _ctx: gluon_wire::GluonCtx, focused: bool);
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                8u32 => {
                    let (id) = self
                        .register_xkb_keymap(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    id.write_owned(&mut out)?;
                }
                _ => {}
            }
            Ok(out)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(obj) = data.read_binder() else {
                        return Ok(());
                    };
                    self.drop_notification_requested(
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        )
                        .await;
                }
                9u32 => {
                    self.absolute_pointer_motion(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                10u32 => {
                    self.relative_pointer_motion(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                11u32 => {
                    self.pointer_button(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                12u32 => {
                    self.pointer_scroll_pixels(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                13u32 => {
                    self.pointer_scroll_discrete(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                14u32 => {
                    self.pointer_scroll_stop(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                15u32 => {
                    self.key(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                16u32 => {
                    self.touch_down(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                17u32 => {
                    self.touch_move(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                18u32 => {
                    self.touch_up(ctx, gluon_wire::GluonConvertable::read(data)?);
                }
                19u32 => {
                    self.close_toplevel(ctx);
                }
                20u32 => {
                    self.resize_toplevel_to_app_request(ctx);
                }
                21u32 => {
                    self.request_toplevel_resize(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                22u32 => {
                    self.toplevel_focused(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct PanelShell {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for PanelShell {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(data)
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(data)?;
        Ok(PanelShell::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl PanelShell {
    pub fn update_surface_dmatex(
        &self,
        surface: SurfaceUpdateTarget,
        dmatex_uid: u64,
        acquire_point: u64,
        release_point: u64,
        opaque: bool,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        surface.write(&mut builder)?;
        dmatex_uid.write(&mut builder)?;
        acquire_point.write(&mut builder)?;
        release_point.write(&mut builder)?;
        opaque.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_resized(
        &self,
        new_size: UVec2,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        new_size.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_fullscreen(
        &self,
        fullscreen_active: bool,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        fullscreen_active.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 10u32, builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_title(
        &self,
        title: String,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        title.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 11u32, builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_app_id(
        &self,
        app_id: String,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        app_id.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 12u32, builder.to_payload())?;
        Ok(())
    }
    pub fn set_cursor_visuals(
        &self,
        geometry: Option<Geometry>,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        geometry.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 13u32, builder.to_payload())?;
        Ok(())
    }
    pub fn create_child(
        &self,
        child: ChildState,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        child.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 14u32, builder.to_payload())?;
        Ok(())
    }
    pub fn move_child(
        &self,
        child_id: u64,
        geometry: Geometry,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        child_id.write(&mut builder)?;
        geometry.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 15u32, builder.to_payload())?;
        Ok(())
    }
    pub fn destroy_child(
        &self,
        child_id: u64,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        child_id.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 16u32, builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: PanelShellHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> PanelShell {
        PanelShell::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> PanelShell {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        PanelShell {
            obj,
            drop_notification,
        }
    }
    pub fn death_or_drop(&self) -> impl Future<Output = ()> + Send + Sync + 'static {
        let death_notification_future = match &self.obj {
            binderbinder::binder_object::BinderObjectOrRef::Ref(r) => {
                Some(r.death_notification())
            }
            binderbinder::binder_object::BinderObjectOrRef::WeakRef(r) => {
                Some(r.death_notification())
            }
            _ => None,
        };
        let drop_notification = self.drop_notification.clone();
        async move {
            if let Some(death) = death_notification_future {
                tokio::select! {
                    _ = death => {} _ = drop_notification.wait() => {}
                }
            } else {
                drop_notification.wait().await;
            }
        }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for PanelShell {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait PanelShellHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn update_surface_dmatex(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceUpdateTarget,
        dmatex_uid: u64,
        acquire_point: u64,
        release_point: u64,
        opaque: bool,
    );
    fn toplevel_resized(&self, _ctx: gluon_wire::GluonCtx, new_size: UVec2);
    fn toplevel_fullscreen(&self, _ctx: gluon_wire::GluonCtx, fullscreen_active: bool);
    fn toplevel_title(&self, _ctx: gluon_wire::GluonCtx, title: String);
    fn toplevel_app_id(&self, _ctx: gluon_wire::GluonCtx, app_id: String);
    fn set_cursor_visuals(&self, _ctx: gluon_wire::GluonCtx, geometry: Option<Geometry>);
    fn create_child(&self, _ctx: gluon_wire::GluonCtx, child: ChildState);
    fn move_child(&self, _ctx: gluon_wire::GluonCtx, child_id: u64, geometry: Geometry);
    fn destroy_child(&self, _ctx: gluon_wire::GluonCtx, child_id: u64);
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                _ => {}
            }
            Ok(out)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(obj) = data.read_binder() else {
                        return Ok(());
                    };
                    self.drop_notification_requested(
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        )
                        .await;
                }
                8u32 => {
                    self.update_surface_dmatex(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                9u32 => {
                    self.toplevel_resized(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                10u32 => {
                    self.toplevel_fullscreen(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                11u32 => {
                    self.toplevel_title(ctx, gluon_wire::GluonConvertable::read(data)?);
                }
                12u32 => {
                    self.toplevel_app_id(ctx, gluon_wire::GluonConvertable::read(data)?);
                }
                13u32 => {
                    self.set_cursor_visuals(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                14u32 => {
                    self.create_child(ctx, gluon_wire::GluonConvertable::read(data)?);
                }
                15u32 => {
                    self.move_child(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                16u32 => {
                    self.destroy_child(ctx, gluon_wire::GluonConvertable::read(data)?);
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct PanelItemAcceptor {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for PanelItemAcceptor {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(data)
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(data)?;
        Ok(PanelItemAcceptor::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl PanelItemAcceptor {
    pub async fn accept(
        &self,
        item: PanelItem,
    ) -> Result<(PanelShell, SpatialRefId), gluon_wire::GluonSendError> {
        let obj = binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
            &self.obj,
        );
        tokio::task::spawn_blocking(move || {
                let mut builder = gluon_wire::GluonDataBuilder::new();
                item.write(&mut builder)?;
                let reader = obj
                    .device()
                    .transact_blocking(&obj, 8u32, builder.to_payload())?
                    .1;
                let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
                Ok((
                    gluon_wire::GluonConvertable::read(&mut reader)?,
                    gluon_wire::GluonConvertable::read(&mut reader)?,
                ))
            })
            .await
            .unwrap()
    }
    pub fn accept_blocking(
        &self,
        item: PanelItem,
    ) -> Result<(PanelShell, SpatialRefId), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        item.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok((
            gluon_wire::GluonConvertable::read(&mut reader)?,
            gluon_wire::GluonConvertable::read(&mut reader)?,
        ))
    }
    pub async fn get_field(&self) -> Result<FieldRefId, gluon_wire::GluonSendError> {
        let obj = binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
            &self.obj,
        );
        tokio::task::spawn_blocking(move || {
                let mut builder = gluon_wire::GluonDataBuilder::new();
                let reader = obj
                    .device()
                    .transact_blocking(&obj, 9u32, builder.to_payload())?
                    .1;
                let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
                Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
            })
            .await
            .unwrap()
    }
    pub fn get_field_blocking(&self) -> Result<FieldRefId, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 9u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: PanelItemAcceptorHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> PanelItemAcceptor {
        PanelItemAcceptor::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> PanelItemAcceptor {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        PanelItemAcceptor {
            obj,
            drop_notification,
        }
    }
    pub fn death_or_drop(&self) -> impl Future<Output = ()> + Send + Sync + 'static {
        let death_notification_future = match &self.obj {
            binderbinder::binder_object::BinderObjectOrRef::Ref(r) => {
                Some(r.death_notification())
            }
            binderbinder::binder_object::BinderObjectOrRef::WeakRef(r) => {
                Some(r.death_notification())
            }
            _ => None,
        };
        let drop_notification = self.drop_notification.clone();
        async move {
            if let Some(death) = death_notification_future {
                tokio::select! {
                    _ = death => {} _ = drop_notification.wait() => {}
                }
            } else {
                drop_notification.wait().await;
            }
        }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for PanelItemAcceptor {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait PanelItemAcceptorHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn accept(
        &self,
        _ctx: gluon_wire::GluonCtx,
        item: PanelItem,
    ) -> impl Future<Output = (PanelShell, SpatialRefId)> + Send + Sync;
    fn get_field(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = FieldRefId> + Send + Sync;
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                8u32 => {
                    let (shell, output_spatial) = self
                        .accept(ctx, gluon_wire::GluonConvertable::read(data)?)
                        .await;
                    shell.write_owned(&mut out)?;
                    output_spatial.write_owned(&mut out)?;
                }
                9u32 => {
                    let (field) = self.get_field(ctx).await;
                    field.write_owned(&mut out)?;
                }
                _ => {}
            }
            Ok(out)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(obj) = data.read_binder() else {
                        return Ok(());
                    };
                    self.drop_notification_requested(
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct PanelItemProvider {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for PanelItemProvider {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(data)
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(data)?;
        Ok(PanelItemProvider::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl PanelItemProvider {
    pub fn register_acceptor(
        &self,
        acceptor: PanelItemAcceptor,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        acceptor.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, builder.to_payload())?;
        Ok(())
    }
    pub fn drop_acceptor(
        &self,
        acceptor: PanelItemAcceptor,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        acceptor.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, builder.to_payload())?;
        Ok(())
    }
    pub async fn startup_token_spatial_ref(
        &self,
        token: String,
        spatial_ref: SpatialRefId,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let obj = binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
            &self.obj,
        );
        tokio::task::spawn_blocking(move || {
                let mut builder = gluon_wire::GluonDataBuilder::new();
                token.write(&mut builder)?;
                spatial_ref.write(&mut builder)?;
                let reader = obj
                    .device()
                    .transact_blocking(&obj, 10u32, builder.to_payload())?
                    .1;
                let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
                Ok(())
            })
            .await
            .unwrap()
    }
    pub fn startup_token_spatial_ref_blocking(
        &self,
        token: String,
        spatial_ref: SpatialRefId,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        token.write(&mut builder)?;
        spatial_ref.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 10u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(())
    }
    pub fn from_handler<H: PanelItemProviderHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> PanelItemProvider {
        PanelItemProvider::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> PanelItemProvider {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        PanelItemProvider {
            obj,
            drop_notification,
        }
    }
    pub fn death_or_drop(&self) -> impl Future<Output = ()> + Send + Sync + 'static {
        let death_notification_future = match &self.obj {
            binderbinder::binder_object::BinderObjectOrRef::Ref(r) => {
                Some(r.death_notification())
            }
            binderbinder::binder_object::BinderObjectOrRef::WeakRef(r) => {
                Some(r.death_notification())
            }
            _ => None,
        };
        let drop_notification = self.drop_notification.clone();
        async move {
            if let Some(death) = death_notification_future {
                tokio::select! {
                    _ = death => {} _ = drop_notification.wait() => {}
                }
            } else {
                drop_notification.wait().await;
            }
        }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for PanelItemProvider {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait PanelItemProviderHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn register_acceptor(&self, _ctx: gluon_wire::GluonCtx, acceptor: PanelItemAcceptor);
    fn drop_acceptor(&self, _ctx: gluon_wire::GluonCtx, acceptor: PanelItemAcceptor);
    fn startup_token_spatial_ref(
        &self,
        _ctx: gluon_wire::GluonCtx,
        token: String,
        spatial_ref: SpatialRefId,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                10u32 => {
                    let () = self
                        .startup_token_spatial_ref(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                }
                _ => {}
            }
            Ok(out)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(obj) = data.read_binder() else {
                        return Ok(());
                    };
                    self.drop_notification_requested(
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        )
                        .await;
                }
                8u32 => {
                    self.register_acceptor(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                9u32 => {
                    self.drop_acceptor(ctx, gluon_wire::GluonConvertable::read(data)?);
                }
                _ => {}
            }
            Ok(())
        }
    }
}
///KeymapId
#[derive(Clone, Hash, Debug)]
pub struct KeymapId {
    pub id: u64,
}
impl gluon_wire::GluonConvertable for KeymapId {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.id.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let id = gluon_wire::GluonConvertable::read(data)?;
        Ok(KeymapId { id })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.id.write_owned(data)?;
        Ok(())
    }
}
///SpatialRef
#[derive(Clone, Debug)]
pub struct SpatialRefId {
    pub id: u64,
}
impl gluon_wire::GluonConvertable for SpatialRefId {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.id.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let id = gluon_wire::GluonConvertable::read(data)?;
        Ok(SpatialRefId { id })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.id.write_owned(data)?;
        Ok(())
    }
}
///FieldRef
#[derive(Clone, Debug)]
pub struct FieldRefId {
    pub id: u64,
}
impl gluon_wire::GluonConvertable for FieldRefId {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.id.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let id = gluon_wire::GluonConvertable::read(data)?;
        Ok(FieldRefId { id })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.id.write_owned(data)?;
        Ok(())
    }
}
///ToplevelState
#[derive(Clone, Debug)]
pub struct ToplevelState {
    pub parent: Option<u64>,
    pub title: Option<String>,
    pub app_id: Option<String>,
    pub size: UVec2,
    pub min_size: Option<UVec2>,
    pub max_size: Option<UVec2>,
}
impl gluon_wire::GluonConvertable for ToplevelState {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.parent.write(data)?;
        self.title.write(data)?;
        self.app_id.write(data)?;
        self.size.write(data)?;
        self.min_size.write(data)?;
        self.max_size.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let parent = gluon_wire::GluonConvertable::read(data)?;
        let title = gluon_wire::GluonConvertable::read(data)?;
        let app_id = gluon_wire::GluonConvertable::read(data)?;
        let size = gluon_wire::GluonConvertable::read(data)?;
        let min_size = gluon_wire::GluonConvertable::read(data)?;
        let max_size = gluon_wire::GluonConvertable::read(data)?;
        Ok(ToplevelState {
            parent,
            title,
            app_id,
            size,
            min_size,
            max_size,
        })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.parent.write_owned(data)?;
        self.title.write_owned(data)?;
        self.app_id.write_owned(data)?;
        self.size.write_owned(data)?;
        self.min_size.write_owned(data)?;
        self.max_size.write_owned(data)?;
        Ok(())
    }
}
///ChildState
#[derive(Clone, Debug)]
pub struct ChildState {
    pub id: u64,
    pub parent: SurfaceId,
    pub geometry: Geometry,
    pub z_order: i32,
    pub input_regions: Vec<Rect>,
}
impl gluon_wire::GluonConvertable for ChildState {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.id.write(data)?;
        self.parent.write(data)?;
        self.geometry.write(data)?;
        self.z_order.write(data)?;
        self.input_regions.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let id = gluon_wire::GluonConvertable::read(data)?;
        let parent = gluon_wire::GluonConvertable::read(data)?;
        let geometry = gluon_wire::GluonConvertable::read(data)?;
        let z_order = gluon_wire::GluonConvertable::read(data)?;
        let input_regions = gluon_wire::GluonConvertable::read(data)?;
        Ok(ChildState {
            id,
            parent,
            geometry,
            z_order,
            input_regions,
        })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.id.write_owned(data)?;
        self.parent.write_owned(data)?;
        self.geometry.write_owned(data)?;
        self.z_order.write_owned(data)?;
        self.input_regions.write_owned(data)?;
        Ok(())
    }
}
///PanelItemInitData
#[derive(Clone, Debug)]
pub struct PanelItemInitData {
    pub cursor: Option<Geometry>,
    pub toplevel: ToplevelState,
    pub children: Vec<ChildState>,
}
impl gluon_wire::GluonConvertable for PanelItemInitData {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.cursor.write(data)?;
        self.toplevel.write(data)?;
        self.children.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let cursor = gluon_wire::GluonConvertable::read(data)?;
        let toplevel = gluon_wire::GluonConvertable::read(data)?;
        let children = gluon_wire::GluonConvertable::read(data)?;
        Ok(PanelItemInitData {
            cursor,
            toplevel,
            children,
        })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.cursor.write_owned(data)?;
        self.toplevel.write_owned(data)?;
        self.children.write_owned(data)?;
        Ok(())
    }
}
///Rect
#[derive(Clone, Debug)]
pub struct Rect {
    pub origin: Vec2,
    pub size: Vec2,
}
impl gluon_wire::GluonConvertable for Rect {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.origin.write(data)?;
        self.size.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let origin = gluon_wire::GluonConvertable::read(data)?;
        let size = gluon_wire::GluonConvertable::read(data)?;
        Ok(Rect { origin, size })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.origin.write_owned(data)?;
        self.size.write_owned(data)?;
        Ok(())
    }
}
///Geometry
#[derive(Clone, Hash, Debug)]
pub struct Geometry {
    pub origin: IVec2,
    pub size: UVec2,
}
impl gluon_wire::GluonConvertable for Geometry {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.origin.write(data)?;
        self.size.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let origin = gluon_wire::GluonConvertable::read(data)?;
        let size = gluon_wire::GluonConvertable::read(data)?;
        Ok(Geometry { origin, size })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.origin.write_owned(data)?;
        self.size.write_owned(data)?;
        Ok(())
    }
}
///Vec2
#[derive(Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl gluon_wire::GluonConvertable for Vec2 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(data)?;
        self.y.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(data)?;
        let y = gluon_wire::GluonConvertable::read(data)?;
        Ok(Vec2 { x, y })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write_owned(data)?;
        self.y.write_owned(data)?;
        Ok(())
    }
}
///UVec2
#[derive(Clone, Hash, Debug)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}
impl gluon_wire::GluonConvertable for UVec2 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(data)?;
        self.y.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(data)?;
        let y = gluon_wire::GluonConvertable::read(data)?;
        Ok(UVec2 { x, y })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write_owned(data)?;
        self.y.write_owned(data)?;
        Ok(())
    }
}
///iVec2
#[derive(Clone, Hash, Debug)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}
impl gluon_wire::GluonConvertable for IVec2 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(data)?;
        self.y.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(data)?;
        let y = gluon_wire::GluonConvertable::read(data)?;
        Ok(IVec2 { x, y })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write_owned(data)?;
        self.y.write_owned(data)?;
        Ok(())
    }
}
///ScrollSource
#[derive(Clone, Hash, Debug)]
pub enum ScrollSource {
    Wheel,
    Touch,
    Continuous,
    WheelTilt,
}
impl gluon_wire::GluonConvertable for ScrollSource {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            ScrollSource::Wheel {} => {
                data.write_u16(0u16)?;
            }
            ScrollSource::Touch {} => {
                data.write_u16(1u16)?;
            }
            ScrollSource::Continuous {} => {
                data.write_u16(2u16)?;
            }
            ScrollSource::WheelTilt {} => {
                data.write_u16(3u16)?;
            }
        };
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        Ok(
            match data.read_u16()? {
                0u16 => ScrollSource::Wheel,
                1u16 => ScrollSource::Touch,
                2u16 => ScrollSource::Continuous,
                3u16 => ScrollSource::WheelTilt,
                v => return Err(gluon_wire::GluonReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            ScrollSource::Wheel {} => {
                data.write_u16(0u16)?;
            }
            ScrollSource::Touch {} => {
                data.write_u16(1u16)?;
            }
            ScrollSource::Continuous {} => {
                data.write_u16(2u16)?;
            }
            ScrollSource::WheelTilt {} => {
                data.write_u16(3u16)?;
            }
        };
        Ok(())
    }
}
///SurfaceId
#[derive(Clone, Hash, Debug)]
pub enum SurfaceId {
    Toplevel,
    Child { id: u64 },
}
impl gluon_wire::GluonConvertable for SurfaceId {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            SurfaceId::Toplevel {} => {
                data.write_u16(0u16)?;
            }
            SurfaceId::Child { id } => {
                data.write_u16(1u16)?;
                id.write(data)?;
            }
        };
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        Ok(
            match data.read_u16()? {
                0u16 => SurfaceId::Toplevel,
                1u16 => {
                    let id = gluon_wire::GluonConvertable::read(data)?;
                    SurfaceId::Child { id }
                }
                v => return Err(gluon_wire::GluonReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            SurfaceId::Toplevel {} => {
                data.write_u16(0u16)?;
            }
            SurfaceId::Child { id } => {
                data.write_u16(1u16)?;
                id.write_owned(data)?;
            }
        };
        Ok(())
    }
}
///SurfaceDmatexTarget
#[derive(Clone, Hash, Debug)]
pub enum SurfaceUpdateTarget {
    Toplevel,
    Child { id: u64 },
    Cursor,
}
impl gluon_wire::GluonConvertable for SurfaceUpdateTarget {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            SurfaceUpdateTarget::Toplevel {} => {
                data.write_u16(0u16)?;
            }
            SurfaceUpdateTarget::Child { id } => {
                data.write_u16(1u16)?;
                id.write(data)?;
            }
            SurfaceUpdateTarget::Cursor {} => {
                data.write_u16(2u16)?;
            }
        };
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        Ok(
            match data.read_u16()? {
                0u16 => SurfaceUpdateTarget::Toplevel,
                1u16 => {
                    let id = gluon_wire::GluonConvertable::read(data)?;
                    SurfaceUpdateTarget::Child { id }
                }
                2u16 => SurfaceUpdateTarget::Cursor,
                v => return Err(gluon_wire::GluonReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            SurfaceUpdateTarget::Toplevel {} => {
                data.write_u16(0u16)?;
            }
            SurfaceUpdateTarget::Child { id } => {
                data.write_u16(1u16)?;
                id.write_owned(data)?;
            }
            SurfaceUpdateTarget::Cursor {} => {
                data.write_u16(2u16)?;
            }
        };
        Ok(())
    }
}
