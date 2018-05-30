use std::collections::BTreeMap;

pub type TarsMap<'a> = BTreeMap<TarsType<'a>, TarsType<'a>>;

pub type TarsSimpleList = Vec<u8>;

pub type TarsList<'a> = Vec<TarsType<'a>>;

#[derive(Debug, PartialEq)]
pub enum TarsType<'a> {
    EnInt8(i8),
    EnInt16(i16),
    EnInt32(i32),
    EnInt64(i64),
    EnFloat(f32),
    EnDouble(f64),
    EnString1(&'a str),
    EnString4(&'a str),
    EnMaps(TarsMap<'a>),
    EnList(TarsList<'a>),
    EnStructBegin,
    EnStructEnd,
    EnZero,
    EnSimplelist(TarsSimpleList),
}


