use bytes::Bytes;
use errors::DecodeErr;
use std::collections::BTreeMap;
use tars_decoder::{DecodeTars, TarsDecodeListTrait, TarsDecodeNormalTrait, TarsDecoder};
use tars_type::{ClassName, ProtocolVersion};

type SimpleTupMap = BTreeMap<String, Bytes>;
type ComplexTupMap = BTreeMap<String, BTreeMap<String, Bytes>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TupDecoder {
    version: ProtocolVersion,
    simple_map: SimpleTupMap,
    complex_map: ComplexTupMap,
}
// for SimpleTup protocol version
impl TupDecoder {
    pub fn new(version: ProtocolVersion) -> Self {
        match version {
            ProtocolVersion::TupSimple => TupDecoder {
                version,
                simple_map: BTreeMap::new(),
                complex_map: BTreeMap::new(),
            },
            ProtocolVersion::TupComplex => TupDecoder {
                version,
                simple_map: BTreeMap::new(),
                complex_map: BTreeMap::new(),
            },
            _ => TupDecoder {
                version,
                simple_map: BTreeMap::new(),
                complex_map: BTreeMap::new(),
            },
        }
    }

    pub fn from_bytes<'a>(buf: &'a Bytes, version: ProtocolVersion) -> Result<Self, DecodeErr> {
        match version {
            ProtocolVersion::TupSimple => Ok(TupDecoder {
                version,
                simple_map: TarsDecoder::individual_decode(buf)?,
                complex_map: BTreeMap::new(),
            }),
            ProtocolVersion::TupComplex => Ok(TupDecoder {
                version,
                simple_map: BTreeMap::new(),
                complex_map: TarsDecoder::individual_decode(buf)?,
            }),
            _ => Ok(TupDecoder {
                version,
                simple_map: BTreeMap::new(),
                complex_map: BTreeMap::new(),
            }),
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
        T: DecodeTars + ClassName,
    {
        match self.version {
            ProtocolVersion::TupSimple => match self.simple_map.get(name) {
                Some(b) => Ok(Some(TarsDecoder::individual_decode(b)?)),
                None => Ok(None),
            },
            ProtocolVersion::TupComplex => unimplemented!(),
            _ => Err(DecodeErr::UnsupportTupVersionErr),
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

    fn read_int16(
        &self,
        name: &String,
        is_require: bool,
        default_value: i16,
    ) -> Result<i16, DecodeErr>;

    fn read_int32(
        &self,
        name: &String,
        is_require: bool,
        default_value: i32,
    ) -> Result<i32, DecodeErr>;

    fn read_int64(
        &self,
        name: &String,
        is_require: bool,
        default_value: i64,
    ) -> Result<i64, DecodeErr>;

}

impl TupDecoderTrait for TupDecoder {
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

    fn read_int16(
        &self,
        name: &String,
        is_require: bool,
        default_value: i16,
    ) -> Result<i16, DecodeErr> {
        match self.find(name)? {
            Some(i) => Ok(i),
            None => Self::return_error_if_required_not_found(is_require, default_value),
        }
    }


    fn read_int32(
        &self,
        name: &String,
        is_require: bool,
        default_value: i32,
    ) -> Result<i32, DecodeErr> {
        match self.find(name)? {
            Some(i) => Ok(i),
            None => Self::return_error_if_required_not_found(is_require, default_value),
        }
    }

    fn read_int64(
        &self,
        name: &String,
        is_require: bool,
        default_value: i64,
    ) -> Result<i64, DecodeErr> {
        match self.find(name)? {
            Some(i) => Ok(i),
            None => Self::return_error_if_required_not_found(is_require, default_value),
        }
    }    

}

#[cfg(test)]
mod tests {
    use super::*;
    use tars_encoder::*;

    #[test]
    fn test_decode_simple_tup() {
        let mut map = BTreeMap::new();

        let key0 = "zero".to_string();
        let value0 = 0;

        let key1 = "hello".to_string();
        let value1 = i8::max_value();

        let key2 = "world".to_string();
        let value2 = i16::max_value();

        map.insert(
            key0.clone(),
            TarsEncoder::individual_encode(&value0).unwrap(),
        );

        map.insert(
            key1.clone(),
            TarsEncoder::individual_encode(&value1).unwrap(),
        );

        map.insert(
            key2.clone(),
            TarsEncoder::individual_encode(&value2).unwrap(),
        );
        // map.insert(
        //     "bar".to_string(),
        //     TarsEncoder::individual_encode(&false).unwrap(),
        // );
        // map.insert(
        //     "foo".to_string(),
        //     TarsEncoder::individual_encode(&128).unwrap(),
        // );

        let tup_de = TupDecoder::from_bytes(
            &TarsEncoder::individual_encode(&map).unwrap(),
            ProtocolVersion::TupSimple,
        ).unwrap();

        let de_0 = tup_de.read_int64(&key0, true, 0).unwrap();
        assert_eq!(de_0, value0);

        let de_i8: i8 = tup_de.read_int8(&key1, true, 0).unwrap();
        assert_eq!(de_i8, value1);

        let de_i16 = tup_de.read_int16(&key2, true, 0).unwrap();
        assert_eq!(de_i16, value2);

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
