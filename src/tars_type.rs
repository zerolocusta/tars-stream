use bytes::{Buf, Bytes};
use errors::TarsTypeErr;
use std::collections::BTreeMap;
use std::io::Cursor;

use tars_decoder::TarsStructDecoder;

pub trait DecodeFrom {
    fn decode_from_bytes(&Bytes) -> Self;
}

impl DecodeFrom for i8 {
    fn decode_from_bytes(b: &Bytes) -> Self {
        let mut cur = Cursor::new(b);
        cur.get_i8()
    }
}

impl DecodeFrom for u8 {
    fn decode_from_bytes(b: &Bytes) -> Self {
        let mut cur = Cursor::new(b);
        cur.get_u8()
    }
}

impl DecodeFrom for bool {
    fn decode_from_bytes(b: &Bytes) -> Self {
        let v = u8::decode_from_bytes(b);
        v != 0
    }
}

impl DecodeFrom for i16 {
    fn decode_from_bytes(b: &Bytes) -> Self {
        let mut cur = Cursor::new(b);
        if b.len() == 1 {
            i16::from(cur.get_i8())
        } else {
            cur.get_i16_be()
        }
    }
}

impl DecodeFrom for u16 {
    fn decode_from_bytes(b: &Bytes) -> Self {
        let mut cur = Cursor::new(b);
        if b.len() == 1 {
            u16::from(cur.get_u8())
        } else {
            cur.get_u16_be()
        }
    }
}

impl DecodeFrom for i32 {
    fn decode_from_bytes(b: &Bytes) -> Self {
        let mut cur = Cursor::new(b);
        if b.len() == 1 {
            i32::from(cur.get_i8())
        } else if b.len() == 2 {
            i32::from(cur.get_i16_be())
        } else {
            cur.get_i32_be()
        }
    }
}

impl DecodeFrom for u32 {
    fn decode_from_bytes(b: &Bytes) -> Self {
        let mut cur = Cursor::new(b);
        if b.len() == 1 {
            u32::from(cur.get_u8())
        } else if b.len() == 2 {
            u32::from(cur.get_u16_be())
        } else {
            cur.get_u32_be()
        }
    }
}

impl DecodeFrom for i64 {
    fn decode_from_bytes(b: &Bytes) -> Self {
        let mut cur = Cursor::new(b);
        if b.len() == 1 {
            i64::from(cur.get_i8())
        } else if b.len() == 2 {
            i64::from(cur.get_i16_be())
        } else if b.len() == 4 {
            i64::from(cur.get_i32_be())
        } else {
            cur.get_i64_be()
        }
    }
}

impl DecodeFrom for u64 {
    fn decode_from_bytes(b: &Bytes) -> Self {
        let mut cur = Cursor::new(b);
        if b.len() == 1 {
            u64::from(cur.get_u8())
        } else if b.len() == 2 {
            u64::from(cur.get_u16_be())
        } else if b.len() == 4 {
            u64::from(cur.get_u32_be())
        } else {
            cur.get_u64_be()
        }
    }
}

impl DecodeFrom for f32 {
    fn decode_from_bytes(b: &Bytes) -> Self {
        let mut cur = Cursor::new(b);
        cur.get_f32_be()
    }
}

impl DecodeFrom for f64 {
    fn decode_from_bytes(b: &Bytes) -> Self {
        let mut cur = Cursor::new(b);
        cur.get_f64_be()
    }
}

impl DecodeFrom for String {
    fn decode_from_bytes(b: &Bytes) -> Self {
        String::from_utf8(b.to_vec()).unwrap()
    }
}

impl<K: DecodeFrom + Ord, V: DecodeFrom> DecodeFrom for BTreeMap<K, V> {
    fn decode_from_bytes(b: &Bytes) -> Self {
        let mut map = BTreeMap::new();
        let mut decoder = TarsStructDecoder::new(&b);
        while decoder.has_remaining() {
            let key_head = decoder.take_head().unwrap();
            let key = decoder.read::<K>(key_head.tars_type).unwrap();
            let value_head = decoder.take_head().unwrap();
            let value = decoder.read::<V>(value_head.tars_type).unwrap();
            map.insert(key, value);
        }
        map
    }
}

// impl<T: DecodeFrom> DecodeFrom for Vec<T> {
//     fn decode_from_bytes(b: &Bytes) -> Self {
//         // String::from_utf8(b.to_vec()).unwrap()
//     }
// }

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

