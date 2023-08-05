mod base;
pub mod intensity;
pub mod intensity_normal;
pub mod normal;
pub mod rgb;
pub mod rgb_normal;
pub mod xyz;

pub use base::{
    PointCloudBase, PointCloudWithColor, PointCloudWithIntensity, PointCloudWithNormal,
};
pub use intensity::PointCloud as PointCloudIntensity;
pub use intensity_normal::PointCloud as PointCloudIntensityNormal;
pub use normal::PointCloud as PointCloudNormal;
pub use rgb::PointCloud as PointCloudRgb;
pub use rgb_normal::PointCloud as PointCloudRgbNormal;
pub use xyz::PointCloud;
