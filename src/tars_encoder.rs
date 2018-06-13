use bytes::{BufMut, Bytes, BytesMut};
use errors::EncodeErr;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::mem;
use tars_type::TarsTypeMark::*;
use tars_type::*;

const MAX_HEADER_LEN: usize = 2;
const MAX_SIZE_LEN: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TarsEncoder {
    buf: BytesMut,
}

impl TarsEncoder {
    pub fn new() -> Self {
        TarsEncoder {
            buf: BytesMut::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    // move out buf
    pub fn to_bytes(self) -> Bytes {
        self.buf.freeze()
    }

    pub fn to_bytes_mut(self) -> BytesMut {
        self.buf
    }

    pub fn unsplit(&mut self, another: TarsEncoder) {
        self.buf.unsplit(another.to_bytes_mut());
    }

    pub fn check_maybe_resize(&mut self, len: usize) {
        if self.buf.remaining_mut() < len {
            let new_len = self.buf.remaining_mut() + len + 1;
            self.buf.reserve(new_len)
        }
    }

    fn put_head(&mut self, tag: u8, tars_type: TarsTypeMark) -> Result<(), EncodeErr> {
        self.check_maybe_resize(MAX_HEADER_LEN);
        if tag > u8::max_value() {
            Err(EncodeErr::TooBigTagErr)
        } else {
            if tag < 15 {
                let head = (tag << 4) | tars_type.value();
                self.buf.put_u8(head);
            } else {
                let head: u16 = u16::from((0xF0u8) | tars_type.value()) << 8 | u16::from(tag);
                self.buf.put_u16_be(head)
            }
            Ok(())
        }
    }
}

pub trait TarsEncoderTrait<T> {
    fn put(&mut self, tag: u8, ele: &T) -> Result<(), EncodeErr>;
}

impl<T> TarsEncoderTrait<T> for TarsEncoder
where
    T: EncodeIntoTars,
{
    // specialization for all tars type expect struct
    default fn put(&mut self, tag: u8, ele: &T) -> Result<(), EncodeErr> {
        self.put_head(tag, EnStructBegin)?;
        ele.encode_into(self)?;
        self.put_head(0, EnStructEnd)?;
        Ok(())
    }
}

impl TarsEncoderTrait<i8> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &i8) -> Result<(), EncodeErr> {
        if *ele == 0 {
            self.put_head(tag, EnZero)?;
        } else {
            self.put_head(tag, EnInt8)?;
            self.check_maybe_resize(mem::size_of::<i8>());
            ele.encode_into(self)?;
        }
        Ok(())
    }
}

impl TarsEncoderTrait<u8> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &u8) -> Result<(), EncodeErr> {
        if *ele == 0 {
            self.put_head(tag, EnZero)?;
        } else {
            self.put_head(tag, EnInt8)?;
            self.check_maybe_resize(mem::size_of::<u8>());
            ele.encode_into(self)?;
        }
        Ok(())
    }
}

impl TarsEncoderTrait<u16> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &u16) -> Result<(), EncodeErr> {
        if *ele >= u16::from(u8::min_value()) && *ele <= u16::from(u8::max_value()) {
            self.put(tag, &(*ele as u8))?;
        } else {
            self.put_head(tag, EnInt16)?;
            self.check_maybe_resize(mem::size_of::<u16>());
            ele.encode_into(self)?;
        }
        Ok(())
    }
}

impl TarsEncoderTrait<i16> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &i16) -> Result<(), EncodeErr> {
        if *ele >= i16::from(i8::min_value()) && *ele <= i16::from(i8::max_value()) {
            self.put(tag, &(*ele as i8))?;
        } else {
            self.put_head(tag, EnInt16)?;
            self.check_maybe_resize(mem::size_of::<i16>());
            ele.encode_into(self)?;
        }
        Ok(())
    }
}