// pub type TarsStruct = BTreeMap<u8, TarsType>;

// pub type TarsMap = BTreeMap<TarsType, TarsType>;

// pub type TarsList = Vec<TarsType>;

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub enum TarsType {
//     EnInt8(i8),   // = 0
//     EnInt16(i16), // = 1
//     EnInt32(i32), // = 2
//     EnInt64(i64), // = 3
//     // need translate from bits f32::from_bits
//     EnFloat(u32), // = 4
//     // need translate from bits f64::from_bits
//     EnDouble(u64),    // = 5
//     EnString(String), // = 6 || 7
//     EnMaps(TarsMap),  // = 8
//     EnList(TarsList), // = 9
//     EnStruct(Bytes),  // = 10 || 11
//                       // EnZero,                       // = 12
//                       // EnSimplelist, // = 13
// }

// impl TarsType {
//     pub fn unwrap_i8(self) -> Result<i8, TarsTypeErr> {
//         match self {
//             TarsType::EnInt8(i) => Ok(i),
//             _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
//         }
//     }

//     pub fn unwrap_u8(self) -> Result<u8, TarsTypeErr> {
//         match self {
//             TarsType::EnInt8(i) => Ok(i as u8),
//             _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
//         }
//     }

//     pub fn unwrap_i16(self) -> Result<i16, TarsTypeErr> {
//         match self {
//             TarsType::EnInt8(i) => Ok(i as i16),
//             TarsType::EnInt16(i) => Ok(i),
//             _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
//         }
//     }

//     pub fn unwrap_u16(self) -> Result<u16, TarsTypeErr> {
//         match self {
//             TarsType::EnInt8(i) => Ok(i as u16),
//             TarsType::EnInt16(i) => Ok(i as u16),
//             _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
//         }
//     }

//     pub fn unwrap_i32(self) -> Result<i32, TarsTypeErr> {
//         match self {
//             TarsType::EnInt8(i) => Ok(i as i32),
//             TarsType::EnInt16(i) => Ok(i as i32),
//             TarsType::EnInt32(i) => Ok(i),
//             _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
//         }
//     }

//     pub fn unwrap_u32(self) -> Result<u32, TarsTypeErr> {
//         match self {
//             TarsType::EnInt8(i) => Ok(i as u32),
//             TarsType::EnInt16(i) => Ok(i as u32),
//             TarsType::EnInt32(i) => Ok(i as u32),
//             _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
//         }
//     }

//     pub fn unwrap_i64(self) -> Result<i64, TarsTypeErr> {
//         match self {
//             TarsType::EnInt8(i) => Ok(i as i64),
//             TarsType::EnInt16(i) => Ok(i as i64),
//             TarsType::EnInt32(i) => Ok(i as i64),
//             TarsType::EnInt64(i) => Ok(i),
//             _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
//         }
//     }

//     pub fn unwrap_u64(self) -> Result<u64, TarsTypeErr> {
//         match self {
//             TarsType::EnInt8(i) => Ok(i as u64),
//             TarsType::EnInt16(i) => Ok(i as u64),
//             TarsType::EnInt32(i) => Ok(i as u64),
//             TarsType::EnInt64(i) => Ok(i as u64),
//             _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
//         }
//     }

//     pub fn unwrap_float(self) -> Result<f32, TarsTypeErr> {
//         match self {
//             TarsType::EnFloat(f) => Ok(f32::from_bits(f)),
//             _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
//         }
//     }

//     pub fn unwrap_double(self) -> Result<f64, TarsTypeErr> {
//         match self {
//             TarsType::EnDouble(f) => Ok(f64::from_bits(f)),
//             _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
//         }
//     }

//     pub fn unwrap_map(self) -> Result<TarsMap, TarsTypeErr> {
//         match self {
//             TarsType::EnMaps(s) => Ok(s),
//             _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
//         }
//     }

//     pub fn unwrap_list(self) -> Result<TarsList, TarsTypeErr> {
//         match self {
//             TarsType::EnList(s) => Ok(s),
//             _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
//         }
//     }

//     pub fn unwrap_struct(self) -> Result<Bytes, TarsTypeErr> {
//         match self {
//             TarsType::EnStruct(s) => Ok(s),
//             _ => Err(TarsTypeErr::DisMatchTarsTypeErr),
//         }
//     }
// }
