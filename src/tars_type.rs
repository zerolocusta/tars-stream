use std::rc::Rc;
use std::marker::PhantomData;
use std::collections::BTreeMap;

pub type TarsMap = BTreeMap<TarsType, TarsType>;

pub type TarsSimpleList = Vec<u8>;

pub type TarsList = Vec<TarsType>;

#[derive(Debug, PartialEq, Clone)]
pub enum TarsType {
    EnInt8(i8),                     // = 0
    EnInt16(i16),                   // = 1
    EnInt32(i32),                   // = 2
    EnInt64(i64),                   // = 3
    EnFloat(f32),                   // = 4
    EnDouble(f64),                  // = 5
    EnString(String),               // = 6 || 7
    EnMaps(TarsMap),                // = 8
    EnList(TarsList),               // = 9
    EnStructBegin,                  // = 10
    EnStructEnd,                    // = 11
    EnZero,                         // = 12
    EnSimplelist(TarsSimpleList),   // = 13
}


