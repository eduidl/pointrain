use std::path::PathBuf;

use pointrain::{
    io::{pcd_read, ply_read},
    pc::{
        PointCloud, PointCloudIntensity, PointCloudIntensityNormal, PointCloudNormal,
        PointCloudRgb, PointCloudRgbNormal,
    },
};
use rerun::RecordingStreamBuilder;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, short = "P")]
    path: PathBuf,
    #[structopt(long, short, default_value = "xyz")]
    point: String,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let (rec, storage) = RecordingStreamBuilder::new("pointrain-test").memory()?;

    let ext = opt.path.extension().unwrap().to_str().unwrap();
    if !matches!(ext, "pcd" | "ply") {
        panic!("Unknown extension: {}", ext);
    }

    match opt.point.as_str() {
        "xyz" => {
            let pc: PointCloud = match ext {
                "pcd" => pcd_read(opt.path)?,
                "ply" => ply_read(opt.path)?,
                _ => unreachable!(),
            };
            rec.log("pointrain", &pc.rerun_points())?;
        }
        "xyz_normal" => {
            let pc: PointCloudNormal = match ext {
                "pcd" => pcd_read(opt.path)?,
                "ply" => ply_read(opt.path)?,
                _ => unreachable!(),
            };
            rec.log("pointrain", &pc.rerun_normals(None))?;
        }
        "xyzi" => {
            let pc: PointCloudIntensity = match ext {
                "pcd" => pcd_read(opt.path)?,
                "ply" => panic!("ply not supported for xyzi"),
                _ => unreachable!(),
            };
            rec.log("pointrain", &pc.rerun_points(None))?;
        }
        "xyzi_normal" => {
            let pc: PointCloudIntensityNormal = match ext {
                "pcd" => pcd_read(opt.path)?,
                "ply" => panic!("ply not supported for xyzi_normal"),
                _ => unreachable!(),
            };
            rec.log("pointrain", &pc.rerun_normals(None, None))?;
        }
        "xyzrgb" => {
            let pc: PointCloudRgb = match ext {
                "pcd" => pcd_read(opt.path)?,
                "ply" => ply_read(opt.path)?,
                _ => unreachable!(),
            };
            rec.log("pointrain", &pc.rerun_points())?;
        }
        "xyzrgb_normal" => {
            let pc: PointCloudRgbNormal = match ext {
                "pcd" => pcd_read(opt.path)?,
                "ply" => ply_read(opt.path)?,
                _ => unreachable!(),
            };
            rec.log("pointrain", &pc.rerun_normals(None))?;
        }
        v => return Err(anyhow::anyhow!("Unknown point type: {}", v)),
    }

    rec.flush_blocking();

    rerun::native_viewer::show(storage.take())?;

    Ok(())
}
