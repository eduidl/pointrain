use approx::assert_abs_diff_eq;
use pointrain_core::{
    nalgebra::{Point3, UnitQuaternion, Vector3},
    pc::{PointCloud, PointCloudBase},
    point::Point,
};

fn test_pc() -> PointCloud {
    let points = vec![
        Point {
            position: Point3::new(0., 0., 0.),
        },
        Point {
            position: Point3::new(1., 2., 3.),
        },
    ];
    points.into_iter().collect()
}

#[test]
fn test_translate() {
    let pc = test_pc().translate(Vector3::new(-1., -2., 3.));

    assert_eq!(pc.len(), 2);
    assert_eq!(pc.positions()[0], Point3::new(-1., -2., 3.));
    assert_eq!(pc.positions()[1], Point3::new(0., 0., 6.));
}

#[test]
fn test_rotate_ident() {
    let pc = test_pc().rotate(UnitQuaternion::identity());

    assert_eq!(pc.len(), 2);
    assert_eq!(pc.positions()[0], Point3::new(0., 0., 0.));
    assert_eq!(pc.positions()[1], Point3::new(1., 2., 3.));
}

#[test]
fn test_rotate() {
    let pc = test_pc().rotate(UnitQuaternion::from_axis_angle(
        &Vector3::x_axis(),
        std::f32::consts::FRAC_PI_2,
    ));

    assert_eq!(pc.len(), 2);
    assert_eq!(pc.positions()[0], Point3::new(0., 0., 0.));
    assert_abs_diff_eq!(pc.positions()[1], Point3::new(1., -3., 2.), epsilon = 1e-6);
}
