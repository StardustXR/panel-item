#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.item.Panel",
    types: &[
        gluon::ExternalGluonType {
            name: "ModifierState",
            supported_derives: gluon::Derives::from_bits_truncate(895u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "ToplevelState",
            supported_derives: gluon::Derives::from_bits_truncate(798u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "ChildState",
            supported_derives: gluon::Derives::from_bits_truncate(778u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "PanelItemInitData",
            supported_derives: gluon::Derives::from_bits_truncate(778u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Rect",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Geometry",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "ScrollSource",
            supported_derives: gluon::Derives::from_bits_truncate(895u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "SurfaceId",
            supported_derives: gluon::Derives::from_bits_truncate(895u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "SurfaceUpdateTarget",
            supported_derives: gluon::Derives::from_bits_truncate(895u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///Modifier state driven by xkbcommon
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifierState {
    pub depressed: u32,
    pub latched: u32,
    pub locked: u32,
    pub layout_group: u32,
}
impl gluon::Convertable for ModifierState {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.depressed.write(gluon_data)?;
        self.latched.write(gluon_data)?;
        self.locked.write(gluon_data)?;
        self.layout_group.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let depressed = gluon::Convertable::read(gluon_data)?;
        let latched = gluon::Convertable::read(gluon_data)?;
        let locked = gluon::Convertable::read(gluon_data)?;
        let layout_group = gluon::Convertable::read(gluon_data)?;
        Ok(ModifierState {
            depressed,
            latched,
            locked,
            layout_group,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.depressed.write_owned(gluon_data)?;
        self.latched.write_owned(gluon_data)?;
        self.locked.write_owned(gluon_data)?;
        self.layout_group.write_owned(gluon_data)?;
        Ok(())
    }
}
///ToplevelState
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ToplevelState {
    pub parent: Option<u64>,
    pub title: Option<String>,
    pub app_id: Option<String>,
    pub size: stardust_xr_protocol::types::proxies::Size2,
    pub min_size: Option<stardust_xr_protocol::types::proxies::Size2>,
    pub max_size: Option<stardust_xr_protocol::types::proxies::Size2>,
}
impl gluon::Convertable for ToplevelState {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.parent.write(gluon_data)?;
        self.title.write(gluon_data)?;
        self.app_id.write(gluon_data)?;
        {
            let __w: stardust_xr_protocol::types::proxied::Size2 = self
                .size
                .clone()
                .into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Option<stardust_xr_protocol::types::proxied::Size2> = self
                .min_size
                .clone()
                .map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Option<stardust_xr_protocol::types::proxied::Size2> = self
                .max_size
                .clone()
                .map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let parent = gluon::Convertable::read(gluon_data)?;
        let title = gluon::Convertable::read(gluon_data)?;
        let app_id = gluon::Convertable::read(gluon_data)?;
        let size: stardust_xr_protocol::types::proxies::Size2 = {
            let __w: stardust_xr_protocol::types::proxied::Size2 = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let min_size: Option<stardust_xr_protocol::types::proxies::Size2> = {
            let __w: Option<stardust_xr_protocol::types::proxied::Size2> = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.map(|__v| __v.into())
        };
        let max_size: Option<stardust_xr_protocol::types::proxies::Size2> = {
            let __w: Option<stardust_xr_protocol::types::proxied::Size2> = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.map(|__v| __v.into())
        };
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
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.parent.write_owned(gluon_data)?;
        self.title.write_owned(gluon_data)?;
        self.app_id.write_owned(gluon_data)?;
        {
            let __w: stardust_xr_protocol::types::proxied::Size2 = self.size.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Option<stardust_xr_protocol::types::proxied::Size2> = self
                .min_size
                .map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Option<stardust_xr_protocol::types::proxied::Size2> = self
                .max_size
                .map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
}
///ChildState
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChildState {
    pub id: u64,
    pub parent: SurfaceId,
    pub geometry: Geometry,
    pub z_order: i32,
    pub input_regions: Vec<Rect>,
}
impl gluon::Convertable for ChildState {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.id.write(gluon_data)?;
        self.parent.write(gluon_data)?;
        self.geometry.write(gluon_data)?;
        self.z_order.write(gluon_data)?;
        self.input_regions.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let id = gluon::Convertable::read(gluon_data)?;
        let parent = gluon::Convertable::read(gluon_data)?;
        let geometry = gluon::Convertable::read(gluon_data)?;
        let z_order = gluon::Convertable::read(gluon_data)?;
        let input_regions = gluon::Convertable::read(gluon_data)?;
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
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.id.write_owned(gluon_data)?;
        self.parent.write_owned(gluon_data)?;
        self.geometry.write_owned(gluon_data)?;
        self.z_order.write_owned(gluon_data)?;
        self.input_regions.write_owned(gluon_data)?;
        Ok(())
    }
}
///PanelItemInitData
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PanelItemInitData {
    pub cursor: Option<Geometry>,
    pub toplevel: ToplevelState,
    pub children: Vec<ChildState>,
}
impl gluon::Convertable for PanelItemInitData {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.cursor.write(gluon_data)?;
        self.toplevel.write(gluon_data)?;
        self.children.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let cursor = gluon::Convertable::read(gluon_data)?;
        let toplevel = gluon::Convertable::read(gluon_data)?;
        let children = gluon::Convertable::read(gluon_data)?;
        Ok(PanelItemInitData {
            cursor,
            toplevel,
            children,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.cursor.write_owned(gluon_data)?;
        self.toplevel.write_owned(gluon_data)?;
        self.children.write_owned(gluon_data)?;
        Ok(())
    }
}
///Rect
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    ///this is +Y == Down +X == Right
    pub origin: stardust_xr_protocol::types::proxies::Vec2F,
    pub size: stardust_xr_protocol::types::proxies::Vec2F,
}
impl gluon::Convertable for Rect {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: stardust_xr_protocol::types::proxied::Vec2F = self
                .origin
                .clone()
                .into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: stardust_xr_protocol::types::proxied::Vec2F = self
                .size
                .clone()
                .into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let origin: stardust_xr_protocol::types::proxies::Vec2F = {
            let __w: stardust_xr_protocol::types::proxied::Vec2F = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let size: stardust_xr_protocol::types::proxies::Vec2F = {
            let __w: stardust_xr_protocol::types::proxied::Vec2F = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        Ok(Rect { origin, size })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: stardust_xr_protocol::types::proxied::Vec2F = self.origin.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: stardust_xr_protocol::types::proxied::Vec2F = self.size.into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
}
///Geometry
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Geometry {
    ///this is +Y == Down +X == Right
    pub origin: stardust_xr_protocol::types::proxies::Vec2I,
    pub size: stardust_xr_protocol::types::proxies::Size2,
}
impl gluon::Convertable for Geometry {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: stardust_xr_protocol::types::proxied::Vec2I = self
                .origin
                .clone()
                .into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: stardust_xr_protocol::types::proxied::Size2 = self
                .size
                .clone()
                .into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let origin: stardust_xr_protocol::types::proxies::Vec2I = {
            let __w: stardust_xr_protocol::types::proxied::Vec2I = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let size: stardust_xr_protocol::types::proxies::Size2 = {
            let __w: stardust_xr_protocol::types::proxied::Size2 = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        Ok(Geometry { origin, size })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: stardust_xr_protocol::types::proxied::Vec2I = self.origin.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: stardust_xr_protocol::types::proxied::Size2 = self.size.into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
}
///ScrollSource
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScrollSource {
    Wheel,
    Touch,
    Continuous,
    WheelTilt,
}
impl gluon::Convertable for ScrollSource {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            ScrollSource::Wheel => {
                gluon_data.write_u16(0u16)?;
            }
            ScrollSource::Touch => {
                gluon_data.write_u16(1u16)?;
            }
            ScrollSource::Continuous => {
                gluon_data.write_u16(2u16)?;
            }
            ScrollSource::WheelTilt => {
                gluon_data.write_u16(3u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => ScrollSource::Wheel,
                1u16 => ScrollSource::Touch,
                2u16 => ScrollSource::Continuous,
                3u16 => ScrollSource::WheelTilt,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            ScrollSource::Wheel => {
                gluon_data.write_u16(0u16)?;
            }
            ScrollSource::Touch => {
                gluon_data.write_u16(1u16)?;
            }
            ScrollSource::Continuous => {
                gluon_data.write_u16(2u16)?;
            }
            ScrollSource::WheelTilt => {
                gluon_data.write_u16(3u16)?;
            }
        };
        Ok(())
    }
}
///SurfaceId
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SurfaceId {
    Toplevel,
    Child { id: u64 },
}
impl gluon::Convertable for SurfaceId {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            SurfaceId::Toplevel => {
                gluon_data.write_u16(0u16)?;
            }
            SurfaceId::Child { id } => {
                gluon_data.write_u16(1u16)?;
                id.write(gluon_data)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => SurfaceId::Toplevel,
                1u16 => {
                    let id = gluon::Convertable::read(gluon_data)?;
                    SurfaceId::Child { id }
                }
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            SurfaceId::Toplevel => {
                gluon_data.write_u16(0u16)?;
            }
            SurfaceId::Child { id } => {
                gluon_data.write_u16(1u16)?;
                id.write_owned(gluon_data)?;
            }
        };
        Ok(())
    }
}
///SurfaceDmatexTarget
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SurfaceUpdateTarget {
    Toplevel,
    Child { id: u64 },
    Cursor,
}
impl gluon::Convertable for SurfaceUpdateTarget {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            SurfaceUpdateTarget::Toplevel => {
                gluon_data.write_u16(0u16)?;
            }
            SurfaceUpdateTarget::Child { id } => {
                gluon_data.write_u16(1u16)?;
                id.write(gluon_data)?;
            }
            SurfaceUpdateTarget::Cursor => {
                gluon_data.write_u16(2u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => SurfaceUpdateTarget::Toplevel,
                1u16 => {
                    let id = gluon::Convertable::read(gluon_data)?;
                    SurfaceUpdateTarget::Child { id }
                }
                2u16 => SurfaceUpdateTarget::Cursor,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            SurfaceUpdateTarget::Toplevel => {
                gluon_data.write_u16(0u16)?;
            }
            SurfaceUpdateTarget::Child { id } => {
                gluon_data.write_u16(1u16)?;
                id.write_owned(gluon_data)?;
            }
            SurfaceUpdateTarget::Cursor => {
                gluon_data.write_u16(2u16)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct PanelItem {
    obj: gluon::Ref,
}
impl gluon::Convertable for PanelItem {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(PanelItem::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PanelItem {
    const ID: &'static str = "org.stardustxr.item.Panel.PanelItem";
}
impl gluon::Interface for PanelItem {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: PanelItemHandler> gluon::HandledBy<H> for PanelItem {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type PanelItemLocal<H> = gluon::LocalRef<PanelItem, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: PanelItemHandler> From<PanelItemLocal<H>> for PanelItem {
    fn from(value: PanelItemLocal<H>) -> PanelItem {
        value.into_proxy()
    }
}
impl gluon::RefExt for PanelItem {
    fn from_ref(obj: gluon::Ref) -> PanelItem {
        PanelItem { obj }
    }
}
impl PanelItem {
    ///delta and position are +Y == Down +X == Right
    pub fn pointer_motion(
        &self,
        surface: impl Into<SurfaceId>,
        delta: Option<stardust_xr_protocol::types::proxies::Vec2F>,
        position: stardust_xr_protocol::types::proxies::Vec2F,
        timestamp: impl Into<Option<stardust_xr_protocol::types::Timestamp>>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let delta: Option<stardust_xr_protocol::types::proxied::Vec2F> = delta
            .map(|__v| __v.into());
        let position: stardust_xr_protocol::types::proxied::Vec2F = position.into();
        let timestamp: Option<stardust_xr_protocol::types::Timestamp> = timestamp.into();
        tracing::trace!(
            interface = "PanelItem", method = "pointer_motion", ? surface, ? delta, ?
            position, ? timestamp, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        delta.write(&mut gluon_builder)?;
        position.write(&mut gluon_builder)?;
        timestamp.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        Ok(())
    }
    ///uses event ids from `input_event_codes.h`
    pub fn pointer_button(
        &self,
        surface: impl Into<SurfaceId>,
        button: impl Into<u32>,
        pressed: impl Into<bool>,
        timestamp: impl Into<Option<stardust_xr_protocol::types::Timestamp>>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let button: u32 = button.into();
        let pressed: bool = pressed.into();
        let timestamp: Option<stardust_xr_protocol::types::Timestamp> = timestamp.into();
        tracing::trace!(
            interface = "PanelItem", method = "pointer_button", ? surface, ? button, ?
            pressed, ? timestamp, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        button.write(&mut gluon_builder)?;
        pressed.write(&mut gluon_builder)?;
        timestamp.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 9u32, gluon_builder)?;
        Ok(())
    }
    ///delta is +Y == Down +X == Right
    pub fn pointer_scroll_pixels(
        &self,
        surface: impl Into<SurfaceId>,
        delta: stardust_xr_protocol::types::proxies::Vec2F,
        source: impl Into<ScrollSource>,
        timestamp: impl Into<Option<stardust_xr_protocol::types::Timestamp>>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let delta: stardust_xr_protocol::types::proxied::Vec2F = delta.into();
        let source: ScrollSource = source.into();
        let timestamp: Option<stardust_xr_protocol::types::Timestamp> = timestamp.into();
        tracing::trace!(
            interface = "PanelItem", method = "pointer_scroll_pixels", ? surface, ?
            delta, ? source, ? timestamp, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        delta.write(&mut gluon_builder)?;
        source.write(&mut gluon_builder)?;
        timestamp.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 10u32, gluon_builder)?;
        Ok(())
    }
    ///delta is +Y == Down +X == Right
    pub fn pointer_scroll_discrete(
        &self,
        surface: impl Into<SurfaceId>,
        delta: stardust_xr_protocol::types::proxies::Vec2F,
        source: impl Into<ScrollSource>,
        timestamp: impl Into<Option<stardust_xr_protocol::types::Timestamp>>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let delta: stardust_xr_protocol::types::proxied::Vec2F = delta.into();
        let source: ScrollSource = source.into();
        let timestamp: Option<stardust_xr_protocol::types::Timestamp> = timestamp.into();
        tracing::trace!(
            interface = "PanelItem", method = "pointer_scroll_discrete", ? surface, ?
            delta, ? source, ? timestamp, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        delta.write(&mut gluon_builder)?;
        source.write(&mut gluon_builder)?;
        timestamp.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 11u32, gluon_builder)?;
        Ok(())
    }
    pub fn pointer_scroll_stop(
        &self,
        surface: impl Into<SurfaceId>,
        timestamp: impl Into<Option<stardust_xr_protocol::types::Timestamp>>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let timestamp: Option<stardust_xr_protocol::types::Timestamp> = timestamp.into();
        tracing::trace!(
            interface = "PanelItem", method = "pointer_scroll_stop", ? surface, ?
            timestamp, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        timestamp.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 12u32, gluon_builder)?;
        Ok(())
    }
    pub fn key(
        &self,
        surface: impl Into<SurfaceId>,
        key: impl Into<u32>,
        pressed: impl Into<bool>,
        modifier_state: impl Into<ModifierState>,
        keymap: impl Into<stardust_xr_protocol::keymap::Keymap>,
        timestamp: impl Into<Option<stardust_xr_protocol::types::Timestamp>>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let key: u32 = key.into();
        let pressed: bool = pressed.into();
        let modifier_state: ModifierState = modifier_state.into();
        let keymap: stardust_xr_protocol::keymap::Keymap = keymap.into();
        let timestamp: Option<stardust_xr_protocol::types::Timestamp> = timestamp.into();
        tracing::trace!(
            interface = "PanelItem", method = "key", ? surface, ? key, ? pressed, ?
            modifier_state, ? keymap, ? timestamp, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        key.write(&mut gluon_builder)?;
        pressed.write(&mut gluon_builder)?;
        modifier_state.write(&mut gluon_builder)?;
        keymap.write(&mut gluon_builder)?;
        timestamp.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 13u32, gluon_builder)?;
        Ok(())
    }
    ///position is +Y == Down +X == Right
    pub fn touch_down(
        &self,
        surface: impl Into<SurfaceId>,
        touch_id: impl Into<u32>,
        position: stardust_xr_protocol::types::proxies::Vec2F,
        timestamp: impl Into<Option<stardust_xr_protocol::types::Timestamp>>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let touch_id: u32 = touch_id.into();
        let position: stardust_xr_protocol::types::proxied::Vec2F = position.into();
        let timestamp: Option<stardust_xr_protocol::types::Timestamp> = timestamp.into();
        tracing::trace!(
            interface = "PanelItem", method = "touch_down", ? surface, ? touch_id, ?
            position, ? timestamp, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        touch_id.write(&mut gluon_builder)?;
        position.write(&mut gluon_builder)?;
        timestamp.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 14u32, gluon_builder)?;
        Ok(())
    }
    ///position is +Y == Down +X == Right
    pub fn touch_move(
        &self,
        touch_id: impl Into<u32>,
        position: stardust_xr_protocol::types::proxies::Vec2F,
        timestamp: impl Into<Option<stardust_xr_protocol::types::Timestamp>>,
    ) -> Result<(), gluon::SendError> {
        let touch_id: u32 = touch_id.into();
        let position: stardust_xr_protocol::types::proxied::Vec2F = position.into();
        let timestamp: Option<stardust_xr_protocol::types::Timestamp> = timestamp.into();
        tracing::trace!(
            interface = "PanelItem", method = "touch_move", ? touch_id, ? position, ?
            timestamp, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        touch_id.write(&mut gluon_builder)?;
        position.write(&mut gluon_builder)?;
        timestamp.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 15u32, gluon_builder)?;
        Ok(())
    }
    pub fn touch_up(
        &self,
        touch_id: impl Into<u32>,
        timestamp: impl Into<Option<stardust_xr_protocol::types::Timestamp>>,
    ) -> Result<(), gluon::SendError> {
        let touch_id: u32 = touch_id.into();
        let timestamp: Option<stardust_xr_protocol::types::Timestamp> = timestamp.into();
        tracing::trace!(
            interface = "PanelItem", method = "touch_up", ? touch_id, ? timestamp, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        touch_id.write(&mut gluon_builder)?;
        timestamp.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 16u32, gluon_builder)?;
        Ok(())
    }
    pub fn close_toplevel(&self) -> Result<(), gluon::SendError> {
        tracing::trace!(interface = "PanelItem", method = "close_toplevel", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        gluon::transact(&self.obj, 17u32, gluon_builder)?;
        Ok(())
    }
    pub fn resize_toplevel_to_app_request(&self) -> Result<(), gluon::SendError> {
        tracing::trace!(
            interface = "PanelItem", method = "resize_toplevel_to_app_request", "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        gluon::transact(&self.obj, 18u32, gluon_builder)?;
        Ok(())
    }
    pub fn request_toplevel_resize(
        &self,
        new_size: stardust_xr_protocol::types::proxies::Size2,
    ) -> Result<(), gluon::SendError> {
        let new_size: stardust_xr_protocol::types::proxied::Size2 = new_size.into();
        tracing::trace!(
            interface = "PanelItem", method = "request_toplevel_resize", ? new_size,
            "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        new_size.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 19u32, gluon_builder)?;
        Ok(())
    }
    pub fn toplevel_focused(
        &self,
        focused: impl Into<bool>,
    ) -> Result<(), gluon::SendError> {
        let focused: bool = focused.into();
        tracing::trace!(
            interface = "PanelItem", method = "toplevel_focused", ? focused, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        focused.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 20u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> PanelItem {
        PanelItem { obj }
    }
}
impl From<PanelItem> for gluon::Ref {
    fn from(value: PanelItem) -> Self {
        value.obj
    }
}
impl gluon::ToRef for PanelItem {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for PanelItem {
    fn death_notifier(&self) -> gluon::DeathNotifier {
        gluon::Liveness::death_notifier(&self.obj)
    }
}
impl std::hash::Hash for PanelItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for PanelItem {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for PanelItem {}
pub trait PanelItemHandler: gluon::Handler + Send + Sync + 'static {
    ///delta and position are +Y == Down +X == Right
    fn pointer_motion(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
        delta: Option<stardust_xr_protocol::types::proxies::Vec2F>,
        position: stardust_xr_protocol::types::proxies::Vec2F,
        timestamp: Option<stardust_xr_protocol::types::Timestamp>,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///uses event ids from `input_event_codes.h`
    fn pointer_button(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
        button: u32,
        pressed: bool,
        timestamp: Option<stardust_xr_protocol::types::Timestamp>,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///delta is +Y == Down +X == Right
    fn pointer_scroll_pixels(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
        delta: stardust_xr_protocol::types::proxies::Vec2F,
        source: ScrollSource,
        timestamp: Option<stardust_xr_protocol::types::Timestamp>,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///delta is +Y == Down +X == Right
    fn pointer_scroll_discrete(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
        delta: stardust_xr_protocol::types::proxies::Vec2F,
        source: ScrollSource,
        timestamp: Option<stardust_xr_protocol::types::Timestamp>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn pointer_scroll_stop(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
        timestamp: Option<stardust_xr_protocol::types::Timestamp>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn key(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
        key: u32,
        pressed: bool,
        modifier_state: ModifierState,
        keymap: stardust_xr_protocol::keymap::Keymap,
        timestamp: Option<stardust_xr_protocol::types::Timestamp>,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///position is +Y == Down +X == Right
    fn touch_down(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
        touch_id: u32,
        position: stardust_xr_protocol::types::proxies::Vec2F,
        timestamp: Option<stardust_xr_protocol::types::Timestamp>,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///position is +Y == Down +X == Right
    fn touch_move(
        &self,
        _ctx: gluon::Context,
        touch_id: u32,
        position: stardust_xr_protocol::types::proxies::Vec2F,
        timestamp: Option<stardust_xr_protocol::types::Timestamp>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn touch_up(
        &self,
        _ctx: gluon::Context,
        touch_id: u32,
        timestamp: Option<stardust_xr_protocol::types::Timestamp>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn close_toplevel(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn resize_toplevel_to_app_request(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn request_toplevel_resize(
        &self,
        _ctx: gluon::Context,
        new_size: stardust_xr_protocol::types::proxies::Size2,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_focused(
        &self,
        _ctx: gluon::Context,
        focused: bool,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon::DataReader,
        ctx: gluon::Context,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let __wire_param_delta: Option<
                        stardust_xr_protocol::types::proxied::Vec2F,
                    > = gluon::Convertable::read(&mut gluon_data)?;
                    let __wire_param_position: stardust_xr_protocol::types::proxied::Vec2F = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_timestamp = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelItem", method = "pointer_motion", ?
                        param_surface, param_delta = ? __wire_param_delta, param_position
                        = ? __wire_param_position, ? param_timestamp, "dispatching"
                    );
                    let param_delta: Option<
                        stardust_xr_protocol::types::proxies::Vec2F,
                    > = {
                        let __w = __wire_param_delta;
                        __w.map(|__v| __v.into())
                    };
                    let param_position: stardust_xr_protocol::types::proxies::Vec2F = {
                        let __w = __wire_param_position;
                        __w.into()
                    };
                    drop(gluon_data);
                    self.pointer_motion(
                            ctx,
                            param_surface,
                            param_delta,
                            param_position,
                            param_timestamp,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelItem", method =
                                "pointer_motion", method_id = 8u32
                            ),
                        )
                        .await;
                }
                9u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let param_button = gluon::Convertable::read(&mut gluon_data)?;
                    let param_pressed = gluon::Convertable::read(&mut gluon_data)?;
                    let param_timestamp = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelItem", method = "pointer_button", ?
                        param_surface, ? param_button, ? param_pressed, ?
                        param_timestamp, "dispatching"
                    );
                    drop(gluon_data);
                    self.pointer_button(
                            ctx,
                            param_surface,
                            param_button,
                            param_pressed,
                            param_timestamp,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelItem", method =
                                "pointer_button", method_id = 9u32
                            ),
                        )
                        .await;
                }
                10u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let __wire_param_delta: stardust_xr_protocol::types::proxied::Vec2F = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_source = gluon::Convertable::read(&mut gluon_data)?;
                    let param_timestamp = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelItem", method = "pointer_scroll_pixels", ?
                        param_surface, param_delta = ? __wire_param_delta, ?
                        param_source, ? param_timestamp, "dispatching"
                    );
                    let param_delta: stardust_xr_protocol::types::proxies::Vec2F = {
                        let __w = __wire_param_delta;
                        __w.into()
                    };
                    drop(gluon_data);
                    self.pointer_scroll_pixels(
                            ctx,
                            param_surface,
                            param_delta,
                            param_source,
                            param_timestamp,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelItem", method =
                                "pointer_scroll_pixels", method_id = 10u32
                            ),
                        )
                        .await;
                }
                11u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let __wire_param_delta: stardust_xr_protocol::types::proxied::Vec2F = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_source = gluon::Convertable::read(&mut gluon_data)?;
                    let param_timestamp = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelItem", method = "pointer_scroll_discrete", ?
                        param_surface, param_delta = ? __wire_param_delta, ?
                        param_source, ? param_timestamp, "dispatching"
                    );
                    let param_delta: stardust_xr_protocol::types::proxies::Vec2F = {
                        let __w = __wire_param_delta;
                        __w.into()
                    };
                    drop(gluon_data);
                    self.pointer_scroll_discrete(
                            ctx,
                            param_surface,
                            param_delta,
                            param_source,
                            param_timestamp,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelItem", method =
                                "pointer_scroll_discrete", method_id = 11u32
                            ),
                        )
                        .await;
                }
                12u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let param_timestamp = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelItem", method = "pointer_scroll_stop", ?
                        param_surface, ? param_timestamp, "dispatching"
                    );
                    drop(gluon_data);
                    self.pointer_scroll_stop(ctx, param_surface, param_timestamp)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelItem", method =
                                "pointer_scroll_stop", method_id = 12u32
                            ),
                        )
                        .await;
                }
                13u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let param_key = gluon::Convertable::read(&mut gluon_data)?;
                    let param_pressed = gluon::Convertable::read(&mut gluon_data)?;
                    let param_modifier_state = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_keymap = gluon::Convertable::read(&mut gluon_data)?;
                    let param_timestamp = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelItem", method = "key", ? param_surface, ?
                        param_key, ? param_pressed, ? param_modifier_state, ?
                        param_keymap, ? param_timestamp, "dispatching"
                    );
                    drop(gluon_data);
                    self.key(
                            ctx,
                            param_surface,
                            param_key,
                            param_pressed,
                            param_modifier_state,
                            param_keymap,
                            param_timestamp,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelItem", method = "key",
                                method_id = 13u32
                            ),
                        )
                        .await;
                }
                14u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let param_touch_id = gluon::Convertable::read(&mut gluon_data)?;
                    let __wire_param_position: stardust_xr_protocol::types::proxied::Vec2F = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_timestamp = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelItem", method = "touch_down", ? param_surface,
                        ? param_touch_id, param_position = ? __wire_param_position, ?
                        param_timestamp, "dispatching"
                    );
                    let param_position: stardust_xr_protocol::types::proxies::Vec2F = {
                        let __w = __wire_param_position;
                        __w.into()
                    };
                    drop(gluon_data);
                    self.touch_down(
                            ctx,
                            param_surface,
                            param_touch_id,
                            param_position,
                            param_timestamp,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelItem", method =
                                "touch_down", method_id = 14u32
                            ),
                        )
                        .await;
                }
                15u32 => {
                    let param_touch_id = gluon::Convertable::read(&mut gluon_data)?;
                    let __wire_param_position: stardust_xr_protocol::types::proxied::Vec2F = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_timestamp = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelItem", method = "touch_move", ? param_touch_id,
                        param_position = ? __wire_param_position, ? param_timestamp,
                        "dispatching"
                    );
                    let param_position: stardust_xr_protocol::types::proxies::Vec2F = {
                        let __w = __wire_param_position;
                        __w.into()
                    };
                    drop(gluon_data);
                    self.touch_move(ctx, param_touch_id, param_position, param_timestamp)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelItem", method =
                                "touch_move", method_id = 15u32
                            ),
                        )
                        .await;
                }
                16u32 => {
                    let param_touch_id = gluon::Convertable::read(&mut gluon_data)?;
                    let param_timestamp = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelItem", method = "touch_up", ? param_touch_id, ?
                        param_timestamp, "dispatching"
                    );
                    drop(gluon_data);
                    self.touch_up(ctx, param_touch_id, param_timestamp)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelItem", method = "touch_up",
                                method_id = 16u32
                            ),
                        )
                        .await;
                }
                17u32 => {
                    tracing::trace!(
                        interface = "PanelItem", method = "close_toplevel", "dispatching"
                    );
                    drop(gluon_data);
                    self.close_toplevel(ctx)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelItem", method =
                                "close_toplevel", method_id = 17u32
                            ),
                        )
                        .await;
                }
                18u32 => {
                    tracing::trace!(
                        interface = "PanelItem", method =
                        "resize_toplevel_to_app_request", "dispatching"
                    );
                    drop(gluon_data);
                    self.resize_toplevel_to_app_request(ctx)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelItem", method =
                                "resize_toplevel_to_app_request", method_id = 18u32
                            ),
                        )
                        .await;
                }
                19u32 => {
                    let __wire_param_new_size: stardust_xr_protocol::types::proxied::Size2 = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "PanelItem", method = "request_toplevel_resize",
                        param_new_size = ? __wire_param_new_size, "dispatching"
                    );
                    let param_new_size: stardust_xr_protocol::types::proxies::Size2 = {
                        let __w = __wire_param_new_size;
                        __w.into()
                    };
                    drop(gluon_data);
                    self.request_toplevel_resize(ctx, param_new_size)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelItem", method =
                                "request_toplevel_resize", method_id = 19u32
                            ),
                        )
                        .await;
                }
                20u32 => {
                    let param_focused = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelItem", method = "toplevel_focused", ?
                        param_focused, "dispatching"
                    );
                    drop(gluon_data);
                    self.toplevel_focused(ctx, param_focused)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelItem", method =
                                "toplevel_focused", method_id = 20u32
                            ),
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<(gluon::Node<Self>, gluon::LocalRef<PanelItem, Self>), gluon::NodeError>
    where
        Self: Sized,
    {
        use gluon::RefExt;
        PanelItem::new_node(self)
    }
    fn to_service(self) -> Result<gluon::LocalRef<PanelItem, Self>, gluon::NodeError>
    where
        Self: Sized,
    {
        use gluon::RefExt;
        PanelItem::new_service(self)
    }
}
#[derive(Debug, Clone)]
pub struct PanelShell {
    obj: gluon::Ref,
}
impl gluon::Convertable for PanelShell {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(PanelShell::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PanelShell {
    const ID: &'static str = "org.stardustxr.item.Panel.PanelShell";
}
impl gluon::Interface for PanelShell {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: PanelShellHandler> gluon::HandledBy<H> for PanelShell {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type PanelShellLocal<H> = gluon::LocalRef<PanelShell, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: PanelShellHandler> From<PanelShellLocal<H>> for PanelShell {
    fn from(value: PanelShellLocal<H>) -> PanelShell {
        value.into_proxy()
    }
}
impl gluon::RefExt for PanelShell {
    fn from_ref(obj: gluon::Ref) -> PanelShell {
        PanelShell { obj }
    }
}
impl PanelShell {
    pub fn update_surface_dmatex(
        &self,
        surface: impl Into<SurfaceUpdateTarget>,
        dmatex: impl Into<stardust_xr_protocol::dmatex::DmatexRef>,
        acquire_point: impl Into<u64>,
        release_point: impl Into<stardust_xr_protocol::dmatex::DmatexSubmitRelease>,
        opaque: impl Into<bool>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceUpdateTarget = surface.into();
        let dmatex: stardust_xr_protocol::dmatex::DmatexRef = dmatex.into();
        let acquire_point: u64 = acquire_point.into();
        let release_point: stardust_xr_protocol::dmatex::DmatexSubmitRelease = release_point
            .into();
        let opaque: bool = opaque.into();
        tracing::trace!(
            interface = "PanelShell", method = "update_surface_dmatex", ? surface, ?
            dmatex, ? acquire_point, ? release_point, ? opaque, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        dmatex.write(&mut gluon_builder)?;
        acquire_point.write(&mut gluon_builder)?;
        release_point.write(&mut gluon_builder)?;
        opaque.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        Ok(())
    }
    pub fn toplevel_resized(
        &self,
        new_size: stardust_xr_protocol::types::proxies::Size2,
    ) -> Result<(), gluon::SendError> {
        let new_size: stardust_xr_protocol::types::proxied::Size2 = new_size.into();
        tracing::trace!(
            interface = "PanelShell", method = "toplevel_resized", ? new_size, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        new_size.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 9u32, gluon_builder)?;
        Ok(())
    }
    pub fn toplevel_max_size(
        &self,
        max_size: Option<stardust_xr_protocol::types::proxies::Size2>,
    ) -> Result<(), gluon::SendError> {
        let max_size: Option<stardust_xr_protocol::types::proxied::Size2> = max_size
            .map(|__v| __v.into());
        tracing::trace!(
            interface = "PanelShell", method = "toplevel_max_size", ? max_size, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        max_size.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 10u32, gluon_builder)?;
        Ok(())
    }
    pub fn toplevel_min_size(
        &self,
        min_size: Option<stardust_xr_protocol::types::proxies::Size2>,
    ) -> Result<(), gluon::SendError> {
        let min_size: Option<stardust_xr_protocol::types::proxied::Size2> = min_size
            .map(|__v| __v.into());
        tracing::trace!(
            interface = "PanelShell", method = "toplevel_min_size", ? min_size, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        min_size.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 11u32, gluon_builder)?;
        Ok(())
    }
    pub fn toplevel_fullscreen(
        &self,
        fullscreen_active: impl Into<bool>,
    ) -> Result<(), gluon::SendError> {
        let fullscreen_active: bool = fullscreen_active.into();
        tracing::trace!(
            interface = "PanelShell", method = "toplevel_fullscreen", ?
            fullscreen_active, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        fullscreen_active.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 12u32, gluon_builder)?;
        Ok(())
    }
    pub fn toplevel_title(
        &self,
        title: impl Into<String>,
    ) -> Result<(), gluon::SendError> {
        let title: String = title.into();
        tracing::trace!(
            interface = "PanelShell", method = "toplevel_title", ? title, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        title.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 13u32, gluon_builder)?;
        Ok(())
    }
    pub fn toplevel_app_id(
        &self,
        app_id: impl Into<String>,
    ) -> Result<(), gluon::SendError> {
        let app_id: String = app_id.into();
        tracing::trace!(
            interface = "PanelShell", method = "toplevel_app_id", ? app_id, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        app_id.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 14u32, gluon_builder)?;
        Ok(())
    }
    pub fn set_cursor_visuals(
        &self,
        geometry: impl Into<Option<Geometry>>,
    ) -> Result<(), gluon::SendError> {
        let geometry: Option<Geometry> = geometry.into();
        tracing::trace!(
            interface = "PanelShell", method = "set_cursor_visuals", ? geometry, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        geometry.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 15u32, gluon_builder)?;
        Ok(())
    }
    pub fn create_child(
        &self,
        child: impl Into<ChildState>,
    ) -> Result<(), gluon::SendError> {
        let child: ChildState = child.into();
        tracing::trace!(
            interface = "PanelShell", method = "create_child", ? child, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        child.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 16u32, gluon_builder)?;
        Ok(())
    }
    pub fn move_child(
        &self,
        child_id: impl Into<u64>,
        geometry: impl Into<Geometry>,
    ) -> Result<(), gluon::SendError> {
        let child_id: u64 = child_id.into();
        let geometry: Geometry = geometry.into();
        tracing::trace!(
            interface = "PanelShell", method = "move_child", ? child_id, ? geometry,
            "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        child_id.write(&mut gluon_builder)?;
        geometry.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 17u32, gluon_builder)?;
        Ok(())
    }
    pub fn destroy_child(
        &self,
        child_id: impl Into<u64>,
    ) -> Result<(), gluon::SendError> {
        let child_id: u64 = child_id.into();
        tracing::trace!(
            interface = "PanelShell", method = "destroy_child", ? child_id, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        child_id.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 18u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> PanelShell {
        PanelShell { obj }
    }
}
impl From<PanelShell> for gluon::Ref {
    fn from(value: PanelShell) -> Self {
        value.obj
    }
}
impl gluon::ToRef for PanelShell {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for PanelShell {
    fn death_notifier(&self) -> gluon::DeathNotifier {
        gluon::Liveness::death_notifier(&self.obj)
    }
}
impl std::hash::Hash for PanelShell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for PanelShell {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for PanelShell {}
pub trait PanelShellHandler: gluon::Handler + Send + Sync + 'static {
    fn update_surface_dmatex(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceUpdateTarget,
        dmatex: stardust_xr_protocol::dmatex::DmatexRef,
        acquire_point: u64,
        release_point: stardust_xr_protocol::dmatex::DmatexSubmitRelease,
        opaque: bool,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_resized(
        &self,
        _ctx: gluon::Context,
        new_size: stardust_xr_protocol::types::proxies::Size2,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_max_size(
        &self,
        _ctx: gluon::Context,
        max_size: Option<stardust_xr_protocol::types::proxies::Size2>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_min_size(
        &self,
        _ctx: gluon::Context,
        min_size: Option<stardust_xr_protocol::types::proxies::Size2>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_fullscreen(
        &self,
        _ctx: gluon::Context,
        fullscreen_active: bool,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_title(
        &self,
        _ctx: gluon::Context,
        title: String,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_app_id(
        &self,
        _ctx: gluon::Context,
        app_id: String,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn set_cursor_visuals(
        &self,
        _ctx: gluon::Context,
        geometry: Option<Geometry>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn create_child(
        &self,
        _ctx: gluon::Context,
        child: ChildState,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn move_child(
        &self,
        _ctx: gluon::Context,
        child_id: u64,
        geometry: Geometry,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn destroy_child(
        &self,
        _ctx: gluon::Context,
        child_id: u64,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon::DataReader,
        ctx: gluon::Context,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let param_dmatex = gluon::Convertable::read(&mut gluon_data)?;
                    let param_acquire_point = gluon::Convertable::read(&mut gluon_data)?;
                    let param_release_point = gluon::Convertable::read(&mut gluon_data)?;
                    let param_opaque = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelShell", method = "update_surface_dmatex", ?
                        param_surface, ? param_dmatex, ? param_acquire_point, ?
                        param_release_point, ? param_opaque, "dispatching"
                    );
                    drop(gluon_data);
                    self.update_surface_dmatex(
                            ctx,
                            param_surface,
                            param_dmatex,
                            param_acquire_point,
                            param_release_point,
                            param_opaque,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelShell", method =
                                "update_surface_dmatex", method_id = 8u32
                            ),
                        )
                        .await;
                }
                9u32 => {
                    let __wire_param_new_size: stardust_xr_protocol::types::proxied::Size2 = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "PanelShell", method = "toplevel_resized",
                        param_new_size = ? __wire_param_new_size, "dispatching"
                    );
                    let param_new_size: stardust_xr_protocol::types::proxies::Size2 = {
                        let __w = __wire_param_new_size;
                        __w.into()
                    };
                    drop(gluon_data);
                    self.toplevel_resized(ctx, param_new_size)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelShell", method =
                                "toplevel_resized", method_id = 9u32
                            ),
                        )
                        .await;
                }
                10u32 => {
                    let __wire_param_max_size: Option<
                        stardust_xr_protocol::types::proxied::Size2,
                    > = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelShell", method = "toplevel_max_size",
                        param_max_size = ? __wire_param_max_size, "dispatching"
                    );
                    let param_max_size: Option<
                        stardust_xr_protocol::types::proxies::Size2,
                    > = {
                        let __w = __wire_param_max_size;
                        __w.map(|__v| __v.into())
                    };
                    drop(gluon_data);
                    self.toplevel_max_size(ctx, param_max_size)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelShell", method =
                                "toplevel_max_size", method_id = 10u32
                            ),
                        )
                        .await;
                }
                11u32 => {
                    let __wire_param_min_size: Option<
                        stardust_xr_protocol::types::proxied::Size2,
                    > = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelShell", method = "toplevel_min_size",
                        param_min_size = ? __wire_param_min_size, "dispatching"
                    );
                    let param_min_size: Option<
                        stardust_xr_protocol::types::proxies::Size2,
                    > = {
                        let __w = __wire_param_min_size;
                        __w.map(|__v| __v.into())
                    };
                    drop(gluon_data);
                    self.toplevel_min_size(ctx, param_min_size)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelShell", method =
                                "toplevel_min_size", method_id = 11u32
                            ),
                        )
                        .await;
                }
                12u32 => {
                    let param_fullscreen_active = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "PanelShell", method = "toplevel_fullscreen", ?
                        param_fullscreen_active, "dispatching"
                    );
                    drop(gluon_data);
                    self.toplevel_fullscreen(ctx, param_fullscreen_active)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelShell", method =
                                "toplevel_fullscreen", method_id = 12u32
                            ),
                        )
                        .await;
                }
                13u32 => {
                    let param_title = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelShell", method = "toplevel_title", ?
                        param_title, "dispatching"
                    );
                    drop(gluon_data);
                    self.toplevel_title(ctx, param_title)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelShell", method =
                                "toplevel_title", method_id = 13u32
                            ),
                        )
                        .await;
                }
                14u32 => {
                    let param_app_id = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelShell", method = "toplevel_app_id", ?
                        param_app_id, "dispatching"
                    );
                    drop(gluon_data);
                    self.toplevel_app_id(ctx, param_app_id)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelShell", method =
                                "toplevel_app_id", method_id = 14u32
                            ),
                        )
                        .await;
                }
                15u32 => {
                    let param_geometry = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelShell", method = "set_cursor_visuals", ?
                        param_geometry, "dispatching"
                    );
                    drop(gluon_data);
                    self.set_cursor_visuals(ctx, param_geometry)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelShell", method =
                                "set_cursor_visuals", method_id = 15u32
                            ),
                        )
                        .await;
                }
                16u32 => {
                    let param_child = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelShell", method = "create_child", ? param_child,
                        "dispatching"
                    );
                    drop(gluon_data);
                    self.create_child(ctx, param_child)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelShell", method =
                                "create_child", method_id = 16u32
                            ),
                        )
                        .await;
                }
                17u32 => {
                    let param_child_id = gluon::Convertable::read(&mut gluon_data)?;
                    let param_geometry = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelShell", method = "move_child", ?
                        param_child_id, ? param_geometry, "dispatching"
                    );
                    drop(gluon_data);
                    self.move_child(ctx, param_child_id, param_geometry)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelShell", method =
                                "move_child", method_id = 17u32
                            ),
                        )
                        .await;
                }
                18u32 => {
                    let param_child_id = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PanelShell", method = "destroy_child", ?
                        param_child_id, "dispatching"
                    );
                    drop(gluon_data);
                    self.destroy_child(ctx, param_child_id)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PanelShell", method =
                                "destroy_child", method_id = 18u32
                            ),
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<(gluon::Node<Self>, gluon::LocalRef<PanelShell, Self>), gluon::NodeError>
    where
        Self: Sized,
    {
        use gluon::RefExt;
        PanelShell::new_node(self)
    }
    fn to_service(self) -> Result<gluon::LocalRef<PanelShell, Self>, gluon::NodeError>
    where
        Self: Sized,
    {
        use gluon::RefExt;
        PanelShell::new_service(self)
    }
}
pub mod proxied {
    use super::*;
}
