#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon_ipc::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon_ipc::ExternalProtocol = gluon_ipc::ExternalProtocol {
    protocol_name: "org.stardustxr.item.PanelAcceptor",
    types: &[],
};
pub mod proxies {
    use super::*;
}
#[derive(Debug, Clone)]
pub struct PanelItemAcceptor {
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for PanelItemAcceptor {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let obj = gluon_ipc::Ref::read(gluon_data)?;
        Ok(PanelItemAcceptor::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PanelItemAcceptor {
    const ID: &'static str = "org.stardustxr.item.PanelAcceptor.PanelItemAcceptor";
}
impl gluon_ipc::Interface for PanelItemAcceptor {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: PanelItemAcceptorHandler> gluon_ipc::HandledBy<H> for PanelItemAcceptor {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type PanelItemAcceptorLocal<H> = gluon_ipc::LocalRef<PanelItemAcceptor, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: PanelItemAcceptorHandler> From<PanelItemAcceptorLocal<H>> for PanelItemAcceptor {
    fn from(value: PanelItemAcceptorLocal<H>) -> PanelItemAcceptor {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for PanelItemAcceptor {
    fn from_ref(obj: gluon_ipc::Ref) -> PanelItemAcceptor {
        PanelItemAcceptor { obj }
    }
}
impl PanelItemAcceptor {
    pub async fn accept(
        &self,
        item: impl Into<super::panel_item::PanelItem>,
    ) -> Result<
        (super::panel_item::PanelShell, stardust_xr_protocol::spatial::SpatialRef),
        gluon_ipc::SendError,
    > {
        let item: super::panel_item::PanelItem = item.into();
        tracing::trace!(
            interface = "PanelItemAcceptor", method = "accept", ? item, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon_ipc::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        item.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_shell = gluon_ipc::Convertable::read(&mut reader)?;
        let __ret_output_spatial = gluon_ipc::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "PanelItemAcceptor", method = "accept", ? __ret_shell, ?
            __ret_output_spatial, "←"
        );
        Ok((__ret_shell, __ret_output_spatial))
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> PanelItemAcceptor {
        PanelItemAcceptor { obj }
    }
}
impl From<PanelItemAcceptor> for gluon_ipc::Ref {
    fn from(value: PanelItemAcceptor) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for PanelItemAcceptor {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for PanelItemAcceptor {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
    }
}
impl std::hash::Hash for PanelItemAcceptor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for PanelItemAcceptor {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for PanelItemAcceptor {}
pub trait PanelItemAcceptorHandler: gluon_ipc::Handler + Send + Sync + 'static {
    fn accept(
        &self,
        _ctx: gluon_ipc::Context,
        item: super::panel_item::PanelItem,
    ) -> impl Future<
        Output = (
            super::panel_item::PanelShell,
            stardust_xr_protocol::spatial::SpatialRef,
        ),
    > + Send + Sync;
    ///Dispatched instead of [`Self::accept`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `accept` and sends the result through `reply`. Override this method instead of `accept` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn accept_oneway(
        &self,
        _ctx: gluon_ipc::Context,
        item: super::panel_item::PanelItem,
        reply: gluon_ipc::ReplySender<
            (super::panel_item::PanelShell, stardust_xr_protocol::spatial::SpatialRef),
        >,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            let (shell, output_spatial) = self.accept(_ctx, item).await;
            reply.send((shell, output_spatial))
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_ipc::DataReader,
        ctx: gluon_ipc::Context,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_item = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelItemAcceptor", method = "accept", ? param_item,
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon_ipc::ReplySender<
                        (
                            super::panel_item::PanelShell,
                            stardust_xr_protocol::spatial::SpatialRef,
                        ),
                    > = gluon_ipc::ReplySender::new(
                        return_callback,
                        |(shell, output_spatial), gluon_out| {
                            tracing::trace!(
                                interface = "PanelItemAcceptor", method = "accept", ? shell,
                                ? output_spatial, "←"
                            );
                            shell.write_owned(gluon_out)?;
                            output_spatial.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.accept_oneway(ctx, param_item, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelItemAcceptor", method =
                                "accept", method_id = 8u32
                            ),
                        )
                        .await?;
                }
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<PanelItemAcceptor, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        PanelItemAcceptor::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<PanelItemAcceptor, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        PanelItemAcceptor::new_service(self)
    }
}
pub mod proxied {
    use super::*;
}
