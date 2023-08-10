use pointrain_core::{nalgebra::Vector3, pc::PointCloudBase, point::PointBase, types::Position};

use crate::utility;

#[derive(Debug)]
pub struct VoxelGrid {
    pub leaf_size: (f32, f32, f32),
    pub min_poins_per_voxel: usize,
}

impl Default for VoxelGrid {
    fn default() -> Self {
        Self {
            leaf_size: (1., 1., 1.),
            min_poins_per_voxel: 1,
        }
    }
}

impl VoxelGrid {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn filter<PC: PointCloudBase>(&self, pc: &PC) -> PC {
        let inv_leaf_size = Vector3::new(
            1. / self.leaf_size.0,
            1. / self.leaf_size.1,
            1. / self.leaf_size.2,
        );

        let mut indices = Vec::with_capacity(pc.len());

        for (i, p) in pc.positions().iter().enumerate() {
            if !utility::is_finite(p) {
                continue;
            }

            let ijk = p
                .coords
                .component_mul(&inv_leaf_size)
                .map(|v| v.floor() as i32);

            indices.push(((ijk.x, ijk.y, ijk.z), i));
        }

        indices.sort_by_key(|i| i.0);

        let mut out_pc = PC::new();
        let mut idx = 0;

        let min_poins_per_voxel = self.min_poins_per_voxel.max(1);

        while idx + min_poins_per_voxel - 1 < indices.len() {
            let cur = &indices[idx].0;
            if &indices[idx + min_poins_per_voxel - 1].0 != cur {
                while &indices[idx].0 != cur && idx < indices.len() {
                    idx += 1;
                }
            }

            let mut centroid = Position::new(0., 0., 0.);
            let mut count = 0;
            while idx < indices.len() && &indices[idx].0 == cur {
                centroid.coords += pc.positions()[indices[idx].1].coords;
                idx += 1;
                count += 1;
            }

            let mut point: PC::Point = Default::default();
            *point.position_mut() = centroid / count as f32;
            out_pc.push(point);
        }

        out_pc
    }
}
