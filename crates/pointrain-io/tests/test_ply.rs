use pointrain_core::{
    pc::{PointCloud, PointCloudBase},
    types::Position,
};
use pointrain_io::ply::ply_read;

#[test]
fn test_ply_read_ascii() {
    let pc = ply_read::<PointCloud>("tests/data/ply/test_ascii.ply").unwrap();

    assert_eq!(pc.len(), 8);
    assert_eq!(pc.positions()[0], Position::new(0., 0., 0.));
    assert_eq!(pc.positions()[7], Position::new(1., 1., 0.));
}
