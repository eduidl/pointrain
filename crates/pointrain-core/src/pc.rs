mod xyz;
mod xyz_normal;
mod xyz_rgb;
mod xyz_rgb_normal;
mod xyzi;
mod xyzi_normal;

pub use xyz::PointCloudXYZ;
pub use xyz_normal::PointCloudXYZNormal;
pub use xyz_rgb::PointCloudXYZRgb;
pub use xyz_rgb_normal::PointCloudXYZRgbNormal;
pub use xyzi::PointCloudXYZI;
pub use xyzi_normal::PointCloudXYZINormal;

pub mod point {
    pub use super::{
        xyz::PointXYZ, xyz_normal::PointXYZNormal, xyz_rgb::PointXYZRgb,
        xyz_rgb_normal::PointXYZRgbNormal, xyzi::PointXYZI, xyzi_normal::PointXYZINormal,
    };
}
