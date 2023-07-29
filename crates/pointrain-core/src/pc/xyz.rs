#[cfg(feature = "rerun")]
use rerun::{EntityPath, MsgSender, MsgSenderError};

use crate::{traits::PointCloud, types::Position};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PointXYZ {
    pub pos: Position,
}

#[derive(Debug, Default, Clone)]
pub struct PointCloudXYZ {
    positions: Vec<Position>,
}

impl PointCloudXYZ {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "rerun")]
    pub fn rerun_msg_sender(
        &self,
        label: impl Into<EntityPath>,
    ) -> Result<MsgSender, MsgSenderError> {
        use rerun::components::Point3D;

        let points: Vec<_> = self
            .positions
            .iter()
            .map(|p| Point3D::new(p.x, p.y, p.z))
            .collect();

        MsgSender::new(label.into()).with_component(&points)
    }
}

impl PointCloud for PointCloudXYZ {
    type Point = PointXYZ;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(capacity),
        }
    }

    fn positions(&self) -> &[Position] {
        &self.positions
    }

    fn add_point(&mut self, p: Self::Point) -> &mut Self {
        self.positions.push(p.pos);
        self
    }
}
