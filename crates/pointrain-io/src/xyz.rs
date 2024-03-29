use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use pointrain_core::{
    pc::{PointCloud, PointCloudBase},
    point::Point,
    types::Position,
};

use crate::error::PointRainIOError;

pub fn xyz_read(f: impl AsRef<Path>) -> Result<PointCloud, PointRainIOError> {
    let file = File::open(f)?;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut pc = PointCloud::new();

    while reader.read_line(&mut line)? > 0 {
        if line.starts_with('#') {
            line.clear();
            continue;
        }

        let tokens: Vec<_> = line.trim().split(&[' ', '\t', '\r']).collect();
        if tokens.len() != 3 {
            return Err(PointRainIOError::Error {
                msg: "Invalid number of tokens in line".into(),
            });
        }

        let x: f32 = tokens[0].parse()?;
        let y: f32 = tokens[1].parse()?;
        let z: f32 = tokens[2].parse()?;

        pc.push(Point {
            position: Position::new(x, y, z),
        });

        line.clear();
    }

    Ok(pc)
}
