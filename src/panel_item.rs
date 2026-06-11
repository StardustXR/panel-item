pub use crate::protocol::panel_item::*;

impl From<SurfaceId> for SurfaceUpdateTarget {
    fn from(value: SurfaceId) -> Self {
        match value {
            SurfaceId::Toplevel => Self::Toplevel,
            SurfaceId::Child { id } => Self::Child { id },
        }
    }
}
