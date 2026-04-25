#![allow(
    unused,
    clippy::single_match,
    clippy::match_single_binding,
    clippy::large_enum_variant
)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.item.Panel",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "KeymapId",
            supported_derives: gluon_wire::Derives::from_bits_truncate(127u32),
        },
        gluon_wire::ExternalGluonType {
            name: "SpatialRefId",
            supported_derives: gluon_wire::Derives::from_bits_truncate(127u32),
        },
        gluon_wire::ExternalGluonType {
            name: "FieldRefId",
            supported_derives: gluon_wire::Derives::from_bits_truncate(127u32),
        },
        gluon_wire::ExternalGluonType {
            name: "ToplevelState",
            supported_derives: gluon_wire::Derives::from_bits_truncate(126u32),
        },
        gluon_wire::ExternalGluonType {
            name: "ChildState",
            supported_derives: gluon_wire::Derives::from_bits_truncate(42u32),
        },
        gluon_wire::ExternalGluonType {
            name: "PanelItemInitData",
            supported_derives: gluon_wire::Derives::from_bits_truncate(42u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Rect",
            supported_derives: gluon_wire::Derives::from_bits_truncate(43u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Geometry",
            supported_derives: gluon_wire::Derives::from_bits_truncate(127u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Vec2",
            supported_derives: gluon_wire::Derives::from_bits_truncate(43u32),
        },
        gluon_wire::ExternalGluonType {
            name: "UVec2",
            supported_derives: gluon_wire::Derives::from_bits_truncate(127u32),
        },
        gluon_wire::ExternalGluonType {
            name: "IVec2",
            supported_derives: gluon_wire::Derives::from_bits_truncate(127u32),
        },
        gluon_wire::ExternalGluonType {
            name: "ScrollSource",
            supported_derives: gluon_wire::Derives::from_bits_truncate(127u32),
        },
        gluon_wire::ExternalGluonType {
            name: "SurfaceId",
            supported_derives: gluon_wire::Derives::from_bits_truncate(127u32),
        },
        gluon_wire::ExternalGluonType {
            name: "SurfaceUpdateTarget",
            supported_derives: gluon_wire::Derives::from_bits_truncate(127u32),
        },
    ],
};
///KeymapId
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeymapId {
    pub id: u64,
}
impl gluon_wire::GluonConvertable for KeymapId {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.id.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let id = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(KeymapId { id })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.id.write_owned(gluon_data)?;
        Ok(())
    }
}
///SpatialRef
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpatialRefId {
    pub id: u64,
}
impl gluon_wire::GluonConvertable for SpatialRefId {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.id.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let id = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(SpatialRefId { id })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.id.write_owned(gluon_data)?;
        Ok(())
    }
}
///FieldRef
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldRefId {
    pub id: u64,
}
impl gluon_wire::GluonConvertable for FieldRefId {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.id.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let id = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(FieldRefId { id })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
impl gluon_wire::GluonConvertable for ToplevelState {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.parent.write(gluon_data)?;
        self.title.write(gluon_data)?;
        self.app_id.write(gluon_data)?;
        self.size.write(gluon_data)?;
        self.min_size.write(gluon_data)?;
        self.max_size.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let parent = gluon_wire::GluonConvertable::read(gluon_data)?;
        let title = gluon_wire::GluonConvertable::read(gluon_data)?;
        let app_id = gluon_wire::GluonConvertable::read(gluon_data)?;
        let size = gluon_wire::GluonConvertable::read(gluon_data)?;
        let min_size = gluon_wire::GluonConvertable::read(gluon_data)?;
        let max_size = gluon_wire::GluonConvertable::read(gluon_data)?;
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
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
impl gluon_wire::GluonConvertable for ChildState {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.id.write(gluon_data)?;
        self.parent.write(gluon_data)?;
        self.geometry.write(gluon_data)?;
        self.z_order.write(gluon_data)?;
        self.input_regions.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let id = gluon_wire::GluonConvertable::read(gluon_data)?;
        let parent = gluon_wire::GluonConvertable::read(gluon_data)?;
        let geometry = gluon_wire::GluonConvertable::read(gluon_data)?;
        let z_order = gluon_wire::GluonConvertable::read(gluon_data)?;
        let input_regions = gluon_wire::GluonConvertable::read(gluon_data)?;
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
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
impl gluon_wire::GluonConvertable for PanelItemInitData {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.cursor.write(gluon_data)?;
        self.toplevel.write(gluon_data)?;
        self.children.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let cursor = gluon_wire::GluonConvertable::read(gluon_data)?;
        let toplevel = gluon_wire::GluonConvertable::read(gluon_data)?;
        let children = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(PanelItemInitData {
            cursor,
            toplevel,
            children,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
impl gluon_wire::GluonConvertable for Rect {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.origin.write(gluon_data)?;
        self.size.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let origin = gluon_wire::GluonConvertable::read(gluon_data)?;
        let size = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Rect { origin, size })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
impl gluon_wire::GluonConvertable for Geometry {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.origin.write(gluon_data)?;
        self.size.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let origin = gluon_wire::GluonConvertable::read(gluon_data)?;
        let size = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Geometry { origin, size })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
impl gluon_wire::GluonConvertable for Vec2 {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(gluon_data)?;
        self.y.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(gluon_data)?;
        let y = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Vec2 { x, y })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
impl gluon_wire::GluonConvertable for UVec2 {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(gluon_data)?;
        self.y.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(gluon_data)?;
        let y = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(UVec2 { x, y })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
impl gluon_wire::GluonConvertable for IVec2 {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(gluon_data)?;
        self.y.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(gluon_data)?;
        let y = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(IVec2 { x, y })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
impl gluon_wire::GluonConvertable for ScrollSource {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        Ok(
            match gluon_data.read_u16()? {
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
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
impl gluon_wire::GluonConvertable for SurfaceId {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => SurfaceId::Toplevel,
                1u16 => {
                    let id = gluon_wire::GluonConvertable::read(gluon_data)?;
                    SurfaceId::Child { id }
                }
                v => return Err(gluon_wire::GluonReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
impl gluon_wire::GluonConvertable for SurfaceUpdateTarget {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => SurfaceUpdateTarget::Toplevel,
                1u16 => {
                    let id = gluon_wire::GluonConvertable::read(gluon_data)?;
                    SurfaceUpdateTarget::Child { id }
                }
                2u16 => SurfaceUpdateTarget::Cursor,
                v => return Err(gluon_wire::GluonReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
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
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for PanelItem {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(gluon_data)?;
        Ok(PanelItem::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PanelItem {
    pub async fn register_xkb_keymap(
        &self,
        xkb_keymap: String,
    ) -> Result<KeymapId, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        xkb_keymap.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn absolute_pointer_motion(
        &self,
        surface: SurfaceId,
        position: Vec2,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        surface.write(&mut gluon_builder)?;
        position.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn relative_pointer_motion(
        &self,
        surface: SurfaceId,
        delta: Vec2,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        surface.write(&mut gluon_builder)?;
        delta.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn pointer_button(
        &self,
        surface: SurfaceId,
        button: u32,
        pressed: bool,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
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
        surface: SurfaceId,
        delta: Vec2,
        source: ScrollSource,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
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
        surface: SurfaceId,
        delta: Vec2,
        source: ScrollSource,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
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
        surface: SurfaceId,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        surface.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 14u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn key(
        &self,
        surface: SurfaceId,
        keymap: KeymapId,
        key: u32,
        pressed: bool,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
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
        surface: SurfaceId,
        touch_id: u32,
        position: Vec2,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
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
        touch_id: u32,
        position: Vec2,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        touch_id.write(&mut gluon_builder)?;
        position.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 17u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn touch_up(&self, touch_id: u32) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        touch_id.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 18u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn close_toplevel(&self) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        self.obj
            .device()
            .transact_one_way(&self.obj, 19u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn resize_toplevel_to_app_request(
        &self,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        self.obj
            .device()
            .transact_one_way(&self.obj, 20u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn request_toplevel_resize(
        &self,
        new_size: UVec2,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        new_size.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 21u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_focused(
        &self,
        focused: bool,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        focused.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 22u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: PanelItemHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        PanelItem { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for PanelItem {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
    ) -> impl Future<Output = ()> + Send + Sync;
    fn relative_pointer_motion(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceId,
        delta: Vec2,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn pointer_button(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceId,
        button: u32,
        pressed: bool,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn pointer_scroll_pixels(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceId,
        delta: Vec2,
        source: ScrollSource,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn pointer_scroll_discrete(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceId,
        delta: Vec2,
        source: ScrollSource,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn pointer_scroll_stop(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceId,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn key(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceId,
        keymap: KeymapId,
        key: u32,
        pressed: bool,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn touch_down(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceId,
        touch_id: u32,
        position: Vec2,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn touch_move(
        &self,
        _ctx: gluon_wire::GluonCtx,
        touch_id: u32,
        position: Vec2,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn touch_up(
        &self,
        _ctx: gluon_wire::GluonCtx,
        touch_id: u32,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn close_toplevel(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn resize_toplevel_to_app_request(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn request_toplevel_resize(
        &self,
        _ctx: gluon_wire::GluonCtx,
        new_size: UVec2,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_focused(
        &self,
        _ctx: gluon_wire::GluonCtx,
        focused: bool,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (id) = self
                        .register_xkb_keymap(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                    id.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    self.absolute_pointer_motion(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                10u32 => {
                    self.relative_pointer_motion(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                11u32 => {
                    self.pointer_button(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                12u32 => {
                    self.pointer_scroll_pixels(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                13u32 => {
                    self.pointer_scroll_discrete(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                14u32 => {
                    self.pointer_scroll_stop(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                15u32 => {
                    self.key(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                16u32 => {
                    self.touch_down(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                17u32 => {
                    self.touch_move(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                18u32 => {
                    self.touch_up(ctx, gluon_wire::GluonConvertable::read(gluon_data)?)
                        .await;
                }
                19u32 => {
                    self.close_toplevel(ctx).await;
                }
                20u32 => {
                    self.resize_toplevel_to_app_request(ctx).await;
                }
                21u32 => {
                    self.request_toplevel_resize(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                22u32 => {
                    self.toplevel_focused(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
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
pub struct PanelShell {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for PanelShell {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(gluon_data)?;
        Ok(PanelShell::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
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
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
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
        new_size: UVec2,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        new_size.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_fullscreen(
        &self,
        fullscreen_active: bool,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        fullscreen_active.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_title(
        &self,
        title: String,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        title.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn toplevel_app_id(
        &self,
        app_id: String,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        app_id.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 12u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn set_cursor_visuals(
        &self,
        geometry: Option<Geometry>,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        geometry.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 13u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn create_child(
        &self,
        child: ChildState,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        child.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 14u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn move_child(
        &self,
        child_id: u64,
        geometry: Geometry,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        child_id.write(&mut gluon_builder)?;
        geometry.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 15u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn destroy_child(
        &self,
        child_id: u64,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        child_id.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 16u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: PanelShellHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        PanelShell { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for PanelShell {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
pub trait PanelShellHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn update_surface_dmatex(
        &self,
        _ctx: gluon_wire::GluonCtx,
        surface: SurfaceUpdateTarget,
        dmatex_uid: u64,
        acquire_point: u64,
        release_point: u64,
        opaque: bool,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_resized(
        &self,
        _ctx: gluon_wire::GluonCtx,
        new_size: UVec2,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_fullscreen(
        &self,
        _ctx: gluon_wire::GluonCtx,
        fullscreen_active: bool,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_title(
        &self,
        _ctx: gluon_wire::GluonCtx,
        title: String,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn toplevel_app_id(
        &self,
        _ctx: gluon_wire::GluonCtx,
        app_id: String,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn set_cursor_visuals(
        &self,
        _ctx: gluon_wire::GluonCtx,
        geometry: Option<Geometry>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn create_child(
        &self,
        _ctx: gluon_wire::GluonCtx,
        child: ChildState,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn move_child(
        &self,
        _ctx: gluon_wire::GluonCtx,
        child_id: u64,
        geometry: Geometry,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn destroy_child(
        &self,
        _ctx: gluon_wire::GluonCtx,
        child_id: u64,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    self.update_surface_dmatex(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                9u32 => {
                    self.toplevel_resized(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                10u32 => {
                    self.toplevel_fullscreen(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                11u32 => {
                    self.toplevel_title(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                12u32 => {
                    self.toplevel_app_id(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                13u32 => {
                    self.set_cursor_visuals(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                14u32 => {
                    self.create_child(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                15u32 => {
                    self.move_child(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                16u32 => {
                    self.destroy_child(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
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
pub struct PanelItemAcceptor {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for PanelItemAcceptor {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(gluon_data)?;
        Ok(PanelItemAcceptor::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PanelItemAcceptor {
    pub async fn accept(
        &self,
        item: PanelItem,
    ) -> Result<(PanelShell, SpatialRefId), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        item.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok((
            gluon_wire::GluonConvertable::read(&mut reader)?,
            gluon_wire::GluonConvertable::read(&mut reader)?,
        ))
    }
    pub async fn get_field(&self) -> Result<FieldRefId, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: PanelItemAcceptorHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        PanelItemAcceptor { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for PanelItemAcceptor {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (shell, output_spatial) = self
                        .accept(ctx, gluon_wire::GluonConvertable::read(gluon_data)?)
                        .await;
                    shell.write_owned(&mut gluon_out)?;
                    output_spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (field) = self.get_field(ctx).await;
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
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for PanelItemProvider {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(gluon_data)?;
        Ok(PanelItemProvider::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PanelItemProvider {
    pub fn register_acceptor(
        &self,
        acceptor: PanelItemAcceptor,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        acceptor.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn drop_acceptor(
        &self,
        acceptor: PanelItemAcceptor,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        acceptor.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub async fn startup_token_spatial_ref(
        &self,
        token: String,
        spatial_ref: SpatialRefId,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        token.write(&mut gluon_builder)?;
        spatial_ref.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(())
    }
    pub fn from_handler<H: PanelItemProviderHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        PanelItemProvider { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for PanelItemProvider {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
pub trait PanelItemProviderHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn register_acceptor(
        &self,
        _ctx: gluon_wire::GluonCtx,
        acceptor: PanelItemAcceptor,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn drop_acceptor(
        &self,
        _ctx: gluon_wire::GluonCtx,
        acceptor: PanelItemAcceptor,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn startup_token_spatial_ref(
        &self,
        _ctx: gluon_wire::GluonCtx,
        token: String,
        spatial_ref: SpatialRefId,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    self.register_acceptor(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                9u32 => {
                    self.drop_acceptor(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let () = self
                        .startup_token_spatial_ref(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
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
