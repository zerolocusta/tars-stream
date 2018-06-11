use std::any::{Any, TypeId};

const LIST_PREFIX: &str = "std::vec::Vec";
const MAP_PREFIX: &str = "std::collections::BTreeMap";

const SIMPLE_LIST_I8: &str = "std::vec::Vec<i8>";
const SIMPLE_LIST_U8: &str = "std::vec::Vec<u8>";
const SIMPLE_LIST_BOOL: &str = "std::vec::Vec<bool>";

pub enum TarsTypeMark {
    EnInt8 = 0,
    EnInt16 = 1,
    EnInt32 = 2,
    EnInt64 = 3,
    EnFloat = 4,
    EnDouble = 5,
    EnString1 = 6,
    EnString4 = 7,
    EnMaps = 8,
    EnList = 9,
    EnStructBegin = 10,
    EnStructEnd = 11,
    EnZero = 12,
    EnSimplelist = 13,
}

impl TarsTypeMark {
    pub fn value(self) -> u8 {
        self as u8
    }
}

pub fn is_u8<T: ?Sized + Any>() -> bool {
    TypeId::of::<T>() == TypeId::of::<u8>()
}

pub fn is_i8<T: ?Sized + Any>() -> bool {
    TypeId::of::<T>() == TypeId::of::<i8>()
}

pub fn is_u16<T: ?Sized + Any>() -> bool {
    TypeId::of::<T>() == TypeId::of::<u16>()
}

pub fn is_i16<T: ?Sized + Any>() -> bool {
    TypeId::of::<T>() == TypeId::of::<i16>()
}

pub fn is_u32<T: ?Sized + Any>() -> bool {
    TypeId::of::<T>() == TypeId::of::<u32>()
}

pub fn is_i32<T: ?Sized + Any>() -> bool {
    TypeId::of::<T>() == TypeId::of::<i32>()
}

pub fn is_u64<T: ?Sized + Any>() -> bool {
    TypeId::of::<T>() == TypeId::of::<u64>()
}

pub fn is_i64<T: ?Sized + Any>() -> bool {
    TypeId::of::<T>() == TypeId::of::<i64>()
}

pub fn is_f32<T: ?Sized + Any>() -> bool {
    TypeId::of::<T>() == TypeId::of::<f32>()
}

pub fn is_f64<T: ?Sized + Any>() -> bool {
    TypeId::of::<T>() == TypeId::of::<f64>()
}

pub fn is_bool<T: ?Sized + Any>() -> bool {
    TypeId::of::<T>() == TypeId::of::<bool>()
}

pub fn is_list<T: ?Sized + Any>() -> bool {
    let name = unsafe{ std::intrinsics::type_name::<T>() };
    name.starts_with(LIST_PREFIX)
}

pub fn is_simple_list<T: ?Sized + Any>() -> bool {
    let name = unsafe{ std::intrinsics::type_name::<T>() };
    name == SIMPLE_LIST_I8 || name == SIMPLE_LIST_U8 || name == SIMPLE_LIST_BOOL
}

pub fn is_simple_list_element<T: ?Sized + Any>() -> bool {
    is_bool::<T>() || is_u8::<T>() || is_i8::<T>()
}

pub fn is_map<T: ?Sized + Any>() -> bool {
    let name = unsafe{ std::intrinsics::type_name::<T>() };
    name.starts_with(MAP_PREFIX)
}