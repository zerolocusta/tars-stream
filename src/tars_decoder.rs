extern crate bytes;

use bytes::Buf;
use std::io::Cursor;
use tars_type::TarsType;
use tars_type::TarsType::*;

use std::collections::BTreeMap;

use errors::DecodeErr;

use std::collections::HashMap;

#[derive(Debug)]
pub struct TarsDecoder<'a> {
    buf: Cursor<&'a [u8]>,
    map: BTreeMap<u8, TarsType<'a>>,
}

impl<'a> TarsDecoder<'a> {
    pub fn new(buf: &'a [u8]) -> TarsDecoder {
        TarsDecoder{
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
            _ => panic!("unknown tars type")
        }
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

    fn read_uint8(&mut self) -> u8 {
        self.buf.get_u8()
    }

    fn read_string() {
        
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
}
