use bytes::{Buf, Bytes, IntoBuf};
use std::collections::BTreeMap;
use std::io::Cursor;
use std::mem;

use errors::DecodeErr;
use tars_type::TarsTypeMark;
use tars_type::TarsTypeMark::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TarsDecoder {
    buf: Bytes,
    pos: usize,
}
#[derive(Debug)]
pub struct Head {
    tag: u8,
    tars_type: TarsTypeMark,
    len: u8,
}

impl TarsDecoder {
    pub fn new() -> TarsDecoder {
        TarsDecoder {
            buf: Bytes::new(),
            pos: 0,
        }
    }

    // pub fn individual_decode<T>(buf: &Bytes) -> Result<T, DecodeErr>
    // where
    //     T: DecodeFromTars,
    // {
    //     let mut decoder = TarsDecoder::from(buf);
    //     let head = decoder.take_head()?;
    //     decoder.read(head.tars_type)
    // }

    fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    fn has_remaining(&self) -> bool {
        self.remaining() > 0
    }

    fn advance(&mut self, cnt: usize) -> Result<(), DecodeErr> {
        if self.remaining() < cnt {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            self.pos += cnt;
            Ok(())
        }
    }

    fn take_then_advance(&mut self, size: usize) -> Result<Bytes, DecodeErr> {
        if self.remaining() < size {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            let b = self.buf.slice(self.pos, self.pos + size);
            self.pos += size;
            Ok(b)
        }
    }

    fn skip_to_tag(&mut self, tag: u8) -> Result<Head, DecodeErr> {
        let mut result: Option<Head> = None;
        while self.has_remaining() {
            let head = self.take_head()?;
            if head.tag == tag {
                result = Some(head);
                break;
            } else {
                self.skip_field(head.tars_type)?;
            }
        }
        match result {
            Some(h) => Ok(h),
            None => Err(DecodeErr::TagNotFoundErr),
        }
    }

    fn take_head(&mut self) -> Result<Head, DecodeErr> {
        if self.remaining() < 1 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            let mut buf = self.take_then_advance(1)?.into_buf();
            let b = buf.get_u8();
            let tars_type = b & 0x0f;
            let mut tag = (b & 0xf0) >> 4;
            let len = if tag < 15 {
                1
            } else {
                let mut buf = self.take_then_advance(1)?.into_buf();
                tag = buf.get_u8();
                2
            };
            Ok(Head {
                tag,
                len,
                tars_type: TarsTypeMark::from(tars_type),
            })
        }
    }

    fn skip_field(&mut self, tars_type: TarsTypeMark) -> Result<(), DecodeErr> {
        match tars_type {
            EnInt8 => self.advance(1),
            EnInt16 => self.advance(2),
            EnInt32 => self.advance(4),
            EnInt64 => self.advance(8),
            EnFloat => self.advance(4),
            EnDouble => self.advance(8),
            EnString1 => self.skip_string1_field(),
            EnString4 => self.skip_string4_field(),
            EnMaps => self.skip_map_field(),
            EnList => self.skip_list_field(),
            EnStructBegin => self.skip_struct_field(),
            EnStructEnd => Ok(()),
            EnZero => Ok(()),
            EnSimplelist => self.skip_simple_list_field(),
        }
    }

    fn skip_string1_field(&mut self) -> Result<(), DecodeErr> {
        let mut buf = self.take_then_advance(1)?.into_buf();
        let size = buf.get_u8() as usize;
        self.advance(size)
    }

    fn skip_string4_field(&mut self) -> Result<(), DecodeErr> {
        let mut buf = self.take_then_advance(4)?.into_buf();
        let size = buf.get_u32_be() as usize;
        self.advance(size)
    }

    fn skip_map_field(&mut self) -> Result<(), DecodeErr> {
        let ele_size = self.read_int32(0)? as usize;
        for _ in 0..ele_size * 2 {
            let head = self.take_head()?;
            self.skip_field(head.tars_type)?;
        }
        Ok(())
    }

    fn skip_list_field(&mut self) -> Result<(), DecodeErr> {
        let ele_size = self.read_int32(0)? as usize;
        for _ in 0..ele_size {
            let head = self.take_head()?;
            self.skip_field(head.tars_type)?;
        }
        Ok(())
    }

    fn skip_simple_list_field(&mut self) -> Result<(), DecodeErr> {
        let head = self.take_head()?; // consume header (list type)
        self.skip_field(head.tars_type)
    }

    fn skip_struct_field(&mut self) -> Result<(), DecodeErr> {
        let mut head = self.take_head()?;
        loop {
            match head.tars_type {
                EnStructEnd => break,
                _ => {
                    self.skip_field(head.tars_type)?;
                    head = self.take_head()?;
                }
            }
        }
        Ok(())
    }
}

