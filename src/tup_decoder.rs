use bytes::Bytes;
use std::collections::BTreeMap;

use errors::DecodeErr;
use tars_decoder::{TarsDecoder, TarsDecodeNormalTrait, TarsDecodeListTrait, DecodeFromTars};
use tars_type::{ClassName, ProtocolVersion};

type SimpleTup = BTreeMap<String, Bytes>;
type ComplexTup = BTreeMap<String, BTreeMap<String, Bytes>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TupDecoder<M> {
    map: M,
}
// for SimpleTup protocol version
impl TupDecoder<SimpleTup> {
    pub fn new() -> Self {
        TupDecoder {
            map: BTreeMap::new(),
        }
    }

    fn return_error_if_required_not_found<T>(
        is_require: bool,
        default_value: T,
    ) -> Result<T, DecodeErr> {
        if is_require {
            Err(DecodeErr::TupKeyNotFoundErr)
        } else {
            Ok(default_value)
        }
    }

    pub fn find<T>(&self, name: &String) -> Result<Option<T>, DecodeErr>
    where
        T: DecodeFromTars,
    {
        match self.map.get(name) {
            Some(b) => Ok(Some(TarsDecoder::individual_decode(b)?)),
            None => Ok(None),
        }
    }
}

// for ComplexTup protocol version
impl TupDecoder<ComplexTup> {
    pub fn new() -> Self {
        TupDecoder {
            map: BTreeMap::new(),
        }
    }
}

impl<'a, K, V> From<&'a Bytes> for TupDecoder<BTreeMap<K, V>>
where
    K: DecodeFromTars + Ord,
    V: DecodeFromTars,
{
    fn from(buf: &'a Bytes) -> Self {
        let mut decoder = TarsDecoder::from(buf);
        match decoder.read_map(0, true, BTreeMap::new()) {
            Err(_) => TupDecoder {
                map: BTreeMap::new(),
            },
            Ok(m) => TupDecoder { map: m },
        }
    }
}

pub trait TupDecoderTrait {
    fn read_int8(
        &self,
        name: &String,
        is_require: bool,
        default_value: i8,
    ) -> Result<i8, DecodeErr>;
}

impl TupDecoderTrait for TupDecoder<SimpleTup> {
    fn read_int8(
        &self,
        name: &String,
        is_require: bool,
        default_value: i8,
    ) -> Result<i8, DecodeErr> {
        match self.find(name)? {
            Some(i) => Ok(i),
            None => Self::return_error_if_required_not_found(is_require, default_value),
        }
    }
}

// impl<T> TupDecoderTrait<T> for TupDecoder<SimpleTup>
// where
//     T: DecodeFromTars,
// {
//     fn get(&self, name: &String) -> Result<T, DecodeErr> {
//         match self.map.get(name) {
//             None => Err(DecodeErr::FieldNotFoundErr(
//                 String::from("TupDecoder<SimpleTup> not found field: ") + name,
//             )),
//             Some(b) => Ok(TarsDecoder::individual_decode::<T>(b)?),
//         }
//     }
// }

// impl<T> TupDecoderTrait<Option<T>> for TupDecoder<SimpleTup>
// where
//     T: DecodeFromTars,
// {
//     fn get(&self, name: &String) -> Result<Option<T>, DecodeErr> {
//         match self.map.get(name) {
//             None => Ok(None),
//             Some(b) => Ok(Some(TarsDecoder::individual_decode::<T>(b)?)),
//         }
//     }
// }

// impl<T> TupDecoderTrait<T> for TupDecoder<ComplexTup>
// where
//     T: DecodeFromTars + ClassName,
// {
//     fn get(&self, name: &String) -> Result<T, DecodeErr> {
//         match self.map.get(name) {
//             None => Err(DecodeErr::FieldNotFoundErr(
//                 String::from("TupDecoder<ComplexTup> not found field: ") + name,
//             )),
//             Some(item) => match item.get(&T::_class_name()) {
//                 None => Err(DecodeErr::TypeNotFoundErr(
//                     "TupDecoder<ComplexTup> not found type: ".to_string() + &T::_class_name(),
//                 )),
//                 Some(b) => Ok(TarsDecoder::individual_decode::<T>(b)?),
//             },
//         }
//     }
// }

