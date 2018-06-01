use bytes::{BufMut, BytesMut};
use tars_type::TarsTypeMark;
use tars_type::TarsTypeMark::*;
#[derive(Debug)]
pub struct TarsStructEncoder {
    buf: BytesMut,
}

impl<'a> TarsStructEncoder {
    pub fn new() -> TarsStructEncoder {
        TarsStructEncoder {
            buf: BytesMut::new(),
        }
    }

    fn put_head(&mut self, tars_type: TarsTypeMark, tag: u8) {
        if tag < 15 {
            let head = (tag << 4) | tars_type.value();
            self.buf.put_u8(head);
        } else {
            let head: u16 = (((0xF0u8) | tars_type.value()) as u16) << 8 | tag as u16;
            self.buf.put_u16_be(head)
        }
    }

    pub fn put_u8(&mut self, tag: u8, value: u8) {
        self.put_head(EnInt8, tag);
        self.buf.put_u8(value);
    }

    pub fn put_i8(&mut self, tag: u8, value: i8) {
        self.put_head(EnInt8, tag);
        self.buf.put_i8(value);
    }

    pub fn to_bytes_ref(&self) -> &BytesMut {
        &self.buf
    }
}

#[cfg(test)]
mod tests {
    use super::TarsStructEncoder;
    use tars_type::TarsTypeMark::*;

    #[test]
    fn test_put_head() {
        let mut en = TarsStructEncoder::new();
        en.put_head(EnInt8, 19);
        en.put_head(EnInt16, 0);

        let buf = en.to_bytes_ref();

        assert_eq!(&buf[..], &b"\xf0\x13\x01"[..]);
    }

    #[test]
    fn test_put_i8() {
        let mut en = TarsStructEncoder::new();
        en.put_i8(0, -1);

        let buf = en.to_bytes_ref();

        assert_eq!(&buf[..], &b"\x00\xff"[..]);
    }

    #[test]
    fn test_put_u8() {
        let mut en = TarsStructEncoder::new();
        en.put_u8(0, 128);

        let buf = en.to_bytes_ref();

        assert_eq!(&buf[..], &b"\x00\x80"[..]);
    }
}