impl TarsEncoderTrait<u32> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &u32) -> Result<(), EncodeErr> {
        if *ele >= u32::from(u16::min_value()) && *ele <= u32::from(u16::max_value()) {
            self.put(tag, &(*ele as u16))?;
        } else {
            self.put_head(tag, EnInt32)?;
            self.check_maybe_resize(mem::size_of::<u32>());
            ele.encode_into(self)?;
        }
        Ok(())
    }
}

impl TarsEncoderTrait<i32> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &i32) -> Result<(), EncodeErr> {
        if *ele >= i32::from(i16::min_value()) && *ele <= i32::from(i16::max_value()) {
            self.put(tag, &(*ele as i16))?;
        } else {
            self.put_head(tag, EnInt32)?;
            self.check_maybe_resize(mem::size_of::<i32>());
            ele.encode_into(self)?;
        }
        Ok(())
    }
}

impl TarsEncoderTrait<i64> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &i64) -> Result<(), EncodeErr> {
        if *ele >= i64::from(i32::min_value()) && *ele <= i64::from(i32::max_value()) {
            self.put(tag, &(*ele as i32))?;
        } else {
            self.put_head(tag, EnInt64)?;
            self.check_maybe_resize(mem::size_of::<i64>());
            ele.encode_into(self)?;
        }
        Ok(())
    }
}

impl TarsEncoderTrait<u64> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &u64) -> Result<(), EncodeErr> {
        if *ele >= u64::from(u32::min_value()) && *ele <= u64::from(u32::max_value()) {
            self.put(tag, &(*ele as u32))?;
        } else {
            self.put_head(tag, EnInt64)?;
            self.check_maybe_resize(mem::size_of::<u64>());
            ele.encode_into(self)?;
        }
        Ok(())
    }
}

impl TarsEncoderTrait<f32> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &f32) -> Result<(), EncodeErr> {
        self.put_head(tag, EnFloat)?;
        self.check_maybe_resize(mem::size_of::<f32>());
        ele.encode_into(self)?;
        Ok(())
    }
}

impl TarsEncoderTrait<f64> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &f64) -> Result<(), EncodeErr> {
        self.put_head(tag, EnDouble)?;
        self.check_maybe_resize(mem::size_of::<f64>());
        ele.encode_into(self)?;
        Ok(())
    }
}

impl TarsEncoderTrait<bool> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &bool) -> Result<(), EncodeErr> {
        if *ele {
            self.put_head(tag, EnInt8)?;
            self.check_maybe_resize(mem::size_of::<bool>());
            ele.encode_into(self)?;
        } else {
            self.put_head(tag, EnZero)?;
        }
        Ok(())
    }
}

impl TarsEncoderTrait<String> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &String) -> Result<(), EncodeErr> {
        let len = ele.len();
        self.check_maybe_resize(MAX_SIZE_LEN + len);

        if len <= usize::from(u8::max_value()) {
            // encode as string1
            self.put_head(tag, EnString1)?;
            match u8::try_from(len) {
                Ok(l) => {
                    self.buf.put_u8(l);
                    ele.encode_into(self)?;
                    Ok(())
                }
                Err(_) => Err(EncodeErr::ConvertU8Err),
            }
        } else if len <= u32::max_value() as usize {
            // encode as string4
            self.put_head(tag, EnString4)?;
            self.buf.put_u32_be(len as u32);
            ele.encode_into(self)?;
            Ok(())
        } else {
            Err(EncodeErr::BufferTooBigErr)
        }
    }
}

impl<K, V> TarsEncoderTrait<BTreeMap<K, V>> for TarsEncoder
where
    K: EncodeIntoTars + Ord,
    V: EncodeIntoTars,
{
    fn put(&mut self, tag: u8, ele: &BTreeMap<K, V>) -> Result<(), EncodeErr> {
        self.put_head(tag, EnMaps)?;
        ele.encode_into(self)?;
        Ok(())
    }
}

