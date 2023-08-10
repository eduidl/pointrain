use pointrain_core::{pc::PointCloudBase, types::Position};
use pointrain_io::{xyz_read, PointRainIOError};

#[test]
fn test_xyz_read() {
    let pc = xyz_read("tests/data/xyz/test.xyz").unwrap();
    assert_eq!(pc.len(), 3);
    assert_eq!(pc.positions()[0], Position::new(0.1, -0.2, 0.3));
    assert_eq!(pc.positions()[1], Position::new(1., -2., 3.));
    assert_eq!(pc.positions()[2], Position::new(10., -200., 3000.));
}

#[test]
fn test_xyz_read_error() {
    let err = xyz_read("tests/data/xyz/invalid_size.xyz").unwrap_err();

    assert!(matches!(err, PointRainIOError::Error { .. }));
}
