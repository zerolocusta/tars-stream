use bytes::Bytes;
use errors::DecodeErr;
use std::collections::BTreeMap;
use tars_decoder::{DecodeTars, TarsDecodeListTrait, TarsDecodeNormalTrait, TarsDecoder};
use tars_trait::{ClassName, EnumFromI32, EnumToI32};
use tars_type::ProtocolVersion;

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

    pub fn read<T>(&self, name: &String, is_require: bool, default_value: T) -> Result<T, DecodeErr>
    where
        T: DecodeTars + ClassName,
    {
        match self.version {
            ProtocolVersion::TupSimple => match self.simple_map.get(name) {
                Some(b) => Ok(TarsDecoder::individual_decode(b)?),
                None => Ok(Self::return_error_if_required_not_found(
                    is_require,
                    default_value,
                )?),
            },
            ProtocolVersion::TupComplex => match self.complex_map.get(name) {
                Some(item) => match item.get(&T::_class_name()) {
                    Some(b) => Ok(TarsDecoder::individual_decode(b)?),
                    None => Ok(Self::return_error_if_required_not_found(
                        is_require,
                        default_value,
                    )?),
                },
                None => Ok(Self::return_error_if_required_not_found(
                    is_require,
                    default_value,
                )?),
            },
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

    fn read_uint8(
        &self,
        name: &String,
        is_require: bool,
        default_value: u8,
    ) -> Result<u8, DecodeErr>;

    fn read_uint16(
        &self,
        name: &String,
        is_require: bool,
        default_value: u16,
    ) -> Result<u16, DecodeErr>;

    fn read_uint32(
        &self,
        name: &String,
        is_require: bool,
        default_value: u32,
    ) -> Result<u32, DecodeErr>;

    fn read_boolean(
        &self,
        name: &String,
        is_require: bool,
        default_value: bool,
    ) -> Result<bool, DecodeErr>;

    fn read_float(
        &self,
        name: &String,
        is_require: bool,
        default_value: f32,
    ) -> Result<f32, DecodeErr>;

    fn read_double(
        &self,
        name: &String,
        is_require: bool,
        default_value: f64,
    ) -> Result<f64, DecodeErr>;

    fn read_string(
        &self,
        name: &String,
        is_require: bool,
        default_value: String,
    ) -> Result<String, DecodeErr>;

    fn read_bytes(
        &self,
        name: &String,
        is_require: bool,
        default_value: Bytes,
    ) -> Result<Bytes, DecodeErr>;

    fn read_list<T>(
        &self,
        name: &String,
        is_require: bool,
        default_value: Vec<T>,
    ) -> Result<Vec<T>, DecodeErr>
    where
        T: DecodeTars + ClassName;

    fn read_map<K, V>(
        &self,
        name: &String,
        is_require: bool,
        default_value: BTreeMap<K, V>,
    ) -> Result<BTreeMap<K, V>, DecodeErr>
    where
        K: DecodeTars + ClassName + Ord,
        V: DecodeTars + ClassName;

    fn read_enum<T>(
        &self,
        name: &String,
        is_require: bool,
        default_value: T,
    ) -> Result<T, DecodeErr>
    where
        T: DecodeTars + ClassName;

    fn read_struct<T>(
        &self,
        name: &String,
        is_require: bool,
        default_value: T,
    ) -> Result<T, DecodeErr>
    where
        T: DecodeTars + ClassName;
}

impl TupDecoderTrait for TupDecoder {
    fn read_int8(
        &self,
        name: &String,
        is_require: bool,
        default_value: i8,
    ) -> Result<i8, DecodeErr> {
        self.read(name, is_require, default_value)
    }

    fn read_int16(
        &self,
        name: &String,
        is_require: bool,
        default_value: i16,
    ) -> Result<i16, DecodeErr> {
        self.read(name, is_require, default_value)
    }

    fn read_int32(
        &self,
        name: &String,
        is_require: bool,
        default_value: i32,
    ) -> Result<i32, DecodeErr> {
        self.read(name, is_require, default_value)
    }

    fn read_int64(
        &self,
        name: &String,
        is_require: bool,
        default_value: i64,
    ) -> Result<i64, DecodeErr> {
        self.read(name, is_require, default_value)
    }

    fn read_uint8(
        &self,
        name: &String,
        is_require: bool,
        default_value: u8,
    ) -> Result<u8, DecodeErr> {
        self.read(name, is_require, default_value)
    }

    fn read_uint16(
        &self,
        name: &String,
        is_require: bool,
        default_value: u16,
    ) -> Result<u16, DecodeErr> {
        self.read(name, is_require, default_value)
    }

    fn read_uint32(
        &self,
        name: &String,
        is_require: bool,
        default_value: u32,
    ) -> Result<u32, DecodeErr> {
        self.read(name, is_require, default_value)
    }

    fn read_boolean(
        &self,
        name: &String,
        is_require: bool,
        default_value: bool,
    ) -> Result<bool, DecodeErr> {
        self.read(name, is_require, default_value)
    }

    fn read_float(
        &self,
        name: &String,
        is_require: bool,
        default_value: f32,
    ) -> Result<f32, DecodeErr> {
        self.read(name, is_require, default_value)
    }

    fn read_double(
        &self,
        name: &String,
        is_require: bool,
        default_value: f64,
    ) -> Result<f64, DecodeErr> {
        self.read(name, is_require, default_value)
    }

    fn read_string(
        &self,
        name: &String,
        is_require: bool,
        default_value: String,
    ) -> Result<String, DecodeErr> {
        self.read(name, is_require, default_value)
    }

    fn read_bytes(
        &self,
        name: &String,
        is_require: bool,
        default_value: Bytes,
    ) -> Result<Bytes, DecodeErr> {
        self.read(name, is_require, default_value)
    }

    fn read_list<T>(
        &self,
        name: &String,
        is_require: bool,
        default_value: Vec<T>,
    ) -> Result<Vec<T>, DecodeErr>
    where
        T: DecodeTars + ClassName,
    {
        self.read(name, is_require, default_value)
    }

    fn read_map<K, V>(
        &self,
        name: &String,
        is_require: bool,
        default_value: BTreeMap<K, V>,
    ) -> Result<BTreeMap<K, V>, DecodeErr>
    where
        K: DecodeTars + ClassName + Ord,
        V: DecodeTars + ClassName,
    {
        self.read(name, is_require, default_value)
    }

    fn read_enum<T>(
        &self,
        name: &String,
        is_require: bool,
        default_value: T,
    ) -> Result<T, DecodeErr>
    where
        T: DecodeTars + ClassName,
    {
        self.read(name, is_require, default_value)
    }

    fn read_struct<T>(
        &self,
        name: &String,
        is_require: bool,
        default_value: T,
    ) -> Result<T, DecodeErr>
    where
        T: DecodeTars + ClassName,
    {
        self.read(name, is_require, default_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tars_encoder::*;



    #[test]
    fn test_decode_simple_tup() {
        let key0 = "zero".to_string();
        let value0 = 0;

        let key1 = "hello".to_string();
        let value1 = i8::max_value();

        let key2 = "world".to_string();
        let value2 = i16::max_value();

        let key3 = "aba".to_string();
        let value3 = i32::max_value();

        let key4 = "i64".to_string();
        let value4 = i64::max_value();

        let key5 = "bool".to_string();
        let value5 = true;

        let key6 = "u8".to_string();
        let value6 = u8::max_value();

        let key7 = "u16".to_string();
        let value7 = u16::max_value();

        let key8 = "u32".to_string();
        let value8 = u32::max_value();

        let key9 = "float".to_string();
        let value9 = 0.333f32;

        let key10 = "double".to_string();
        let value10 = 1.77721337f64;

        let key11 = "string".to_string();
        let value11 = String::from("hello wrold! foo bar!");

        let key12 = "bytes".to_string();
        let value12 = Bytes::from("hello wrold! foo bar!");

        let key13 = "vec".to_string();
        let value13: Vec<u32> = vec![1, 2, 3, 4];

        let key14 = "map".to_string();
        let value14: BTreeMap<String, String> = BTreeMap::new();

        let fake_key = "fake_key".to_string();

        let mut map = BTreeMap::new();

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

        map.insert(
            key3.clone(),
            TarsEncoder::individual_encode(&value3).unwrap(),
        );

        map.insert(
            key4.clone(),
            TarsEncoder::individual_encode(&value4).unwrap(),
        );

        map.insert(
            key5.clone(),
            TarsEncoder::individual_encode(&value5).unwrap(),
        );

        map.insert(
            key6.clone(),
            TarsEncoder::individual_encode(&value6).unwrap(),
        );

        map.insert(
            key7.clone(),
            TarsEncoder::individual_encode(&value7).unwrap(),
        );

        map.insert(
            key8.clone(),
            TarsEncoder::individual_encode(&value8).unwrap(),
        );

        map.insert(
            key9.clone(),
            TarsEncoder::individual_encode(&value9).unwrap(),
        );

        map.insert(
            key10.clone(),
            TarsEncoder::individual_encode(&value10).unwrap(),
        );

        map.insert(
            key11.clone(),
            TarsEncoder::individual_encode(&value11).unwrap(),
        );

        map.insert(
            key12.clone(),
            TarsEncoder::individual_encode(&value12).unwrap(),
        );

        map.insert(
            key13.clone(),
            TarsEncoder::individual_encode(&value13).unwrap(),
        );

        map.insert(
            key14.clone(),
            TarsEncoder::individual_encode(&value14).unwrap(),
        );

        let tup_de = TupDecoder::from_bytes(
            &TarsEncoder::individual_encode(&map).unwrap(),
            ProtocolVersion::TupSimple,
        ).unwrap();

        let de_0 = tup_de.read(&key0, true, 0).unwrap();
        assert_eq!(de_0, value0);

        let de_i8: i8 = tup_de.read(&key1, true, 0).unwrap();
        assert_eq!(de_i8, value1);

        let de_i16 = tup_de.read(&key2, true, 0).unwrap();
        assert_eq!(de_i16, value2);

        let de_i32 = tup_de.read(&key3, true, 0).unwrap();
        assert_eq!(de_i32, value3);

        let de_i64 = tup_de.read(&key4, true, 0).unwrap();
        assert_eq!(de_i64, value4);

        let de_bool = tup_de.read(&key5, true, false).unwrap();
        assert_eq!(de_bool, value5);

        let de_u8 = tup_de.read(&key6, true, 0).unwrap();
        assert_eq!(de_u8, value6);

        let de_u16 = tup_de.read(&key7, true, 0).unwrap();
        assert_eq!(de_u16, value7);

        let de_u32 = tup_de.read(&key8, true, 0).unwrap();
        assert_eq!(de_u32, value8);

        let de_f32 = tup_de.read(&key9, true, 0.0).unwrap();
        assert_eq!(de_f32, value9);

        let de_f64 = tup_de.read(&key10, true, 0.0).unwrap();
        assert_eq!(de_f64, value10);

        let de_string = tup_de.read(&key11, true, String::from("")).unwrap();
        assert_eq!(de_string, value11);

        let de_bytes = tup_de.read(&key12, true, Bytes::default()).unwrap();
        assert_eq!(de_bytes, value12);

        let de_vec: Vec<u32> = tup_de.read(&key13, true, vec![]).unwrap();
        assert_eq!(de_vec, value13);

        let de_map: BTreeMap<String, String> =
            tup_de.read(&key14, true, BTreeMap::new()).unwrap();
        assert_eq!(de_map, value14);

        let de_fake_value_err = tup_de.read(&fake_key, true, 0);
        assert_eq!(de_fake_value_err, Err(DecodeErr::TupKeyNotFoundErr));

        let de_fake_value = tup_de.read(&fake_key, false, 0).unwrap();
        assert_eq!(de_fake_value, 0);
    }

    #[test]
    fn test_decode_complex_tup() {
        let key0 = "zero".to_string();
        let value0 = 0;
        let mut item0: BTreeMap<String, Bytes> = BTreeMap::new();
        item0.insert(
            i64::_class_name(),
            TarsEncoder::individual_encode(&value0).unwrap(),
        );

        let key1 = "hello".to_string();
        let value1 = i8::max_value();
        let mut item1: BTreeMap<String, Bytes> = BTreeMap::new();
        item1.insert(
            i8::_class_name(),
            TarsEncoder::individual_encode(&value1).unwrap(),
        );

        let key2 = "world".to_string();
        let value2 = i16::max_value();
        let mut item2: BTreeMap<String, Bytes> = BTreeMap::new();
        item2.insert(
            i16::_class_name(),
            TarsEncoder::individual_encode(&value2).unwrap(),
        );

        let key3 = "aba".to_string();
        let value3 = i32::max_value();
        let mut item3: BTreeMap<String, Bytes> = BTreeMap::new();
        item3.insert(
            i32::_class_name(),
            TarsEncoder::individual_encode(&value3).unwrap(),
        );

        let key4 = "i64".to_string();
        let value4 = i64::max_value();
        let mut item4: BTreeMap<String, Bytes> = BTreeMap::new();
        item4.insert(
            i64::_class_name(),
            TarsEncoder::individual_encode(&value4).unwrap(),
        );

        let key5 = "bool".to_string();
        let value5 = true;
        let mut item5: BTreeMap<String, Bytes> = BTreeMap::new();
        item5.insert(
            bool::_class_name(),
            TarsEncoder::individual_encode(&value5).unwrap(),
        );

        let key6 = "u8".to_string();
        let value6 = u8::max_value();

        let key7 = "u16".to_string();
        let value7 = u16::max_value();

        let key8 = "u32".to_string();
        let value8 = u32::max_value();

        let key9 = "float".to_string();
        let value9 = 0.333f32;

        let key10 = "double".to_string();
        let value10 = 1.77721337f64;

        let key11 = "string".to_string();
        let value11 = String::from("hello wrold! foo bar!");

        let key12 = "bytes".to_string();
        let value12 = Bytes::from("hello wrold! foo bar!");

        let key13 = "vec".to_string();
        let value13: Vec<u32> = vec![1, 2, 3, 4];

        let key14 = "map".to_string();
        let value14: BTreeMap<String, String> = BTreeMap::new();

        let fake_key = "fake_key".to_string();

        let mut map: BTreeMap<String, BTreeMap<String, Bytes>> = BTreeMap::new();

        map.insert(key0.clone(), item0);
        map.insert(key1.clone(), item1);
        map.insert(key2.clone(), item2);
        map.insert(key3.clone(), item3);
        map.insert(key4.clone(), item4);
        map.insert(key5.clone(), item5);

        let tup_de = TupDecoder::from_bytes(
            &TarsEncoder::individual_encode(&map).unwrap(),
            ProtocolVersion::TupComplex,
        ).unwrap();

        let de_0 = tup_de.read(&key0, true, 0).unwrap();
        assert_eq!(de_0, value0);

        let de_i8: i8 = tup_de.read(&key1, true, 0).unwrap();
        assert_eq!(de_i8, value1);

        let de_i16 = tup_de.read(&key2, true, 0).unwrap();
        assert_eq!(de_i16, value2);

        let de_i32 = tup_de.read(&key3, true, 0).unwrap();
        assert_eq!(de_i32, value3);

        let de_i64 = tup_de.read(&key4, true, 0).unwrap();
        assert_eq!(de_i64, value4);

        let de_bool = tup_de.read(&key5, true, false).unwrap();
        assert_eq!(de_bool, value5);
    }
}
