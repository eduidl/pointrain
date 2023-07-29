use std::path::PathBuf;

use pointrain::{
    io::pcd_read,
    pc::{PointCloudXYZ, PointCloudXYZI, PointCloudXYZNormal},
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

    match opt.point.as_str() {
        "xyz" => {
            let pc: PointCloudXYZ = pcd_read(opt.path)?;
            pc.rerun_msg_sender("my_points")?.send(&stream)?;
        }
        "xyz_normal" => {
            let pc: PointCloudXYZNormal = pcd_read(opt.path)?;
            pc.rerun_msg_sender("my_points", None)?.send(&stream)?;
        }
        "xyzi" => {
            let pc: PointCloudXYZI = pcd_read(opt.path)?;
            pc.rerun_msg_sender("my_points")?.send(&stream)?;
        }
        v => return Err(anyhow::anyhow!("Unknown point type: {}", v)),
    }

    stream.flush_blocking();

    rerun::native_viewer::show(storage.take())?;

    Ok(())
}
