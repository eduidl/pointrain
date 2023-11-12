use super::PointCloudBase;
use crate::{
    point::{
        xyz::{Point, PointRef, PointRefMut},
        PointBase,
    },
    types::Position,
};

#[derive(Debug, Default, Clone)]
pub struct PointCloud {
    positions: Vec<Position>,
}

impl PointCloud {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "rerun")]
    pub fn rerun_points(&self) -> re_types::archetypes::Points3D {
        self.pos_component_base()
    }
}

impl FromIterator<Point> for PointCloud {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let mut pc = Self::new();
        for p in iter {
            pc.push(p);
        }
        pc
    }
}

impl<'a> FromIterator<PointRef<'a>> for PointCloud {
    fn from_iter<T: IntoIterator<Item = PointRef<'a>>>(iter: T) -> Self {
        let mut pc = Self::new();
        for p in iter {
            pc.push_ref(p);
        }
        pc
    }
}

impl PointCloudBase for PointCloud {
    type Point = Point;
    type Iter<'a> = Iter<'a>;
    type IterMut<'a> = IterMut<'a>;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(capacity),
        }
    }

    fn resize(&mut self, new_len: usize, value: Self::Point) {
        self.positions.resize(new_len, value.position);
    }

    fn positions(&self) -> &[Position] {
        &self.positions
    }

    fn positions_mut(&mut self) -> &mut [Position] {
        &mut self.positions
    }

    fn push(&mut self, p: Self::Point) -> &mut Self {
        self.positions.push(p.position);
        self
    }

    fn push_ref(&mut self, p: <Self::Point as PointBase>::Ref<'_>) -> &mut Self {
        self.positions.push(*p.position);
        self
    }

    fn iter(&self) -> Self::Iter<'_> {
        Self::Iter {
            positions: self.positions.iter(),
        }
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        Self::IterMut {
            positions: self.positions.iter_mut(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'a> {
    positions: std::slice::Iter<'a, Position>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = PointRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::Item {
            position: self.positions.next()?,
        })
    }
}

#[derive(Debug)]
pub struct IterMut<'a> {
    positions: std::slice::IterMut<'a, Position>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = PointRefMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::Item {
            position: self.positions.next()?,
        })
    }
}
