use errors::TarsTypeErr;
use std::collections::BTreeMap;
use tars_decoder::TarsStructDecoder;

// pub type TarsStruct = BTreeMap<u8, TarsType>;

pub type TarsMap = BTreeMap<TarsType , TarsType>;

pub type TarsSimpleList = Vec<u8>;

pub type TarsList = Vec<TarsType>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TarsType {
    EnInt8(i8),   // = 0
    EnInt16(i16), // = 1
    EnInt32(i32), // = 2
    EnInt64(i64), // = 3
    // need translate from bits f32::from_bits
    EnFloat(u32), // = 4
    // need translate from bits f64::from_bits
    EnDouble(u64),                // = 5
    EnString(String),             // = 6 || 7
    EnMaps(TarsMap),              // = 8
    EnList(TarsList),             // = 9
    EnStruct(TarsStructDecoder),         // = 10 || 11
    EnZero,                       // = 12
    EnSimplelist(TarsSimpleList), // = 13
}

impl TarsType {
    pub fn unwrap_i8(self) -> Result<i8, TarsTypeErr> {
        match self {
            TarsType::EnInt8(i) => Ok(i),
            _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
        }
    }

    pub fn unwrap_float(self) -> Result<f32, TarsTypeErr> {
        match self {
            TarsType::EnFloat(f) => Ok(f32::from_bits(f)),
            _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
        }
    }

    pub fn unwrap_struct(self) -> Result<TarsStructDecoder, TarsTypeErr> {
        match self {
            TarsType::EnStruct(s) => Ok(s),
            _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
        }
    }

    pub fn unwrap_struct_ref(&self) -> Result<&TarsStructDecoder, TarsTypeErr> {
        match &self {
            TarsType::EnStruct(s) => Ok(s),
            _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
        }
    }
}
