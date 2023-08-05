use super::PointBase;
use crate::types::{Float, Normal, Position, Rgb};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    pub position: Position,
    pub color: Rgb,
    pub normal: Normal,
    pub curvature: Float,
}

impl PointBase for Point {
    type Ref<'a> = PointRef<'a>;
    type RefMut<'a> = PointRefMut<'a>;

    fn as_ref(&self) -> Self::Ref<'_> {
        Self::Ref {
            position: &self.position,
            color: &self.color,
            normal: &self.normal,
            curvature: &self.curvature,
        }
    }

    fn as_ref_mut(&mut self) -> Self::RefMut<'_> {
        Self::RefMut {
            position: &mut self.position,
            color: &mut self.color,
            normal: &mut self.normal,
            curvature: &mut self.curvature,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointRef<'a> {
    pub position: &'a Position,
    pub color: &'a Rgb,
    pub normal: &'a Normal,
    pub curvature: &'a Float,
}

#[derive(Debug, PartialEq)]
pub struct PointRefMut<'a> {
    pub position: &'a mut Position,
    pub color: &'a mut Rgb,
    pub normal: &'a mut Normal,
    pub curvature: &'a mut Float,
}
