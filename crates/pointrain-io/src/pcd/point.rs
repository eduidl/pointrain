use pointrain_core::{
    pc::point::{
        PointXYZ, PointXYZI, PointXYZINormal, PointXYZNormal, PointXYZRgb, PointXYZRgbNormal,
    },
    types::{Normal, Position},
};

use super::field::{PointField, PointFieldDatum};
use crate::error::{MissingField, PointRainIOError};

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
        find_field(fields, "normal_x")?,
        find_field(fields, "normal_y")?,
        find_field(fields, "normal_z")?,
        find_field(fields, "curvature")?,
    ))
}

fn find_intensity(fields: &[PointField]) -> Result<usize, MissingField> {
    find_field(fields, "intensity")
}

fn find_rgb(fields: &[PointField]) -> Result<usize, MissingField> {
    find_field(fields, "rgb")
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

impl PointReadable for PointXYZI {
    fn read_data_func(fields: &[PointField]) -> Result<PointMapper<Self>, PointRainIOError> {
        let (x, y, z) = find_xyz(fields)?;
        let intensity = find_intensity(fields)?;

        let closure = move |data: &[PointFieldDatum]| {
            Ok(Self {
                pos: Position::new(data[x].to_float(), data[y].to_float(), data[z].to_float()),
                intensity: data[intensity].to_float(),
            })
        };

        Ok(Box::new(closure))
    }
}

impl PointReadable for PointXYZINormal {
    fn read_data_func(fields: &[PointField]) -> Result<PointMapper<Self>, PointRainIOError> {
        let (x, y, z) = find_xyz(fields)?;
        let intensity = find_intensity(fields)?;
        let (nx, ny, nz, curvature) = find_normal(fields)?;

        let closure = move |data: &[PointFieldDatum]| {
            Ok(Self {
                pos: Position::new(data[x].to_float(), data[y].to_float(), data[z].to_float()),
                intensity: data[intensity].to_float(),
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
        let rgb = find_rgb(fields)?;

        let closure = move |data: &[PointFieldDatum]| {
            Ok(Self {
                pos: Position::new(data[x].to_float(), data[y].to_float(), data[z].to_float()),
                color: data[rgb].to_color()?,
            })
        };

        Ok(Box::new(closure))
    }
}

impl PointReadable for PointXYZRgbNormal {
    fn read_data_func(fields: &[PointField]) -> Result<PointMapper<Self>, PointRainIOError> {
        let (x, y, z) = find_xyz(fields)?;
        let rgb = find_rgb(fields)?;
        let (nx, ny, nz, curvature) = find_normal(fields)?;

        let closure = move |data: &[PointFieldDatum]| {
            Ok(Self {
                pos: Position::new(data[x].to_float(), data[y].to_float(), data[z].to_float()),
                color: data[rgb].to_color()?,
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
