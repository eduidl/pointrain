use crate::{
    traits::{PointCloud, PointCloudWithIntensity, PointCloudWithNormal},
    types::{Float, Normal, Position},
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PointXYZINormal {
    pub pos: Position,
    pub normal: Normal,
    pub intensity: Float,
    pub curvature: Float,
}

#[derive(Debug, Default, Clone)]
pub struct PointCloudXYZINormal {
    positions: Vec<Position>,
    intensities: Vec<Float>,
    normals: Vec<Normal>,
    curvatures: Vec<Float>,
}

impl PointCloudXYZINormal {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PointCloud for PointCloudXYZINormal {
    type Point = PointXYZINormal;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(capacity),
            intensities: Vec::with_capacity(capacity),
            normals: Vec::with_capacity(capacity),
            curvatures: Vec::with_capacity(capacity),
        }
    }

    fn positions(&self) -> &[Position] {
        &self.positions
    }

    fn add_point(&mut self, p: Self::Point) -> &mut Self {
        self.positions.push(p.pos);
        self.intensities.push(p.intensity);
        self.normals.push(p.normal);
        self.curvatures.push(p.curvature);
        self
    }
}

impl PointCloudWithIntensity for PointCloudXYZINormal {
    fn intensities(&self) -> &[Float] {
        &self.intensities
    }
}

impl PointCloudWithNormal for PointCloudXYZINormal {
    fn normals(&self) -> &[Normal] {
        &self.normals
    }

    fn curvatures(&self) -> &[Float] {
        &self.curvatures
    }
}