impl<T> TarsEncoderTrait<Vec<T>> for TarsEncoder
where
    T: EncodeIntoTars,
{
    default fn put(&mut self, tag: u8, ele: &Vec<T>) -> Result<(), EncodeErr> {
        self.put_head(tag, EnList)?;
        ele.encode_into(self)?;
        Ok(())
    }
}

impl TarsEncoderTrait<Vec<i8>> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &Vec<i8>) -> Result<(), EncodeErr> {
        self.put_head(tag, EnSimplelist)?;
        self.put_head(0, EnInt8)?;
        ele.encode_into(self)?;
        Ok(())
    }
}

impl TarsEncoderTrait<Vec<u8>> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &Vec<u8>) -> Result<(), EncodeErr> {
        self.put_head(tag, EnSimplelist)?;
        self.put_head(0, EnInt8)?;
        ele.encode_into(self)?;
        Ok(())
    }
}

impl TarsEncoderTrait<Vec<bool>> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &Vec<bool>) -> Result<(), EncodeErr> {
        self.put_head(tag, EnSimplelist)?;
        self.put_head(0, EnInt8)?;
        ele.encode_into(self)?;
        Ok(())
    }
}

impl TarsEncoderTrait<Bytes> for TarsEncoder {
    fn put(&mut self, tag: u8, ele: &Bytes) -> Result<(), EncodeErr> {
        self.put_head(tag, EnSimplelist)?;
        self.put_head(0, EnInt8)?;
        ele.encode_into(self)?;
        Ok(())
    }
}

impl<T> TarsEncoderTrait<Option<T>> for TarsEncoder
where
    T: EncodeIntoTars,
{
    fn put(&mut self, tag: u8, ele: &Option<T>) -> Result<(), EncodeErr> {
        match ele {
            None => Ok(()),
            Some(e) => {
                self.put(tag, e)?;
                Ok(())
            }
        }
    }
}

// EncodeIntoTars Trait, 各类型将自身写入 TarsEncoder 中
pub trait EncodeIntoTars {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr>;
}

impl EncodeIntoTars for i8 {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.buf.put_i8(*self);
        Ok(())
    }
}

impl EncodeIntoTars for u8 {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.buf.put_u8(*self);
        Ok(())
    }
}

impl EncodeIntoTars for i16 {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.buf.put_i16_be(*self);
        Ok(())
    }
}

impl EncodeIntoTars for u16 {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.buf.put_u16_be(*self);
        Ok(())
    }
}

impl EncodeIntoTars for i32 {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.buf.put_i32_be(*self);
        Ok(())
    }
}

impl EncodeIntoTars for u32 {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.buf.put_u32_be(*self);
        Ok(())
    }
}

impl EncodeIntoTars for i64 {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.buf.put_i64_be(*self);
        Ok(())
    }
}

impl EncodeIntoTars for u64 {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.buf.put_u64_be(*self);
        Ok(())
    }
}

impl EncodeIntoTars for f32 {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.buf.put_f32_be(*self);
        Ok(())
    }
}

impl EncodeIntoTars for f64 {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.buf.put_f64_be(*self);
        Ok(())
    }
}

impl EncodeIntoTars for bool {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        let value: u8 = if *self { 1 } else { 0 };
        value.encode_into(encoder)
    }
}

impl EncodeIntoTars for String {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.buf.put(self);
        Ok(())
    }
}

impl<K, V> EncodeIntoTars for BTreeMap<K, V>
where
    K: EncodeIntoTars + Ord,
    V: EncodeIntoTars,
{
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        let mut inner_encoder = TarsEncoder::new();
        for (key, value) in self.iter() {
            inner_encoder.put(0, key)?;
            inner_encoder.put(0, value)?;
        }

        if inner_encoder.len() > u32::max_value() as usize {
            Err(EncodeErr::BufferTooBigErr)
        } else {
            encoder.check_maybe_resize(inner_encoder.len() + MAX_SIZE_LEN);
            encoder.buf.put_u32_be(inner_encoder.len() as u32);
            if inner_encoder.len() > 0 {
                encoder.unsplit(inner_encoder);
            }
            Ok(())
        }
    }
}

