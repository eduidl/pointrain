use nalgebra as na;
#[cfg(feature = "rerun")]
use rerun::components::{Arrow3D, ColorRGBA, Point3D, Vec3D};

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
    fn pos_component(&self) -> Vec<Point3D> {
        self.positions()
            .iter()
            .map(|p| Point3D::new(p.x, p.y, p.z))
            .collect()
    }
}

pub trait PointCloudWithIntensity: PointCloudBase {
    fn intensities(&self) -> &[Float];
    fn intensities_mut(&mut self) -> &mut [Float];

    #[cfg(feature = "rerun")]
    fn intensity_color_component(&self, scale: Option<f32>) -> Vec<ColorRGBA> {
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
                ColorRGBA::from_rgb(r, g, b)
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
    fn normal_component(&self, scale: Option<f32>) -> Vec<Arrow3D> {
        let scale = scale.unwrap_or(0.005);

        self.positions()
            .iter()
            .zip(self.normals().iter())
            .map(|(p, n)| Arrow3D {
                origin: Vec3D::new(p.x, p.y, p.z),
                vector: Vec3D::new(n.x * scale, n.y * scale, n.z * scale),
            })
            .collect()
    }
}

pub trait PointCloudWithColor: PointCloudBase {
    fn colors(&self) -> &[Rgb];
    fn colors_mut(&mut self) -> &mut [Rgb];

    #[cfg(feature = "rerun")]
    fn color_component(&self) -> Vec<ColorRGBA> {
        self.colors()
            .iter()
            .map(|c| ColorRGBA::from_rgb(c.x, c.y, c.z))
            .collect()
    }
}
