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

pub trait EnumMarker {}

pub trait ClassName {
    fn class_name() -> String;
}

impl ClassName for bool {
    fn class_name() -> String {
        String::from("bool")
    }
}

impl ClassName for i8 {
    fn class_name() -> String {
        String::from("char")
    }
}

impl ClassName for i16 {
    fn class_name() -> String {
        String::from("short")
    }
}

impl ClassName for i32 {
    fn class_name() -> String {
        String::from("int32")
    }
}

impl ClassName for i64 {
    fn class_name() -> String {
        String::from("int64")
    }
}

impl ClassName for u8 {
    fn class_name() -> String {
        String::from("char")
    }
}

impl ClassName for u16 {
    fn class_name() -> String {
        String::from("short")
    }
}

impl ClassName for u32 {
    fn class_name() -> String {
        String::from("int32")
    }
}

impl ClassName for f32 {
    fn class_name() -> String {
        String::from("float")
    }
}

impl ClassName for f64 {
    fn class_name() -> String {
        String::from("double")
    }
}

impl ClassName for String {
    fn class_name() -> String {
        String::from("string")
    }
}

impl<T: EnumMarker> ClassName for T {
    fn class_name() -> String {
        String::from("i32")
    } 
}