use nalgebra as na;
#[cfg(feature = "rerun")]
use re_types::{
    archetypes::{Arrows3D, Points3D},
    components::{Color, Position3D},
};

use crate::{
    point::PointBase,
    types::{Float, Normal, Position, Rgb},
};

pub trait PointCloudBase: Default {
    type Point: PointBase;
    type Iter<'a>: Iterator<Item = <Self::Point as PointBase>::Ref<'a>>
    where
        Self: 'a;
    type IterMut<'a>: Iterator<Item = <Self::Point as PointBase>::RefMut<'a>>
    where
        Self: 'a;

    fn new() -> Self {
        Default::default()
    }
    fn with_capacity(capacity: usize) -> Self;
    fn resize(&mut self, new_len: usize, value: Self::Point);

    fn positions(&self) -> &[Position];
    fn positions_mut(&mut self) -> &mut [Position];

    fn push(&mut self, p: Self::Point) -> &mut Self;
    fn push_ref(&mut self, p: <Self::Point as PointBase>::Ref<'_>) -> &mut Self;

    fn iter(&self) -> Self::Iter<'_>;
    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    fn len(&self) -> usize {
        self.positions().len()
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn translate(mut self, translation: impl Into<na::Translation3<Float>>) -> Self {
        self.translate_mut(translation);
        self
    }

    fn translate_mut(&mut self, translation: impl Into<na::Translation3<Float>>) -> &mut Self {
        let translation: na::Translation3<Float> = translation.into();

        for p in self.positions_mut() {
            *p = translation.transform_point(p);
        }
        self
    }

    fn rotate(mut self, rotation: impl Into<na::UnitQuaternion<Float>>) -> Self {
        self.rotate_mut(rotation);
        self
    }

    fn rotate_mut(&mut self, rotation: impl Into<na::UnitQuaternion<Float>>) -> &mut Self {
        let rotation: na::UnitQuaternion<Float> = rotation.into();

        for p in self.positions_mut() {
            *p = rotation.transform_point(p);
        }
        self
    }

    fn transform(mut self, transform: impl Into<na::Transform3<Float>>) -> Self {
        self.transform_mut(transform);
        self
    }

    fn transform_mut(&mut self, transform: impl Into<na::Transform3<Float>>) -> &mut Self {
        let transform: na::Transform3<Float> = transform.into();

        for p in self.positions_mut() {
            *p = transform.transform_point(p);
        }
        self
    }

    #[cfg(feature = "rerun")]
    fn rerun_positions(&self) -> Vec<Position3D> {
        self.positions()
            .iter()
            .map(|p| Position3D::new(p.x, p.y, p.z))
            .collect()
    }

    #[cfg(feature = "rerun")]
    fn pos_component_base(&self) -> Points3D {
        Points3D::new(self.rerun_positions())
    }
}

pub trait PointCloudWithIntensity: PointCloudBase {
    fn intensities(&self) -> &[Float];
    fn intensities_mut(&mut self) -> &mut [Float];

    #[cfg(feature = "rerun")]
    fn intensity_colors(&self, scale: Option<f32>) -> Vec<Color> {
        let scale = scale.unwrap_or_else(|| {
            let max_intensity = self.intensities().iter().fold(f32::NAN, |a, b| b.max(a));
            assert!(max_intensity.is_finite());
            max_intensity
        });

        let turbo = colorgrad::turbo();
        self.intensities()
            .iter()
            .map(|i| {
                let t = i / (scale + 1e-6);
                let [r, g, b, _] = turbo.at(t as f64).to_rgba8();
                Color::from_rgb(r, g, b)
            })
            .collect()
    }
}

pub trait PointCloudWithNormal: PointCloudBase {
    fn normals(&self) -> &[Normal];
    fn normals_mut(&mut self) -> &mut [Normal];
    fn curvatures(&self) -> &[Float];
    fn curvatures_mut(&mut self) -> &mut [Float];

    #[cfg(feature = "rerun")]
    fn normal_component_base(&self, scale: Option<f32>) -> Arrows3D {
        let scale = scale.unwrap_or(0.005);

        Arrows3D::from_vectors(
            self.normals()
                .iter()
                .map(|n| (n.x * scale, n.y * scale, n.z * scale)),
        )
        .with_origins(self.rerun_positions())
    }
}

pub trait PointCloudWithColor: PointCloudBase {
    fn colors(&self) -> &[Rgb];
    fn colors_mut(&mut self) -> &mut [Rgb];

    #[cfg(feature = "rerun")]
    fn rerun_colors(&self) -> Vec<Color> {
        self.colors()
            .iter()
            .map(|c| Color::from_rgb(c.x, c.y, c.z))
            .collect()
    }
}
