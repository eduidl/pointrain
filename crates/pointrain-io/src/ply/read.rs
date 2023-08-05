use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

use pointrain_core::pc::PointCloudBase;

use super::point::PointReadable;
use crate::{
    field::{PointField, PointFieldDatum, PointFieldType},
    PointRainIOError,
};

#[derive(Debug, Default, Clone, Copy)]
enum PlyDataFormat {
    #[default]
    Ascii,
    BinaryLE,
    BinaryBE,
}

impl TryFrom<&str> for PlyDataFormat {
    type Error = String;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Ok(match from {
            "ascii" => Self::Ascii,
            "binary_little_endian" => Self::BinaryLE,
            "binary_big_endian" => Self::BinaryBE,
            _ => {
                return Err(format!("Unknown data format: {from}"));
            }
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct PlyHeader {
    format: PlyDataFormat,
    vertices_size: usize,
    vertices: Vec<PointField>,
}

pub fn ply_read<PC>(f: impl AsRef<Path>) -> Result<PC, PointRainIOError>
where
    PC: PointCloudBase,
    PC::Point: PointReadable,
{
    let file = File::open(f)?;

    let (mut reader, header) = ply_read_header(&file)?;
    ply_read_data::<PC>(&mut reader, &header)
}

fn ply_read_header(file: &File) -> Result<(BufReader<&File>, PlyHeader), PointRainIOError> {
    let mut reader = BufReader::new(file);
    let mut header = PlyHeader::default();
    let mut line = String::new();
    let mut current_element: Option<String> = None;

    while reader.read_line(&mut line)? > 0 {
        let tokens: Vec<_> = line.trim().split(&[' ', '\t', '\r']).collect();
        match tokens[0] {
            "ply" | "comment" => {}
            "format" => {
                header.format = PlyDataFormat::try_from(tokens[1])?;
            }
            "element" => {
                if tokens[1] == "vertex" {
                    header.vertices_size = tokens[2].parse()?;
                }
                current_element = Some(tokens[1].into());
            }
            "property" => {
                if current_element == Some("vertex".into()) {
                    let field = PointField {
                        name: tokens[2].into(),
                        datatype: PointFieldType::from_ply_type(tokens[1])?,
                        count: 1,
                    };
                    header.vertices.push(field);
                }
            }
            "end_header" => break,
            _ => {
                return Err(PointRainIOError::Error {
                    msg: format!("Unknown token: {}", tokens[0]).into(),
                });
            }
        }

        line.clear();
    }

    Ok((reader, header))
}

fn ply_read_data<PC>(
    reader: &mut BufReader<&File>,
    header: &PlyHeader,
) -> Result<PC, PointRainIOError>
where
    PC: PointCloudBase,
    PC::Point: PointReadable,
{
    let mut pc = PC::with_capacity(header.vertices_size);
    let func = PC::Point::read_data_func(&header.vertices)?;

    match header.format {
        PlyDataFormat::Ascii => {
            for line in reader.lines().take(header.vertices_size) {
                let data = ply_read_ascii_datum(header, line?.as_str())?;
                pc.push(func(&data)?);
            }
        }
        PlyDataFormat::BinaryLE | PlyDataFormat::BinaryBE => {
            let chunk_size = header.vertices.iter().map(PointField::bytes).sum();
            let mut chunk = vec![0; chunk_size];
            for _ in 0..header.vertices_size {
                reader.read_exact(&mut chunk)?;
                let data = pcd_read_binary_datum(header, &mut chunk.as_slice());
                pc.push(func(&data)?);
            }
        }
    }

    Ok(pc)
}

fn ply_read_ascii_datum(
    header: &PlyHeader,
    line: &str,
) -> Result<Vec<PointFieldDatum>, PointRainIOError> {
    let tokens: Vec<_> = line.trim().split(&[' ', '\t', '\r']).collect();

    if tokens.len() != header.vertices.len() {
        return Err(PointRainIOError::Error {
            msg: format!(
                "Invalid number of tokens: expected {}, got {}",
                header.vertices.len(),
                tokens.len()
            )
            .into(),
        });
    }

    Ok(tokens
        .iter()
        .zip(header.vertices.iter())
        .map(|(token, field)| PointFieldDatum::parse(token, field.datatype))
        .collect::<Result<_, _>>()?)
}

fn pcd_read_binary_datum(header: &PlyHeader, chunk: &mut &[u8]) -> Vec<PointFieldDatum> {
    let func = match header.format {
        PlyDataFormat::BinaryLE => PointFieldDatum::from_bytes_le,
        PlyDataFormat::BinaryBE => PointFieldDatum::from_bytes_be,
        _ => unreachable!(),
    };

    header
        .vertices
        .iter()
        .map(|field| func(chunk, field.datatype))
        .collect()
}
