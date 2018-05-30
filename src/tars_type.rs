use std::rc::Rc;
use std::marker::PhantomData;
use std::collections::BTreeMap;

pub type TarsStruct = BTreeMap<u8, TarsType>;

pub type TarsMap = BTreeMap<TarsType, TarsType>;

pub type TarsSimpleList = Vec<u8>;

pub type TarsList = Vec<TarsType>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TarsType {
    EnInt8(i8),                     // = 0
    EnInt16(i16),                   // = 1
    EnInt32(i32),                   // = 2
    EnInt64(i64),                   // = 3
    // need translate from bits f32::from_bits
    EnFloat(u32),                   // = 4
    // need translate from bits f64::from_bits
    EnDouble(u64),                  // = 5
    EnString(String),               // = 6 || 7
    EnMaps(TarsMap),                // = 8
    EnList(TarsList),               // = 9
    EnStruct(TarsStruct),           // = 10 || 11
    EnZero,                         // = 12
    EnSimplelist(TarsSimpleList),   // = 13
}


