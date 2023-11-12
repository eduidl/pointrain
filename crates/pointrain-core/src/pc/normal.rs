use super::{PointCloudBase, PointCloudWithNormal};
use crate::{
    point::{
        normal::{Point, PointRef, PointRefMut},
        PointBase,
    },
    types::{Float, Normal, Position},
};

#[derive(Debug, Default, Clone)]
pub struct PointCloud {
    positions: Vec<Position>,
    normals: Vec<Normal>,
    curvatures: Vec<Float>,
}

impl PointCloud {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "rerun")]
    pub fn rerun_points(&self) -> re_types::archetypes::Points3D {
        self.pos_component_base()
    }

    #[cfg(feature = "rerun")]
    pub fn rerun_normals(&self, normal_scale: Option<f32>) -> re_types::archetypes::Arrows3D {
        self.normal_component_base(normal_scale)
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
            normals: Vec::with_capacity(capacity),
            curvatures: Vec::with_capacity(capacity),
        }
    }

    fn resize(&mut self, new_len: usize, value: Self::Point) {
        self.positions.resize(new_len, value.position);
        self.normals.resize(new_len, value.normal);
        self.curvatures.resize(new_len, value.curvature);
    }

    fn positions(&self) -> &[Position] {
        &self.positions
    }

    fn positions_mut(&mut self) -> &mut [Position] {
        &mut self.positions
    }

    fn push(&mut self, p: Self::Point) -> &mut Self {
        self.positions.push(p.position);
        self.normals.push(p.normal);
        self.curvatures.push(p.curvature);
        self
    }

    fn push_ref(&mut self, p: <Self::Point as PointBase>::Ref<'_>) -> &mut Self {
        self.positions.push(*p.position);
        self.normals.push(*p.normal);
        self.curvatures.push(*p.curvature);
        self
    }

    fn iter(&self) -> Self::Iter<'_> {
        Self::Iter {
            positions: self.positions.iter(),
            normals: self.normals.iter(),
            curvatures: self.curvatures.iter(),
        }
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        Self::IterMut {
            positions: self.positions.iter_mut(),
            normals: self.normals.iter_mut(),
            curvatures: self.curvatures.iter_mut(),
        }
    }
}

impl PointCloudWithNormal for PointCloud {
    fn normals(&self) -> &[Normal] {
        &self.normals
    }

    fn normals_mut(&mut self) -> &mut [Normal] {
        &mut self.normals
    }

    fn curvatures(&self) -> &[Float] {
        &self.curvatures
    }

    fn curvatures_mut(&mut self) -> &mut [Float] {
        &mut self.curvatures
    }
}

pub struct Iter<'a> {
    positions: std::slice::Iter<'a, Position>,
    normals: std::slice::Iter<'a, Normal>,
    curvatures: std::slice::Iter<'a, Float>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = PointRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::Item {
            position: self.positions.next()?,
            normal: self.normals.next()?,
            curvature: self.curvatures.next()?,
        })
    }
}

pub struct IterMut<'a> {
    positions: std::slice::IterMut<'a, Position>,
    normals: std::slice::IterMut<'a, Normal>,
    curvatures: std::slice::IterMut<'a, Float>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = PointRefMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::Item {
            position: self.positions.next()?,
            normal: self.normals.next()?,
            curvature: self.curvatures.next()?,
        })
    }
}
