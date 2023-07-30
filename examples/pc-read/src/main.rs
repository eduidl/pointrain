use std::path::PathBuf;

use pointrain::{
    io::{pcd_read, ply_read},
    pc::{PointCloudXYZ, PointCloudXYZI, PointCloudXYZNormal, PointCloudXYZRgb},
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

    let (stream, storage) = RecordingStreamBuilder::new("pointrain-test").memory()?;
    let ext = opt.path.extension().unwrap().to_str().unwrap();

    match opt.point.as_str() {
        "xyz" => {
            let pc: PointCloudXYZ = match ext {
                "pcd" => pcd_read(opt.path)?,
                "ply" => ply_read(opt.path)?,
                _ => panic!("Unknown extension: {}", ext),
            };
            pc.rerun_msg_sender("my_points")?.send(&stream)?;
        }
        "xyz_normal" => {
            let pc: PointCloudXYZNormal = match ext {
                "pcd" => pcd_read(opt.path)?,
                "ply" => ply_read(opt.path)?,
                _ => panic!("Unknown extension: {}", ext),
            };
            pc.rerun_msg_sender("my_points", None)?.send(&stream)?;
        }
        "xyzi" => {
            let pc: PointCloudXYZI = match ext {
                "pcd" => pcd_read(opt.path)?,
                "ply" => panic!("ply not supported for xyzi"),
                _ => panic!("Unknown extension: {}", ext),
            };
            pc.rerun_msg_sender("my_points")?.send(&stream)?;
        }
        "xyzrgb" => {
            let pc: PointCloudXYZRgb = match ext {
                "pcd" => pcd_read(opt.path)?,
                "ply" => ply_read(opt.path)?,
                _ => panic!("Unknown extension: {}", ext),
            };
            pc.rerun_msg_sender("my_points")?.send(&stream)?;
        }
        v => return Err(anyhow::anyhow!("Unknown point type: {}", v)),
    }

    stream.flush_blocking();

    rerun::native_viewer::show(storage.take())?;

    Ok(())
}