impl<T> EncodeIntoTars for Vec<T>
where
    T: EncodeIntoTars,
{
    default fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        let mut inner_encoder = TarsEncoder::new();
        for ele in self.into_iter() {
            inner_encoder.put(0, ele).unwrap();
        }
        if inner_encoder.len() > u32::max_value() as usize {
            Err(EncodeErr::BufferTooBigErr)
        } else {
            // TODO fix this ugly code
            encoder.check_maybe_resize(inner_encoder.len() + MAX_SIZE_LEN);
            encoder.buf.put_u32_be(inner_encoder.len() as u32);
            if encoder.len() > 0 {
                encoder.unsplit(inner_encoder);
            }
            Ok(())
        }
    }
}

impl EncodeIntoTars for Vec<u8> {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        if self.len() > u32::max_value() as usize {
            Err(EncodeErr::BufferTooBigErr)
        } else {
            encoder.buf.reserve(MAX_SIZE_LEN + self.len());
            encoder.buf.put_u32_be(self.len() as u32);
            encoder.buf.extend_from_slice(self);
            Ok(())
        }
    }
}

impl EncodeIntoTars for Vec<i8> {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        if self.len() > u32::max_value() as usize {
            Err(EncodeErr::BufferTooBigErr)
        } else {
            encoder.buf.reserve(MAX_SIZE_LEN + self.len());
            encoder.buf.put_u32_be(self.len() as u32);
            encoder
                .buf
                .extend_from_slice(unsafe { mem::transmute(self.as_slice()) });
            Ok(())
        }
    }
}

impl EncodeIntoTars for Vec<bool> {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        if self.len() > u32::max_value() as usize {
            Err(EncodeErr::BufferTooBigErr)
        } else {
            encoder.buf.reserve(MAX_SIZE_LEN + self.len());
            encoder.buf.put_u32_be(self.len() as u32);
            encoder
                .buf
                .extend_from_slice(unsafe { mem::transmute(self.as_slice()) });
            Ok(())
        }
    }
}

