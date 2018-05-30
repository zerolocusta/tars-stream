extern crate bytes;

use bytes::Buf;
use std::io::Cursor;
use std::mem;
use tars_type::TarsType;
use tars_type::TarsType::*;

use std::collections::BTreeMap;

use errors::DecodeErr;

use std::collections::HashMap;

#[derive(Debug)]
pub struct TarsDecoder<'a> {
    buf: Cursor<&'a [u8]>,
    map: BTreeMap<u8, TarsType>,
}

impl<'a> TarsDecoder<'a> {
    pub fn new(buf: &'a [u8]) -> TarsDecoder {
        TarsDecoder {
            buf: Cursor::new(buf),
            map: BTreeMap::new(),
        }
    }

    pub fn decode(&mut self) {
        let (tars_type, tag) = self.get_type_and_tag();
        match tars_type {
            _ if tars_type == 0 => {
                let value = self.read_int8();
                self.map.insert(tag, EnInt8(value));
            },
            _ if tars_type == 1 => {
                let value = self.read_int16();
                self.map.insert(tag, EnInt16(value));
            },
            _ if tars_type == 2 => {
                let value = self.read_int32();
                self.map.insert(tag, EnInt32(value));
            },
            _ if tars_type == 3 => {
                let value = self.read_int64();
                self.map.insert(tag, EnInt64(value));
            },
            _ if tars_type == 4 => {
                let value = self.read_float();
                self.map.insert(tag, EnFloat(value));
            },
            _ if tars_type == 5 => {
                let value = self.read_double();
                self.map.insert(tag, EnDouble(value));
            }
            _ if tars_type == 6 || tars_type == 7 => {
                let size = self.take_string_size(tars_type);
                let value = self.read_string(size);
                self.map.insert(tag, EnString(value));
            },
            _ => panic!("unknown tars type"),
        }
    }

    pub fn get_map(&mut self) -> BTreeMap<u8, TarsType> {
        mem::replace(&mut self.map, BTreeMap::new())
    }

    fn get_type_and_tag(&mut self) -> (u8, u8) {
        let b = self.buf.get_u8();
        let tars_type = b >> 4;
        let mut tag = b & 0xf;
        if tag == 15 {
            tag = self.buf.get_u8();
        }
        (tars_type, tag)
    }

    fn read_int8(&mut self) -> i8 {
        self.buf.get_i8()
    }

    fn read_int16(&mut self) -> i16 {
        self.buf.get_i16_be()
    }

    fn read_int32(&mut self) -> i32 {
        self.buf.get_i32_be()
    }

    fn read_int64(&mut self) -> i64 {
        self.buf.get_i64_be()
    }

    fn read_float(&mut self) -> f32 {
        self.buf.get_f32_be()
    }

    fn read_double(&mut self) -> f64 {
        self.buf.get_f64_be()
    }

    fn take_string_size(&mut self, tars_type: u8) -> usize {
        if tars_type == 6 {
            self.buf.get_u8() as usize
        } else if tars_type == 7 {
            self.buf.get_u32_be() as usize
        } else {
            panic!("unknow tars string type")
        }
    }

    fn read_string(&mut self, size: usize) -> String {
        let b = self.buf.by_ref().take(size);
        String::from_utf8(b.bytes().iter().map(|byte| *byte).collect()).unwrap()
    }
}

mod tests {
    use super::TarsDecoder;

    #[test]
    fn test_read_int8() {
        let b: [u8; 10] = [1; 10];
        let mut de = TarsDecoder::new(&b[..]);
        for _ in 0..10 {
            assert_eq!(de.read_int8(), 1);
        }

        let b2: [u8; 10] = [63; 10];
        let mut de2 = TarsDecoder::new(&b2[..]);
        for _ in 0..10 {
            assert_eq!(de2.read_int8(), 63);
        }
    }

    #[test]
    fn test_read_string() {
        let mut de = TarsDecoder::new(&b"hello world"[..]);
        assert_eq!(de.read_string(11), String::from(&"hello world"[..]));
    }
}