impl<'a> From<&'a [u8]> for TarsDecoder {
    fn from(buf: &'a [u8]) -> Self {
        let b = Bytes::from(buf);
        TarsDecoder { buf: b, pos: 0 }
    }
}

impl<'a> From<&'a Bytes> for TarsDecoder {
    fn from(buf: &'a Bytes) -> Self {
        let b = buf.clone();
        TarsDecoder { buf: b, pos: 0 }
    }
}

impl From<Vec<u8>> for TarsDecoder {
    fn from(buf: Vec<u8>) -> Self {
        let b = Bytes::from(buf);
        TarsDecoder { buf: b, pos: 0 }
    }
}

pub trait TarsDecodeSimpleTrait {
    fn read_int8(&mut self, tag: u8) -> Result<i8, DecodeErr>;
    fn read_boolean(&mut self, tag: u8) -> Result<bool, DecodeErr>;
    fn read_int16(&mut self, tag: u8) -> Result<i16, DecodeErr>;
    fn read_int32(&mut self, tag: u8) -> Result<i32, DecodeErr>;
    fn read_int64(&mut self, tag: u8) -> Result<i64, DecodeErr>;

    fn read_uint8(&mut self, tag: u8) -> Result<u8, DecodeErr>;
    fn read_uint16(&mut self, tag: u8) -> Result<u16, DecodeErr>;
    fn read_uint32(&mut self, tag: u8) -> Result<u32, DecodeErr>;

    fn read_float(&mut self, tag: u8) -> Result<f32, DecodeErr>;
    fn read_double(&mut self, tag: u8) -> Result<f64, DecodeErr>;
    fn read_string(&mut self, tag: u8) -> Result<String, DecodeErr>;

    fn read_bytes(&mut self, tag: u8) -> Result<Bytes, DecodeErr>;
}

pub trait TarsDecodeListTrait<T>
where
    T: DecodeFromTars,
{
    fn read_list(&mut self, tag: u8) -> Result<Vec<T>, DecodeErr>;
}

pub trait TarsDecodeMapTrait<K, V>
where
    K: DecodeFromTars + Ord,
    V: DecodeFromTars,
{
    fn read_map(&mut self, tag: u8) -> Result<BTreeMap<K, V>, DecodeErr>;
}

pub trait TarsDecodeStructTrait<T>
where
    T: DecodeFromTars,
{
    fn read_struct(&mut self, tag: u8) -> Result<T, DecodeErr>;
}

// for require field
impl TarsDecodeSimpleTrait for TarsDecoder {
    fn read_int8(&mut self, tag: u8) -> Result<i8, DecodeErr> {
        let head = self.skip_to_tag(tag)?;
        match head.tars_type {
            EnZero => Ok(0),
            EnInt8 => {
                let mut buf = self.take_then_advance(1)?.into_buf();
                Ok(buf.get_i8())
            }
            _ => Err(DecodeErr::MisMatchTarsTypeErr),
        }
    }

    fn read_boolean(&mut self, tag: u8) -> Result<bool, DecodeErr> {
        self.read_int8(tag).map(|i| i != 0)
    }

    fn read_int16(&mut self, tag: u8) -> Result<i16, DecodeErr> {
        let head = self.skip_to_tag(tag)?;
        match head.tars_type {
            EnZero => Ok(0),
            EnInt8 => {
                let mut buf = self.take_then_advance(1)?.into_buf();
                Ok(i16::from(buf.get_i8()))
            }
            EnInt16 => {
                let mut buf = self.take_then_advance(2)?.into_buf();
                Ok(buf.get_i16_be())
            }
            _ => Err(DecodeErr::MisMatchTarsTypeErr),
        }
    }

