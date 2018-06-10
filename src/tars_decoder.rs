use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::io::Cursor;

use bytes::{Buf, Bytes};

use errors::DecodeErr;
use tars_type::TarsTypeMark;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TarsDecoder {
    buf: Bytes,
    pos: usize,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Head {
    pub tag: u8,
    pub tars_type: u8,
    pub len: u8,
}

impl TarsDecoder {
    pub fn new(buf: &[u8]) -> TarsDecoder {
        let mut b = Bytes::new();
        b.extend_from_slice(buf);
        TarsDecoder { buf: b, pos: 0 }
    }
    // TODO: may not reset pos
    pub fn get_require<R: DecodeFrom>(&mut self, tag: u8) -> Result<R, DecodeErr> {
        self.pos = 0;
        if let Ok(head) = self.skip_to_tag(tag) {
            Ok(self.read::<R>(head.tars_type)?)
        } else {
            Err(DecodeErr::TagNotFoundErr)
        }
    }

    pub fn get_option<R: DecodeFrom>(&mut self, tag: u8) -> Result<Option<R>, DecodeErr> {
        self.pos = 0;
        if let Ok(head) = self.skip_to_tag(tag) {
            Ok(Some(self.read::<R>(head.tars_type)?))
        } else {
            Ok(None)
        }
    }

    pub fn has_remaining(&self) -> bool {
        self.pos < self.buf.len()
    }

    fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    fn skip_to_tag(&mut self, tag: u8) -> Result<Head, DecodeErr> {
        let mut result: Option<Head> = None;
        while self.has_remaining() {
            let head = self.take_head()?;
            println!("{:?}", head);
            if head.tag == tag {
                result = Some(head);
                break;
            } else {
                let taked_size = self.take_size(head.tars_type)?;
                self.pos += taked_size;
            }
        }
        match result {
            Some(h) => Ok(h),
            None => Err(DecodeErr::NoEnoughDataErr),
        }
    }

    pub fn read<R: DecodeFrom>(&mut self, tars_type: u8) -> Result<R, DecodeErr> {
        match tars_type {
            _ if tars_type == TarsTypeMark::EnZero.value() => {
                let b = Bytes::from(&b"\0"[..]);
                Ok(R::decode_from_bytes(&b)?)
            }
            _ if tars_type == TarsTypeMark::EnSimplelist.value() => {
                let head = self.take_head()?; // consume header (list type)
                if head.tars_type != TarsTypeMark::EnInt8.value() {
                    Err(DecodeErr::WrongSimpleListTarsTypeErr)
                } else {
                    let size = self.take_size(tars_type)?;
                    let value = self.take_then_advance(size)?;
                    // let _ = self.take_then_advance(1)?;
                    Ok(R::simple_list_from_bytes(&value)?)
                }
            }
            _ => {
                let size = self.take_size(tars_type)?;
                let value = self.take_then_advance(size)?;
                Ok(R::decode_from_bytes(&value)?)
            }
        }
    }

    pub fn take_head(&mut self) -> Result<Head, DecodeErr> {
        if self.remaining() < 1 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            let b = self.read::<u8>(TarsTypeMark::EnInt8.value())?;
            let tars_type = b & 0x0f;
            let mut tag = (b & 0xf0) >> 4;
            let len = if tag < 15 {
                1
            } else {
                tag = self.read::<u8>(TarsTypeMark::EnInt8.value())?;
                2
            };
            Ok(Head {
                tag,
                len,
                tars_type,
            })
        }
    }

    fn take_size(&mut self, tars_type: u8) -> Result<usize, DecodeErr> {
        match tars_type {
            _ if tars_type == TarsTypeMark::EnInt8.value() => Ok(1),
            _ if tars_type == TarsTypeMark::EnInt16.value() => Ok(2),
            _ if tars_type == TarsTypeMark::EnInt32.value() => Ok(4),
            _ if tars_type == TarsTypeMark::EnInt64.value() => Ok(8),
            _ if tars_type == TarsTypeMark::EnFloat.value() => Ok(4),
            _ if tars_type == TarsTypeMark::EnDouble.value() => Ok(8),
            _ if tars_type == TarsTypeMark::EnString1.value()
                || tars_type == TarsTypeMark::EnString4.value() =>
            {
                Ok(self.take_string_size(tars_type)?)
            }
            _ if tars_type == TarsTypeMark::EnMaps.value() => Ok(self.take_map_size()?),
            _ if tars_type == TarsTypeMark::EnList.value() => Ok(self.take_list_size()?),
            _ if tars_type == TarsTypeMark::EnStructBegin.value() => Ok(self.take_struct_size()?),
            _ if tars_type == TarsTypeMark::EnZero.value() => Ok(1),
            _ if tars_type == TarsTypeMark::EnSimplelist.value() => {
                Ok(self.take_simple_list_size()?)
            }
            _ => Err(DecodeErr::UnknownTarsTypeErr),
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

    fn take_string_size(&mut self, tars_type: u8) -> Result<usize, DecodeErr> {
        if tars_type == TarsTypeMark::EnString1.value() {
            Ok(self.read::<u8>(TarsTypeMark::EnInt8.value())? as usize)
        } else if tars_type == TarsTypeMark::EnString4.value() {
            Ok(self.read::<u32>(TarsTypeMark::EnInt32.value())? as usize)
        } else {
            Err(DecodeErr::UnknownTarsTypeErr)
        }
    }

    fn take_map_size(&mut self) -> Result<usize, DecodeErr> {
        Ok(self.read::<u32>(TarsTypeMark::EnInt32.value())? as usize)
    }

    fn take_list_size(&mut self) -> Result<usize, DecodeErr> {
        Ok(self.read::<u32>(TarsTypeMark::EnInt32.value())? as usize)
    }

    fn take_simple_list_size(&mut self) -> Result<usize, DecodeErr> {
        Ok(self.read::<u32>(TarsTypeMark::EnInt32.value())? as usize)
    }

    fn take_struct_size(&mut self) -> Result<usize, DecodeErr> {
        let before_pos = self.pos;
        // 0x0B means (tag, type) => (0, EnStructEnd) => (0, 11)
        let mut head = self.take_head()?;
        while head.tars_type != TarsTypeMark::EnStructEnd.value() {
            // 递归获取 struct 内部元素大小
            let ele_size = self.take_size(head.tars_type).unwrap();
            // 跳过元素内容
            self.pos += ele_size;
            // 获取下一个头部
            head = self.take_head()?;
        }
        // 获取当前位置
        let after_pos = self.pos;
        // rollback to before_pos
        self.pos = before_pos;
        Ok((after_pos - before_pos) as usize)
    }
}

pub trait DecodeFrom {
    fn decode_from_bytes(&Bytes) -> Result<Self, DecodeErr>
    where
        Self: Sized;
    fn simple_list_from_bytes(&Bytes) -> Result<Self, DecodeErr>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl DecodeFrom for i8 {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut cur = Cursor::new(b);
        Ok(cur.get_i8())
    }
}

impl DecodeFrom for u8 {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut cur = Cursor::new(b);
        Ok(cur.get_u8())
    }
}

impl DecodeFrom for bool {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let v = u8::decode_from_bytes(b)?;
        Ok(v != 0)
    }
}

