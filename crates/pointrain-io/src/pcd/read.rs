use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

use nalgebra::{Quaternion, Vector3};
use pointrain_core::traits::PointCloud;

use super::point::PointReadable;
use crate::{
    field::{PointField, PointFieldDatum, PointFieldType},
    PointRainIOError,
};

#[derive(Debug, Default, Clone, Copy)]
enum PcdDataFormat {
    #[default]
    Ascii,
    Binary,
    BinaryCompressed,
}

impl TryFrom<&str> for PcdDataFormat {
    type Error = String;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Ok(match from {
            "ascii" => Self::Ascii,
            "binary" => Self::Binary,
            "binary_compressed" => Self::BinaryCompressed,
            _ => {
                return Err(format!("Unknown data format: {from}"));
            }
        })
    }
}

#[derive(Debug, Default)]
struct PcdHeader {
    format: PcdDataFormat,
    fields: Vec<PointField>,
    width: usize,
    height: usize,
    origin: Vector3<f32>,
    orientation: Quaternion<f32>,
}

// https://github.com/PointCloudLibrary/pcl/blob/master/io/src/pcd_io.cpp
pub fn pcd_read<PC>(f: impl AsRef<Path>) -> Result<PC, PointRainIOError>
where
    PC: PointCloud,
    PC::Point: PointReadable,
{
    let file = File::open(f)?;

    let (header, mut reader) = pcd_read_header(&file)?;
    pcd_read_data::<PC>(&header, &mut reader)
}

fn pcd_read_header(file: &File) -> Result<(PcdHeader, BufReader<&File>), PointRainIOError> {
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut header = PcdHeader::default();
    let mut field_sizes = Vec::new();

    while reader.read_line(&mut line)? > 0 {
        if line.starts_with('#') {
            line.clear();
            continue;
        }

        let tokens: Vec<_> = line.trim().split(&[' ', '\t', '\r']).collect();
        match tokens[0] {
            "VERSION" => (),
            "FIELDS" | "COLUMNS" => {
                header.fields = tokens
                    .iter()
                    .skip(1)
                    .map(|token| PointField {
                        name: token.to_string(),
                        datatype: PointFieldType::F32,
                        count: 1,
                    })
                    .collect();
            }
            "SIZE" => {
                if header.fields.is_empty() {
                    return Err(PointRainIOError::Error {
                        msg: "[SIZE] SIZE of FIELDS specified before FIELDS in header!".into(),
                    });
                }

                if tokens.len().saturating_sub(1) != header.fields.len() {
                    return Err(PointRainIOError::Error{
                       msg: "[SIZE] The number of elements in <SIZE> differs than the number of elements in <FIELDS>!".into(),
                    });
                }

                field_sizes = tokens
                    .iter()
                    .skip(1)
                    .map(|token| token.parse())
                    .collect::<Result<Vec<_>, _>>()?;
            }
            "TYPE" => {
                if field_sizes.is_empty() {
                    return Err(PointRainIOError::Error {
                        msg: "[TYPE] TYPE of FIELDS specified before SIZE in header!".into(),
                    });
                }

                if tokens.len().saturating_sub(1) != header.fields.len() {
                    return Err(PointRainIOError::Error {
                        msg: "[TYPE] The number of elements in <TYPE> differs than the number of elements in <FIELDS>!".into(),
                    });
                }

                for ((token, size), field) in tokens
                    .iter()
                    .skip(1)
                    .zip(field_sizes.iter())
                    .zip(header.fields.iter_mut())
                {
                    let type_ = token.chars().next().unwrap();
                    field.datatype = PointFieldType::from_pcd_type_and_size(type_, *size)?;
                }
            }
            "COUNT" => {
                if header.fields.is_empty() {
                    return Err(PointRainIOError::Error {
                        msg: "[COUNT] COUNT of FIELDS specified before FIELDS in header!".into(),
                    });
                }

                if tokens.len().saturating_sub(1) != header.fields.len() {
                    return Err(PointRainIOError::Error{
                        msg: "[COUNT] The number of elements in <TYPE> differs than the number of elements in <FIELDS>!".into(),
                    });
                }

                for (token, field) in tokens.iter().skip(1).zip(header.fields.iter_mut()) {
                    field.count = token.parse()?;

                    if field.count == 0 {
                        return Err(PointRainIOError::Error {
                            msg: "[COUNT] Invalid COUNT value specified.".into(),
                        });
                    }
                }
            }
            "WIDTH" => {
                header.width = tokens[1].parse()?;
            }
            "HEIGHT" => {
                header.height = tokens[1].parse()?;
            }
            "VIEWPOINT" => {
                if tokens.len() < 8 {
                    return Err(PointRainIOError::Error{
                        msg: "[VIEWPOINT] Not enough number of elements in <VIEWPOINT>! Need 7 values (tx ty tz qw qx qy qz).".into(),
                    });
                }

                let vp = tokens
                    .iter()
                    .skip(1)
                    .map(|token| token.parse())
                    .collect::<Result<Vec<_>, _>>()?;

                header.origin = Vector3::new(vp[0], vp[1], vp[2]);
                header.orientation = Quaternion::new(vp[3], vp[4], vp[5], vp[6]);
            }
            "POINTS" => {
                let size = tokens[1].parse()?;

                if header.width == 0 && header.height == 0 {
                    header.width = size;
                    header.height = 1;
                }

                if size != header.width * header.height {
                    return Err(PointRainIOError::Error {
                        msg: format!(
                            "[POINTS] HEIGHT ({}) x WIDTH ({}) != number of points ({size})",
                            header.height, header.width
                        )
                        .into(),
                    });
                }
            }
            "DATA" => {
                header.format = tokens[1].try_into()?;
                break;
            }
            _ => {
                return Err(PointRainIOError::Error {
                    msg: format!("Unknown token: {}", tokens[0]).into(),
                });
            }
        }

        line.clear();
    }

    if header.width == 0 {
        return Err(PointRainIOError::Error {
            msg: "WIDTH is not given!".into(),
        });
    }

    if header.height == 0 {
        header.height = 1;
    }

    Ok((header, reader))
}

