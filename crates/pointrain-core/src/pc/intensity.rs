#[cfg(feature = "rerun")]
use rerun::{EntityPath, MsgSender, MsgSenderError};

use super::{PointCloudBase, PointCloudWithIntensity};
use crate::{
    point::{
        intensity::{Point, PointRef, PointRefMut},
        PointBase,
    },
    types::{Float, Position},
};

#[derive(Debug, Default, Clone)]
pub struct PointCloud {
    positions: Vec<Position>,
    intensities: Vec<Float>,
}

impl PointCloud {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "rerun")]
    pub fn rerun_msg_sender(
        &self,
        label: impl Into<EntityPath>,
    ) -> Result<MsgSender, MsgSenderError> {
        MsgSender::new(label.into())
            .with_component(&self.pos_component())?
            .with_component(&self.intensity_color_component(None))
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
            intensities: Vec::with_capacity(capacity),
        }
    }

    fn resize(&mut self, new_len: usize, value: Self::Point) {
        self.positions.resize(new_len, value.position);
        self.intensities.resize(new_len, value.intensity);
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
        self
    }

    fn push_ref(&mut self, p: <Self::Point as PointBase>::Ref<'_>) -> &mut Self {
        self.positions.push(*p.position);
        self.intensities.push(*p.intensity);
        self
    }

    fn iter(&self) -> Self::Iter<'_> {
        Self::Iter {
            positions: self.positions.iter(),
            intensities: self.intensities.iter(),
        }
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        Self::IterMut {
            positions: self.positions.iter_mut(),
            intensities: self.intensities.iter_mut(),
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

pub struct Iter<'a> {
    positions: std::slice::Iter<'a, Position>,
    intensities: std::slice::Iter<'a, Float>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = PointRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::Item {
            position: self.positions.next()?,
            intensity: self.intensities.next()?,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.positions.size_hint()
    }
}

pub struct IterMut<'a> {
    positions: std::slice::IterMut<'a, Position>,
    intensities: std::slice::IterMut<'a, Float>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = PointRefMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::Item {
            position: self.positions.next()?,
            intensity: self.intensities.next()?,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.positions.size_hint()
    }
}
