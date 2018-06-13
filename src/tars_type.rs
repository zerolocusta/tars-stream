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