impl DecodeFrom for i16 {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut cur = Cursor::new(b);
        if b.len() == 1 {
            Ok(i16::from(cur.get_i8()))
        } else {
            Ok(cur.get_i16_be())
        }
    }
}

impl DecodeFrom for u16 {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut cur = Cursor::new(b);
        if b.len() == 1 {
            Ok(u16::from(cur.get_u8()))
        } else {
            Ok(cur.get_u16_be())
        }
    }
}

impl DecodeFrom for i32 {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut cur = Cursor::new(b);
        if b.len() == 1 {
            Ok(i32::from(cur.get_i8()))
        } else if b.len() == 2 {
            Ok(i32::from(cur.get_i16_be()))
        } else {
            Ok(cur.get_i32_be())
        }
    }
}

impl DecodeFrom for u32 {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut cur = Cursor::new(b);
        if b.len() == 1 {
            Ok(u32::from(cur.get_u8()))
        } else if b.len() == 2 {
            Ok(u32::from(cur.get_u16_be()))
        } else {
            Ok(cur.get_u32_be())
        }
    }
}

impl DecodeFrom for i64 {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut cur = Cursor::new(b);
        if b.len() == 1 {
            Ok(i64::from(cur.get_i8()))
        } else if b.len() == 2 {
            Ok(i64::from(cur.get_i16_be()))
        } else if b.len() == 4 {
            Ok(i64::from(cur.get_i32_be()))
        } else {
            Ok(cur.get_i64_be())
        }
    }
}