impl EncodeIntoTars for Bytes {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        if self.len() > u32::max_value() as usize {
            Err(EncodeErr::BufferTooBigErr)
        } else {
            encoder.buf.reserve(MAX_SIZE_LEN + self.len());
            encoder.buf.put_u32_be(self.len() as u32);
            encoder.buf.extend_from_slice(self);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_option() {
        let mut encoder = TarsEncoder::new();
        let a: Option<i64> = Some(-1337);
        let b: Option<i64> = None;
        encoder.put(0, &a).unwrap();
        encoder.put(0, &b).unwrap();

        assert_eq!(&encoder.to_bytes(), &b"\x01\xfa\xc7"[..]);
    }

    #[test]
    fn test_encode_i8() {
        let mut encoder = TarsEncoder::new();
        let i0: i8 = -127;
        encoder.put(0, &i0).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x00\x81"[..]);

        let mut encoder = TarsEncoder::new();
        let i1: i8 = 127;
        encoder.put(14, &i1).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\xe0\x7f"[..]);

        let mut encoder = TarsEncoder::new();
        let i2: i8 = -1;
        encoder.put(255, &i2).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\xf0\xff\xff"[..]);

        let mut encoder = TarsEncoder::new();
        let i3: i8 = 0;
        encoder.put(3, &i3).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x3c"[..]);
    }

    #[test]
    fn test_encode_u8() {
        let mut encoder = TarsEncoder::new();
        let u0: u8 = 127;
        encoder.put(0, &u0).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x00\x7f"[..]);

        let mut encoder = TarsEncoder::new();
        let u1: u8 = 255;
        encoder.put(14, &u1).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\xe0\xff"[..]);

        let mut encoder = TarsEncoder::new();
        let u2: u8 = 0;
        encoder.put(255, &u2).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\xfc\xff"[..]);
    }

    #[test]
    fn test_encode_i16() {
        let mut encoder = TarsEncoder::new();
        let i0: i16 = -32768;
        encoder.put(0, &i0).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x01\x80\x00"[..]);

        let mut encoder = TarsEncoder::new();
        let i1: i16 = -127;
        encoder.put(15, &i1).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\xf0\x0f\x81"[..]);

        let mut encoder = TarsEncoder::new();
        let i2: i16 = 32767;
        encoder.put(19, &i2).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\xf1\x13\x7f\xff"[..]);
    }

    #[test]
    fn test_encode_u16() {
        let mut encoder = TarsEncoder::new();
        let i0: u16 = 32768;
        encoder.put(0, &i0).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x01\x80\x00"[..]);

        let mut encoder = TarsEncoder::new();
        let i1: u16 = 255;
        encoder.put(15, &i1).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\xf0\x0f\xff"[..]);

        let mut encoder = TarsEncoder::new();
        let i2: u16 = 65535;
        encoder.put(19, &i2).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\xf1\x13\xff\xff"[..]);
    }

    #[test]
    fn test_encode_i32() {
        let mut encoder = TarsEncoder::new();
        let i0: i32 = 90909;
        encoder.put(0, &i0).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x02\x00\x01\x63\x1d"[..]);

        let mut encoder = TarsEncoder::new();
        let i1: i32 = 255;
        encoder.put(15, &i1).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\xf1\x0f\x00\xff"[..]);

        let mut encoder = TarsEncoder::new();
        let i2: i32 = -127;
        encoder.put(14, &i2).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\xe0\x81"[..]);

        let mut encoder = TarsEncoder::new();
        let i3: i32 = -95234;
        encoder.put(14, &i3).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\xe2\xff\xfe\x8b\xfe"[..]);
    }

    #[test]
    fn test_encode_u32() {
        let mut encoder = TarsEncoder::new();
        let u0: u32 = 88888;
        encoder.put(0, &u0).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x02\x00\x01\x5b\x38"[..]);

        let mut encoder = TarsEncoder::new();
        let u1: u32 = 254;
        encoder.put(14, &u1).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\xe0\xfe"[..]);

        let mut encoder = TarsEncoder::new();
        let u2: u32 = 256;
        encoder.put(14, &u2).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\xe1\x01\x00"[..]);
    }

    #[test]
    fn test_encode_i64() {
        let mut encoder = TarsEncoder::new();
        let i0: i64 = -1;
        encoder.put(0, &i0).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x00\xff"[..]);

        let mut encoder = TarsEncoder::new();
        let i1: i64 = -129;
        encoder.put(0, &i1).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x01\xff\x7f"[..]);

        let mut encoder = TarsEncoder::new();
        let i2: i64 = -32769;
        encoder.put(0, &i2).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x02\xff\xff\x7f\xff"[..]);

        let mut encoder = TarsEncoder::new();
        let i3: i64 = -2147483649;
        encoder.put(0, &i3).unwrap();
        assert_eq!(
            &encoder.to_bytes(),
            &b"\x03\xff\xff\xff\xff\x7f\xff\xff\xff"[..]
        );
    }

    #[test]
    fn test_encode_u64() {
        let mut encoder = TarsEncoder::new();
        let u0: u64 = 255;
        encoder.put(0, &u0).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x00\xff"[..]);

        let mut encoder = TarsEncoder::new();
        let u1: u64 = 65535;
        encoder.put(0, &u1).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x01\xff\xff"[..]);

        let mut encoder = TarsEncoder::new();
        let u2: u64 = 4294967295;
        encoder.put(0, &u2).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x02\xff\xff\xff\xff"[..]);

        let mut encoder = TarsEncoder::new();
        let u3: u64 = 18446744073709551615;
        encoder.put(255, &u3).unwrap();
        assert_eq!(
            &encoder.to_bytes(),
            &b"\xf3\xff\xff\xff\xff\xff\xff\xff\xff\xff"[..]
        );
    }

    #[test]
    fn test_encode_f32() {
        let mut encoder = TarsEncoder::new();
        let f1: f32 = 0.1472;
        encoder.put(0, &f1).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x04\x3e\x16\xbb\x99"[..]);
    }

    #[test]
    fn test_encode_f64() {
        let mut encoder = TarsEncoder::new();
        let f1: f64 = 0.14723333;
        encoder.put(0, &f1).unwrap();
        assert_eq!(
            &encoder.to_bytes(),
            &b"\x05\x3f\xc2\xd8\x8a\xb0\x9d\x97\x2a"[..]
        );
    }

    #[test]
    fn test_encode_bool() {
        let mut encoder = TarsEncoder::new();
        encoder.put(0, &false).unwrap();
        encoder.put(1, &true).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x0c\x10\x01"[..]);
    }

    #[test]
    fn test_encode_string() {
        let mut encoder = TarsEncoder::new();
        let s: String = "hello wrold!".to_string();
        let expect_buf = "\x06\x0c".to_string() + &s;
        encoder.put(0, &s).unwrap();
        assert_eq!(&encoder.to_bytes(), &expect_buf);

        let mut encoder = TarsEncoder::new();
        let mut s1: String = String::new();
        for _ in 0..0xf7f7f {
            s1.push('z');
        }
        let expect_buf = "\x07\x00\x0f\x7f\x7f".to_string() + &s1;
        encoder.put(0, &s1).unwrap();
        assert_eq!(&encoder.to_bytes(), &expect_buf);
    }

    #[test]
    fn test_encode_map() {
        let mut map: BTreeMap<String, i32> = BTreeMap::new();
        map.insert("hello".to_string(), 32);
        map.insert("world".to_string(), 42);

        let mut encoder = TarsEncoder::new();
        encoder.put(0, &map).unwrap();
        assert_eq!(
            &encoder.to_bytes(),
            &b"\x08\0\0\0\x12\x06\x05hello\x00\x20\x06\x05world\x00\x2a"[..]
        );
    }

    #[test]
    fn test_encode_vec() {
        let mut v1: Vec<u8> = Vec::with_capacity(0xf7f7f);
        for _ in 0..0xf7f7f {
            v1.push(255);
        }
        let mut encoder = TarsEncoder::new();
        encoder.put(0, &v1).unwrap();
        let mut header_v = Vec::from(&b"\x0d\x00\x00\x0f\x7f\x7f"[..]);
        header_v.extend_from_slice(&v1);
        assert_eq!(&encoder.to_bytes(), &header_v);

        let mut v2: Vec<i8> = Vec::with_capacity(0xf7f7f);
        for _ in 0..0xf7f7f {
            v2.push(-127);
        }
        let mut encoder = TarsEncoder::new();
        encoder.put(0, &v2).unwrap();
        let mut header_v: Vec<u8> = Vec::from(&b"\x0d\x00\x00\x0f\x7f\x7f"[..]);
        header_v.extend_from_slice(unsafe { mem::transmute(&v2[..]) });
        assert_eq!(&encoder.to_bytes(), &header_v);

        let mut v3: Vec<bool> = Vec::with_capacity(0xf6f7f);
        let mut b = false;
        for _ in 0..0xf6f7f {
            v3.push(b);
            b = !b;
        }
        let mut encoder = TarsEncoder::new();
        encoder.put(0, &v3).unwrap();
        let mut header_v: Vec<u8> = Vec::from(&b"\x0d\x00\x00\x0f\x6f\x7f"[..]);
        header_v.extend_from_slice(unsafe { mem::transmute(&v3[..]) });
        assert_eq!(&encoder.to_bytes(), &header_v);

        let mut v4: Vec<String> = Vec::with_capacity(0xf6f7e);
        let str4 = "hello".repeat(128);
        let str1 = "hello".to_string();
        let times = 0xf6f7e / 2;
        for _ in 0..times {
            v4.push(str4.clone());
        }
        for _ in 0..times {
            v4.push(str1.clone());
        }

        let mut encoder = TarsEncoder::new();
        encoder.put(10, &v4).unwrap();
        let buf = encoder.to_bytes();
        assert_eq!(&buf[0..1], &b"\xa9"[..]);
        let len_in_u8: [u8; 4] = [buf[1], buf[2], buf[3], buf[4]];
        let len: u32 = u32::from_be(unsafe { mem::transmute(len_in_u8) });
        // (header len + string size + string in bytes)
        let expect_len = (1 + 4 + str4.len()) * times + (1 + 1 + str1.len()) * times;
        assert_eq!(len, expect_len as u32);
    }

    #[test]
    fn test_encode_bytes() {
        let b = Bytes::from(&b"hello world!"[..]);
        let mut encoder = TarsEncoder::new();
        encoder.put(9, &b).unwrap();
        assert_eq!(&encoder.to_bytes(), &b"\x9d\x00\x00\x00\x00\x0chello world!"[..]);
    }

    #[test]
    fn test_encode_struct() {
        #[derive(Clone, Debug, PartialEq)]
        struct TestStruct {
            a: i8,             // tag 0
            b: u16,            // tag 1
            v1: Vec<u8>,       // tag 2
            c: Option<String>, // tag 3 option
        }

        impl TestStruct {
            pub fn new() -> Self {
                TestStruct {
                    a: 0,
                    b: 0,
                    v1: vec![],
                    c: None,
                }
            }
        }

        impl EncodeIntoTars for TestStruct {
            fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
                encoder.put(0, &self.a)?;
                encoder.put(1, &self.b)?;
                encoder.put(2, &self.v1)?;
                encoder.put(3, &self.c)?;
                Ok(())
            }
        }

        let mut s = TestStruct::new();

        let mut encoder = TarsEncoder::new();
        s.encode_into(&mut encoder).unwrap();
        assert_eq!(
            &encoder.to_bytes(),
            &b"\x0c\x1c\x2d\x00\x00\x00\x00\x00"[..]
        );

        let mut encoder = TarsEncoder::new();
        s.a = -1;
        s.b = 65535;
        s.v1.push(255);
        s.v1.push(0);
        s.c = Some("hello".to_string());
        s.encode_into(&mut encoder).unwrap();
        assert_eq!(
            &encoder.to_bytes(),
            &b"\x00\xff\x11\xff\xff\x2d\x00\x00\x00\x00\x02\xff\x00\x36\x05hello"[..]
        );

        #[derive(Clone, Debug, PartialEq)]
        struct TestStruct2 {
            f: f32,                      // 0
            s: TestStruct,               // 1
            m: BTreeMap<String, String>, // 2
            s2: TestStruct,              // 3
            y: Option<u8>,               // 4 option
        }

        impl TestStruct2 {
            pub fn new() -> Self {
                TestStruct2 {
                    f: 0.0,
                    s: TestStruct::new(),
                    m: BTreeMap::new(),
                    s2: TestStruct::new(),
                    y: None,
                }
            }
        }

        impl EncodeIntoTars for TestStruct2 {
            fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
                encoder.put(0, &self.f)?;
                encoder.put(1, &self.s)?;
                encoder.put(2, &self.m)?;
                encoder.put(3, &self.s2)?;
                encoder.put(4, &self.y)?;
                Ok(())
            }
        }

        let t2 = TestStruct2::new();
        let mut encoder = TarsEncoder::new();
        t2.encode_into(&mut encoder).unwrap();
        assert_eq!(
                &encoder.to_bytes(),
                &b"\x04\x00\x00\x00\x00\x1a\x0c\x1c\x2d\x00\x00\x00\x00\x00\x0b\x28\x00\x00\x00\x00\x3a\x0c\x1c\x2d\x00\x00\x00\x00\x00\x0b"[..]
            );
    }
}