fn pcd_read_data<PC>(
    header: &PcdHeader,
    reader: &mut BufReader<&File>,
) -> Result<PC, PointRainIOError>
where
    PC: PointCloud,
    PC::Point: PointReadable,
{
    let mut pc = PC::with_capacity(header.width * header.height);
    let func = PC::Point::read_data_func(&header.fields)?;

    match header.format {
        PcdDataFormat::Ascii => {
            for line in reader.lines() {
                let data = pcd_read_ascii_datum(header, line?.as_str())?;
                pc.add_point(func(&data)?);
            }
        }
        PcdDataFormat::Binary => {
            let chunk_size = header.fields.iter().map(PointField::bytes).sum();
            let mut chunk = vec![0; chunk_size];
            while reader.read_exact(&mut chunk).is_ok() {
                let data = pcd_read_binary_datum(header, &mut chunk.as_slice());
                pc.add_point(func(&data)?);
            }
            if !reader.fill_buf()?.is_empty() {
                return Err(PointRainIOError::Error {
                    msg: "extra data remains".into(),
                });
            }
        }
        PcdDataFormat::BinaryCompressed => {
            unimplemented!("binary compressed format is not supported");
        }
    }

    if pc.len() != header.width * header.height {
        return Err(PointRainIOError::Error {
            msg: format!(
                "The number of points ({}) does not match the number of points specified in the header ({} x {} = {})",
                pc.len(),
                header.width,
                header.height,
                header.width * header.height
            )
            .into(),
        });
    }

    Ok(pc)
}

fn pcd_read_ascii_datum(
    header: &PcdHeader,
    line: &str,
) -> Result<Vec<PointFieldDatum>, PointRainIOError> {
    let tokens: Vec<_> = line.trim().split(&[' ', '\t', '\r']).collect();

    if tokens.len() != header.fields.len() {
        return Err(PointRainIOError::Error {
            msg: format!(
                "Invalid number of tokens: expected {}, got {}",
                header.fields.len(),
                tokens.len()
            )
            .into(),
        });
    }

    Ok(tokens
        .iter()
        .zip(header.fields.iter())
        .map(|(token, field)| PointFieldDatum::parse(token, field.datatype))
        .collect::<Result<_, _>>()?)
}

fn pcd_read_binary_datum(header: &PcdHeader, chunk: &mut &[u8]) -> Vec<PointFieldDatum> {
    header
        .fields
        .iter()
        .map(|field| PointFieldDatum::from_bytes_le(chunk, field.datatype))
        .collect()
}
