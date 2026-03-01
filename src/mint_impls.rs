use mint::{IntoMint, Vector2};

use crate::protocol::{IVec2, UVec2, Vec2};
macro_rules! impl_mint {
    ($proto:ty, $mint:ty, $($field:ident),*) => {

impl IntoMint for $proto {
    type MintType = $mint;
}
impl From<$proto> for $mint {
    fn from(v: $proto) -> Self {
        Self {$($field: v.$field),*}
    }
}
impl From<$mint> for $proto {
    fn from(v: $mint) -> Self {
        Self {$($field: v.$field),*}
    }
}
    };
}
impl_mint!(UVec2, Vector2<u32>, x, y);
impl_mint!(IVec2, Vector2<i32>, x, y);
impl_mint!(Vec2, Vector2<f32>, x, y);
