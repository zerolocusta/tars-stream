use bytes::{BufMut, BytesMut};
use errors::EncodeErr;
use tars_type::TarsTypeMark;
use tars_type::TarsTypeMark::*;

fn put_head(buf: &mut BytesMut, tag: u8, tars_type: TarsTypeMark) -> Result<(), EncodeErr> {
    if tag > u8::max_value() {
        Err(EncodeErr::TooBigTagErr)
    } else {
        if tag < 15 {
            let head = (tag << 4) | tars_type.value();
            buf.put_u8(head);
        } else {
            let head: u16 = u16::from((0xF0u8) | tars_type.value()) << 8 | u16::from(tag);
            buf.put_u16_be(head)
        }
        Ok(())
    }
}

pub trait EncodeTo {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr>;
}

impl EncodeTo for i8 {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        put_head(buf, tag, EnInt8)?;
        buf.put_i8(*self);
        Ok(())
    }
}

impl EncodeTo for u8 {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        put_head(buf, tag, EnInt8)?;
        buf.put_u8(*self);
        Ok(())
    }
}

impl EncodeTo for i16 {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        if *self >= i16::from(i8::min_value()) && *self <= i16::from(i8::max_value()) {
            (*self as i8).encode_into_bytes(tag, buf)?;
        } else {
            put_head(buf, tag, EnInt16)?;
            buf.put_i16_be(*self);
        }
        Ok(())
    }
}

impl EncodeTo for u16 {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        if *self >= u16::from(u8::min_value()) && *self <= u16::from(u8::max_value()) {
            (*self as u8).encode_into_bytes(tag, buf)?;
        } else {
            put_head(buf, tag, EnInt16)?;
            buf.put_u16_be(*self);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_head() {
        let mut buf = BytesMut::new();
        put_head(&mut buf, 19, EnInt8).unwrap();
        put_head(&mut buf, 0, EnInt16).unwrap();

        assert_eq!(&buf[..], &b"\xf0\x13\x01"[..]);
    }

    #[test]
    fn test_encode_i8() {
        let mut buf = BytesMut::new();
        let i0: i8 = -127;
        i0.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x00\x81"[..]);

        let i1: i8 = 127;
        i1.encode_into_bytes(14, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x00\x81\xe0\x7f"[..]);

        let i2: i8 = -1;
        i2.encode_into_bytes(255, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x00\x81\xe0\x7f\xf0\xff\xff"[..]);
    }

    #[test]
    fn test_encode_u8() {
        let mut buf = BytesMut::new();
        let u0: u8 = 127;
        u0.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x00\x7f"[..]);

        let u1: u8 = 255;
        u1.encode_into_bytes(14, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x00\x7f\xe0\xff"[..]);

        let u2: u8 = 0;
        u2.encode_into_bytes(255, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x00\x7f\xe0\xff\xf0\xff\x00"[..]);
    }

    #[test]
    fn test_encode_i16() {
        let mut buf = BytesMut::new();
        let i0: i16 = -32768;
        i0.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x01\x80\x00"[..]);

        let mut buf = BytesMut::new();
        let i1: i16 = -127;
        i1.encode_into_bytes(15, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\xf0\x0f\x81"[..]);

        let mut buf = BytesMut::new();
        let i2: i16 = 32767;
        i2.encode_into_bytes(19, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\xf1\x13\x7f\xff"[..]);
    }
}
