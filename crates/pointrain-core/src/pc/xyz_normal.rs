#[cfg(feature = "rerun")]
use rerun::{EntityPath, MsgSender, MsgSenderError};

use crate::{
    traits::{PointCloud, PointCloudWithNormal},
    types::{Float, Normal, Position},
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PointXYZNormal {
    pub pos: Position,
    pub normal: Normal,
    pub curvature: Float,
}

#[derive(Debug, Default, Clone)]
pub struct PointCloudXYZNormal {
    positions: Vec<Position>,
    normals: Vec<Normal>,
    curvatures: Vec<Float>,
}

impl PointCloudXYZNormal {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "rerun")]
    pub fn rerun_msg_sender(
        &self,
        label: impl Into<EntityPath>,
        scale: Option<f32>,
    ) -> Result<MsgSender, MsgSenderError> {
        use rerun::components::{Arrow3D, Vec3D};

        let scale = scale.unwrap_or(0.005);

        let arrows: Vec<_> = self
            .positions
            .iter()
            .zip(self.normals.iter())
            .map(|(p, n)| Arrow3D {
                origin: Vec3D::new(p.x, p.y, p.z),
                vector: Vec3D::new(n.x * scale, n.y * scale, n.z * scale),
            })
            .collect();

        MsgSender::new(label.into()).with_component(&arrows)
    }
}

impl PointCloud for PointCloudXYZNormal {
    type Point = PointXYZNormal;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(capacity),
            normals: Vec::with_capacity(capacity),
            curvatures: Vec::with_capacity(capacity),
        }
    }

    fn positions(&self) -> &[Position] {
        &self.positions
    }

    fn add_point(&mut self, p: Self::Point) -> &mut Self {
        self.positions.push(p.pos);
        self.normals.push(p.normal);
        self.curvatures.push(p.curvature);
        self
    }
}

impl PointCloudWithNormal for PointCloudXYZNormal {
    fn normals(&self) -> &[Normal] {
        &self.normals
    }

    fn curvatures(&self) -> &[Float] {
        &self.curvatures
    }
}
