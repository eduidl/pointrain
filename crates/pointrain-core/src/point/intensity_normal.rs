use super::PointBase;
use crate::types::{Float, Normal, Position};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    pub position: Position,
    pub intensity: Float,
    pub normal: Normal,
    pub curvature: Float,
}

impl PointBase for Point {
    type Ref<'a> = PointRef<'a>;
    type RefMut<'a> = PointRefMut<'a>;

    fn as_ref(&self) -> Self::Ref<'_> {
        Self::Ref {
            position: &self.position,
            intensity: &self.intensity,
            normal: &self.normal,
            curvature: &self.curvature,
        }
    }

    fn as_ref_mut(&mut self) -> Self::RefMut<'_> {
        Self::RefMut {
            position: &mut self.position,
            intensity: &mut self.intensity,
            normal: &mut self.normal,
            curvature: &mut self.curvature,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointRef<'a> {
    pub position: &'a Position,
    pub intensity: &'a Float,
    pub normal: &'a Normal,
    pub curvature: &'a Float,
}

#[derive(Debug, PartialEq)]
pub struct PointRefMut<'a> {
    pub position: &'a mut Position,
    pub intensity: &'a mut Float,
    pub normal: &'a mut Normal,
    pub curvature: &'a mut Float,
}
