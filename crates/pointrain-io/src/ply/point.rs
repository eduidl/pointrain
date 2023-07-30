use pointrain_core::{
    pc::point::{PointXYZ, PointXYZNormal, PointXYZRgb, PointXYZRgbNormal},
    types::{Normal, Position, Rgb},
};

use crate::{
    error::{MissingField, PointRainIOError},
    field::{PointField, PointFieldDatum},
};

fn find_field(fields: &[PointField], name: &'static str) -> Result<usize, MissingField> {
    fields
        .iter()
        .enumerate()
        .find(|(_, f)| f.name == name)
        .map(|(i, _)| i)
        .ok_or(MissingField(name))
}

fn find_xyz(fields: &[PointField]) -> Result<(usize, usize, usize), MissingField> {
    Ok((
        find_field(fields, "x")?,
        find_field(fields, "y")?,
        find_field(fields, "z")?,
    ))
}

fn find_normal(fields: &[PointField]) -> Result<(usize, usize, usize, usize), MissingField> {
    Ok((
        find_field(fields, "nx")?,
        find_field(fields, "ny")?,
        find_field(fields, "nz")?,
        find_field(fields, "curvature")?,
    ))
}

fn find_rgb(fields: &[PointField]) -> Result<(usize, usize, usize), MissingField> {
    Ok((
        find_field(fields, "red")?,
        find_field(fields, "green")?,
        find_field(fields, "blue")?,
    ))
}

pub type PointMapper<T> = Box<dyn Fn(&[PointFieldDatum]) -> Result<T, PointRainIOError>>;

pub trait PointReadable: Sized {
    fn read_data_func(fields: &[PointField]) -> Result<PointMapper<Self>, PointRainIOError>;
}

impl PointReadable for PointXYZ {
    fn read_data_func(fields: &[PointField]) -> Result<PointMapper<Self>, PointRainIOError> {
        let (x, y, z) = find_xyz(fields)?;

        let closure = move |data: &[PointFieldDatum]| {
            Ok(Self {
                pos: Position::new(data[x].to_float(), data[y].to_float(), data[z].to_float()),
            })
        };

        Ok(Box::new(closure))
    }
}

impl PointReadable for PointXYZNormal {
    fn read_data_func(fields: &[PointField]) -> Result<PointMapper<Self>, PointRainIOError> {
        let (x, y, z) = find_xyz(fields)?;
        let (nx, ny, nz, curvature) = find_normal(fields)?;

        let closure = move |data: &[PointFieldDatum]| {
            Ok(Self {
                pos: Position::new(data[x].to_float(), data[y].to_float(), data[z].to_float()),
                normal: Normal::new(
                    data[nx].to_float(),
                    data[ny].to_float(),
                    data[nz].to_float(),
                ),
                curvature: data[curvature].to_float(),
            })
        };

        Ok(Box::new(closure))
    }
}

impl PointReadable for PointXYZRgb {
    fn read_data_func(fields: &[PointField]) -> Result<PointMapper<Self>, PointRainIOError> {
        let (x, y, z) = find_xyz(fields)?;
        let (r, g, b) = find_rgb(fields)?;

        let closure = move |data: &[PointFieldDatum]| {
            Ok(Self {
                pos: Position::new(data[x].to_float(), data[y].to_float(), data[z].to_float()),
                color: Rgb::new(data[r].as_u8()?, data[g].as_u8()?, data[b].as_u8()?),
            })
        };

        Ok(Box::new(closure))
    }
}

impl PointReadable for PointXYZRgbNormal {
    fn read_data_func(fields: &[PointField]) -> Result<PointMapper<Self>, PointRainIOError> {
        let (x, y, z) = find_xyz(fields)?;
        let (r, g, b) = find_rgb(fields)?;
        let (nx, ny, nz, curvature) = find_normal(fields)?;

        let closure = move |data: &[PointFieldDatum]| {
            Ok(Self {
                pos: Position::new(data[x].to_float(), data[y].to_float(), data[z].to_float()),
                color: Rgb::new(data[r].as_u8()?, data[g].as_u8()?, data[b].as_u8()?),
                normal: Normal::new(
                    data[nx].to_float(),
                    data[ny].to_float(),
                    data[nz].to_float(),
                ),
                curvature: data[curvature].to_float(),
            })
        };

        Ok(Box::new(closure))
    }
}
