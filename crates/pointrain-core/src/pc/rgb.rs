use super::{PointCloudBase, PointCloudWithColor};
use crate::{
    point::{
        rgb::{Point, PointRef, PointRefMut},
        PointBase,
    },
    types::{Position, Rgb},
};

#[derive(Debug, Default, Clone)]
pub struct PointCloud {
    positions: Vec<Position>,
    colors: Vec<Rgb>,
}

impl PointCloud {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "rerun")]
    pub fn rerun_points(&self) -> re_types::archetypes::Points3D {
        self.pos_component_base().with_colors(self.rerun_colors())
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
            colors: Vec::with_capacity(capacity),
        }
    }

    fn resize(&mut self, new_len: usize, value: Self::Point) {
        self.positions.resize(new_len, value.position);
        self.colors.resize(new_len, value.color);
    }

    fn positions(&self) -> &[Position] {
        &self.positions
    }

    fn positions_mut(&mut self) -> &mut [Position] {
        &mut self.positions
    }

    fn push(&mut self, p: Self::Point) -> &mut Self {
        self.positions.push(p.position);
        self.colors.push(p.color);
        self
    }

    fn push_ref(&mut self, p: <Self::Point as PointBase>::Ref<'_>) -> &mut Self {
        self.positions.push(*p.position);
        self.colors.push(*p.color);
        self
    }

    fn iter(&self) -> Self::Iter<'_> {
        Self::Iter {
            positions: self.positions.iter(),
            colors: self.colors.iter(),
        }
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        Self::IterMut {
            positions: self.positions.iter_mut(),
            colors: self.colors.iter_mut(),
        }
    }
}

impl PointCloudWithColor for PointCloud {
    fn colors(&self) -> &[Rgb] {
        &self.colors
    }

    fn colors_mut(&mut self) -> &mut [Rgb] {
        &mut self.colors
    }
}

pub struct Iter<'a> {
    positions: std::slice::Iter<'a, Position>,
    colors: std::slice::Iter<'a, Rgb>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = PointRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::Item {
            position: self.positions.next()?,
            color: self.colors.next()?,
        })
    }
}

pub struct IterMut<'a> {
    positions: std::slice::IterMut<'a, Position>,
    colors: std::slice::IterMut<'a, Rgb>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = PointRefMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::Item {
            position: self.positions.next()?,
            color: self.colors.next()?,
        })
    }
}
