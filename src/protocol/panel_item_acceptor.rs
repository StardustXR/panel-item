#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.item.PanelAcceptor",
    types: &[],
};
pub mod proxies {
    use super::*;
}
#[derive(Debug, Clone)]
pub struct PanelItemAcceptor {
    obj: gluon::Ref,
}
impl gluon::Convertable for PanelItemAcceptor {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(PanelItemAcceptor::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PanelItemAcceptor {
    const ID: &'static str = "org.stardustxr.item.PanelAcceptor.PanelItemAcceptor";
}
impl gluon::Interface for PanelItemAcceptor {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: PanelItemAcceptorHandler> gluon::HandledBy<H> for PanelItemAcceptor {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type PanelItemAcceptorLocal<H> = gluon::LocalRef<PanelItemAcceptor, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: PanelItemAcceptorHandler> From<PanelItemAcceptorLocal<H>> for PanelItemAcceptor {
    fn from(value: PanelItemAcceptorLocal<H>) -> PanelItemAcceptor {
        value.into_proxy()
    }
}
impl gluon::RefExt for PanelItemAcceptor {
    fn from_ref(obj: gluon::Ref) -> PanelItemAcceptor {
        PanelItemAcceptor { obj }
    }
}
impl PanelItemAcceptor {
    pub async fn accept(
        &self,
        item: impl Into<super::panel_item::PanelItem>,
    ) -> Result<
        (super::panel_item::PanelShell, stardust_xr_protocol::spatial::SpatialRef),
        gluon::SendError,
    > {
        let item: super::panel_item::PanelItem = item.into();
        tracing::trace!(
            interface = "PanelItemAcceptor", method = "accept", ? item, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        item.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_shell = gluon::Convertable::read(&mut reader)?;
        let __ret_output_spatial = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "PanelItemAcceptor", method = "accept", ? __ret_shell, ?
            __ret_output_spatial, "←"
        );
        Ok((__ret_shell, __ret_output_spatial))
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> PanelItemAcceptor {
        PanelItemAcceptor { obj }
    }
}
impl From<PanelItemAcceptor> for gluon::Ref {
    fn from(value: PanelItemAcceptor) -> Self {
        value.obj
    }
}
impl gluon::ToRef for PanelItemAcceptor {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for PanelItemAcceptor {
    fn death_notifier(&self) -> gluon::DeathNotifier {
        gluon::Liveness::death_notifier(&self.obj)
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
pub trait PanelItemAcceptorHandler: gluon::Handler + Send + Sync + 'static {
    fn accept(
        &self,
        _ctx: gluon::Context,
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
        _ctx: gluon::Context,
        item: super::panel_item::PanelItem,
        reply: gluon::ReplySender<
            (super::panel_item::PanelShell, stardust_xr_protocol::spatial::SpatialRef),
        >,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let (shell, output_spatial) = self.accept(_ctx, item).await;
            reply.send((shell, output_spatial))
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon::DataReader,
        ctx: gluon::Context,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_item = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelItemAcceptor", method = "accept", ? param_item,
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        (
                            super::panel_item::PanelShell,
                            stardust_xr_protocol::spatial::SpatialRef,
                        ),
                    > = gluon::ReplySender::new(
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
        (gluon::Node<Self>, gluon::LocalRef<PanelItemAcceptor, Self>),
        gluon::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon::RefExt;
        PanelItemAcceptor::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon::LocalRef<PanelItemAcceptor, Self>, gluon::NodeError>
    where
        Self: Sized,
    {
        use gluon::RefExt;
        PanelItemAcceptor::new_service(self)
    }
}
pub mod proxied {
    use super::*;
}
