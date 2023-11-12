pub mod utility;
pub mod voxel_grid;

use kiddo::KdTree;
use pointrain_core::pc::PointCloudBase;

pub fn kdtree<PC: PointCloudBase>(pc: &PC) -> KdTree<f32, 3> {
    let mut kdtree = KdTree::with_capacity(pc.len());

    for (i, p) in pc.positions().iter().enumerate() {
        kdtree.add(&[p.x, p.y, p.z], i.try_into().unwrap());
    }
    kdtree
}

pub use voxel_grid::VoxelGrid;
