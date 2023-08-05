use super::PointBase;
use crate::types::Position;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    pub position: Position,
}

impl PointBase for Point {
    type Ref<'a> = PointRef<'a>;
    type RefMut<'a> = PointRefMut<'a>;

    fn as_ref(&self) -> Self::Ref<'_> {
        Self::Ref {
            position: &self.position,
        }
    }

    fn as_ref_mut(&mut self) -> Self::RefMut<'_> {
        Self::RefMut {
            position: &mut self.position,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointRef<'a> {
    pub position: &'a Position,
}

#[derive(Debug, PartialEq)]
pub struct PointRefMut<'a> {
    pub position: &'a mut Position,
}
