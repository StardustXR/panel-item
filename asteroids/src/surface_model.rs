use std::sync::{Arc, Mutex};

use binderbinder::binder_object::BinderObject;
use stardust_xr_asteroids::{CustomElement, Transformable, ValidState};
use stardust_xr_fusion::{
    AbortOnDrop,
    drawable::{self, DmatexSubmitInfo, MaterialParameter, Model, ModelPart, ModelPartAspect},
    node::{NodeError, NodeResult, NodeType},
    spatial::{Spatial, SpatialRef, Transform},
    values::{ResourceID, color::rgba_linear},
};

use stardust_xr_panel_item::protocol::SurfaceUpdateTarget;

use crate::panel_shell::PanelShellHandler;

#[derive(Debug)]
pub struct SurfaceModel {
    transform: Transform,
    surface: SurfaceUpdateTarget,
    model_resource: ResourceID,
    shell: Arc<PanelShellHandler>,
    part_path: String,
}
impl SurfaceModel {
    pub fn new(
        shell: &BinderObject<PanelShellHandler>,
        surface: impl Into<SurfaceUpdateTarget>,
        resource: ResourceID,
        surface_part_path: &str,
    ) -> Self {
        Self {
            transform: Transform::none(),
            shell: shell.handler_arc().clone(),
            part_path: surface_part_path.to_string(),
            model_resource: resource,
            surface: surface.into(),
        }
    }
}
impl<State: ValidState> CustomElement<State> for SurfaceModel {
    type Inner = SurfaceModelInner;

    type Resource = ();

    type Error = NodeError;

    fn create_inner(
        &self,
        _asteroids_context: &stardust_xr_asteroids::Context,
        info: stardust_xr_asteroids::CreateInnerInfo,
        _resource: &mut Self::Resource,
    ) -> Result<Self::Inner, Self::Error> {
        SurfaceModelInner::new(info.parent_space, self)
    }

    fn diff(&self, old_self: &Self, inner: &mut Self::Inner, _resource: &mut Self::Resource) {
        self.apply_transform(old_self, &inner.root);
        if self.model_resource != old_self.model_resource {
            _ = inner.recreate_model(self);
        } else if self.part_path != old_self.part_path {
            if let Ok(new_part) = inner.model.part(&self.part_path) {
                *inner.part.lock().unwrap() = new_part;
            }
        }
        if inner
            .task
            .as_ref()
            .is_none_or(|_| !Arc::ptr_eq(&self.shell, &old_self.shell))
            || self.surface != old_self.surface
        {
            let task = tokio::spawn({
                let part = inner.part.clone();
                let surface_rx = self.shell.surface_rx.clone();
                let target = self.surface;
                async move {
                    let recv = surface_rx
                        .read()
                        .await
                        .get(&target)
                        .expect("invalid surface target")
                        .clone();
                    loop {
                        let mut recv = recv.write().await;
                        if let Some(msg) = recv.recv().await {
                            let part = part.lock().unwrap();
                            _ = part.set_material_parameter(
                                "opaque",
                                MaterialParameter::Bool(msg.opaque),
                            );
                            _ = part.set_material_parameter("unlit", MaterialParameter::Bool(true));
                            _ = part.set_material_parameter(
                                "color",
                                MaterialParameter::Color(rgba_linear!(1.0, 1.0, 1.0, 1.0)),
                            );
                            let dmatex_id = part.client().generate_id();
                            _ = drawable::import_dmatex_uid(
                                part.client(),
                                dmatex_id,
                                msg.dmatex_uid,
                            );
                            _ = part.set_material_parameter(
                                "diffuse",
                                MaterialParameter::Dmatex(DmatexSubmitInfo {
                                    dmatex_id,
                                    acquire_point: msg.acquire_point,
                                    release_point: msg.release_point,
                                }),
                            );
                            _ = drawable::unregister_dmatex(part.client(), dmatex_id);
                        }
                    }
                }
            });
            inner.task.replace(task.into());
        }
    }

    fn spatial_aspect(&self, inner: &Self::Inner) -> stardust_xr_fusion::spatial::SpatialRef {
        inner.root.clone().as_spatial_ref()
    }
}
impl Transformable for SurfaceModel {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}
pub struct SurfaceModelInner {
    root: Spatial,
    part: Arc<Mutex<ModelPart>>,
    model: Model,
    task: Option<AbortOnDrop>,
}
impl SurfaceModelInner {
    fn new(parent: &SpatialRef, info: &SurfaceModel) -> NodeResult<Self> {
        let root = Spatial::create(parent, info.transform)?;
        let model = Model::create(&root, Transform::identity(), &info.model_resource)?;
        let part = model.part(&info.part_path)?;
        Ok(SurfaceModelInner {
            root,
            part: Arc::new(Mutex::new(part)),
            model,
            task: None,
        })
    }
    fn recreate_model(&mut self, info: &SurfaceModel) -> NodeResult<()> {
        let model = Model::create(&self.root, info.transform, &info.model_resource)?;
        let part = model.part(&info.part_path)?;
        self.model = model;
        *self.part.lock().unwrap() = part;
        self.task.take();
        Ok(())
    }
}
