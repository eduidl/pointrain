#[cfg(feature = "rerun")]
use rerun::{EntityPath, MsgSender, MsgSenderError};

use crate::{
    traits::{PointCloud, PointCloudWithColor},
    types::{Position, Rgb},
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PointXYZRgb {
    pub pos: Position,
    pub color: Rgb,
}

#[derive(Debug, Default, Clone)]
pub struct PointCloudXYZRgb {
    positions: Vec<Position>,
    colors: Vec<Rgb>,
}

impl PointCloudXYZRgb {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "rerun")]
    pub fn rerun_msg_sender(
        &self,
        label: impl Into<EntityPath>,
    ) -> Result<MsgSender, MsgSenderError> {
        use rerun::components::{ColorRGBA, Point3D};

        let points: Vec<_> = self
            .positions
            .iter()
            .map(|p| Point3D::new(p.x, p.y, p.z))
            .collect();

        let colors: Vec<_> = self
            .colors
            .iter()
            .map(|c| ColorRGBA::from_rgb(c.x, c.y, c.z))
            .collect();

        MsgSender::new(label.into())
            .with_component(&points)?
            .with_component(&colors)
    }
}

impl PointCloud for PointCloudXYZRgb {
    type Point = PointXYZRgb;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(capacity),
            colors: Vec::with_capacity(capacity),
        }
    }

    fn positions(&self) -> &[Position] {
        &self.positions
    }

    fn add_point(&mut self, p: Self::Point) -> &mut Self {
        self.positions.push(p.pos);
        self.colors.push(p.color);
        self
    }
}

impl PointCloudWithColor for PointCloudXYZRgb {
    fn colors(&self) -> &[Rgb] {
        &self.colors
    }
}
