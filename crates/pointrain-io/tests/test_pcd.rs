use pointrain_core::{
    pc::{PointCloudXYZI, PointCloudXYZNormal},
    traits::{PointCloud, PointCloudWithNormal},
    types::{Normal, Position},
};
use pointrain_io::{pcd_read, PointRainIOError};

#[test]
fn test_pcd_read_ascii() {
    let pc: PointCloudXYZNormal = pcd_read("tests/data/pcd/test_ascii.pcd").unwrap();

    assert_eq!(pc.len(), 3);
    assert_eq!(pc.positions()[0], Position::new(1.0, 2.0, 3.0));
    assert_eq!(pc.normals()[0], Normal::new(4.0, 5.0, 6.0));
    assert_eq!(pc.curvatures()[0], 7.0);
}

#[test]
fn test_pcd_read_ascii_missing_field() {
    let err = pcd_read::<PointCloudXYZI>("tests/data/pcd/test_ascii.pcd").unwrap_err();

    assert!(matches!(
        err,
        PointRainIOError::MissingFieldError("intensity")
    ));
}

#[test]
fn test_pcd_read_binary() {
    let pc: PointCloudXYZNormal = pcd_read("tests/data/pcd/test_binary.pcd").unwrap();

    assert_eq!(pc.len(), 3);
    assert_eq!(pc.positions()[0], Position::new(1.0, 2.0, 3.0));
    assert_eq!(pc.normals()[0], Normal::new(4.0, 5.0, 6.0));
    assert_eq!(pc.curvatures()[0], 7.0);
}
