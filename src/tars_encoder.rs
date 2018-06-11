use bytes::{BufMut, Bytes, BytesMut};
use errors::EncodeErr;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::mem;
use tars_type::is_simple_list_element;
use tars_type::TarsTypeMark::*;
use tars_type::*;
const MAX_HEADER_LEN: usize = 2;
const MAX_SIZE_LEN: usize = 4;

#[derive(Debug, Clone)]
pub struct TarsEncoder {
    buf: BytesMut,
    need_remove_struct_tag: bool,
}

impl TarsEncoder {
    pub fn new() -> Self {
        TarsEncoder {
            buf: BytesMut::new(),
            need_remove_struct_tag: false,
        }
    }
    pub fn to_bytes(self) -> Bytes {
        self.buf.freeze()
    }
}

fn check_maybe_resize(buf: &mut BytesMut, len: usize) {
    if buf.remaining_mut() < len {
        let new_len = buf.remaining_mut() + len + 1;
        buf.reserve(new_len)
    }
}

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

pub fn write_struct<T: EncodeTo>(tag: u8, buf: &mut BytesMut, s: &T) -> Result<(), EncodeErr> {
    put_head(buf, tag, EnStructBegin)?;
    s.encode_into_bytes(tag, buf)?;
    put_head(buf, 0, EnStructEnd)?;
    Ok(())
}

pub trait EncodeTo {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr>;

    #[allow(unused_variables)]
    fn encode_raw_bytes(&self, buf: &mut BytesMut) {
        unimplemented!() // only implement by i8 u8 bool
    }
}

impl EncodeTo for i8 {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        if *self == 0 {
            check_maybe_resize(buf, MAX_HEADER_LEN);
            put_head(buf, tag, EnZero)?;
        } else {
            check_maybe_resize(buf, MAX_HEADER_LEN + mem::size_of::<i8>());
            put_head(buf, tag, EnInt8)?;
            buf.put_i8(*self);
        }
        Ok(())
    }

    fn encode_raw_bytes(&self, buf: &mut BytesMut) {
        buf.put_i8(*self)
    }
}

impl EncodeTo for u8 {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        if *self == 0 {
            check_maybe_resize(buf, MAX_HEADER_LEN);
            put_head(buf, tag, EnZero)?;
        } else {
            check_maybe_resize(buf, MAX_HEADER_LEN + mem::size_of::<u8>());
            put_head(buf, tag, EnInt8)?;
            buf.put_u8(*self);
        }
        Ok(())
    }

    fn encode_raw_bytes(&self, buf: &mut BytesMut) {
        buf.put_u8(*self)
    }
}

impl EncodeTo for i16 {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        if *self >= i16::from(i8::min_value()) && *self <= i16::from(i8::max_value()) {
            (*self as i8).encode_into_bytes(tag, buf)?;
        } else {
            check_maybe_resize(buf, MAX_HEADER_LEN + mem::size_of::<i16>());
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
            check_maybe_resize(buf, MAX_HEADER_LEN + mem::size_of::<u16>());
            put_head(buf, tag, EnInt16)?;
            buf.put_u16_be(*self);
        }
        Ok(())
    }
}

impl EncodeTo for i32 {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        if *self >= i32::from(i16::min_value()) && *self <= i32::from(i16::max_value()) {
            (*self as i16).encode_into_bytes(tag, buf)?;
        } else {
            check_maybe_resize(buf, MAX_HEADER_LEN + mem::size_of::<i32>());
            put_head(buf, tag, EnInt32)?;
            buf.put_i32_be(*self);
        }
        Ok(())
    }
}

impl EncodeTo for u32 {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        if *self >= u32::from(u16::min_value()) && *self <= u32::from(u16::max_value()) {
            (*self as u16).encode_into_bytes(tag, buf)?;
        } else {
            check_maybe_resize(buf, MAX_HEADER_LEN + mem::size_of::<u32>());
            put_head(buf, tag, EnInt32)?;
            buf.put_u32_be(*self);
        }
        Ok(())
    }
}