    fn read_int32(&mut self, tag: u8) -> Result<i32, DecodeErr> {
        let head = self.skip_to_tag(tag)?;
        match head.tars_type {
            EnZero => Ok(0),
            EnInt8 => {
                let mut buf = self.take_then_advance(1)?.into_buf();
                Ok(i32::from(buf.get_i8()))
            }
            EnInt16 => {
                let mut buf = self.take_then_advance(2)?.into_buf();
                Ok(i32::from(buf.get_i16_be()))
            }
            EnInt32 => {
                let mut buf = self.take_then_advance(4)?.into_buf();
                Ok(buf.get_i32_be())
            }
            _ => Err(DecodeErr::MisMatchTarsTypeErr),
        }
    }

    fn read_int64(&mut self, tag: u8) -> Result<i64, DecodeErr> {
        let head = self.skip_to_tag(tag)?;
        match head.tars_type {
            EnZero => Ok(0),
            EnInt8 => {
                let mut buf = self.take_then_advance(1)?.into_buf();
                Ok(i64::from(buf.get_i8()))
            }
            EnInt16 => {
                let mut buf = self.take_then_advance(2)?.into_buf();
                Ok(i64::from(buf.get_i16_be()))
            }
            EnInt32 => {
                let mut buf = self.take_then_advance(4)?.into_buf();
                Ok(i64::from(buf.get_i32_be()))
            }
            EnInt64 => {
                let mut buf = self.take_then_advance(8)?.into_buf();
                Ok(buf.get_i64_be())
            }
            _ => Err(DecodeErr::MisMatchTarsTypeErr),
        }
    }

    fn read_uint8(&mut self, tag: u8) -> Result<u8, DecodeErr> {
        self.read_int16(tag).map(|i| i as u8)
    }

    fn read_uint16(&mut self, tag: u8) -> Result<u16, DecodeErr> {
        self.read_int32(tag).map(|i| i as u16)
    }

    fn read_uint32(&mut self, tag: u8) -> Result<u32, DecodeErr> {
        self.read_int64(tag).map(|i| i as u32)
    }

    fn read_float(&mut self, tag: u8) -> Result<f32, DecodeErr> {
        let head = self.skip_to_tag(tag)?;
        match head.tars_type {
            EnZero => Ok(0.0),
            EnFloat => {
                let mut buf = self.take_then_advance(4)?.into_buf();
                Ok(buf.get_f32_be())
            }
            _ => Err(DecodeErr::MisMatchTarsTypeErr),
        }
    }

    fn read_double(&mut self, tag: u8) -> Result<f64, DecodeErr> {
        let head = self.skip_to_tag(tag)?;
        match head.tars_type {
            EnZero => Ok(0.0),
            EnDouble => {
                let mut buf = self.take_then_advance(8)?.into_buf();
                Ok(buf.get_f64_be())
            }
            _ => Err(DecodeErr::MisMatchTarsTypeErr),
        }
    }

    fn read_string(&mut self, tag: u8) -> Result<String, DecodeErr> {
        let head = self.skip_to_tag(tag)?;
        match head.tars_type {
            EnString1 => {
                let mut size_buf = self.take_then_advance(1)?.into_buf();
                let size = size_buf.get_u8() as usize;
                let field_buf = self.take_then_advance(size)?.into_buf();
                let cow = String::from_utf8_lossy(field_buf.bytes());
                Ok(String::from(cow))
            }
            EnString4 => {
                let mut size_buf = self.take_then_advance(4)?.into_buf();
                let size = size_buf.get_u32_be() as usize;
                let field_buf = self.take_then_advance(size)?.into_buf();
                let cow = String::from_utf8_lossy(field_buf.bytes());
                Ok(String::from(cow))
            }
            _ => Err(DecodeErr::MisMatchTarsTypeErr),
        }
    }

