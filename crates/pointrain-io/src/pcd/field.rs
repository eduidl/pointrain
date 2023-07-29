use bytes::Buf;
use pointrain_core::types::{Float, Rgb};

use crate::error::ParseNumberError;

#[derive(Debug, Clone, Copy)]
pub enum PointFieldType {
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
    pub fn from_type_and_size(r#type: char, size: u8) -> Result<Self, String> {
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

    pub fn bytes(self) -> usize {
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
    pub fn parse(s: &str, r#type: PointFieldType) -> Result<Self, ParseNumberError> {
        use PointFieldType::*;

        Ok(match r#type {
            U8 => s.parse().map(Self::U8)?,
            U16 => s.parse().map(Self::U16)?,
            U32 => s.parse().map(Self::U32)?,
            I8 => s.parse().map(Self::I8)?,
            I16 => s.parse().map(Self::I16)?,
            I32 => s.parse().map(Self::I32)?,
            F32 => s.parse().map(Self::F32)?,
            F64 => s.parse().map(Self::F64)?,
        })
    }

    pub fn from_bytes(bytes: &mut &[u8], r#type: PointFieldType) -> Self {
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

    pub fn to_float(self) -> Float {
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

    pub fn to_color(self) -> Result<Rgb, String> {
        Ok(match self {
            Self::F32(v) => {
                let bytes = v.to_le_bytes();
                Rgb::new(bytes[0], bytes[1], bytes[2])
            }
            _ => return Err(format!("{:?} cannot be parsed as RGB", self)),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PointField {
    pub name: String,
    pub datatype: PointFieldType,
    pub count: usize,
}

impl PointField {
    pub(crate) fn bytes(&self) -> usize {
        self.count * self.datatype.bytes()
    }
}
