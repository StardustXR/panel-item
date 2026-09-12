use crate::panel_shell::PanelShellHandler;
use gluon_ipc::Node;
use stardust_xr_asteroids::{CustomElement, Transformable, ValidState};
use stardust_xr_fusion::{
    Error,
    client::{Client, ClientHandler},
    drawable::{MaterialParameter, Model, ModelExt as _, ModelPart},
    spatial::{Spatial, SpatialExt as _, SpatialRef, Transform},
    types::{Resource, ResourceLoadError, rgba_linear},
};
use stardust_xr_panel_item::panel_item::SurfaceUpdateTarget;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::task::AbortHandle;

#[derive(Debug)]
pub struct SurfaceModel {
    transform: Transform,
    surface: SurfaceUpdateTarget,
    model_resource: Resource,
    shell: Arc<PanelShellHandler>,
    part_path: String,
}
impl SurfaceModel {
    pub fn new(
        shell: &Node<PanelShellHandler>,
        surface: impl Into<SurfaceUpdateTarget>,
        resource: Resource,
        surface_part_path: &str,
    ) -> Self {
        Self {
            transform: Transform::IDENTITY,
            shell: shell.handler().clone(),
            part_path: surface_part_path.to_string(),
            model_resource: resource,
            surface: surface.into(),
        }
    }
}
impl<State: ValidState> CustomElement<State> for SurfaceModel {
    type Inner = SurfaceModelInner;

    type Error = Error;

    fn create_inner(
        &self,
        ctx: &stardust_xr_asteroids::Context,
        info: stardust_xr_asteroids::CreateInnerInfo,
    ) -> impl Future<Output = Result<Self::Inner, Self::Error>> {
        SurfaceModelInner::new(info.parent_space, &ctx.stardust_client, self)
    }

    fn diff(
        &self,
        old_self: &Self,
        _ctx: &stardust_xr_asteroids::Context,
        inner: &mut Self::Inner,
    ) {
        self.apply_transform(old_self, &inner.root);
        if self.model_resource != old_self.model_resource {
            tracing::warn!(
                "changing the SurfaceModel resource after creation is currently not supported"
            )
        } else if self.part_path != old_self.part_path
            && let Some(new_part) = inner.parts.get(&self.part_path)
        {
            *inner.part.lock().unwrap() = new_part.clone();
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
                            let part = part.lock().unwrap().clone();
                            _ = part
                                .set_material_parameter(
                                    "opaque",
                                    MaterialParameter::Bool { value: msg.opaque },
                                )
                                .await;
                            _ = part
                                .set_material_parameter(
                                    "unlit",
                                    MaterialParameter::Bool { value: true },
                                )
                                .await;
                            _ = part
                                .set_material_parameter(
                                    "color",
                                    MaterialParameter::Color {
                                        value: rgba_linear!(1.0, 1.0, 1.0, 1.0),
                                    },
                                )
                                .await;
                            _ = part
                                .set_material_parameter(
                                    "diffuse",
                                    MaterialParameter::Dmatex {
                                        dmatex: msg.dmatex,
                                        acquire_point: msg.acquire_point,
                                        release_point: msg.release_point,
                                    },
                                )
                                .await;
                        }
                    }
                }
            });
            inner.task.replace(task.abort_handle());
        }
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
    _model: Model,
    task: Option<AbortHandle>,
    parts: HashMap<String, ModelPart>,
}
impl SurfaceModelInner {
    async fn new(
        parent: SpatialRef,
        client: &Client<impl ClientHandler>,
        info: &SurfaceModel,
    ) -> stardust_xr_fusion::Result<Self> {
        let (root, _) = Spatial::new(client, &parent, info.transform).await?;
        let model = Model::new(client, &root, info.model_resource.clone()).await?;
        let model_parts = model.enumerate_parts().await?;
        let mut parts = HashMap::with_capacity(model_parts.len());
        for part in model_parts {
            let Ok(path) = part.get_part_path().await else {
                continue;
            };
            parts.insert(path, part);
        }
        let part = model
            .get_part(&info.part_path)
            .await?
            .ok_or(Error::ResourceLoad(ResourceLoadError::NotFound))?;
        Ok(SurfaceModelInner {
            root,
            part: Arc::new(Mutex::new(part)),
            _model: model,
            parts,
            task: None,
        })
    }
}
impl Drop for SurfaceModelInner {
    fn drop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}
