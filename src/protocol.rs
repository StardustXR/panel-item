#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.item.Panel",
    types: &[
        gluon::ExternalGluonType {
            name: "KeymapId",
            supported_derives: gluon::Derives::from_bits_truncate(127u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "SpatialRefId",
            supported_derives: gluon::Derives::from_bits_truncate(127u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "FieldRefId",
            supported_derives: gluon::Derives::from_bits_truncate(127u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "ToplevelState",
            supported_derives: gluon::Derives::from_bits_truncate(126u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "ChildState",
            supported_derives: gluon::Derives::from_bits_truncate(42u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "PanelItemInitData",
            supported_derives: gluon::Derives::from_bits_truncate(42u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Rect",
            supported_derives: gluon::Derives::from_bits_truncate(43u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Geometry",
            supported_derives: gluon::Derives::from_bits_truncate(127u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Vec2",
            supported_derives: gluon::Derives::from_bits_truncate(43u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "UVec2",
            supported_derives: gluon::Derives::from_bits_truncate(127u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "IVec2",
            supported_derives: gluon::Derives::from_bits_truncate(127u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "ScrollSource",
            supported_derives: gluon::Derives::from_bits_truncate(127u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "SurfaceId",
            supported_derives: gluon::Derives::from_bits_truncate(127u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "SurfaceUpdateTarget",
            supported_derives: gluon::Derives::from_bits_truncate(127u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///KeymapId
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeymapId {
    pub id: u64,
}
impl gluon::Convertable for KeymapId {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.id.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let id = gluon::Convertable::read(gluon_data)?;
        Ok(KeymapId { id })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.id.write_owned(gluon_data)?;
        Ok(())
    }
}
///SpatialRef
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpatialRefId {
    pub id: u64,
}
impl gluon::Convertable for SpatialRefId {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.id.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let id = gluon::Convertable::read(gluon_data)?;
        Ok(SpatialRefId { id })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.id.write_owned(gluon_data)?;
        Ok(())
    }
}
///FieldRef
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldRefId {
    pub id: u64,
}
impl gluon::Convertable for FieldRefId {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.id.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let id = gluon::Convertable::read(gluon_data)?;
        Ok(FieldRefId { id })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.id.write_owned(gluon_data)?;
        Ok(())
    }
}
///ToplevelState
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ToplevelState {
    pub parent: Option<u64>,
    pub title: Option<String>,
    pub app_id: Option<String>,
    pub size: UVec2,
    pub min_size: Option<UVec2>,
    pub max_size: Option<UVec2>,
}
impl gluon::Convertable for ToplevelState {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.parent.write(gluon_data)?;
        self.title.write(gluon_data)?;
        self.app_id.write(gluon_data)?;
        self.size.write(gluon_data)?;
        self.min_size.write(gluon_data)?;
        self.max_size.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let parent = gluon::Convertable::read(gluon_data)?;
        let title = gluon::Convertable::read(gluon_data)?;
        let app_id = gluon::Convertable::read(gluon_data)?;
        let size = gluon::Convertable::read(gluon_data)?;
        let min_size = gluon::Convertable::read(gluon_data)?;
        let max_size = gluon::Convertable::read(gluon_data)?;
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
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.parent.write_owned(gluon_data)?;
        self.title.write_owned(gluon_data)?;
        self.app_id.write_owned(gluon_data)?;
        self.size.write_owned(gluon_data)?;
        self.min_size.write_owned(gluon_data)?;
        self.max_size.write_owned(gluon_data)?;
        Ok(())
    }
}
///ChildState
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ChildState {
    pub id: u64,
    pub parent: SurfaceId,
    pub geometry: Geometry,
    pub z_order: i32,
    pub input_regions: Vec<Rect>,
}
impl gluon::Convertable for ChildState {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
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
        gluon_data: &mut gluon::DataBuilder<'_>,
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
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PanelItemInitData {
    pub cursor: Option<Geometry>,
    pub toplevel: ToplevelState,
    pub children: Vec<ChildState>,
}
impl gluon::Convertable for PanelItemInitData {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
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
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.cursor.write_owned(gluon_data)?;
        self.toplevel.write_owned(gluon_data)?;
        self.children.write_owned(gluon_data)?;
        Ok(())
    }
}
///Rect
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Rect {
    pub origin: Vec2,
    pub size: Vec2,
}
impl gluon::Convertable for Rect {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.origin.write(gluon_data)?;
        self.size.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let origin = gluon::Convertable::read(gluon_data)?;
        let size = gluon::Convertable::read(gluon_data)?;
        Ok(Rect { origin, size })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.origin.write_owned(gluon_data)?;
        self.size.write_owned(gluon_data)?;
        Ok(())
    }
}
///Geometry
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Geometry {
    pub origin: IVec2,
    pub size: UVec2,
}
impl gluon::Convertable for Geometry {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.origin.write(gluon_data)?;
        self.size.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let origin = gluon::Convertable::read(gluon_data)?;
        let size = gluon::Convertable::read(gluon_data)?;
        Ok(Geometry { origin, size })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.origin.write_owned(gluon_data)?;
        self.size.write_owned(gluon_data)?;
        Ok(())
    }
}
///Vec2
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl gluon::Convertable for Vec2 {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.x.write(gluon_data)?;
        self.y.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let x = gluon::Convertable::read(gluon_data)?;
        let y = gluon::Convertable::read(gluon_data)?;
        Ok(Vec2 { x, y })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.x.write_owned(gluon_data)?;
        self.y.write_owned(gluon_data)?;
        Ok(())
    }
}
///UVec2
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}
impl gluon::Convertable for UVec2 {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.x.write(gluon_data)?;
        self.y.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let x = gluon::Convertable::read(gluon_data)?;
        let y = gluon::Convertable::read(gluon_data)?;
        Ok(UVec2 { x, y })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.x.write_owned(gluon_data)?;
        self.y.write_owned(gluon_data)?;
        Ok(())
    }
}
///IVec2
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}
impl gluon::Convertable for IVec2 {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.x.write(gluon_data)?;
        self.y.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let x = gluon::Convertable::read(gluon_data)?;
        let y = gluon::Convertable::read(gluon_data)?;
        Ok(IVec2 { x, y })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.x.write_owned(gluon_data)?;
        self.y.write_owned(gluon_data)?;
        Ok(())
    }
}
///ScrollSource
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScrollSource {
    Wheel,
    Touch,
    Continuous,
    WheelTilt,
}
impl gluon::Convertable for ScrollSource {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
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
        gluon_data: &mut gluon::DataBuilder<'_>,
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
pub enum SurfaceId {
    Toplevel,
    Child { id: u64 },
}
impl gluon::Convertable for SurfaceId {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
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
        gluon_data: &mut gluon::DataBuilder<'_>,
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
pub enum SurfaceUpdateTarget {
    Toplevel,
    Child { id: u64 },
    Cursor,
}
impl gluon::Convertable for SurfaceUpdateTarget {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
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
        gluon_data: &mut gluon::DataBuilder<'_>,
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
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for PanelItem {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(PanelItem::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PanelItem {
    pub async fn register_xkb_keymap(
        &self,
        xkb_keymap: impl Into<String>,
    ) -> Result<KeymapId, gluon::SendError> {
        let xkb_keymap: String = xkb_keymap.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        xkb_keymap.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub fn absolute_pointer_motion(
        &self,
        surface: impl Into<SurfaceId>,
        position: impl Into<Vec2>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let position: Vec2 = position.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        position.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn relative_pointer_motion(
        &self,
        surface: impl Into<SurfaceId>,
        delta: impl Into<Vec2>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let delta: Vec2 = delta.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        delta.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn pointer_button(
        &self,
        surface: impl Into<SurfaceId>,
        button: impl Into<u32>,
        pressed: impl Into<bool>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let button: u32 = button.into();
        let pressed: bool = pressed.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        button.write(&mut gluon_builder)?;
        pressed.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn pointer_scroll_pixels(
        &self,
        surface: impl Into<SurfaceId>,
        delta: impl Into<Vec2>,
        source: impl Into<ScrollSource>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let delta: Vec2 = delta.into();
        let source: ScrollSource = source.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        delta.write(&mut gluon_builder)?;
        source.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 12u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn pointer_scroll_discrete(
        &self,
        surface: impl Into<SurfaceId>,
        delta: impl Into<Vec2>,
        source: impl Into<ScrollSource>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let delta: Vec2 = delta.into();
        let source: ScrollSource = source.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        delta.write(&mut gluon_builder)?;
        source.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 13u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn pointer_scroll_stop(
        &self,
        surface: impl Into<SurfaceId>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 14u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn key(
        &self,
        surface: impl Into<SurfaceId>,
        keymap: impl Into<KeymapId>,
        key: impl Into<u32>,
        pressed: impl Into<bool>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let keymap: KeymapId = keymap.into();
        let key: u32 = key.into();
        let pressed: bool = pressed.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        keymap.write(&mut gluon_builder)?;
        key.write(&mut gluon_builder)?;
        pressed.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 15u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn touch_down(
        &self,
        surface: impl Into<SurfaceId>,
        touch_id: impl Into<u32>,
        position: impl Into<Vec2>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceId = surface.into();
        let touch_id: u32 = touch_id.into();
        let position: Vec2 = position.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        touch_id.write(&mut gluon_builder)?;
        position.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 16u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn touch_move(
        &self,
        touch_id: impl Into<u32>,
        position: impl Into<Vec2>,
    ) -> Result<(), gluon::SendError> {
        let touch_id: u32 = touch_id.into();
        let position: Vec2 = position.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        touch_id.write(&mut gluon_builder)?;
        position.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 17u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn touch_up(&self, touch_id: impl Into<u32>) -> Result<(), gluon::SendError> {
        let touch_id: u32 = touch_id.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        touch_id.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 18u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn close_toplevel(&self) -> Result<(), gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        self.obj
            .device()
            .transact_one_way(&self.obj, 19u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn resize_toplevel_to_app_request(&self) -> Result<(), gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        self.obj
            .device()
            .transact_one_way(&self.obj, 20u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn request_toplevel_resize(
        &self,
        new_size: impl Into<UVec2>,
    ) -> Result<(), gluon::SendError> {
        let new_size: UVec2 = new_size.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        new_size.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 21u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_focused(
        &self,
        focused: impl Into<bool>,
    ) -> Result<(), gluon::SendError> {
        let focused: bool = focused.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        focused.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 22u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: PanelItemHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> PanelItem {
        PanelItem::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> PanelItem {
        PanelItem { obj }
    }
}
impl From<PanelItem> for gluon::ObjectOrRef {
    fn from(value: PanelItem) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for PanelItem {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
    fn register_xkb_keymap(
        &self,
        _ctx: gluon::Context,
        xkb_keymap: String,
    ) -> impl Future<Output = KeymapId> + Send + Sync;
    fn absolute_pointer_motion(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
        position: Vec2,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn relative_pointer_motion(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
        delta: Vec2,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn pointer_button(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
        button: u32,
        pressed: bool,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn pointer_scroll_pixels(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
        delta: Vec2,
        source: ScrollSource,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn pointer_scroll_discrete(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
        delta: Vec2,
        source: ScrollSource,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn pointer_scroll_stop(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn key(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
        keymap: KeymapId,
        key: u32,
        pressed: bool,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn touch_down(
        &self,
        _ctx: gluon::Context,
        surface: SurfaceId,
        touch_id: u32,
        position: Vec2,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn touch_move(
        &self,
        _ctx: gluon::Context,
        touch_id: u32,
        position: Vec2,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn touch_up(
        &self,
        _ctx: gluon::Context,
        touch_id: u32,
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
        new_size: UVec2,
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
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_xkb_keymap = gluon::Convertable::read(&mut gluon_data)?;
                    let (id) = self.register_xkb_keymap(ctx, param_xkb_keymap).await;
                    drop(gluon_data);
                    id.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let param_position = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.absolute_pointer_motion(ctx, param_surface, param_position)
                        .await;
                }
                10u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let param_delta = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.relative_pointer_motion(ctx, param_surface, param_delta).await;
                }
                11u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let param_button = gluon::Convertable::read(&mut gluon_data)?;
                    let param_pressed = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.pointer_button(ctx, param_surface, param_button, param_pressed)
                        .await;
                }
                12u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let param_delta = gluon::Convertable::read(&mut gluon_data)?;
                    let param_source = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.pointer_scroll_pixels(
                            ctx,
                            param_surface,
                            param_delta,
                            param_source,
                        )
                        .await;
                }
                13u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let param_delta = gluon::Convertable::read(&mut gluon_data)?;
                    let param_source = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.pointer_scroll_discrete(
                            ctx,
                            param_surface,
                            param_delta,
                            param_source,
                        )
                        .await;
                }
                14u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.pointer_scroll_stop(ctx, param_surface).await;
                }
                15u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let param_keymap = gluon::Convertable::read(&mut gluon_data)?;
                    let param_key = gluon::Convertable::read(&mut gluon_data)?;
                    let param_pressed = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.key(ctx, param_surface, param_keymap, param_key, param_pressed)
                        .await;
                }
                16u32 => {
                    let param_surface = gluon::Convertable::read(&mut gluon_data)?;
                    let param_touch_id = gluon::Convertable::read(&mut gluon_data)?;
                    let param_position = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.touch_down(ctx, param_surface, param_touch_id, param_position)
                        .await;
                }
                17u32 => {
                    let param_touch_id = gluon::Convertable::read(&mut gluon_data)?;
                    let param_position = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.touch_move(ctx, param_touch_id, param_position).await;
                }
                18u32 => {
                    let param_touch_id = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.touch_up(ctx, param_touch_id).await;
                }
                19u32 => {
                    drop(gluon_data);
                    self.close_toplevel(ctx).await;
                }
                20u32 => {
                    drop(gluon_data);
                    self.resize_toplevel_to_app_request(ctx).await;
                }
                21u32 => {
                    let param_new_size = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.request_toplevel_resize(ctx, param_new_size).await;
                }
                22u32 => {
                    let param_focused = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.toplevel_focused(ctx, param_focused).await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct PanelShell {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for PanelShell {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(PanelShell::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PanelShell {
    pub fn update_surface_dmatex(
        &self,
        surface: impl Into<SurfaceUpdateTarget>,
        dmatex_uid: impl Into<u64>,
        acquire_point: impl Into<u64>,
        release_point: impl Into<u64>,
        opaque: impl Into<bool>,
    ) -> Result<(), gluon::SendError> {
        let surface: SurfaceUpdateTarget = surface.into();
        let dmatex_uid: u64 = dmatex_uid.into();
        let acquire_point: u64 = acquire_point.into();
        let release_point: u64 = release_point.into();
        let opaque: bool = opaque.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        surface.write(&mut gluon_builder)?;
        dmatex_uid.write(&mut gluon_builder)?;
        acquire_point.write(&mut gluon_builder)?;
        release_point.write(&mut gluon_builder)?;
        opaque.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_resized(
        &self,
        new_size: impl Into<UVec2>,
    ) -> Result<(), gluon::SendError> {
        let new_size: UVec2 = new_size.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        new_size.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_max_size(
        &self,
        max_size: impl Into<Option<UVec2>>,
    ) -> Result<(), gluon::SendError> {
        let max_size: Option<UVec2> = max_size.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        max_size.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_min_size(
        &self,
        min_size: impl Into<Option<UVec2>>,
    ) -> Result<(), gluon::SendError> {
        let min_size: Option<UVec2> = min_size.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        min_size.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_fullscreen(
        &self,
        fullscreen_active: impl Into<bool>,
    ) -> Result<(), gluon::SendError> {
        let fullscreen_active: bool = fullscreen_active.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        fullscreen_active.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 12u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_title(
        &self,
        title: impl Into<String>,
    ) -> Result<(), gluon::SendError> {
        let title: String = title.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        title.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 13u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_app_id(
        &self,
        app_id: impl Into<String>,
    ) -> Result<(), gluon::SendError> {
        let app_id: String = app_id.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        app_id.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 14u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn set_cursor_visuals(
        &self,
        geometry: impl Into<Option<Geometry>>,
    ) -> Result<(), gluon::SendError> {
        let geometry: Option<Geometry> = geometry.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        geometry.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 15u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn create_child(
        &self,
        child: impl Into<ChildState>,
    ) -> Result<(), gluon::SendError> {
        let child: ChildState = child.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        child.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 16u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn move_child(
        &self,
        child_id: impl Into<u64>,
        geometry: impl Into<Geometry>,
    ) -> Result<(), gluon::SendError> {
        let child_id: u64 = child_id.into();
        let geometry: Geometry = geometry.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        child_id.write(&mut gluon_builder)?;
        geometry.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 17u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn destroy_child(
        &self,
        child_id: impl Into<u64>,
    ) -> Result<(), gluon::SendError> {
        let child_id: u64 = child_id.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        child_id.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 18u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: PanelShellHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> PanelShell {
        PanelShell::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> PanelShell {
        PanelShell { obj }
    }
}
impl From<PanelShell> for gluon::ObjectOrRef {
    fn from(value: PanelShell) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for PanelShell {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
        dmatex_uid: u64,
        acquire_point: u64,
        release_point: u64,
        opaque: bool,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_resized(
        &self,
        _ctx: gluon::Context,
        new_size: UVec2,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_max_size(
        &self,
        _ctx: gluon::Context,
        max_size: Option<UVec2>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_min_size(
        &self,
        _ctx: gluon::Context,
        min_size: Option<UVec2>,
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
                    let param_dmatex_uid = gluon::Convertable::read(&mut gluon_data)?;
                    let param_acquire_point = gluon::Convertable::read(&mut gluon_data)?;
                    let param_release_point = gluon::Convertable::read(&mut gluon_data)?;
                    let param_opaque = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.update_surface_dmatex(
                            ctx,
                            param_surface,
                            param_dmatex_uid,
                            param_acquire_point,
                            param_release_point,
                            param_opaque,
                        )
                        .await;
                }
                9u32 => {
                    let param_new_size = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.toplevel_resized(ctx, param_new_size).await;
                }
                10u32 => {
                    let param_max_size = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.toplevel_max_size(ctx, param_max_size).await;
                }
                11u32 => {
                    let param_min_size = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.toplevel_min_size(ctx, param_min_size).await;
                }
                12u32 => {
                    let param_fullscreen_active = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    drop(gluon_data);
                    self.toplevel_fullscreen(ctx, param_fullscreen_active).await;
                }
                13u32 => {
                    let param_title = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.toplevel_title(ctx, param_title).await;
                }
                14u32 => {
                    let param_app_id = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.toplevel_app_id(ctx, param_app_id).await;
                }
                15u32 => {
                    let param_geometry = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.set_cursor_visuals(ctx, param_geometry).await;
                }
                16u32 => {
                    let param_child = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.create_child(ctx, param_child).await;
                }
                17u32 => {
                    let param_child_id = gluon::Convertable::read(&mut gluon_data)?;
                    let param_geometry = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.move_child(ctx, param_child_id, param_geometry).await;
                }
                18u32 => {
                    let param_child_id = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.destroy_child(ctx, param_child_id).await;
                }
                _ => {}
            }
            Ok(())
        }
    }
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
impl PanelItemAcceptor {
    pub async fn accept(
        &self,
        item: impl Into<PanelItem>,
    ) -> Result<(PanelShell, SpatialRefId), gluon::SendError> {
        let item: PanelItem = item.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        item.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok((
            gluon::Convertable::read(&mut reader)?,
            gluon::Convertable::read(&mut reader)?,
        ))
    }
    pub async fn get_field(&self) -> Result<FieldRefId, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
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
        item: PanelItem,
    ) -> impl Future<Output = (PanelShell, SpatialRefId)> + Send + Sync;
    fn get_field(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = FieldRefId> + Send + Sync;
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
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_item = gluon::Convertable::read(&mut gluon_data)?;
                    let (shell, output_spatial) = self.accept(ctx, param_item).await;
                    drop(gluon_data);
                    shell.write_owned(&mut gluon_out)?;
                    output_spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (field) = self.get_field(ctx).await;
                    drop(gluon_data);
                    field.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct PanelItemProvider {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for PanelItemProvider {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(PanelItemProvider::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PanelItemProvider {
    pub fn register_acceptor(
        &self,
        acceptor: impl Into<PanelItemAcceptor>,
    ) -> Result<(), gluon::SendError> {
        let acceptor: PanelItemAcceptor = acceptor.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        acceptor.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn drop_acceptor(
        &self,
        acceptor: impl Into<PanelItemAcceptor>,
    ) -> Result<(), gluon::SendError> {
        let acceptor: PanelItemAcceptor = acceptor.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        acceptor.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub async fn startup_token_spatial_ref(
        &self,
        token: impl Into<String>,
        spatial_ref: impl Into<SpatialRefId>,
    ) -> Result<(), gluon::SendError> {
        let token: String = token.into();
        let spatial_ref: SpatialRefId = spatial_ref.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        token.write(&mut gluon_builder)?;
        spatial_ref.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(())
    }
    pub fn from_handler<H: PanelItemProviderHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> PanelItemProvider {
        PanelItemProvider::from_object_or_ref(
            gluon::OwnedObjectRef::to_object_or_ref(obj),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> PanelItemProvider {
        PanelItemProvider { obj }
    }
}
impl From<PanelItemProvider> for gluon::ObjectOrRef {
    fn from(value: PanelItemProvider) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for PanelItemProvider {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl std::hash::Hash for PanelItemProvider {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for PanelItemProvider {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for PanelItemProvider {}
pub trait PanelItemProviderHandler: gluon::Handler + Send + Sync + 'static {
    fn register_acceptor(
        &self,
        _ctx: gluon::Context,
        acceptor: PanelItemAcceptor,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn drop_acceptor(
        &self,
        _ctx: gluon::Context,
        acceptor: PanelItemAcceptor,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn startup_token_spatial_ref(
        &self,
        _ctx: gluon::Context,
        token: String,
        spatial_ref: SpatialRefId,
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
                    let param_acceptor = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.register_acceptor(ctx, param_acceptor).await;
                }
                9u32 => {
                    let param_acceptor = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.drop_acceptor(ctx, param_acceptor).await;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_token = gluon::Convertable::read(&mut gluon_data)?;
                    let param_spatial_ref = gluon::Convertable::read(&mut gluon_data)?;
                    let () = self
                        .startup_token_spatial_ref(ctx, param_token, param_spatial_ref)
                        .await;
                    drop(gluon_data);
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
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