impl DecodeFrom for u64 {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut cur = Cursor::new(b);
        if b.len() == 1 {
            Ok(u64::from(cur.get_u8()))
        } else if b.len() == 2 {
            Ok(u64::from(cur.get_u16_be()))
        } else if b.len() == 4 {
            Ok(u64::from(cur.get_u32_be()))
        } else {
            Ok(cur.get_u64_be())
        }
    }
}

impl DecodeFrom for f32 {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut cur = Cursor::new(b);
        Ok(cur.get_f32_be())
    }
}

impl DecodeFrom for f64 {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut cur = Cursor::new(b);
        Ok(cur.get_f64_be())
    }
}

impl DecodeFrom for String {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let cow = String::from_utf8_lossy(&b);
        Ok(String::from(cow.borrow()))
    }
}

// from struct decoding
impl DecodeFrom for Bytes {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        // clone will not copy [u8]
        Ok(Bytes::from(&b[0..b.len() - 1]))
        // b.clone()
    }
}

impl<K: DecodeFrom + Ord, V: DecodeFrom> DecodeFrom for BTreeMap<K, V> {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut map = BTreeMap::new();
        let mut decoder = TarsDecoder::new(&b);
        while decoder.has_remaining() {
            let key_head = decoder.take_head()?;
            let key = decoder.read::<K>(key_head.tars_type)?;
            let value_head = decoder.take_head()?;
            let value = decoder.read::<V>(value_head.tars_type)?;
            map.insert(key, value);
        }
        Ok(map)
    }
}

impl<T: DecodeFrom> DecodeFrom for Vec<T> {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut v = vec![];
        let mut decoder = TarsDecoder::new(&b);
        while decoder.has_remaining() {
            let ele_type = decoder.take_head()?;
            let ele = decoder.read::<T>(ele_type.tars_type)?;
            v.push(ele);
        }
        Ok(v)
    }

    fn simple_list_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut v: Vec<T> = vec![];
        let mut decoder = TarsDecoder::new(&b);
        while decoder.has_remaining() {
            let ele = decoder.read::<T>(TarsTypeMark::EnInt8.value())?;
            v.push(ele)
        }
        Ok(v)
    }
}

// impl<T: DecodeFrom> DecodeFrom for Option<T> {
//     fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
//         Ok(Some(T::decode_from_bytes(b)?))
//     }
// }

#[cfg(test)]
mod tests {
    use super::{DecodeFrom, TarsDecoder};
    use bytes::Bytes;
    use errors::DecodeErr;
    use std::collections::BTreeMap;
    use std::mem;
    use tars_type::TarsTypeMark;

    #[derive(Clone, Debug)]
    struct TestStruct {
        a: i8,       // tag 0
        b: u16,      // tag 1
        v1: Vec<u8>, // tag 2
        c: Option<String>, // tag 3 option
    }

    #[derive(Clone, Debug)]
    struct TestStruct2 {
        f: f32,                      // 0
        s: TestStruct,               // 1
        m: BTreeMap<String, String>, // 2
        s2: TestStruct,              // 3
        y: Option<u8>,               // 4 option
    }

