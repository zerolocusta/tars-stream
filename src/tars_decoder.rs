use std::collections::BTreeMap;
use std::io::{BufRead, Cursor};

use std::mem;

use bytes::Buf;

use errors::DecodeErr;
use tars_type::TarsType::*;
use tars_type::*;

#[derive(Debug)]
pub struct TarsDecoder<'a> {
    buf: Cursor<&'a [u8]>,
    map: TarsStruct,
}
#[derive(Debug)]
struct Head {
    tag: u8,
    tars_type: u8,
}

impl<'a> TarsDecoder<'a> {
    pub fn new(buf: &'a [u8]) -> TarsDecoder {
        TarsDecoder {
            buf: Cursor::new(buf),
            map: TarsStruct::new(),
        }
    }

    pub fn decode_all(&mut self) -> Result<(), DecodeErr> {
        self.buf.set_position(0);
        while self.buf.has_remaining() {
            let head = self.get_head()?;
            let value = self.read(head.tars_type)?;
            self.map.insert(head.tag, value);
        }
        Ok(())
    }

    fn skip_to_tag(&mut self, tag: u8) -> Result<bool, DecodeErr> {
        let mut success = false;
        while self.buf.has_remaining() {
            let head = self.get_head()?;
            if head.tag != tag {}
            success = true;
        }
        Ok(success)
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
            _ => panic!("unknown tars type"),
        }
    }

    pub fn get_map(&mut self) -> BTreeMap<u8, TarsType> {
        mem::replace(&mut self.map, BTreeMap::new())
    }

    fn get_head(&mut self) -> Result<Head, DecodeErr> {
        if self.buf.remaining() < 1 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            let b = self.buf.get_u8();
            let tars_type = b & 0x0f;
            let mut tag = (b & 0xf0) >> 4;
            if tag >= 15 {
                tag = self.buf.get_u8();
            }
            Ok(Head {
                tag: tag,
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
            _ => panic!("unknown tars type"),
        }
    }

    pub fn take_int8(&mut self) -> Result<i8, DecodeErr> {
        if self.buf.remaining() < 1 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.buf.get_i8())
        }
    }

    fn take_int16(&mut self) -> Result<i16, DecodeErr> {
        if self.buf.remaining() < 2 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.buf.get_i16_be())
        }
    }

    fn take_int32(&mut self) -> Result<i32, DecodeErr> {
        if self.buf.remaining() < 4 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.buf.get_i32_be())
        }
    }

    fn take_int64(&mut self) -> Result<i64, DecodeErr> {
        if self.buf.remaining() < 8 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.buf.get_i64_be())
        }
    }

    fn take_float(&mut self) -> Result<u32, DecodeErr> {
        if self.buf.remaining() < 4 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.buf.get_u32_be())
        }
    }

    fn take_double(&mut self) -> Result<u64, DecodeErr> {
        if self.buf.remaining() < 8 {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            Ok(self.buf.get_u64_be())
        }
    }

    fn take_string_size(&mut self, tars_type: u8) -> Result<usize, DecodeErr> {
        if tars_type == 6 {
            Ok(self.take_int8()? as usize)
        } else if tars_type == 7 {
            Ok(self.take_int32()? as usize)
        } else {
            Err(DecodeErr::UnknownTarsTypeErr)
        }
    }

    fn take_string(&mut self, size: usize) -> Result<String, DecodeErr> {
        if self.buf.remaining() < size {
            Err(DecodeErr::NoEnoughDataErr)
        } else {
            let mut b = vec![];
            for _ in 0..size {
                b.push(self.buf.get_u8())
            }
            Ok(String::from_utf8(b).unwrap())
        }
    }

    fn take_map_size(&mut self) -> Result<usize, DecodeErr> {
        Ok(self.take_int32()? as usize)
    }

    fn take_map(&mut self, size: usize) -> Result<TarsMap, DecodeErr> {
        let mut map = BTreeMap::new();
        for _ in 0..size {
            let key_head = self.get_head()?;
            let key = self.read(key_head.tars_type)?;
            let value_head = self.get_head()?;
            let value = self.read(value_head.tars_type)?;
            map.insert(key, value);
        }
        Ok(map)
    }

    fn take_list_size(&mut self) -> Result<usize, DecodeErr> {
        Ok(self.take_int32()? as usize)
    }

    fn take_struct_size(&mut self) -> Result<usize, DecodeErr> {
        let cur_pos = self.buf.position();
        // 0x0B means (tag, type) => (0, EnStructEnd) => (0, 11)
        let num_bytes = self.buf.read_until(0x0B, &mut vec![]);
        self.buf.set_position(cur_pos);
        match num_bytes {
            Ok(size) => Ok(size),
            Err(_) => Err(DecodeErr::NoEnoughDataErr)
        }
    }
}

mod tests {
    use super::TarsDecoder;
    use errors::DecodeErr;
    use tars_type::TarsType::*;

    #[test]
    fn test_take_int8() {
        let b2: [u8; 10] = [63; 10];
        let mut de2 = TarsDecoder::new(&b2[..]);
        for _ in 0..10 {
            assert_eq!(de2.take_int8(), Ok(63));
        }

        assert_eq!(de2.take_int8(), Err(DecodeErr::NoEnoughDataErr));

        let b: [u8; 10] = [1; 10];
        let mut de = TarsDecoder::new(&b[..]);
        for _ in 0..10 {
            assert_eq!(de.read(0), Ok(EnInt8(1)));
        }

        assert_eq!(de2.read(0), Err(DecodeErr::NoEnoughDataErr));
    }

    #[test]
    fn test_take_string() {
        let mut de = TarsDecoder::new(&b"hello world"[..]);
        assert_eq!(de.take_string(11), Ok(String::from(&"hello world"[..])));
        assert_eq!(de.take_string(1), Err(DecodeErr::NoEnoughDataErr));

    }
}
