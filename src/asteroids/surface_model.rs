use std::sync::{Arc, Mutex};

use binderbinder::binder_object::BinderObject;
use stardust_xr_asteroids::{CustomElement, ValidState};
use stardust_xr_fusion::{
    AbortOnDrop,
    drawable::{self, DmatexSubmitInfo, MaterialParameter, Model, ModelPart, ModelPartAspect},
    node::{NodeError, NodeResult, NodeType},
    spatial::{SpatialRef, Transform},
    values::ResourceID,
};

use crate::{asteroids::panel_shell::PanelShellHandler, protocol::SurfaceUpdateTarget};

#[derive(Debug)]
pub struct SurfaceModel {
    transform: Transform,
    surface: SurfaceUpdateTarget,
    model_resource: ResourceID,
    shell: Arc<BinderObject<PanelShellHandler>>,
    part_path: String,
}
impl SurfaceModel {
    pub fn new(
        shell: &Arc<BinderObject<PanelShellHandler>>,
        surface: impl Into<SurfaceUpdateTarget>,
        resource: ResourceID,
        surface_part_path: &str,
    ) -> Self {
        Self {
            transform: Transform::none(),
            shell: shell.clone(),
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
        if self.model_resource != old_self.model_resource {
            if let Ok(new) = SurfaceModelInner::new(&inner.parent, self) {
                *inner = new;
            }
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
                        if let Some(msg) = recv.write().await.recv().await {
                            let part = part.lock().unwrap();
                            _ = part.set_material_parameter(
                                "opaque",
                                MaterialParameter::Bool(msg.opaque),
                            );
                            _ = part.set_material_parameter("unlit", MaterialParameter::Bool(true));
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
                        }
                    }
                }
            });
            inner.task.replace(task.into());
        }
    }

    fn spatial_aspect(&self, inner: &Self::Inner) -> stardust_xr_fusion::spatial::SpatialRef {
        inner.model.clone().as_spatial_ref()
    }
}
pub struct SurfaceModelInner {
    part: Arc<Mutex<ModelPart>>,
    model: Model,
    parent: SpatialRef,
    task: Option<AbortOnDrop>,
}
impl SurfaceModelInner {
    fn new(parent: &SpatialRef, info: &SurfaceModel) -> NodeResult<Self> {
        let model = Model::create(parent, info.transform, &info.model_resource)?;
        let part = model.part(&info.part_path)?;
        Ok(SurfaceModelInner {
            part: Arc::new(Mutex::new(part)),
            parent: parent.clone(),
            model,
            task: None,
        })
    }
}