    fn read_bytes(&mut self, tag: u8) -> Result<Bytes, DecodeErr> {
        // Bytes 实为 vector<char>, 以 simple list 表示
        let head = self.skip_to_tag(tag)?;
        match head.tars_type {
            EnSimplelist => {
                let head = self.take_head()?;
                match head.tars_type {
                    EnInt8 => {
                        let size = self.read_int32(0)? as usize;
                        self.take_then_advance(size)
                    }
                    _ => Err(DecodeErr::WrongSimpleListTarsTypeErr),
                }
            }
            _ => Err(DecodeErr::MisMatchTarsTypeErr),
        }
    }
}

impl<T> TarsDecodeListTrait<T> for TarsDecoder
where
    T: DecodeFromTars,
{
    default fn read_list(&mut self, tag: u8) -> Result<Vec<T>, DecodeErr> {
        let head = self.skip_to_tag(tag)?;
        match head.tars_type {
            EnList => {
                let size = self.read_int32(0)? as usize;
                let mut v = vec![];
                for _ in 0..size {
                    let ele = T::decode_from_tars(self, 0)?;
                    v.push(ele);
                }
                Ok(v)
            }
            _ => Err(DecodeErr::MisMatchTarsTypeErr),
        }
    }
}

impl TarsDecodeListTrait<i8> for TarsDecoder {
    fn read_list(&mut self, tag: u8) -> Result<Vec<i8>, DecodeErr> {
        self.read_bytes(tag)
            .map(|v| unsafe { mem::transmute(v.to_vec()) })
    }
}

impl TarsDecodeListTrait<bool> for TarsDecoder {
    fn read_list(&mut self, tag: u8) -> Result<Vec<bool>, DecodeErr> {
        self.read_bytes(tag)
            .map(|v| unsafe { mem::transmute(v.to_vec()) })
    }
}

impl<K, V> TarsDecodeMapTrait<K, V> for TarsDecoder
where
    K: DecodeFromTars + Ord,
    V: DecodeFromTars,
{
    fn read_map(&mut self, tag: u8) -> Result<BTreeMap<K, V>, DecodeErr> {
        let head = self.skip_to_tag(tag)?;
        match head.tars_type {
            EnMaps => {
                let mut size_buf = self.take_then_advance(4)?.into_buf();
                let size = size_buf.get_u32_be() as usize;
                let mut m = BTreeMap::new();
                for _ in 0..size {
                    let key = K::decode_from_tars(self, 0)?;
                    let value = V::decode_from_tars(self, 0)?;
                    m.insert(key, value);
                }
                Ok(m)
            }
            _ => Err(DecodeErr::MisMatchTarsTypeErr),
        }
    }
}

impl<T> TarsDecodeStructTrait<T> for TarsDecoder
where
    T: DecodeFromTars,
{
    fn read_struct(&mut self, tag: u8) -> Result<T, DecodeErr> {
        let head = self.skip_to_tag(tag)?;
        match head.tars_type {
            EnStructBegin => T::decode_from_tars(self, 0),
            _ => Err(DecodeErr::MisMatchTarsTypeErr),
        }
    }
}

// // for optional field
// impl<T> TarsDecoderTrait<Option<T>> for TarsDecoder
// where
//     T: DecodeFromTars,
// {
//     fn get(&mut self, tag: u8) -> Result<Option<T>, DecodeErr> {
//         self.pos = 0;
//         if let Ok(head) = self.skip_to_tag(tag) {
//             Ok(Some(self.read::<T>(head.tars_type)?))
//         } else {
//             Ok(None)
//         }
//     }
// }

pub trait DecodeFromTars {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr>
    where
        Self: Sized;
}

impl DecodeFromTars for i8 {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_int8(tag)
    }
}

impl DecodeFromTars for bool {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_boolean(tag)
    }
}

impl DecodeFromTars for i16 {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_int16(tag)
    }
}

impl DecodeFromTars for i32 {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_int32(tag)
    }
}

impl DecodeFromTars for i64 {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_int64(tag)
    }
}

impl DecodeFromTars for u8 {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_uint8(tag)
    }
}

impl DecodeFromTars for u16 {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_uint16(tag)
    }
}

