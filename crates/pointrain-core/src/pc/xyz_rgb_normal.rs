use crate::{
    traits::{PointCloud, PointCloudWithColor, PointCloudWithNormal},
    types::{Float, Normal, Position, Rgb},
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PointXYZRgbNormal {
    pub pos: Position,
    pub normal: Normal,
    pub color: Rgb,
    pub curvature: Float,
}

#[derive(Debug, Default, Clone)]
pub struct PointCloudXYZRgbNormal {
    positions: Vec<Position>,
    colors: Vec<Rgb>,
    normals: Vec<Normal>,
    curvatures: Vec<Float>,
}

impl PointCloudXYZRgbNormal {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PointCloud for PointCloudXYZRgbNormal {
    type Point = PointXYZRgbNormal;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(capacity),
            colors: Vec::with_capacity(capacity),
            normals: Vec::with_capacity(capacity),
            curvatures: Vec::with_capacity(capacity),
        }
    }

    fn positions(&self) -> &[Position] {
        &self.positions
    }

    fn add_point(&mut self, p: Self::Point) -> &mut Self {
        self.positions.push(p.pos);
        self.colors.push(p.color);
        self.normals.push(p.normal);
        self.curvatures.push(p.curvature);
        self
    }
}

impl PointCloudWithColor for PointCloudXYZRgbNormal {
    fn colors(&self) -> &[Rgb] {
        &self.colors
    }
}

impl PointCloudWithNormal for PointCloudXYZRgbNormal {
    fn normals(&self) -> &[Normal] {
        &self.normals
    }

    fn curvatures(&self) -> &[Float] {
        &self.curvatures
    }
}