impl EncodeTo for i64 {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        if *self >= i64::from(i32::min_value()) && *self <= i64::from(i32::max_value()) {
            (*self as i32).encode_into_bytes(tag, buf)?;
        } else {
            check_maybe_resize(buf, MAX_HEADER_LEN + mem::size_of::<i64>());
            put_head(buf, tag, EnInt64)?;
            buf.put_i64_be(*self);
        }
        Ok(())
    }
}

impl EncodeTo for u64 {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        if *self >= u64::from(u32::min_value()) && *self <= u64::from(u32::max_value()) {
            (*self as u32).encode_into_bytes(tag, buf)?;
        } else {
            check_maybe_resize(buf, MAX_HEADER_LEN + mem::size_of::<u64>());
            put_head(buf, tag, EnInt64)?;
            buf.put_u64_be(*self);
        }
        Ok(())
    }
}

impl EncodeTo for f32 {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        check_maybe_resize(buf, MAX_HEADER_LEN + mem::size_of::<f32>());
        put_head(buf, tag, EnFloat)?;
        buf.put_f32_be(*self);
        Ok(())
    }
}

impl EncodeTo for f64 {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        check_maybe_resize(buf, MAX_HEADER_LEN + mem::size_of::<f64>());
        put_head(buf, tag, EnDouble)?;
        buf.put_f64_be(*self);
        Ok(())
    }
}

impl EncodeTo for bool {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        let value: u8 = if *self { 1 } else { 0 };
        value.encode_into_bytes(tag, buf)
    }

    fn encode_raw_bytes(&self, buf: &mut BytesMut) {
        let value: u8 = if *self { 1 } else { 0 };
        value.encode_raw_bytes(buf);
    }
}

impl EncodeTo for String {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        let len = self.len();
        check_maybe_resize(buf, MAX_HEADER_LEN + MAX_SIZE_LEN + len);
        if len <= usize::from(u8::max_value()) {
            put_head(buf, tag, EnString1)?;
            match u8::try_from(len) {
                Ok(l) => {
                    buf.put_u8(l);
                    buf.put(self);
                    Ok(())
                }
                Err(_) => Err(EncodeErr::ConvertU8Err),
            }
        } else if len <= u32::max_value() as usize {
            put_head(buf, tag, EnString4)?;
            buf.put_u32_be(len as u32);
            buf.put(self);
            Ok(())
        } else {
            Err(EncodeErr::BufferTooBigErr)
        }
    }
}

impl<K: EncodeTo + Ord, V: EncodeTo> EncodeTo for BTreeMap<K, V> {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        let mut inner_bytes = BytesMut::new();
        for (key, value) in self.iter() {
            key.encode_into_bytes(0, &mut inner_bytes)?;
            value.encode_into_bytes(0, &mut inner_bytes)?;
        }

        if inner_bytes.len() > u32::max_value() as usize {
            Err(EncodeErr::BufferTooBigErr)
        } else {
            check_maybe_resize(buf, inner_bytes.len() + MAX_HEADER_LEN + MAX_SIZE_LEN);
            put_head(buf, tag, EnMaps)?;
            buf.put_u32_be(inner_bytes.len() as u32);
            if inner_bytes.len() > 0 {
                buf.unsplit(inner_bytes);
            }
            Ok(())
        }
    }
}

impl<T> EncodeTo for Vec<T>
where
    T: 'static + EncodeTo,
{
    default fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        let mut inner_bytes = BytesMut::new();
        for ele in self.into_iter() {
            ele.encode_into_bytes(0, &mut inner_bytes)?;
        }
        if inner_bytes.len() > u32::max_value() as usize {
            Err(EncodeErr::BufferTooBigErr)
        } else {
            check_maybe_resize(buf, inner_bytes.len() + MAX_HEADER_LEN + MAX_SIZE_LEN);
            put_head(buf, tag, EnList)?;
            buf.put_u32_be(inner_bytes.len() as u32);
            if inner_bytes.len() > 0 {
                buf.unsplit(inner_bytes);
            }
            Ok(())
        }
    }
}

