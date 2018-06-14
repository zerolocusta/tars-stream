use bytes::Bytes;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
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

impl From<u8> for TarsTypeMark {
    fn from(v: u8) -> Self {
        match v {
            0 => TarsTypeMark::EnInt8,
            1 => TarsTypeMark::EnInt16,
            2 => TarsTypeMark::EnInt32,
            3 => TarsTypeMark::EnInt64,
            4 => TarsTypeMark::EnFloat,
            5 => TarsTypeMark::EnDouble,
            6 => TarsTypeMark::EnString1,
            7 => TarsTypeMark::EnString4,
            8 => TarsTypeMark::EnMaps,
            9 => TarsTypeMark::EnList,
            10 => TarsTypeMark::EnStructBegin,
            11 => TarsTypeMark::EnStructEnd,
            12 => TarsTypeMark::EnZero,
            13 => TarsTypeMark::EnSimplelist,
            _ => TarsTypeMark::EnZero, // unknown type, read nothing from buffer
        }
    }
}

pub enum ProtocolVersion {
    Tars = 1,
    TupSimple = 2,
    TupComplex = 3,
}

impl ProtocolVersion {
    pub fn value(self) -> u8 {
        self as u8
    }
}

impl From<u8> for ProtocolVersion {
    fn from(v: u8) -> Self {
        if v == 1 {
            ProtocolVersion::Tars
        } else if v == 2 {
            ProtocolVersion::TupSimple
        } else {
            ProtocolVersion::TupComplex
        }
    }
}

// for tup encoding/decoding
pub trait ClassName {
    fn _class_name() -> String;
    fn _type_name() -> &'static str;
}

impl ClassName for bool {
    fn _class_name() -> String {
        String::from("bool")
    }
    fn _type_name() -> &'static str {
        "bool"
    }
}

impl ClassName for i8 {
    fn _class_name() -> String {
        String::from("char")
    }
    fn _type_name() -> &'static str {
        "char"
    }
}

impl ClassName for i16 {
    fn _class_name() -> String {
        String::from("short")
    }
    fn _type_name() -> &'static str {
        "short"
    }
}

impl ClassName for i32 {
    fn _class_name() -> String {
        String::from("int32")
    }
    fn _type_name() -> &'static str {
        "int32"
    }
}

impl ClassName for i64 {
    fn _class_name() -> String {
        String::from("int64")
    }
    fn _type_name() -> &'static str {
        "int64"
    }
}

impl ClassName for u8 {
    fn _class_name() -> String {
        String::from("short")
    }
    fn _type_name() -> &'static str {
        "short"
    }
}

impl ClassName for u16 {
    fn _class_name() -> String {
        String::from("int32")
    }
    fn _type_name() -> &'static str {
        "int32"
    }
}

impl ClassName for u32 {
    fn _class_name() -> String {
        String::from("int64")
    }
    fn _type_name() -> &'static str {
        "int64"
    }
}

impl ClassName for f32 {
    fn _class_name() -> String {
        String::from("float")
    }
    fn _type_name() -> &'static str {
        "float"
    }
}

impl ClassName for f64 {
    fn _class_name() -> String {
        String::from("double")
    }
    fn _type_name() -> &'static str {
        "double"
    }
}

impl ClassName for String {
    fn _class_name() -> String {
        String::from("string")
    }
    fn _type_name() -> &'static str {
        "string"
    }
}

impl<K, V> ClassName for BTreeMap<K, V>
where
    K: ClassName + Ord,
    V: ClassName,
{
    fn _class_name() -> String {
        String::from("map<")
            + &K::_class_name()
            + &String::from(",")
            + &V::_class_name()
            + &String::from(">")
    }
    fn _type_name() -> &'static str {
        "map"
    }
}

impl<T> ClassName for Vec<T>
where
    T: ClassName,
{
    fn _class_name() -> String {
        String::from("list<") + &T::_class_name() + &String::from(">")
    }
    fn _type_name() -> &'static str {
        "list"
    }
}

impl ClassName for Bytes {
    fn _class_name() -> String {
        String::from("list<byte>")
    }
    fn _type_name() -> &'static str {
        "list"
    }
}
