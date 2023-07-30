use bytes::Buf;
use pointrain_core::types::{Float, Rgb};

use crate::error::ParseNumberError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PointFieldType {
    U8,
    U16,
    U32,
    I8,
    I16,
    I32,
    F32,
    F64,
}

impl PointFieldType {
    pub fn from_pcd_type_and_size(r#type: char, size: u8) -> Result<Self, String> {
        Ok(match (r#type, size) {
            ('U', 1) => Self::U8,
            ('U', 2) => Self::U16,
            ('U', 4) => Self::U32,
            ('I', 1) => Self::I8,
            ('I', 2) => Self::I16,
            ('I', 4) => Self::I32,
            ('F', 4) => Self::F32,
            ('F', 8) => Self::F64,
            _ => {
                return Err(format!(
                    "Invalid pair of type and size: {} (type) {} (size)",
                    r#type, size
                ));
            }
        })
    }

    pub fn from_ply_type(r#type: &str) -> Result<Self, String> {
        Ok(match r#type {
            "char" => Self::I8,
            "uchar" => Self::U8,
            "short" => Self::I16,
            "ushort" => Self::U16,
            "int" => Self::I32,
            "uint" => Self::U32,
            "float" => Self::F32,
            "double" => Self::F64,
            _ => {
                return Err(format!("Unknown point field type: {}", r#type));
            }
        })
    }

    fn bytes(self) -> usize {
        match self {
            Self::U8 => 1,
            Self::U16 => 2,
            Self::U32 => 4,
            Self::I8 => 1,
            Self::I16 => 2,
            Self::I32 => 4,
            Self::F32 => 4,
            Self::F64 => 8,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PointField {
    pub(crate) name: String,
    pub(crate) datatype: PointFieldType,
    pub(crate) count: usize,
}

impl PointField {
    pub(crate) fn bytes(&self) -> usize {
        self.count * self.datatype.bytes()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PointFieldDatum {
    U8(u8),
    U16(u16),
    U32(u32),
    I8(i8),
    I16(i16),
    I32(i32),
    F32(f32),
    F64(f64),
}

impl PointFieldDatum {
    pub(crate) fn parse(s: &str, r#type: PointFieldType) -> Result<Self, ParseNumberError> {
        use PointFieldType::*;

        Ok(match r#type {
            U8 => Self::U8(s.parse()?),
            U16 => Self::U16(s.parse()?),
            U32 => Self::U32(s.parse()?),
            I8 => Self::I8(s.parse()?),
            I16 => Self::I16(s.parse()?),
            I32 => Self::I32(s.parse()?),
            F32 => Self::F32(s.parse()?),
            F64 => Self::F64(s.parse()?),
        })
    }

    pub(crate) fn from_bytes_le(bytes: &mut &[u8], r#type: PointFieldType) -> Self {
        use PointFieldType::*;

        match r#type {
            U8 => Self::U8(bytes.get_u8()),
            U16 => Self::U16(bytes.get_u16_le()),
            U32 => Self::U32(bytes.get_u32_le()),
            I8 => Self::I8(bytes.get_i8()),
            I16 => Self::I16(bytes.get_i16_le()),
            I32 => Self::I32(bytes.get_i32_le()),
            F32 => Self::F32(bytes.get_f32_le()),
            F64 => Self::F64(bytes.get_f64_le()),
        }
    }

    pub(crate) fn from_bytes_be(bytes: &mut &[u8], r#type: PointFieldType) -> Self {
        use PointFieldType::*;

        match r#type {
            U8 => Self::U8(bytes.get_u8()),
            U16 => Self::U16(bytes.get_u16()),
            U32 => Self::U32(bytes.get_u32()),
            I8 => Self::I8(bytes.get_i8()),
            I16 => Self::I16(bytes.get_i16()),
            I32 => Self::I32(bytes.get_i32()),
            F32 => Self::F32(bytes.get_f32()),
            F64 => Self::F64(bytes.get_f64()),
        }
    }

    pub(crate) fn to_float(self) -> Float {
        match self {
            Self::U8(v) => v.into(),
            Self::U16(v) => v.into(),
            Self::U32(v) => f64::from(v) as f32,
            Self::I8(v) => v.into(),
            Self::I16(v) => v.into(),
            Self::I32(v) => f64::from(v) as f32,
            Self::F32(v) => v,
            Self::F64(v) => v as f32,
        }
    }

    pub(crate) fn as_u8(self) -> Result<u8, String> {
        match self {
            Self::U8(v) => Ok(v),
            _ => Err(format!("{:?} cannot be parsed as u8", self)),
        }
    }

    pub(crate) fn to_color(self) -> Result<Rgb, String> {
        Ok(match self {
            Self::F32(v) => {
                let bytes = v.to_le_bytes();
                Rgb::new(bytes[0], bytes[1], bytes[2])
            }
            _ => return Err(format!("{:?} cannot be parsed as RGB", self)),
        })
    }
}