impl EncodeTo for Vec<u8> {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        if self.len() > u32::max_value() as usize {
            Err(EncodeErr::BufferTooBigErr)
        } else {
            buf.reserve(2 * MAX_HEADER_LEN + MAX_SIZE_LEN + self.len());
            put_head(buf, tag, EnSimplelist)?;
            put_head(buf, 0, EnInt8)?;
            buf.put_u32_be(self.len() as u32);
            for ele in self.into_iter() {
                ele.encode_raw_bytes(buf);
            }
            Ok(())
        }
    }
}

impl EncodeTo for Vec<i8> {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        if self.len() > u32::max_value() as usize {
            Err(EncodeErr::BufferTooBigErr)
        } else {
            buf.reserve(2 * MAX_HEADER_LEN + MAX_SIZE_LEN + self.len());
            put_head(buf, tag, EnSimplelist)?;
            put_head(buf, 0, EnInt8)?;
            buf.put_u32_be(self.len() as u32);
            for ele in self.into_iter() {
                ele.encode_raw_bytes(buf);
            }
            Ok(())
        }
    }
}

impl EncodeTo for Vec<bool> {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        if self.len() > u32::max_value() as usize {
            Err(EncodeErr::BufferTooBigErr)
        } else {
            buf.reserve(2 * MAX_HEADER_LEN + MAX_SIZE_LEN + self.len());
            put_head(buf, tag, EnSimplelist)?;
            put_head(buf, 0, EnInt8)?;
            buf.put_u32_be(self.len() as u32);
            for ele in self.into_iter() {
                ele.encode_raw_bytes(buf);
            }
            Ok(())
        }
    }
}

