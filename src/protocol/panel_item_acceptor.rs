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
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for PanelItemAcceptor {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(PanelItemAcceptor::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for PanelItemAcceptor {
    const ID: &'static str = "org.stardustxr.item.PanelAcceptor.PanelItemAcceptor";
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
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        item.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_shell = gluon::Convertable::read(&mut reader)?;
        let __ret_output_spatial = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "PanelItemAcceptor", method = "accept", ? __ret_shell, ?
            __ret_output_spatial, "←"
        );
        Ok((__ret_shell, __ret_output_spatial))
    }
    pub fn from_handler<H: PanelItemAcceptorHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> PanelItemAcceptor {
        PanelItemAcceptor::from_object_or_ref(
            gluon::OwnedObjectRef::to_object_or_ref(obj),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> PanelItemAcceptor {
        PanelItemAcceptor { obj }
    }
}
impl From<PanelItemAcceptor> for gluon::ObjectOrRef {
    fn from(value: PanelItemAcceptor) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for PanelItemAcceptor {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl gluon::Liveness for PanelItemAcceptor {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
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
                    let return_callback = gluon_data.read_binder()?;
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
}
pub mod proxied {
    use super::*;
}