    impl DecodeFrom for TestStruct2 {
        fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
            let mut de = TarsDecoder::new(&b);
            let s = TestStruct::decode_from_bytes(&de.get_require(1)?)?;
            let s2 = TestStruct::decode_from_bytes(&de.get_require(3)?)?;
            let m = de.get_require(2)?;
            let f = de.get_require(0)?;
            let y = de.get_option(4)?;
            Ok(TestStruct2 { f, s, m, s2, y })
        }
    }

    impl DecodeFrom for TestStruct {
        fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
            println!("{:?}", b);
            let mut de = TarsDecoder::new(&b);
            let a = de.get_require(0)?;
            let b = de.get_require(1)?;
            let v1 = de.get_require(2)?;
            let c = de.get_option(3)?;
            Ok(TestStruct { a, b, v1, c })
        }
    }

    #[test]
    fn test_decode_struct() {
        let i8_field_0: i8 = -127;

        let u16_field_1: [u8; 2] = unsafe { mem::transmute(0x0acbi16.to_be()) };

        let list_head_2: [u8; 4] = unsafe { mem::transmute(4u32.to_be()) };
        let list_field_2: Vec<u8> = vec![4, 5, 6, 7];

        let buf: [u8; 15] = [
            0x00, // {i8 field start, tag 0}
            i8_field_0 as u8,
            0x11, // {u16 field start, tag 1}
            u16_field_1[0],
            u16_field_1[1],
            0x2d, // {simple list field start, tag 2}
            0x00,
            list_head_2[0],
            list_head_2[1],
            list_head_2[2],
            list_head_2[3],
            list_field_2[0],
            list_field_2[1],
            list_field_2[2],
            list_field_2[3], // {simple list field end}
        ];

        let s = TestStruct::decode_from_bytes(&Bytes::from(&buf[..])).unwrap();
        assert_eq!(s.a, i8_field_0);
        assert_eq!(s.b, 0x0acbi16 as u16);
        assert_eq!(s.v1, list_field_2);
        assert_eq!(s.c, None);
        let size: [u8; 4] = unsafe { mem::transmute(22u32.to_be()) };
        let map_field: [u8; 26] = [
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

        let float_field: [u8; 4] = unsafe { mem::transmute(0.332134f32.to_bits().to_be()) };

        let mut bytes1 = Bytes::new();
        bytes1.extend_from_slice(&b"\x1a"[..]); // struct begin, tag 1
        bytes1.extend_from_slice(&buf); // struct begin, tag 1
        bytes1.extend_from_slice(&b"\x0b"[..]); // struct begin, tag 1

        bytes1.extend_from_slice(&b"\x3a"[..]); // struct begin, tag 3
        bytes1.extend_from_slice(&buf); // struct begin, tag 1
        bytes1.extend_from_slice(&b"\x0b"[..]); // struct begin, tag 3

        bytes1.extend_from_slice(&b"\x28"[..]);
        bytes1.extend_from_slice(&map_field);

        bytes1.extend_from_slice(&b"\x04"[..]);
        bytes1.extend_from_slice(&float_field);

        let u8_option_field_4: [u8; 1] = [128];
        bytes1.extend_from_slice(&b"\x40"[..]);
        bytes1.extend_from_slice(&u8_option_field_4);

        let s2 = TestStruct2::decode_from_bytes(&bytes1).unwrap();
        assert_eq!(s2.s.a, i8_field_0);
        assert_eq!(s2.s.b, 0x0acbi16 as u16);
        assert_eq!(s2.s.v1, list_field_2);
        assert_eq!(s2.s.c, None);
        assert_eq!(s2.s2.a, i8_field_0);
        assert_eq!(s2.s2.b, 0x0acbi16 as u16);
        assert_eq!(s2.s2.v1, list_field_2);
        assert_eq!(s2.s2.c, None);

        let value2 = s2.m.get(&String::from(&"foo bar"[..])).unwrap();
        assert_eq!(value2, &String::from(&"hello world"[..]));

        assert_approx_eq!(s2.f, 0.332134f32);

        assert_eq!(s2.y, Some(128));

    }

    #[test]
    fn test_take_simple_list() {
        let head: [u8; 4] = unsafe { mem::transmute(4u32.to_be()) };
        let b: [u8; 9] = [
            0x00, // {tag: 0, type: 0}
            head[0],
            head[1],
            head[2],
            head[3],
            4,
            5,
            6,
            7,
        ];
        let mut de = TarsDecoder::new(&b);
        let list: Vec<u8> = de.read(TarsTypeMark::EnSimplelist.value()).unwrap();
        let result: Vec<u8> = vec![4, 5, 6, 7];
        assert_eq!(list, result);
    }

    #[test]
    fn test_decode_zero() {
        let mut de = TarsDecoder::new(&b""[..]);
        let v0: u8 = de.read(TarsTypeMark::EnZero.value()).unwrap();
        let v1: u16 = de.read(TarsTypeMark::EnZero.value()).unwrap();
        let v2: u32 = de.read(TarsTypeMark::EnZero.value()).unwrap();
        let v3: u64 = de.read(TarsTypeMark::EnZero.value()).unwrap();
        let v4: i8 = de.read(TarsTypeMark::EnZero.value()).unwrap();
        let v5: i16 = de.read(TarsTypeMark::EnZero.value()).unwrap();
        let v6: i32 = de.read(TarsTypeMark::EnZero.value()).unwrap();
        let v7: i64 = de.read(TarsTypeMark::EnZero.value()).unwrap();

        assert_eq!(v0, 0);
        assert_eq!(v1, 0);
        assert_eq!(v2, 0);
        assert_eq!(v3, 0);
        assert_eq!(v4, 0);
        assert_eq!(v5, 0);
        assert_eq!(v6, 0);
        assert_eq!(v7, 0);
    }

    #[test]
    fn test_decode_list() {
        let size: [u8; 4] = unsafe { mem::transmute(22u32.to_be()) };
        let b: [u8; 26] = [
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
        let mut de = TarsDecoder::new(&b[..]);
        let list: Vec<String> = de.read(TarsTypeMark::EnList.value()).unwrap();
        assert_eq!(list[0], String::from(&"foo bar"[..]));
        assert_eq!(list[1], String::from(&"hello world"[..]));

        assert_eq!(
            de.read::<Vec<String>>(TarsTypeMark::EnList.value()),
            Err(DecodeErr::NoEnoughDataErr)
        );
    }

    #[test]
    fn test_take_map() {
        let size: [u8; 4] = unsafe { mem::transmute(22u32.to_be()) };
        let b: [u8; 26] = [
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
        let mut de2 = TarsDecoder::new(&b[..]);
        let map = de2
            .read::<BTreeMap<String, String>>(TarsTypeMark::EnMaps.value())
            .unwrap();
        let value2 = map.get(&String::from(&"foo bar"[..])).unwrap();
        assert_eq!(value2, &String::from(&"hello world"[..]));
    }

    #[test]
    fn test_take_int64() {
        let b: [u8; 8] = unsafe { mem::transmute(0x0acb8b9d9d9d9d9di64.to_be()) };
        let mut de2 = TarsDecoder::new(&b[..]);
        let i: i64 = de2.read(TarsTypeMark::EnInt64.value()).unwrap();
        assert_eq!(i, 0x0acb8b9d9d9d9d9d);
    }

    #[test]
    fn test_take_int32() {
        let b: [u8; 4] = unsafe { mem::transmute(0x0acb8b9di32.to_be()) };
        let mut de2 = TarsDecoder::new(&b[..]);
        let i: i32 = de2.read(TarsTypeMark::EnInt32.value()).unwrap();
        assert_eq!(i, 0x0acb8b9d);
    }

    #[test]
    fn test_decode_int16() {
        let b: [u8; 2] = unsafe { mem::transmute(0x0acbi16.to_be()) };
        let mut de = TarsDecoder::new(&b[..]);
        assert_eq!(de.read(TarsTypeMark::EnInt16.value()), Ok(0x0acb as i16));
        assert_eq!(
            de.read::<i16>(TarsTypeMark::EnInt16.value()),
            Err(DecodeErr::NoEnoughDataErr)
        );

        // test int compress read u16 from u8
        let mut v = vec![];
        for i in 0..10 as u8 {
            let head = (i << 4) | TarsTypeMark::EnInt8.value();
            v.push(head);
            v.push(42 + i);
        }
        let mut de2 = TarsDecoder::new(&v);

        for i in 0..10 as u8 {
            assert_eq!(de2.get_require::<u16>(i), Ok((42 + i) as u16));
        }

        // test get i16
        let mut v2 = vec![];
        let value = -42i16;
        let value_arr: [u8; 2] = unsafe { mem::transmute(value.to_be()) };

        for i in 0..10 as u8 {
            let head = (i << 4) | TarsTypeMark::EnInt16.value();
            v2.push(head);
            v2.push(value_arr[0]);
            v2.push(value_arr[1]);
        }

        let mut de3 = TarsDecoder::new(&v2);

        for i in 0..10 as u8 {
            assert_eq!(de3.get_require::<i16>(i), Ok(value));
        }
    }

    #[test]
    fn test_decode_int8() {
        let value: u8 = 1;
        let b: [u8; 10] = [value; 10];
        let mut de = TarsDecoder::new(&b[..]);
        for _ in 0..10 {
            assert_eq!(de.read(TarsTypeMark::EnInt8.value()), Ok(value));
        }

        assert_eq!(de.read::<u8>(0), Err(DecodeErr::NoEnoughDataErr));

        let value2: i8 = -1;
        let b2: [u8; 10] = [value2 as u8; 10];
        let mut de2 = TarsDecoder::new(&b2[..]);
        for _ in 0..10 {
            assert_eq!(de2.read(TarsTypeMark::EnInt8.value()), Ok(value2));
        }

        assert_eq!(
            de2.read::<i8>(TarsTypeMark::EnInt8.value()),
            Err(DecodeErr::NoEnoughDataErr)
        );

        let mut v = vec![];
        let value3: i8 = -42;
        for i in 0..10 as u8 {
            let head = (i << 4) | TarsTypeMark::EnInt8.value();
            v.push(head);
            v.push(value3 as u8);
        }
        let mut de3 = TarsDecoder::new(&v);

        for i in 0..10 as u8 {
            assert_eq!(de3.get_require(i), Ok(value3));
        }
    }

    #[test]
    fn test_decode_double() {
        let b2: [u8; 8] = unsafe { mem::transmute(0.633313f64.to_bits().to_be()) };
        let mut de2 = TarsDecoder::new(&b2[..]);
        let f: f64 = de2.read(TarsTypeMark::EnDouble.value()).unwrap();
        assert_approx_eq!(f, 0.633313f64);
    }

    #[test]
    fn test_decode_float() {
        let b2: [u8; 4] = unsafe { mem::transmute(0.35524f32.to_bits().to_be()) };
        let mut de2 = TarsDecoder::new(&b2[..]);
        let f: f32 = de2.read(TarsTypeMark::EnFloat.value()).unwrap();
        assert_approx_eq!(f, 0.35524f32);
    }

    #[test]
    fn test_decode_string() {
        // test read string1
        let d: [u8; 8] = [7, b'f', b'o', b'o', b' ', b'b', b'a', b'r'];
        let mut de = TarsDecoder::new(&d);
        assert_eq!(
            de.read(TarsTypeMark::EnString1.value()),
            Ok(String::from(&"foo bar"[..]))
        );

        // test read string4
        let size: [u8; 4] = unsafe { mem::transmute(7u32.to_be()) };
        let d2: [u8; 11] = [
            size[0], size[1], size[2], size[3], b'f', b'o', b'o', b' ', b'b', b'a', b'r',
        ];
        let mut de2 = TarsDecoder::new(&d2);
        assert_eq!(
            de2.read(TarsTypeMark::EnString4.value()),
            Ok(String::from(&"foo bar"[..]))
        );

        // test get string by tag
        let mut d3 = vec![];
        d3.push(0x07);
        d3.extend_from_slice(&d2);
        let mut de3 = TarsDecoder::new(&d3);
        assert_eq!(de3.get_require(0), Ok(String::from(&"foo bar"[..])));
    }
}