impl<T: EncodeTo> EncodeTo for Option<T> {
    fn encode_into_bytes(&self, tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        match self {
            Some(ele) => ele.encode_into_bytes(tag, buf),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // #[test]
    // fn test_put() {
    //     let i: u8 = 1;
    //     let mut en = TarsEncoder::new();
    //     en.put(0, &i);
    // }

    #[test]
    fn test_put_head() {
        let mut buf = BytesMut::new();
        put_head(&mut buf, 19, EnInt8).unwrap();
        put_head(&mut buf, 0, EnInt16).unwrap();

        assert_eq!(&buf[..], &b"\xf0\x13\x01"[..]);
    }

    #[test]
    fn test_encode_option() {
        let mut buf = BytesMut::new();
        let a: Option<i64> = Some(-1337);
        let b: Option<i64> = None;
        a.encode_into_bytes(0, &mut buf).unwrap();
        b.encode_into_bytes(1, &mut buf).unwrap();

        assert_eq!(&buf[..], &b"\x01\xfa\xc7"[..]);
    }

    #[test]
    fn test_encode_i8() {
        let mut buf = BytesMut::new();
        let i0: i8 = -127;
        i0.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x00\x81"[..]);

        let mut buf = BytesMut::new();
        let i1: i8 = 127;
        i1.encode_into_bytes(14, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\xe0\x7f"[..]);

        let mut buf = BytesMut::new();
        let i2: i8 = -1;
        i2.encode_into_bytes(255, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\xf0\xff\xff"[..]);

        let mut buf = BytesMut::new();
        let i3: i8 = 0;
        i3.encode_into_bytes(3, &mut buf).unwrap();
        print!("{:?}", buf);
        assert_eq!(&buf[..], &b"\x3c"[..]);
    }

    #[test]
    fn test_encode_u8() {
        let mut buf = BytesMut::new();
        let u0: u8 = 127;
        u0.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x00\x7f"[..]);

        let mut buf = BytesMut::new();
        let u1: u8 = 255;
        u1.encode_into_bytes(14, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\xe0\xff"[..]);

        let mut buf = BytesMut::new();
        let u2: u8 = 0;
        u2.encode_into_bytes(255, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\xfc\xff"[..]);
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

    #[test]
    fn test_encode_u16() {
        let mut buf = BytesMut::new();
        let i0: u16 = 32768;
        i0.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x01\x80\x00"[..]);

        let mut buf = BytesMut::new();
        let i1: u16 = 255;
        i1.encode_into_bytes(15, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\xf0\x0f\xff"[..]);

        let mut buf = BytesMut::new();
        let i2: u16 = 65535;
        i2.encode_into_bytes(19, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\xf1\x13\xff\xff"[..]);
    }

    #[test]
    fn test_encode_i32() {
        let mut buf = BytesMut::new();
        let i0: i32 = 90909;
        i0.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x02\x00\x01\x63\x1d"[..]);

        let mut buf = BytesMut::new();
        let i1: i32 = 255;
        i1.encode_into_bytes(15, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\xf1\x0f\x00\xff"[..]);

        let mut buf = BytesMut::new();
        let i2: i32 = -127;
        i2.encode_into_bytes(14, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\xe0\x81"[..]);

        let mut buf = BytesMut::new();
        let i3: i32 = -95234;
        i3.encode_into_bytes(14, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\xe2\xff\xfe\x8b\xfe"[..]);
    }

    #[test]
    fn test_encode_u32() {
        let mut buf = BytesMut::new();
        let u0: u32 = 88888;
        u0.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x02\x00\x01\x5b\x38"[..]);

        let mut buf = BytesMut::new();
        let u0: u32 = 254;
        u0.encode_into_bytes(14, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\xe0\xfe"[..]);

        let mut buf = BytesMut::new();
        let u0: u32 = 256;
        u0.encode_into_bytes(14, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\xe1\x01\x00"[..]);
    }

    #[test]
    fn test_encode_i64() {
        let mut buf = BytesMut::new();
        let i0: i64 = -1;
        i0.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x00\xff"[..]);

        let mut buf = BytesMut::new();
        let i1: i64 = -129;
        i1.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x01\xff\x7f"[..]);

        let mut buf = BytesMut::new();
        let i2: i64 = -32769;
        i2.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x02\xff\xff\x7f\xff"[..]);

        let mut buf = BytesMut::new();
        let i3: i64 = -2147483649;
        i3.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x03\xff\xff\xff\xff\x7f\xff\xff\xff"[..]);
    }

    #[test]
    fn test_encode_u64() {
        let mut buf = BytesMut::new();
        let u0: u64 = 255;
        u0.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x00\xff"[..]);

        let mut buf = BytesMut::new();
        let u1: u64 = 65535;
        u1.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x01\xff\xff"[..]);

        let mut buf = BytesMut::new();
        let u2: u64 = 4294967295;
        u2.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x02\xff\xff\xff\xff"[..]);

        let mut buf = BytesMut::new();
        let u3: u64 = 18446744073709551615;
        u3.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf[..], &b"\x03\xff\xff\xff\xff\xff\xff\xff\xff"[..]);
    }

    #[test]
    fn test_encode_f32() {
        let mut buf = BytesMut::new();
        let f1: f32 = 0.1472;
        f1.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf, &b"\x04\x3e\x16\xbb\x99"[..]);
    }

    #[test]
    fn test_encode_f64() {
        let mut buf = BytesMut::new();
        let f1: f64 = 0.14723333;
        f1.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf, &b"\x05\x3f\xc2\xd8\x8a\xb0\x9d\x97\x2a"[..]);
    }

    #[test]
    fn test_encode_bool() {
        let mut buf = BytesMut::new();
        false.encode_into_bytes(0, &mut buf).unwrap();
        true.encode_into_bytes(1, &mut buf).unwrap();
        assert_eq!(&buf, &b"\x0c\x10\x01"[..]);
    }

    #[test]
    fn test_encode_string() {
        let mut buf = BytesMut::new();
        let s: String = "hello wrold!".to_string();
        let expect_buf = "\x06\x0c".to_string() + &s;
        s.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf, &expect_buf);

        let mut buf = BytesMut::new();
        let mut s1: String = String::new();
        for _ in 0..0xf7f7f {
            s1.push('z');
        }
        let expect_buf = "\x07\x00\x0f\x7f\x7f".to_string() + &s1;
        s1.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf, &expect_buf);
    }

    #[test]
    fn test_encode_map() {
        let mut map: BTreeMap<String, i32> = BTreeMap::new();
        map.insert("hello".to_string(), 32);
        map.insert("world".to_string(), 42);

        let mut buf = BytesMut::new();
        map.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(
            &buf,
            &b"\x08\0\0\0\x12\x06\x05hello\x00\x20\x06\x05world\x00\x2a"[..]
        );
    }

    #[test]
    fn test_encode_vec() {
        let mut v1: Vec<u8> = Vec::with_capacity(0xf7f7f);
        for _ in 0..0xf7f7f {
            v1.push(255);
        }
        let mut buf = BytesMut::new();
        v1.encode_into_bytes(0, &mut buf).unwrap();
        let mut header_v = Vec::from(&b"\x0d\x00\x00\x0f\x7f\x7f"[..]);
        header_v.extend_from_slice(&v1);
        assert_eq!(&buf, &header_v);

        let mut v2: Vec<i8> = Vec::with_capacity(0xf7f7f);
        for _ in 0..0xf7f7f {
            v2.push(-127);
        }
        let mut buf = BytesMut::new();
        v2.encode_into_bytes(0, &mut buf).unwrap();
        let mut header_v: Vec<u8> = Vec::from(&b"\x0d\x00\x00\x0f\x7f\x7f"[..]);
        header_v.extend_from_slice(unsafe { mem::transmute(&v2[..]) });
        assert_eq!(&buf, &header_v);

        let mut v3: Vec<bool> = Vec::with_capacity(0xf6f7f);
        let mut b = false;
        for _ in 0..0xf6f7f {
            v3.push(b);
            b = !b;
        }
        let mut buf = BytesMut::new();
        v3.encode_into_bytes(0, &mut buf).unwrap();
        let mut header_v: Vec<u8> = Vec::from(&b"\x0d\x00\x00\x0f\x6f\x7f"[..]);
        header_v.extend_from_slice(unsafe { mem::transmute(&v3[..]) });
        assert_eq!(&buf, &header_v);

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

        let mut buf = BytesMut::new();
        v4.encode_into_bytes(10, &mut buf).unwrap();

        assert_eq!(&buf[0..1], &b"\xa9"[..]);
        let len_in_u8: [u8; 4] = [buf[1], buf[2], buf[3], buf[4]];
        let len: u32 = u32::from_be(unsafe { mem::transmute(len_in_u8) });
        // (header len + string size + string in bytes)
        let expect_len = (1 + 4 + str4.len()) * times + (1 + 1 + str1.len()) * times;
        assert_eq!(len, expect_len as u32);
    }

    #[test]
    fn test_encode_struct() {
        #[derive(Clone, Debug)]
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

        impl EncodeTo for TestStruct {
            fn encode_into_bytes(&self, _tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
                self.a.encode_into_bytes(0, buf)?;
                self.b.encode_into_bytes(1, buf)?;
                self.v1.encode_into_bytes(2, buf)?;
                self.c.encode_into_bytes(3, buf)?;
                Ok(())
            }
        }

        let mut s = TestStruct::new();

        let mut buf = BytesMut::new();
        s.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(&buf, &b"\x0c\x1c\x2d\x00\x00\x00\x00\x00"[..]);

        let mut buf = BytesMut::new();
        s.a = -1;
        s.b = 65535;
        s.v1.push(255);
        s.v1.push(0);
        s.c = Some("hello".to_string());
        s.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(
            &buf,
            &b"\x00\xff\x11\xff\xff\x2d\x00\x00\x00\x00\x02\xff\x00\x36\x05hello"[..]
        );

        #[derive(Clone, Debug)]
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

        impl EncodeTo for TestStruct2 {
            fn encode_into_bytes(&self, _tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
                self.f.encode_into_bytes(0, buf)?;
                write_struct(1, buf, &self.s)?;
                self.m.encode_into_bytes(2, buf)?;
                write_struct(3, buf, &self.s2)?;
                self.y.encode_into_bytes(4, buf)?;
                Ok(())
            }
        }

        let t2 = TestStruct2::new();
        let mut buf = BytesMut::new();
        t2.encode_into_bytes(0, &mut buf).unwrap();
        assert_eq!(
            &buf,
            &b"\x04\x00\x00\x00\x00\x1a\x0c\x1c\x2d\x00\x00\x00\x00\x00\x0b\x28\x00\x00\x00\x00\x3a\x0c\x1c\x2d\x00\x00\x00\x00\x00\x0b"[..]
        );
    }
}