// impl<T> TupDecoderTrait<Option<T>> for TupDecoder<ComplexTup>
// where
//     T: DecodeFromTars + ClassName,
// {
//     fn get(&self, name: &String) -> Result<Option<T>, DecodeErr> {
//         match self.map.get(name) {
//             None => Ok(None),
//             Some(item) => match item.get(&T::_class_name()) {
//                 None => Ok(None),
//                 Some(b) => Ok(Some(TarsDecoder::individual_decode::<T>(b)?)),
//             },
//         }
//     }
// }

// pub trait DecodeFromTup {
//     fn decode_from_tup(b: &Bytes) -> Result<Self, DecodeErr>
//     where
//         Self: Sized;
// }

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub struct UniTupDecoder {
//     simple_tup_decoder: TupDecoder<SimpleTup>,
//     complex_tup_decoder: TupDecoder<ComplexTup>,
// }

#[cfg(test)]
mod tests {
    use super::*;
    use tars_encoder::*;

    #[test]
    fn test_decode_simple_tup() {
        let mut map = BTreeMap::new();

        let key1 = "hello".to_string();
        let value1 = i8::max_value();

        map.insert(
            key1.clone(),
            TarsEncoder::individual_encode(&value1).unwrap(),
        );
        // map.insert(
        //     "bar".to_string(),
        //     TarsEncoder::individual_encode(&false).unwrap(),
        // );
        // map.insert(
        //     "foo".to_string(),
        //     TarsEncoder::individual_encode(&128).unwrap(),
        // );

        let tup_de: TupDecoder<SimpleTup> =
            TupDecoder::from(&TarsEncoder::individual_encode(&map).unwrap());
        let de_i8: i8 = tup_de.read_int8(&key1, true, 0).unwrap();
        assert_eq!(de_i8, value1);

        // let de_bool: bool = tup_de.get(&"bar".to_string()).unwrap();
        // assert_eq!(de_bool, false);

        // let n: Option<i32> = tup_de.get(&"easy".to_string()).unwrap();
        // assert_eq!(n, None);

        // let n: Option<i32> = tup_de.get(&"foo".to_string()).unwrap();
        // assert_eq!(n, Some(128));
    }

//     #[test]
//     fn test_decode_complex_tup() {
//         let mut map = BTreeMap::new();

//         let mut item1 = BTreeMap::new();
//         let key1 = String::from("hello");
//         let value1 = String::from("world");
//         item1.insert(
//             String::_class_name(),
//             TarsEncoder::individual_encode(&value1).unwrap(),
//         );
//         map.insert(key1.clone(), item1);

//         let mut item2 = BTreeMap::new();
//         let key2 = String::from("foo");
//         let value2: u8 = 255;
//         item2.insert(
//             u8::_class_name(),
//             TarsEncoder::individual_encode(&value2).unwrap(),
//         );
//         map.insert(key2.clone(), item2);

//         let mut item2 = BTreeMap::new();
//         let key2 = String::from("foo");
//         let value2: u16 = 65535;
//         item2.insert(
//             u16::_class_name(),
//             TarsEncoder::individual_encode(&value2).unwrap(),
//         );
//         map.insert(key2.clone(), item2);

//         let tup_de: TupDecoder<ComplexTup> =
//             TupDecoder::from(&TarsEncoder::individual_encode(&map).unwrap());
//         let de_value1: String = tup_de.get(&key1).unwrap();
//         assert_eq!(value1, de_value1);

//         let de_value2: u16 = tup_de.get(&key2).unwrap();
//         assert_eq!(value2, de_value2);
//     }
}