impl DecodeFromTars for u32 {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_uint32(tag)
    }
}

impl DecodeFromTars for f32 {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_float(tag)
    }
}

impl DecodeFromTars for f64 {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_double(tag)
    }
}

impl DecodeFromTars for String {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_string(tag)
    }
}

impl<K, V> DecodeFromTars for BTreeMap<K, V>
where
    K: DecodeFromTars + Ord,
    V: DecodeFromTars,
{
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_map(tag)
    }
}

impl<T> DecodeFromTars for Vec<T>
where
    T: DecodeFromTars,
{
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_list(tag)
    }
}

impl DecodeFromTars for Bytes {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_bytes(tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use errors::DecodeErr;
    use std::collections::BTreeMap;
    use std::mem;
    use tars_type::TarsTypeMark;
    use tars_type::TarsTypeMark::*;

    #[test]
    fn test_take_simple_list() {
        let head: [u8; 4] = unsafe { mem::transmute(4u32.to_be()) };
        let b: [u8; 11] = [0x7d, 0x00, 0x02, head[0], head[1], head[2], head[3], 4, 5, 6, 7];
        let mut de = TarsDecoder::from(&b[..]);
        let list: Vec<i8> = de.read_list(7).unwrap();
        let result: Vec<i8> = vec![4, 5, 6, 7];
        assert_eq!(list, result);

        let b2: [u8; 11] = [0xed, 0x00, 0x02, head[0], head[1], head[2], head[3], 1, 0, 1, 0];
        let mut de2 = TarsDecoder::from(&b2[..]);
        let list: Vec<bool> = de2.read_list(14).unwrap();
        let result: Vec<bool> = vec![true, false, true, false];
        assert_eq!(list, result);
    }

    #[test]
    fn test_decode_zero() {
        let mut de = TarsDecoder::from(&b"\x0c\x1c\x2c\x3c\x4c\x5c\xfc\xff\x9c\xac\xec"[..]);
        let v0: u8 = de.read_uint8(0).unwrap();
        let v1: u16 = de.read_uint16(1).unwrap();
        let v2: u32 = de.read_uint32(2).unwrap();
        let v3: i8 = de.read_int8(3).unwrap();
        let v4: i16 = de.read_int16(4).unwrap();
        let v5: i32 = de.read_int32(5).unwrap();
        let v6: i64 = de.read_int64(255).unwrap();
        let v7: f32 = de.read_float(9).unwrap();
        let v8: f64 = de.read_double(10).unwrap();
        let v9: bool = de.read_boolean(14).unwrap();

        assert_eq!(v0, 0);
        assert_eq!(v1, 0);
        assert_eq!(v2, 0);
        assert_eq!(v3, 0);
        assert_eq!(v4, 0);
        assert_eq!(v5, 0);
        assert_eq!(v6, 0);
        assert_eq!(v7, 0.0);
        assert_eq!(v8, 0.0);
        assert_eq!(v9, false);
    }

    #[test]
    fn test_decode_list() {
        let size: [u8; 4] = unsafe { mem::transmute(2u32.to_be()) };
        let b: [u8; 28] = [
            0xa9,
            0x02,
            size[0],
            size[1],
            size[2],
            size[3],
            // {tag: 0, type: 6}
            0x06,
            7,
            b'f',
            b'o',
            b'o',
            b' ',
            b'b',
            b'a',
            b'r',
            // {tag: 0, type: 6}
            0x06,
            11,
            b'h',
            b'e',
            b'l',
            b'l',
            b'o',
            b' ',
            b'w',
            b'o',
            b'r',
            b'l',
            b'd',
        ];
        let mut de = TarsDecoder::from(&b[..]);
        let list: Vec<String> = de.read_list(10).unwrap();
        println!("{:?}", list);
        assert_eq!(list[0], String::from(&"foo bar"[..]));
        assert_eq!(list[1], String::from(&"hello world"[..]));

        assert_eq!(
            de.read_list(10) as Result<Vec<String>, DecodeErr>,
            Err(DecodeErr::TagNotFoundErr)
        );

        let b2: [u8; 6] = [0x99, 0x02, 0, 0, 0, 0];
        let mut de2 = TarsDecoder::from(&b2[..]);
        let v2: Vec<BTreeMap<String, i32>> = de2.read_list(9).unwrap();
        assert_eq!(v2, vec![]);
    }

    // #[test]
    // fn test_take_map() {
    //     let size: [u8; 4] = unsafe { mem::transmute(22u32.to_be()) };
    //     let b: [u8; 26] = [
    //         size[0],
    //         size[1],
    //         size[2],
    //         size[3],
    //         // {tag: 0, type: 6}
    //         0x06,
    //         7,
    //         b'f',
    //         b'o',
    //         b'o',
    //         b' ',
    //         b'b',
    //         b'a',
    //         b'r',
    //         // {tag: 0, type: 6}
    //         0x06,
    //         11,
    //         b'h',
    //         b'e',
    //         b'l',
    //         b'l',
    //         b'o',
    //         b' ',
    //         b'w',
    //         b'o',
    //         b'r',
    //         b'l',
    //         b'd',
    //     ];
    //     let mut de = TarsDecoder::from(&b[..]);
    //     let map: BTreeMap<String, String> = de.read(TarsTypeMark::EnMaps.value()).unwrap();
    //     let value2 = map.get(&String::from(&"foo bar"[..])).unwrap();
    //     assert_eq!(value2, &String::from(&"hello world"[..]));

    //     let b2: [u8; 5] = [0x48, 0, 0, 0, 0];
    //     let mut de2 = TarsDecoder::from(&b2[..]);
    //     let map2: BTreeMap<Vec<String>, BTreeMap<i32, String>> = de2.get(4).unwrap();
    //     assert_eq!(map2, BTreeMap::new());
    // }

    // #[test]
    // fn test_take_int64() {
    //     let b: [u8; 8] = unsafe { mem::transmute(0x0acb8b9d9d9d9d9di64.to_be()) };
    //     let mut de2 = TarsDecoder::from(&b[..]);
    //     let i: i64 = de2.read(TarsTypeMark::EnInt64.value()).unwrap();
    //     assert_eq!(i, 0x0acb8b9d9d9d9d9d);
    // }

    // #[test]
    // fn test_take_int32() {
    //     let b: [u8; 4] = unsafe { mem::transmute(0x0acb8b9di32.to_be()) };
    //     let mut de2 = TarsDecoder::from(&b[..]);
    //     let i: i32 = de2.read(TarsTypeMark::EnInt32.value()).unwrap();
    //     assert_eq!(i, 0x0acb8b9d);
    // }

    // #[test]
    // fn test_decode_int16() {
    //     let b: [u8; 2] = unsafe { mem::transmute(0x0acbi16.to_be()) };
    //     let mut de = TarsDecoder::from(&b[..]);
    //     assert_eq!(de.read(TarsTypeMark::EnInt16.value()), Ok(0x0acb as i16));
    //     assert_eq!(
    //         de.read::<i16>(TarsTypeMark::EnInt16.value()),
    //         Err(DecodeErr::NoEnoughDataErr)
    //     );

    //     // test int compress read u16 from u8
    //     let mut v = vec![];
    //     for i in 0..10 as u8 {
    //         let head = (i << 4) | TarsTypeMark::EnInt8.value();
    //         v.push(head);
    //         v.push(42 + i);
    //     }
    //     let mut de2 = TarsDecoder::from(v);

    //     for i in 0..10 as u8 {
    //         assert_eq!(de2.get(i), Ok((42 + i) as u16));
    //     }

    //     // test get i16
    //     let mut v2 = vec![];
    //     let value = -42i16;
    //     let value_arr: [u8; 2] = unsafe { mem::transmute(value.to_be()) };

    //     for i in 0..10 as u8 {
    //         let head = (i << 4) | TarsTypeMark::EnInt16.value();
    //         v2.push(head);
    //         v2.push(value_arr[0]);
    //         v2.push(value_arr[1]);
    //     }

    //     let mut de3 = TarsDecoder::from(v2);

    //     for i in 0..10 as u8 {
    //         assert_eq!(de3.get(i), Ok(value));
    //     }
    // }

    // #[test]
    // fn test_decode_int8() {
    //     let value: u8 = 1;
    //     let b: [u8; 10] = [value; 10];
    //     let mut de = TarsDecoder::from(&b[..]);
    //     for _ in 0..10 {
    //         assert_eq!(de.read(TarsTypeMark::EnInt8.value()), Ok(value));
    //     }

    //     assert_eq!(de.read::<u8>(0), Err(DecodeErr::NoEnoughDataErr));

    //     let value2: i8 = -1;
    //     let b2: [u8; 10] = [value2 as u8; 10];
    //     let mut de2 = TarsDecoder::from(&b2[..]);
    //     for _ in 0..10 {
    //         assert_eq!(de2.read(TarsTypeMark::EnInt8.value()), Ok(value2));
    //     }

    //     assert_eq!(
    //         de2.read::<i8>(TarsTypeMark::EnInt8.value()),
    //         Err(DecodeErr::NoEnoughDataErr)
    //     );

    //     let mut v = vec![];
    //     let value3: i8 = -42;
    //     for i in 0..10 as u8 {
    //         let head = (i << 4) | TarsTypeMark::EnInt8.value();
    //         v.push(head);
    //         v.push(value3 as u8);
    //     }
    //     let mut de3 = TarsDecoder::from(v);

    //     for i in 0..10 as u8 {
    //         assert_eq!(de3.get(i), Ok(value3));
    //     }
    // }

    // #[test]
    // fn test_decode_double() {
    //     let b2: [u8; 8] = unsafe { mem::transmute(0.633313f64.to_bits().to_be()) };
    //     let mut de2 = TarsDecoder::from(&b2[..]);
    //     let f: f64 = de2.read(TarsTypeMark::EnDouble.value()).unwrap();
    //     assert!(f == 0.633313f64);
    // }

    // #[test]
    // fn test_decode_float() {
    //     let b2: [u8; 4] = unsafe { mem::transmute(0.35524f32.to_bits().to_be()) };
    //     let mut de2 = TarsDecoder::from(&b2[..]);
    //     let f: f32 = de2.read(TarsTypeMark::EnFloat.value()).unwrap();
    //     assert!(f == 0.35524f32);
    // }

    #[test]
    fn test_decode_string() {
        // test read string1
        let d: [u8; 9] = [0x06, 0x07, b'f', b'o', b'o', b' ', b'b', b'a', b'r'];
        let mut de = TarsDecoder::from(&d[..]);
        assert_eq!(de.read_string(0).unwrap(), String::from(&"foo bar"[..]));

        // // test read string4
        // let size: [u8; 4] = unsafe { mem::transmute(7u32.to_be()) };
        // let d2: [u8; 11] = [
        //     size[0], size[1], size[2], size[3], b'f', b'o', b'o', b' ', b'b', b'a', b'r',
        // ];
        // let mut de2 = TarsDecoder::from(&d2[..]);
        // assert_eq!(
        //     de2.read(TarsTypeMark::EnString4.value()),
        //     Ok(String::from(&"foo bar"[..]))
        // );

        // // test get string by tag
        // let mut d3 = vec![];
        // d3.push(0x07);
        // d3.extend_from_slice(&d2);
        // let mut de3 = TarsDecoder::from(&d3[..]);
        // assert_eq!(de3.get(0), Ok(String::from(&"foo bar"[..])));
    }

    // #[test]
    // fn test_decode_bool() {
    //     let d: [u8; 3] = [0x0c, 0x10, 0x01];
    //     let mut de = TarsDecoder::from(&d[..]);
    //     let b: bool = de.get(0).unwrap();
    //     let b2: bool = de.get(1).unwrap();
    //     assert_eq!(b, false);
    //     assert_eq!(b2, true);
    // }

    // #[test]
    // fn test_decode_bytes() {
    //     let d: [u8; 18] = *b"\x9d\x00\x00\x00\x00\x0chello world!";
    //     let mut de = TarsDecoder::from(&d[..]);
    //     let b: Bytes = de.get(9).unwrap();
    //     assert_eq!(b, Bytes::from(&b"hello world!"[..]));
    // }
}
