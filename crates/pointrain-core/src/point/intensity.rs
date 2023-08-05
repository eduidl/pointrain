use super::PointBase;
use crate::types::{Float, Position};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    pub position: Position,
    pub intensity: Float,
}

impl PointBase for Point {
    type Ref<'a> = PointRef<'a>;
    type RefMut<'a> = PointRefMut<'a>;

    fn as_ref(&self) -> Self::Ref<'_> {
        Self::Ref {
            position: &self.position,
            intensity: &self.intensity,
        }
    }

    fn as_ref_mut(&mut self) -> Self::RefMut<'_> {
        Self::RefMut {
            position: &mut self.position,
            intensity: &mut self.intensity,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointRef<'a> {
    pub position: &'a Position,
    pub intensity: &'a Float,
}

#[derive(Debug, PartialEq)]
pub struct PointRefMut<'a> {
    pub position: &'a mut Position,
    pub intensity: &'a mut Float,
}
