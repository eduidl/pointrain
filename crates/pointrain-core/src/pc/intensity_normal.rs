use super::{PointCloudBase, PointCloudWithIntensity, PointCloudWithNormal};
use crate::{
    point::{
        intensity_normal::{Point, PointRef, PointRefMut},
        PointBase,
    },
    types::{Float, Normal, Position},
};

#[derive(Debug, Default, Clone)]
pub struct PointCloud {
    positions: Vec<Position>,
    intensities: Vec<Float>,
    normals: Vec<Normal>,
    curvatures: Vec<Float>,
}

impl PointCloud {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "rerun")]
    pub fn rerun_points(&self, intensity_scale: Option<f32>) -> re_types::archetypes::Points3D {
        self.pos_component_base()
            .with_colors(self.intensity_colors(intensity_scale))
    }

    #[cfg(feature = "rerun")]
    pub fn rerun_normals(
        &self,
        normal_scale: Option<f32>,
        intensity_scale: Option<f32>,
    ) -> re_types::archetypes::Arrows3D {
        self.normal_component_base(normal_scale)
            .with_colors(self.intensity_colors(intensity_scale))
    }
}

impl FromIterator<Point> for PointCloud {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Point>,
    {
        let mut pc = Self::new();
        for p in iter {
            pc.push(p);
        }
        pc
    }
}

impl<'a> FromIterator<PointRef<'a>> for PointCloud {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = PointRef<'a>>,
    {
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
            intensities: Vec::with_capacity(capacity),
            normals: Vec::with_capacity(capacity),
            curvatures: Vec::with_capacity(capacity),
        }
    }

    fn resize(&mut self, new_len: usize, value: Self::Point) {
        self.positions.resize(new_len, value.position);
        self.intensities.resize(new_len, value.intensity);
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
        self.intensities.push(p.intensity);
        self.normals.push(p.normal);
        self.curvatures.push(p.curvature);
        self
    }

    fn push_ref(&mut self, p: <Self::Point as PointBase>::Ref<'_>) -> &mut Self {
        self.positions.push(*p.position);
        self.intensities.push(*p.intensity);
        self.normals.push(*p.normal);
        self.curvatures.push(*p.curvature);
        self
    }

    fn iter(&self) -> Self::Iter<'_> {
        Self::Iter {
            positions: self.positions.iter(),
            intensities: self.intensities.iter(),
            normals: self.normals.iter(),
            curvatures: self.curvatures.iter(),
        }
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        Self::IterMut {
            positions: self.positions.iter_mut(),
            intensities: self.intensities.iter_mut(),
            normals: self.normals.iter_mut(),
            curvatures: self.curvatures.iter_mut(),
        }
    }
}

impl PointCloudWithIntensity for PointCloud {
    fn intensities(&self) -> &[Float] {
        &self.intensities
    }

    fn intensities_mut(&mut self) -> &mut [Float] {
        &mut self.intensities
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
    intensities: std::slice::Iter<'a, Float>,
    normals: std::slice::Iter<'a, Normal>,
    curvatures: std::slice::Iter<'a, Float>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = PointRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(PointRef {
            position: self.positions.next()?,
            intensity: self.intensities.next()?,
            normal: self.normals.next()?,
            curvature: self.curvatures.next()?,
        })
    }
}

pub struct IterMut<'a> {
    positions: std::slice::IterMut<'a, Position>,
    intensities: std::slice::IterMut<'a, Float>,
    normals: std::slice::IterMut<'a, Normal>,
    curvatures: std::slice::IterMut<'a, Float>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = PointRefMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(PointRefMut {
            position: self.positions.next()?,
            intensity: self.intensities.next()?,
            normal: self.normals.next()?,
            curvature: self.curvatures.next()?,
        })
    }
}
