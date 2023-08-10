use std::f32::{INFINITY, NAN, NEG_INFINITY};

use approx::assert_relative_eq;
use pointrain_core::{
    pc::{PointCloud, PointCloudBase},
    point::Point,
    types::Position,
};
use pointrain_filter::VoxelGrid;

#[test]
fn test_voxel_grid() {
    let voxel_grid = VoxelGrid::new();

    let points = vec![
        Point {
            position: Position::new(0.4, 0.1, 0.8),
        },
        Point {
            position: Position::new(0.1, 0.1, 0.7),
        },
        Point {
            position: Position::new(0.2, 0.2, 0.1),
        },
        Point {
            position: Position::new(0.3, 0.5, 0.2),
        },
    ];
    let pc: PointCloud = points.into_iter().collect();
    let filterd = voxel_grid.filter(&pc);

    assert_eq!(filterd.len(), 1);
    assert_relative_eq!(
        filterd.positions()[0],
        Position::new(1. / 4., 0.9 / 4., 1.8 / 4.)
    );
}

#[test]
fn test_voxel_grid_minimum() {
    let mut voxel_grid = VoxelGrid {
        min_poins_per_voxel: 5,
        ..Default::default()
    };

    let points = vec![
        Point {
            position: Position::new(0.4, 0.1, 0.8),
        },
        Point {
            position: Position::new(0.1, 0.1, 0.7),
        },
        Point {
            position: Position::new(0.2, 0.2, 0.1),
        },
        Point {
            position: Position::new(0.3, 0.5, 0.2),
        },
    ];
    let pc: PointCloud = points.into_iter().collect();

    assert_eq!(voxel_grid.filter(&pc).len(), 0);

    voxel_grid.min_poins_per_voxel = 4;

    assert_eq!(voxel_grid.filter(&pc).len(), 1);
}

#[test]
fn test_voxel_grid_invalid_points() {
    let voxel_grid = VoxelGrid::new();

    let points = vec![
        Point {
            position: Position::new(INFINITY, 100., 100.),
        },
        Point {
            position: Position::new(NEG_INFINITY, 100., 100.),
        },
        Point {
            position: Position::new(100., NAN, 100.),
        },
        Point {
            position: Position::new(100., 100., NAN),
        },
    ];
    let pc: PointCloud = points.into_iter().collect();
    let filterd = voxel_grid.filter(&pc);

    assert_eq!(filterd.len(), 0);
}
