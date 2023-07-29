use crate::types::{Float, Normal, Position, Rgb};

pub trait PointCloud: Default {
    type Point;

    fn new() -> Self {
        Self::default()
    }
    fn with_capacity(capacity: usize) -> Self;
    fn positions(&self) -> &[Position];
    fn add_point(&mut self, p: Self::Point) -> &mut Self;

    fn len(&self) -> usize {
        self.positions().len()
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait PointCloudWithIntensity: PointCloud {
    fn intensities(&self) -> &[Float];
}

pub trait PointCloudWithNormal: PointCloud {
    fn normals(&self) -> &[Normal];
    fn curvatures(&self) -> &[Float];
}

pub trait PointCloudWithColor: PointCloud {
    fn colors(&self) -> &[Rgb];
}
