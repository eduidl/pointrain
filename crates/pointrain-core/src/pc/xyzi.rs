#[cfg(feature = "rerun")]
use rerun::{EntityPath, MsgSender, MsgSenderError};

use crate::{
    traits::{PointCloud, PointCloudWithIntensity},
    types::{Float, Position},
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PointXYZI {
    pub pos: Position,
    pub intensity: Float,
}

#[derive(Debug, Default, Clone)]
pub struct PointCloudXYZI {
    positions: Vec<Position>,
    intensities: Vec<Float>,
}

impl PointCloudXYZI {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "rerun")]
    pub fn rerun_msg_sender(
        &self,
        label: impl Into<EntityPath>,
    ) -> Result<MsgSender, MsgSenderError> {
        use rerun::components::{ColorRGBA, Point3D};

        let max_intensity = self.intensities.iter().fold(f32::NAN, |a, b| b.max(a));
        assert!(max_intensity.is_finite());

        let turbo = colorgrad::turbo();
        let colors: Vec<_> = self
            .intensities
            .iter()
            .map(|i| {
                let t = i / (max_intensity + 1e-6);
                let [r, g, b, _] = turbo.at(t as f64).to_rgba8();
                ColorRGBA::from_rgb(r, g, b)
            })
            .collect();

        let points: Vec<_> = self
            .positions
            .iter()
            .map(|p| Point3D::new(p.x, p.y, p.z))
            .collect();

        MsgSender::new(label.into())
            .with_component(&points)?
            .with_component(&colors)
    }
}

impl PointCloud for PointCloudXYZI {
    type Point = PointXYZI;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(capacity),
            intensities: Vec::with_capacity(capacity),
        }
    }

    fn positions(&self) -> &[Position] {
        &self.positions
    }

    fn add_point(&mut self, p: Self::Point) -> &mut Self {
        self.positions.push(p.pos);
        self.intensities.push(p.intensity);
        self
    }
}

impl PointCloudWithIntensity for PointCloudXYZI {
    fn intensities(&self) -> &[Float] {
        &self.intensities
    }
}
