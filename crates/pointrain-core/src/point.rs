use crate::types::Position;

pub mod intensity;
pub mod intensity_normal;
pub mod normal;
pub mod rgb;
pub mod rgb_normal;
pub mod xyz;

pub use intensity::{
    Point as PointIntensity, PointRef as PointIntensityRef, PointRefMut as PointIntensityRefMut,
};
pub use intensity_normal::{
    Point as PointIntensityNormal, PointRef as PointIntensityNormalRef,
    PointRefMut as PointIntensityNormalRefMut,
};
pub use normal::{
    Point as PointNormal, PointRef as PointNormalRef, PointRefMut as PointNormalRefMut,
};
pub use rgb::{Point as PointRgb, PointRef as PointRgbRef, PointRefMut as PointRgbRefMut};
pub use rgb_normal::{
    Point as PointRgbNormal, PointRef as PointRgbNormalRef, PointRefMut as PointRgbNormalRefMut,
};
pub use xyz::{Point, PointRef, PointRefMut};

pub trait PointBase: Default {
    type Ref<'a>
    where
        Self: 'a;
    type RefMut<'a>
    where
        Self: 'a;

    fn as_ref(&self) -> Self::Ref<'_>;
    fn as_ref_mut(&mut self) -> Self::RefMut<'_>;

    fn position(&self) -> &Position;
    fn position_mut(&mut self) -> &mut Position;
}
