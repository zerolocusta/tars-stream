use std::io::{BufRead, Cursor};

use std::mem;

use bytes::Buf;
use bytes::Bytes;

use errors::DecodeErr;
use tars_type::TarsType::*;
use tars_type::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TarsStructDecoder {
    buf: Bytes,
    pos: usize,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Head {
    tag: u8,
    tars_type: u8,
    len: u8,
}

impl TarsStructDecoder {
    pub fn new(buf: &[u8]) -> TarsStructDecoder {
        let mut b = Bytes::new();
        b.extend_from_slice(buf);
        TarsStructDecoder { buf: b, pos: 0 }
    }

    pub fn get(&mut self, tag: u8) -> Result<TarsType, DecodeErr> {
        self.pos = 0;
        if let Ok(head) = self.skip_to_tag(tag) {
            Ok(self.read(head.tars_type)?)
        } else {
            Err(DecodeErr::TagNotFoundErr)
        }
    }

    fn has_remaining(&self) -> bool {
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

    fn read(&mut self, tars_type: u8) -> Result<TarsType, DecodeErr> {
        match tars_type {
            _ if tars_type == 0 => {
                let value = self.take_int8()?;
                Ok(EnInt8(value))
            }
            _ if tars_type == 1 => {
                let value = self.take_int16()?;
                Ok(EnInt16(value))
            }
            _ if tars_type == 2 => {
                let value = self.take_int32()?;
                Ok(EnInt32(value))
            }
            _ if tars_type == 3 => {
                let value = self.take_int64()?;
                Ok(EnInt64(value))
            }
            _ if tars_type == 4 => {
                let value = self.take_float()?;
                Ok(EnFloat(value))
            }
            _ if tars_type == 5 => {
                let value = self.take_double()?;
                Ok(EnDouble(value))
            }
            _ if tars_type == 6 || tars_type == 7 => {
                let size = self.take_string_size(tars_type)?;
                let value = self.take_string(size)?;
                Ok(EnString(value))
            }
            _ if tars_type == 8 => {
                let size = self.take_map_size()?;
                let value = self.take_map(size)?;
                Ok(EnMaps(value))
            }
            _ if tars_type == 9 => {
                let size = self.take_list_size()?;
                let value = self.take_list(size)?;
                Ok(EnList(value))
            }
            _ if tars_type == 10 => {
                let size = self.take_struct_size()?;
                let value = self.take_struct(size)?;
                Ok(EnStruct(value))
            }
            _ if tars_type == 12 => Ok(EnZero),
            // TODO: add more test
            _ if tars_type == 13 => {
                let size = self.take_simple_list_size()?;
                let value = self.take_simple_list(size)?;
                Ok(EnSimplelist(value))
            }
            _ => Err(DecodeErr::UnknownTarsTypeErr),
        }
    }

    fn take_head(&mut self) -> Result<Head, DecodeErr> {
        if self.remaining() < 1 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            let b = self.get_u8();
            let tars_type = b & 0x0f;
            let mut tag = (b & 0xf0) >> 4;
            let mut len = 1;
            if tag >= 15 {
                tag = self.get_u8();
                len = 2;
            }
            Ok(Head {
                tag: tag,
                len: len,
                tars_type: tars_type,
            })
        }
    }

    fn take_size(&mut self, tars_type: u8) -> Result<usize, DecodeErr> {
        match tars_type {
            _ if tars_type == 0 => Ok(1),
            _ if tars_type == 1 => Ok(2),
            _ if tars_type == 2 => Ok(4),
            _ if tars_type == 3 => Ok(8),
            _ if tars_type == 4 => Ok(4),
            _ if tars_type == 5 => Ok(8),
            _ if tars_type == 6 || tars_type == 7 => Ok(self.take_string_size(tars_type)?),
            _ if tars_type == 8 => Ok(self.take_map_size()?),
            _ if tars_type == 9 => Ok(self.take_list_size()?),
            _ if tars_type == 10 => Ok(self.take_struct_size()?),
            _ if tars_type == 12 => Ok(1),
            _ if tars_type == 13 => Ok(self.take_simple_list_size()?),
            _ => Err(DecodeErr::UnknownTarsTypeErr),
        }
    }

    fn take_int8(&mut self) -> Result<i8, DecodeErr> {
        if self.remaining() < 1 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.get_i8())
        }
    }

    fn take_int16(&mut self) -> Result<i16, DecodeErr> {
        if self.remaining() < 2 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.get_i16_be())
        }
    }

    fn take_int32(&mut self) -> Result<i32, DecodeErr> {
        if self.remaining() < 4 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.get_i32_be())
        }
    }

    fn take_int64(&mut self) -> Result<i64, DecodeErr> {
        if self.remaining() < 8 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.get_i64_be())
        }
    }

    fn take_float(&mut self) -> Result<u32, DecodeErr> {
        if self.remaining() < 4 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.get_i32_be() as u32)
        }
    }

    fn take_double(&mut self) -> Result<u64, DecodeErr> {
        if self.remaining() < 8 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.get_i64_be() as u64)
        }
    }

    fn take_string_size(&mut self, tars_type: u8) -> Result<usize, DecodeErr> {
        if tars_type == 6 {
            let s = self.take_int8()?;
            Ok(s as usize)
        } else if tars_type == 7 {
            Ok(self.take_int32()? as usize)
        } else {
            Err(DecodeErr::UnknownTarsTypeErr)
        }
    }

    fn take_string(&mut self, size: usize) -> Result<String, DecodeErr> {
        if self.remaining() < size {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            let mut b = vec![];
            for _ in 0..size {
                b.push(self.get_u8())
            }
            Ok(String::from_utf8(b).unwrap())
        }
    }

    fn take_map_size(&mut self) -> Result<usize, DecodeErr> {
        Ok(self.take_int32()? as usize)
    }

    fn take_map(&mut self, size: usize) -> Result<TarsMap, DecodeErr> {
        let mut map = TarsMap::new();
        let before_pos = self.pos;
        while self.pos < before_pos + size {
            let key_head = self.take_head()?;
            let key = self.read(key_head.tars_type)?;
            let value_head = self.take_head()?;
            let value = self.read(value_head.tars_type)?;
            map.insert(key, value);
        }
        assert_eq!(self.pos, before_pos + size);
        Ok(map)
    }

    fn take_list_size(&mut self) -> Result<usize, DecodeErr> {
        Ok(self.take_int32()? as usize)
    }

    fn take_list(&mut self, size: usize) -> Result<TarsList, DecodeErr> {
        let mut v = vec![];
        let before_pos = self.pos;
        while self.pos < before_pos + size {
            let value_head = self.take_head()?;
            let value = self.read(value_head.tars_type)?;
            v.push(value);
        }
        assert_eq!(self.pos, before_pos + size);
        Ok(v)
    }

    fn take_simple_list_size(&mut self) -> Result<usize, DecodeErr> {
        Ok(self.take_int32()? as usize)
    }

    fn take_simple_list(&mut self, size: usize) -> Result<TarsSimpleList, DecodeErr> {
        let mut v = vec![];
        let head = self.take_head()?;
        if head.tars_type != 0 {
            Err(DecodeErr::WrongSimpleListTarsTypeErr)
        } else {
            let before_pos = self.pos;
            while self.pos < before_pos + size - head.len as usize{
                v.push(self.get_u8());
            }
            assert_eq!(self.pos, before_pos + size - head.len as usize);
            Ok(v)
        }
    }

    fn take_struct_size(&mut self) -> Result<usize, DecodeErr> {
        let before_pos = self.pos;
        // 0x0B means (tag, type) => (0, EnStructEnd) => (0, 11)
        let mut head = self.take_head()?;
        while head.tars_type != 11 {
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

    fn take_struct(&mut self, size: usize) -> Result<TarsStructDecoder, DecodeErr> {
        // Abandon StructEnd (-1)
        let (left, right) = (self.pos, self.pos + size - 1);
        self.pos += size;
        Ok(TarsStructDecoder::new(&self.buf[left..right]))
    }
}

mod tests {
    use super::TarsStructDecoder;
    use errors::DecodeErr;
    use std::mem;
    use tars_type::TarsType::*;
    use tars_type::*;

    #[test]
    fn test_get() {
        let d: [u8; 44] = [
            // {tag: 1, type: 6}
            0x16,
            7,
            b'f',
            b'o',
            b'o',
            b' ',
            b'b',
            b'a',
            b'r',
            // {tag: 2, type: 6}
            0x26,
            7,
            b'f',
            b'o',
            b'o',
            b' ',
            b'b',
            b'a',
            b'r',
            // {tag: 3, type: 0}
            0x30,
            8,
            // {tag: 4, type: 10} start inner struct
            0x4A,
            // {struct: {tag: 1, type: 6} }
            0x16,
            7,
            b'f',
            b'o',
            b'o',
            b' ',
            b'b',
            b'a',
            b'r',
            // {struct: {tag: 2, type: 6} }
            0x26,
            7,
            b'f',
            b'o',
            b'o',
            b' ',
            b'b',
            b'a',
            b'r',
            // {struct: {tag: 3, type: 6} }
            0x30,
            8,
            // StructEnd
            0x0B,
            // {struct: {tag: 5, type: 6} }
            0x50,
            16,
        ];

        let mut de = TarsStructDecoder::new(&d);

        for _ in 0..10 {
            assert_eq!(de.get(1), Ok(EnString(String::from(&"foo bar"[..]))));
            assert_eq!(de.get(2), Ok(EnString(String::from(&"foo bar"[..]))));
            assert_eq!(de.get(3), Ok(EnInt8(8)));
            assert_eq!(de.get(5), Ok(EnInt8(16)));

            let mut inner_struct = de.get(4).unwrap().unwrap_struct().unwrap();
            assert_eq!(
                inner_struct.get(1),
                Ok(EnString(String::from(&"foo bar"[..])))
            );
            assert_eq!(
                inner_struct.get(2),
                Ok(EnString(String::from(&"foo bar"[..])))
            );
            assert_eq!(inner_struct.get(3), Ok(EnInt8(8)));
            assert_eq!(de.get(0), Err(DecodeErr::TagNotFoundErr));
            assert_eq!(de.get(200), Err(DecodeErr::TagNotFoundErr));
            assert_eq!(de.get(255), Err(DecodeErr::TagNotFoundErr));
        }
    }

    #[test]
    fn test_take_simple_list() {
        let head: [u8; 4] = unsafe{ mem::transmute(5u32.to_be()) };
        let b: [u8; 9] = [
            head[0],
            head[1],
            head[2],
            head[3],
            0x00,
            4,
            5,
            6,
            7,
        ];
        let mut de = TarsStructDecoder::new(&b);
        let list_size = de.take_simple_list_size().unwrap();
        assert_eq!(list_size, 5);
        let list = de.take_simple_list(list_size).unwrap();
        assert_eq!(list, vec![4, 5, 6, 7]);
    }

    #[test]
    fn test_take_zero() {
        let b: [u8; 2] = [0x0C; 2];
        let mut de = TarsStructDecoder::new(&b);
        let head = de.take_head().unwrap();
        assert_eq!(de.read(head.tars_type).unwrap(), EnZero);

        let head = de.take_head().unwrap();
        assert_eq!(de.read(head.tars_type).unwrap(), EnZero);

        assert_eq!(de.take_head(), Err(DecodeErr::NoEnoughDataErr));
    }

    #[test]
    fn test_take_list() {
        let size: [u8; 4] = unsafe { mem::transmute(26u32.to_be()) };
        let b: [u8; 30] = [
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
            // {tag: 0, type: 0}
            0x00,
            8,
            // {tag: 0, type: 0}
            0x00,
            9,
        ];
        let mut de = TarsStructDecoder::new(&b[..]);
        let list_size = de.take_list_size().unwrap();
        assert_eq!(list_size, 26);
        let list = de.take_list(list_size).unwrap();
        assert_eq!(list[0], EnString(String::from(&"foo bar"[..])));
        assert_eq!(list[1], EnString(String::from(&"hello world"[..])));
        assert_eq!(list[2], EnInt8(8));
        assert_eq!(list[3], EnInt8(9));
    }

    #[test]
    fn test_take_map() {
        let size: [u8; 4] = unsafe { mem::transmute(26u32.to_be()) };
        let b: [u8; 30] = [
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
            // {tag: 0, type: 0}
            0x00,
            8,
            // {tag: 0, type: 0}
            0x00,
            9,
        ];
        let mut de2 = TarsStructDecoder::new(&b[..]);
        let map_size = de2.take_map_size().unwrap();
        assert_eq!(map_size, 26);
        let map = de2.take_map(map_size);
        match map {
            Ok(m) => {
                let value1 = m.get(&EnInt8(8)).unwrap();
                assert_eq!(value1, &EnInt8(9));
                let value2 = m.get(&EnString(String::from(&"foo bar"[..]))).unwrap();
                assert_eq!(value2, &EnString(String::from(&"hello world"[..])));
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_take_int64() {
        let b: [u8; 8] = unsafe { mem::transmute(0x0acb8b9d9d9d9d9di64.to_be()) };
        let mut de2 = TarsStructDecoder::new(&b[..]);
        let i = de2.take_int64();
        assert_eq!(i, Ok(0x0acb8b9d9d9d9d9d));
        assert_eq!(de2.take_int64(), Err(DecodeErr::NoEnoughDataErr));
    }

    #[test]
    fn test_take_int32() {
        let b: [u8; 4] = unsafe { mem::transmute(0x0acb8b9di32.to_be()) };
        let mut de2 = TarsStructDecoder::new(&b[..]);
        assert_eq!(de2.take_int32(), Ok(0x0acb8b9d));
        assert_eq!(de2.take_int32(), Err(DecodeErr::NoEnoughDataErr));
    }

    #[test]
    fn test_take_int16() {
        let b: [u8; 2] = unsafe { mem::transmute(0x0acbi16.to_be()) };
        let mut de2 = TarsStructDecoder::new(&b[..]);
        assert_eq!(de2.take_int16(), Ok(0x0acb));
        assert_eq!(de2.take_int16(), Err(DecodeErr::NoEnoughDataErr));
    }

    #[test]
    fn test_take_int8() {
        let b2: [u8; 10] = [63; 10];
        let mut de2 = TarsStructDecoder::new(&b2[..]);
        for _ in 0..10 {
            assert_eq!(de2.take_int8(), Ok(63));
        }

        assert_eq!(de2.take_int8(), Err(DecodeErr::NoEnoughDataErr));

        let b: [u8; 10] = [1; 10];
        let mut de = TarsStructDecoder::new(&b[..]);
        for _ in 0..10 {
            assert_eq!(de.read(0), Ok(EnInt8(1)));
        }

        assert_eq!(de2.read(0), Err(DecodeErr::NoEnoughDataErr));
    }

    #[test]
    fn test_take_double() {
        let b2: [u8; 8] = unsafe { mem::transmute(0.6f64.to_bits().to_be()) };
        let mut de2 = TarsStructDecoder::new(&b2[..]);
        let f = f64::from_bits(de2.take_double().unwrap());
        assert_approx_eq!(f, 0.6f64);
        assert_eq!(de2.take_float(), Err(DecodeErr::NoEnoughDataErr));
    }

    #[test]
    fn test_take_float() {
        let b2: [u8; 4] = unsafe { mem::transmute(0.3f32.to_bits().to_be()) };
        let mut de2 = TarsStructDecoder::new(&b2[..]);
        let f = f32::from_bits(de2.take_float().unwrap());
        assert_approx_eq!(f, 0.3f32);
        assert_eq!(de2.take_float(), Err(DecodeErr::NoEnoughDataErr));
    }

    #[test]
    fn test_take_string() {
        let mut de = TarsStructDecoder::new(&b"hello world"[..]);
        assert_eq!(de.take_string(11), Ok(String::from(&"hello world"[..])));
        assert_eq!(de.take_string(1), Err(DecodeErr::NoEnoughDataErr));

        // test read string1
        let d2: [u8; 8] = [7, b'f', b'o', b'o', b' ', b'b', b'a', b'r'];
        let mut de2 = TarsStructDecoder::new(&d2);
        assert_eq!(de2.read(6), Ok(EnString(String::from(&"foo bar"[..]))));

        // test read string4
        let size: [u8; 4] = unsafe { mem::transmute(7u32.to_be()) };
        let d3: [u8; 11] = [
            size[0], size[1], size[2], size[3], b'f', b'o', b'o', b' ', b'b', b'a', b'r',
        ];
        let mut de3 = TarsStructDecoder::new(&d3);
        assert_eq!(de3.read(7), Ok(EnString(String::from(&"foo bar"[..]))));
    }

    #[test]
    fn test_take_struct() {
        let d: [u8; 42] = [
            // {tag: 1, type: 6}
            0x16,
            7,
            b'f',
            b'o',
            b'o',
            b' ',
            b'b',
            b'a',
            b'r',
            // {tag: 2, type: 6}
            0x26,
            7,
            b'f',
            b'o',
            b'o',
            b' ',
            b'b',
            b'a',
            b'r',
            // {tag: 3, type: 0}
            0x30,
            8,
            // StructEnd
            0x0B,
            // {tag: 4, type: 6}
            0x46,
            7,
            b'f',
            b'o',
            b'o',
            b' ',
            b'b',
            b'a',
            b'r',
            // {tag: 5, type: 6}
            0x56,
            7,
            b'f',
            b'o',
            b'o',
            b' ',
            b'b',
            b'a',
            b'r',
            // {tag: 6, type: 0}
            0x60,
            8,
            // StructEnd
            0x0B,
        ];
        let mut de = TarsStructDecoder::new(&d);

        let struct_size = de.take_struct_size().unwrap();
        match de.take_struct(struct_size) {
            Ok(mut inner_de) => {
                assert_eq!(inner_de.get(1), Ok(EnString(String::from(&"foo bar"[..]))));
                assert_eq!(inner_de.get(2), Ok(EnString(String::from(&"foo bar"[..]))));
                assert_eq!(inner_de.get(3), Ok(EnInt8(8)));
            }
            Err(_) => assert!(false),
        }

        let struct_size = de.take_struct_size().unwrap();
        match de.take_struct(struct_size) {
            Ok(mut inner_de) => {
                assert_eq!(inner_de.get(4), Ok(EnString(String::from(&"foo bar"[..]))));
                assert_eq!(inner_de.get(5), Ok(EnString(String::from(&"foo bar"[..]))));
                assert_eq!(inner_de.get(6), Ok(EnInt8(8)));
            }
            Err(_) => assert!(false),
        }

        // take_struct() won't consume header
        let mut d2: Vec<u8> = vec![];
        // insert {tag:0, type: 11}
        // 三层嵌套 struct
        d2.push(0x0A);
        d2.push(0x0A);
        d2.extend_from_slice(&d[0..21]);
        d2.push(0x0B);
        d2.push(0x0B);
        let mut de2 = TarsStructDecoder::new(&d2);
        let struct_size = de2.take_struct_size().unwrap();
        match de2.take_struct(struct_size) {
            Ok(mut struct_1) => {
                println!("\nstruct_1: {:?}\n", struct_1);
                // 获取首层
                let mut inner_de = struct_1.get(0).unwrap();
                println!("inner_de: {:?}\n", inner_de);
                // 获取第二层
                let mut inner_struct = inner_de.unwrap_struct().unwrap();
                println!("inner_struct: {:?}\n", inner_struct);

                // 获取第三层
                let mut inner_struct2 = inner_struct.get(0).unwrap().unwrap_struct().unwrap();
                println!("inner_struct2: {:?}\n", inner_struct2);
                assert_eq!(
                    inner_struct2.get(1),
                    Ok(EnString(String::from(&"foo bar"[..])))
                );
                assert_eq!(
                    inner_struct2.get(2),
                    Ok(EnString(String::from(&"foo bar"[..])))
                );
                assert_eq!(inner_struct2.get(3), Ok(EnInt8(8)));
            }
            Err(_) => assert!(false),
        }
    }
}
