use std::collections::BTreeMap;
use std::io::Cursor;

use bytes::{Buf, Bytes};

use errors::DecodeErr;
use tars_type::TarsTypeMark;
use tars_type::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TarsStructDecoder {
    buf: Bytes,
    pos: usize,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Head {
    pub tag: u8,
    pub tars_type: u8,
    pub len: u8,
}

impl TarsStructDecoder {
    pub fn new(buf: &[u8]) -> TarsStructDecoder {
        let mut b = Bytes::new();
        b.extend_from_slice(buf);
        TarsStructDecoder { buf: b, pos: 0 }
    }
    // TODO: may not reset pos
    pub fn get<R: DecodeFrom>(&mut self, tag: u8) -> Result<R, DecodeErr> {
        self.pos = 0;
        if let Ok(head) = self.skip_to_tag(tag) {
            Ok(self.read::<R>(head.tars_type)?)
        } else {
            Err(DecodeErr::TagNotFoundErr)
        }
    }

    pub fn has_remaining(&self) -> bool {
        self.pos < self.buf.len()
    }

    fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    fn get_u8(&mut self) -> u8 {
        self.pos += 1;
        self.buf[self.pos - 1]
    }

    fn get_i8(&mut self) -> i8 {
        self.get_u8() as i8
    }

    fn get_i16_be(&mut self) -> i16 {
        self.pos += 2;
        let mut cur = Cursor::new(&self.buf[self.pos - 2..]);
        cur.get_i16_be()
    }

    fn get_i32_be(&mut self) -> i32 {
        self.pos += 4;
        let mut cur = Cursor::new(&self.buf[self.pos - 4..]);
        cur.get_i32_be()
    }

    fn get_i64_be(&mut self) -> i64 {
        self.pos += 8;
        let mut cur = Cursor::new(&self.buf[self.pos - 8..]);
        cur.get_i64_be()
    }

    fn skip_to_tag(&mut self, tag: u8) -> Result<Head, DecodeErr> {
        let mut result: Option<Head> = None;
        while self.has_remaining() {
            let head = self.take_head()?;
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
            _ if tars_type == TarsTypeMark::EnInt8.value() => {
                let value = self.take_int8()?;
                Ok(R::decode_from_bytes(&value))
            }
            _ if tars_type == TarsTypeMark::EnInt16.value() => {
                let value = self.take_int16()?;
                Ok(R::decode_from_bytes(&value))
            }
            _ if tars_type == TarsTypeMark::EnInt32.value() => {
                let value = self.take_int32()?;
                Ok(R::decode_from_bytes(&value))
            }
            // _ if tars_type == TarsTypeMark::EnInt64.value() => {
            //     let value = self.take_int64()?;
            //     Ok(EnInt64(value))
            // }
            // _ if tars_type == TarsTypeMark::EnFloat.value() => {
            //     let value = self.take_float()?;
            //     Ok(EnFloat(value))
            // }
            // _ if tars_type == TarsTypeMark::EnDouble.value() => {
            //     let value = self.take_double()?;
            //     Ok(EnDouble(value))
            // }
            _ if tars_type == TarsTypeMark::EnString1.value()
                || tars_type == TarsTypeMark::EnString4.value() =>
            {
                let size = self.take_string_size(tars_type)?;
                let value = self.take_string(size)?;
                Ok(R::decode_from_bytes(&value))
            }
            _ if tars_type == TarsTypeMark::EnMaps.value() => {
                let size = self.take_map_size()?;
                let value = self.take_map(size)?;
                Ok(R::decode_from_bytes(&value))
            }
            // _ if tars_type == TarsTypeMark::EnList.value() => {
            //     let size = self.take_list_size()?;
            //     let value = self.take_list(size)?;
            //     Ok(EnList(value))
            // }
            // _ if tars_type == TarsTypeMark::EnStructBegin.value() => {
            //     let size = self.take_struct_size()?;
            //     let value = self.take_struct(size)?;
            //     Ok(EnStruct(value))
            // }
            _ if tars_type == TarsTypeMark::EnZero.value() => {
                let b = Bytes::from(&b"\0"[..]);
                Ok(R::decode_from_bytes(&b))
            }
            // // TODO: add more test
            // _ if tars_type == TarsTypeMark::EnSimplelist.value() => {
            //     let value = self.take_simple_list()?;
            //     Ok(EnList(value))
            // }
            _ => Err(DecodeErr::UnknownTarsTypeErr),
        }
    }

    pub fn take_head(&mut self) -> Result<Head, DecodeErr> {
        if self.remaining() < 1 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            let b = self.get_u8();
            let tars_type = b & 0x0f;
            let mut tag = (b & 0xf0) >> 4;
            let len = if tag < 15 {
                1
            } else {
                tag = self.get_u8();
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
            // _ if tars_type == TarsTypeMark::EnInt32.value() => Ok(4),
            // _ if tars_type == TarsTypeMark::EnInt64.value() => Ok(8),
            // _ if tars_type == TarsTypeMark::EnFloat.value() => Ok(4),
            // _ if tars_type == TarsTypeMark::EnDouble.value() => Ok(8),
            // _ if tars_type == TarsTypeMark::EnString1.value()
            //     || tars_type == TarsTypeMark::EnString4.value() =>
            // {
            //     Ok(self.take_string_size(tars_type)?)
            // }
            // _ if tars_type == TarsTypeMark::EnMaps.value() => Ok(self.take_map_size()?),
            // _ if tars_type == TarsTypeMark::EnList.value() => Ok(self.take_list_size()?),
            // _ if tars_type == TarsTypeMark::EnStructBegin.value() => Ok(self.take_struct_size()?),
            // _ if tars_type == TarsTypeMark::EnZero.value() => Ok(1),
            // _ if tars_type == TarsTypeMark::EnSimplelist.value() => {
            //     Ok(self.take_simple_list_size()?)
            // }
            _ => Err(DecodeErr::UnknownTarsTypeErr),
        }
    }

    fn take_then_advance(&mut self, size: usize) -> Bytes {
        let b = self.buf.slice(self.pos, self.pos + size);
        self.pos += size;
        b
    }

    fn take_int8(&mut self) -> Result<Bytes, DecodeErr> {
        if self.remaining() < 1 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.take_then_advance(1))
        }
    }

    fn take_int16(&mut self) -> Result<Bytes, DecodeErr> {
        if self.remaining() < 2 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.take_then_advance(2))
        }
    }

    fn take_int32(&mut self) -> Result<Bytes, DecodeErr> {
        if self.remaining() < 4 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.take_then_advance(4))
        }
    }

    // fn take_int64(&mut self) -> Result<i64, DecodeErr> {
    //     if self.remaining() < 8 {
    //         Err(DecodeErr::NoEnoughDataErr)
    //     } else {
    //         Ok(self.get_i64_be())
    //     }
    // }

    // fn take_float(&mut self) -> Result<u32, DecodeErr> {
    //     if self.remaining() < 4 {
    //         Err(DecodeErr::NoEnoughDataErr)
    //     } else {
    //         Ok(self.get_i32_be() as u32)
    //     }
    // }

    // fn take_double(&mut self) -> Result<u64, DecodeErr> {
    //     if self.remaining() < 8 {
    //         Err(DecodeErr::NoEnoughDataErr)
    //     } else {
    //         Ok(self.get_i64_be() as u64)
    //     }
    // }

    fn take_string_size(&mut self, tars_type: u8) -> Result<usize, DecodeErr> {
        if tars_type == TarsTypeMark::EnString1.value() {
            Ok(self.read::<u8>(TarsTypeMark::EnInt8.value())? as usize)
        } else if tars_type == TarsTypeMark::EnString4.value() {
            Ok(self.read::<u32>(TarsTypeMark::EnInt32.value())? as usize)
        } else {
            Err(DecodeErr::UnknownTarsTypeErr)
        }
    }

    fn take_string(&mut self, size: usize) -> Result<Bytes, DecodeErr> {
        if self.remaining() < size {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.take_then_advance(size))
        }
    }

    fn take_map_size(&mut self) -> Result<usize, DecodeErr> {
        Ok(self.read::<u32>(TarsTypeMark::EnInt32.value())? as usize)
    }

    fn take_map(&mut self, size: usize) -> Result<Bytes, DecodeErr> {
        // let before_pos = self.pos;
        // while self.pos < before_pos + size {
        //     let key_head = self.take_head()?;
        //     let key = self.read::<K>(key_head.tars_type)?;
        //     let value_head = self.take_head()?;
        //     let value = self.read::<V>(value_head.tars_type)?;
        //     map.insert(key, value);
        // }
        Ok(self.take_then_advance(size))
    }

    // fn take_map2<K: DecodeFrom + Ord, V: DecodeFrom + Ord>(
    //     &mut self,
    //     size: usize,
    // ) -> Result<BTreeMap<K, V>, DecodeErr> {
    //     let mut map: BTreeMap<K, V> = BTreeMap::new();
    //     let before_pos = self.pos;
    //     while self.pos < before_pos + size {
    //         let key_head = self.take_head()?;

    //     }
    //     map.insert(
    //         K::decode_from_bytes(&Bytes::new()),
    //         V::decode_from_bytes(&Bytes::new()),
    //     );
    //     Ok(map)
    // }

    fn take_list_size(&mut self) -> Result<usize, DecodeErr> {
        Ok(self.get_i32_be() as usize)
    }

    fn take_list(&mut self, size: usize) -> Result<Bytes, DecodeErr> {
        Ok(self.take_then_advance(size))
    }

    // fn take_simple_list_size(&mut self) -> Result<usize, DecodeErr> {
    //     Ok(self.take_int32()? as usize)
    // }

    // fn take_simple_list(&mut self) -> Result<TarsList, DecodeErr> {
    //     let mut v = vec![];
    //     let head = self.take_head()?;
    //     if head.tars_type != TarsTypeMark::EnInt8.value() {
    //         Err(DecodeErr::WrongSimpleListTarsTypeErr)
    //     } else {
    //         let size = self.take_simple_list_size()?;
    //         let before_pos = self.pos;
    //         while self.pos < before_pos + size {
    //             v.push(EnInt8(self.get_i8()));
    //         }
    //         assert_eq!(self.pos, before_pos + size);
    //         Ok(v)
    //     }
    // }

    // fn take_struct_size(&mut self) -> Result<usize, DecodeErr> {
    //     let before_pos = self.pos;
    //     // 0x0B means (tag, type) => (0, EnStructEnd) => (0, 11)
    //     let mut head = self.take_head()?;
    //     while head.tars_type != TarsTypeMark::EnStructEnd.value() {
    //         // 递归获取 struct 内部元素大小
    //         let ele_size = self.take_size(head.tars_type).unwrap();
    //         // 跳过元素内容
    //         self.pos += ele_size;
    //         // 获取下一个头部
    //         head = self.take_head()?;
    //     }
    //     // 获取当前位置
    //     let after_pos = self.pos;
    //     // rollback to before_pos
    //     self.pos = before_pos;
    //     Ok((after_pos - before_pos) as usize)
    // }

    // fn take_struct(&mut self, size: usize) -> Result<Bytes, DecodeErr> {
    //     // Abandon StructEnd (-1)
    //     let (left, right) = (self.pos, self.pos + size - 1);
    //     self.pos += size;
    //     let mut b = Bytes::new();
    //     b.extend_from_slice(&self.buf[left..right]);
    //     Ok(b)
    // }

    // fn take_struct2(&mut self, size: usize, s: Rc<TarsStruct>) -> Result<(), DecodeErr> {
    //     let (left, right) = (self.pos, self.pos + size - 1);
    //     self.pos += size;
    //     let mut b = Bytes::new();
    //     b.extend_from_slice(&self.buf[left..right]);
    //     *s.from_bytes(b);
    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use super::TarsStructDecoder;
    use errors::DecodeErr;
    use std::collections::BTreeMap;
    use std::mem;
    use tars_type::TarsTypeMark;

    //     #[test]
    //     fn test_get() {
    //         let d: [u8; 44] = [
    //             // {tag: 1, type: 6}
    //             0x16,
    //             7,
    //             b'f',
    //             b'o',
    //             b'o',
    //             b' ',
    //             b'b',
    //             b'a',
    //             b'r',
    //             // {tag: 2, type: 6}
    //             0x26,
    //             7,
    //             b'f',
    //             b'o',
    //             b'o',
    //             b' ',
    //             b'b',
    //             b'a',
    //             b'r',
    //             // {tag: 3, type: 0}
    //             0x30,
    //             8,
    //             // {tag: 4, type: 10} start inner struct
    //             0x4A,
    //             // {struct: {tag: 1, type: 6} }
    //             0x16,
    //             7,
    //             b'f',
    //             b'o',
    //             b'o',
    //             b' ',
    //             b'b',
    //             b'a',
    //             b'r',
    //             // {struct: {tag: 2, type: 6} }
    //             0x26,
    //             7,
    //             b'f',
    //             b'o',
    //             b'o',
    //             b' ',
    //             b'b',
    //             b'a',
    //             b'r',
    //             // {struct: {tag: 3, type: 6} }
    //             0x30,
    //             8,
    //             // StructEnd
    //             0x0B,
    //             // {struct: {tag: 5, type: 6} }
    //             0x50,
    //             16,
    //         ];

    //         let mut de = TarsStructDecoder::new(&d);

    //         for _ in 0..10 {
    //             assert_eq!(de.get(1), Ok(EnString(String::from(&"foo bar"[..]))));
    //             assert_eq!(de.get(2), Ok(EnString(String::from(&"foo bar"[..]))));
    //             assert_eq!(de.get(3), Ok(EnInt8(8)));
    //             assert_eq!(de.get(5), Ok(EnInt8(16)));

    //             let mut inner_struct = de.get(4).unwrap().unwrap_struct().unwrap();
    //             let mut inner_struct_de = TarsStructDecoder::new(&inner_struct);
    //             assert_eq!(
    //                 inner_struct_de.get(1),
    //                 Ok(EnString(String::from(&"foo bar"[..])))
    //             );
    //             assert_eq!(
    //                 inner_struct_de.get(2),
    //                 Ok(EnString(String::from(&"foo bar"[..])))
    //             );
    //             assert_eq!(inner_struct_de.get(3), Ok(EnInt8(8)));
    //             assert_eq!(de.get(0), Err(DecodeErr::TagNotFoundErr));
    //             assert_eq!(de.get(200), Err(DecodeErr::TagNotFoundErr));
    //             assert_eq!(de.get(255), Err(DecodeErr::TagNotFoundErr));
    //         }
    //     }

    //     #[test]
    //     fn test_take_simple_list() {
    //         let head: [u8; 4] = unsafe { mem::transmute(4u32.to_be()) };
    //         let b: [u8; 9] = [
    //             0x00, // {tag: 0, type: 0}
    //             head[0],
    //             head[1],
    //             head[2],
    //             head[3],
    //             4,
    //             5,
    //             6,
    //             7,
    //         ];
    //         let mut de = TarsStructDecoder::new(&b);
    //         let list = de.take_simple_list().unwrap();
    //         assert_eq!(list, vec![EnInt8(4), EnInt8(5), EnInt8(6), EnInt8(7)]);
    //     }

    //     #[test]
    //     fn test_take_zero() {
    //         let b: [u8; 2] = [0x0C; 2];
    //         let mut de = TarsStructDecoder::new(&b);
    //         let head = de.take_head().unwrap();
    //         assert_eq!(de.read(head.tars_type).unwrap(), EnInt8(0));

    //         let head = de.take_head().unwrap();
    //         assert_eq!(de.read(head.tars_type).unwrap(), EnInt8(0));

    //         assert_eq!(de.take_head(), Err(DecodeErr::NoEnoughDataErr));
    //     }

    //     #[test]
    //     fn test_take_list() {
    //         let size: [u8; 4] = unsafe { mem::transmute(26u32.to_be()) };
    //         let b: [u8; 30] = [
    //             size[0],
    //             size[1],
    //             size[2],
    //             size[3],
    //             // {tag: 0, type: 6}
    //             0x06,
    //             7,
    //             b'f',
    //             b'o',
    //             b'o',
    //             b' ',
    //             b'b',
    //             b'a',
    //             b'r',
    //             // {tag: 0, type: 6}
    //             0x06,
    //             11,
    //             b'h',
    //             b'e',
    //             b'l',
    //             b'l',
    //             b'o',
    //             b' ',
    //             b'w',
    //             b'o',
    //             b'r',
    //             b'l',
    //             b'd',
    //             // {tag: 0, type: 0}
    //             0x00,
    //             8,
    //             // {tag: 0, type: 0}
    //             0x00,
    //             9,
    //         ];
    //         let mut de = TarsStructDecoder::new(&b[..]);
    //         let list_size = de.take_list_size().unwrap();
    //         assert_eq!(list_size, 26);
    //         let list = de.take_list(list_size).unwrap();
    //         assert_eq!(list[0], EnString(String::from(&"foo bar"[..])));
    //         assert_eq!(list[1], EnString(String::from(&"hello world"[..])));
    //         assert_eq!(list[2], EnInt8(8));
    //         assert_eq!(list[3], EnInt8(9));
    //     }

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
        let mut de2 = TarsStructDecoder::new(&b[..]);
        let map = de2.read::<BTreeMap<String, String>>(TarsTypeMark::EnMaps.value())
            .unwrap();
        let value2 = map.get(&String::from(&"foo bar"[..])).unwrap();
        assert_eq!(value2, &String::from(&"hello world"[..]));
    }

    //     #[test]
    //     fn test_take_int64() {
    //         let b: [u8; 8] = unsafe { mem::transmute(0x0acb8b9d9d9d9d9di64.to_be()) };
    //         let mut de2 = TarsStructDecoder::new(&b[..]);
    //         let i = de2.take_int64();
    //         assert_eq!(i, Ok(0x0acb8b9d9d9d9d9d));
    //         assert_eq!(de2.take_int64(), Err(DecodeErr::NoEnoughDataErr));
    //     }

    //     #[test]
    //     fn test_take_int32() {
    //         let b: [u8; 4] = unsafe { mem::transmute(0x0acb8b9di32.to_be()) };
    //         let mut de2 = TarsStructDecoder::new(&b[..]);
    //         assert_eq!(de2.take_int32(), Ok(0x0acb8b9d));
    //         assert_eq!(de2.take_int32(), Err(DecodeErr::NoEnoughDataErr));
    //     }

    #[test]
    fn test_decode_int16() {
        let b: [u8; 2] = unsafe { mem::transmute(0x0acbi16.to_be()) };
        let mut de = TarsStructDecoder::new(&b[..]);
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
        let mut de2 = TarsStructDecoder::new(&v);

        for i in 0..10 as u8 {
            assert_eq!(de2.get::<u16>(i), Ok((42 + i) as u16));
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

        let mut de3 = TarsStructDecoder::new(&v2);

        for i in 0..10 as u8 {
            assert_eq!(de3.get::<i16>(i), Ok(value));
            println!("hello");
        }
    }

    #[test]
    fn test_decode_int8() {
        let value: u8 = 1;
        let b: [u8; 10] = [value; 10];
        let mut de = TarsStructDecoder::new(&b[..]);
        for _ in 0..10 {
            assert_eq!(de.read(TarsTypeMark::EnInt8.value()), Ok(value));
        }

        assert_eq!(de.read::<u8>(0), Err(DecodeErr::NoEnoughDataErr));

        let value2: i8 = -1;
        let b2: [u8; 10] = [value2 as u8; 10];
        let mut de2 = TarsStructDecoder::new(&b2[..]);
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
        let mut de3 = TarsStructDecoder::new(&v);

        for i in 0..10 as u8 {
            assert_eq!(de3.get(i), Ok(value3));
        }
    }

    //     #[test]
    //     fn test_take_double() {
    //         let b2: [u8; 8] = unsafe { mem::transmute(0.6f64.to_bits().to_be()) };
    //         let mut de2 = TarsStructDecoder::new(&b2[..]);
    //         let f = f64::from_bits(de2.take_double().unwrap());
    //         assert_approx_eq!(f, 0.6f64);
    //         assert_eq!(de2.take_float(), Err(DecodeErr::NoEnoughDataErr));
    //     }

    //     #[test]
    //     fn test_take_float() {
    //         let b2: [u8; 4] = unsafe { mem::transmute(0.3f32.to_bits().to_be()) };
    //         let mut de2 = TarsStructDecoder::new(&b2[..]);
    //         let f = f32::from_bits(de2.take_float().unwrap());
    //         assert_approx_eq!(f, 0.3f32);
    //         assert_eq!(de2.take_float(), Err(DecodeErr::NoEnoughDataErr));
    //     }

    #[test]
    fn test_decode_string() {
        // test read string1
        let d: [u8; 8] = [7, b'f', b'o', b'o', b' ', b'b', b'a', b'r'];
        let mut de = TarsStructDecoder::new(&d);
        assert_eq!(
            de.read(TarsTypeMark::EnString1.value()),
            Ok(String::from(&"foo bar"[..]))
        );

        // test read string4
        let size: [u8; 4] = unsafe { mem::transmute(7u32.to_be()) };
        let d2: [u8; 11] = [
            size[0], size[1], size[2], size[3], b'f', b'o', b'o', b' ', b'b', b'a', b'r',
        ];
        let mut de2 = TarsStructDecoder::new(&d2);
        assert_eq!(
            de2.read(TarsTypeMark::EnString4.value()),
            Ok(String::from(&"foo bar"[..]))
        );

        // test get string by tag
        let mut d3 = vec![];
        d3.push(0x07);
        d3.extend_from_slice(&d2);
        let mut de3 = TarsStructDecoder::new(&d3);
        assert_eq!(de3.get(0), Ok(String::from(&"foo bar"[..])));
    }

    // #[test]
    // fn test_take_struct() {
    //     let d: [u8; 42] = [
    //         // {tag: 1, type: 6}
    //         0x16,
    //         7,
    //         b'f',
    //         b'o',
    //         b'o',
    //         b' ',
    //         b'b',
    //         b'a',
    //         b'r',
    //         // {tag: 2, type: 6}
    //         0x26,
    //         7,
    //         b'f',
    //         b'o',
    //         b'o',
    //         b' ',
    //         b'b',
    //         b'a',
    //         b'r',
    //         // {tag: 3, type: 0}
    //         0x30,
    //         8,
    //         // StructEnd
    //         0x0B,
    //         // {tag: 4, type: 6}
    //         0x46,
    //         7,
    //         b'f',
    //         b'o',
    //         b'o',
    //         b' ',
    //         b'b',
    //         b'a',
    //         b'r',
    //         // {tag: 5, type: 6}
    //         0x56,
    //         7,
    //         b'f',
    //         b'o',
    //         b'o',
    //         b' ',
    //         b'b',
    //         b'a',
    //         b'r',
    //         // {tag: 6, type: 0}
    //         0x60,
    //         8,
    //         // StructEnd
    //         0x0B,
    //     ];
    //     let mut de = TarsStructDecoder::new(&d);

    //     let struct_size = de.take_struct_size().unwrap();
    //     match de.take_struct(struct_size) {
    //         Ok(inner_struct) => {
    //             let mut inner_struct_de = TarsStructDecoder::new(&inner_struct);
    //             assert_eq!(
    //                 inner_struct_de.get(1),
    //                 Ok(EnString(String::from(&"foo bar"[..])))
    //             );
    //             assert_eq!(
    //                 inner_struct_de.get(2),
    //                 Ok(EnString(String::from(&"foo bar"[..])))
    //             );
    //             assert_eq!(inner_struct_de.get(3), Ok(EnInt8(8)));
    //         }
    //         Err(_) => assert!(false),
    //     }

    //     let struct_size = de.take_struct_size().unwrap();
    //     match de.take_struct(struct_size) {
    //         Ok(inner_struct) => {
    //             let mut inner_struct_de = TarsStructDecoder::new(&inner_struct);
    //             assert_eq!(
    //                 inner_struct_de.get(4),
    //                 Ok(EnString(String::from(&"foo bar"[..])))
    //             );
    //             assert_eq!(
    //                 inner_struct_de.get(5),
    //                 Ok(EnString(String::from(&"foo bar"[..])))
    //             );
    //             assert_eq!(inner_struct_de.get(6), Ok(EnInt8(8)));
    //         }
    //         Err(_) => assert!(false),
    //     }

    //     // take_struct() won't consume header
    //     let mut d2: Vec<u8> = vec![];
    //     // insert {tag:0, type: 11}
    //     // 三层嵌套 struct
    //     d2.push(0x0A);
    //     d2.push(0x0A);
    //     d2.extend_from_slice(&d[0..21]);
    //     d2.push(0x0B);
    //     d2.push(0x0B);
    //     let mut de2 = TarsStructDecoder::new(&d2);
    //     let struct_size = de2.take_struct_size().unwrap();
    //     match de2.take_struct(struct_size) {
    //         Ok(struct_1) => {
    //             println!("\nstruct_1: {:?}\n", struct_1);
    //             // 获取首层
    //             let mut inner_de = TarsStructDecoder::new(&struct_1);
    //             println!("inner_de: {:?}\n", inner_de);

    //             let mut inner_struct = inner_de.get(0).unwrap().unwrap_struct().unwrap();
    //             println!("inner_struct: {:?}\n", inner_struct);

    //             // 获取第二层
    //             let mut inner_de2 = TarsStructDecoder::new(&inner_struct);
    //             println!("inner_de2: {:?}\n", inner_de2);

    //             let mut inner_struct2 = inner_de2.get(0).unwrap().unwrap_struct().unwrap();
    //             println!("inner_struct2: {:?}\n", inner_struct);

    //             // 获取第三层
    //             let mut inner_de3 = TarsStructDecoder::new(&inner_struct2);
    //             println!("inner_de2: {:?}\n", inner_de2);

    //             // let mut inner_struct3 = inner_de2.get(0).unwrap().unwrap_struct().unwrap();
    //             // println!("inner_struct3: {:?}\n", inner_struct3);

    //             assert_eq!(inner_de3.get(1), Ok(EnString(String::from(&"foo bar"[..]))));
    //             assert_eq!(inner_de3.get(2), Ok(EnString(String::from(&"foo bar"[..]))));
    //             assert_eq!(inner_de3.get(3), Ok(EnInt8(8)));
    //         }
    //         Err(_) => assert!(false),
    //     }
    // }
}
